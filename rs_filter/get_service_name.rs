use indexmap::IndexMap;
use log::trace;
use petgraph::Graph;
use petgraph::Incoming;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::convert::TryFrom;
use std::time::Duration;

use utils::graph::iso::find_mapping_shamir_centralized;
use utils::graph::serde::FerriedData;
use utils::graph::serde::Property;
use utils::graph::utils::generate_target_graph;
use utils::graph::utils::get_node_with_id;
use utils::misc::headers::*;

fn fetch_data_from_headers(ctx: &HttpHeaders, request_type: HttpType) -> FerriedData {
    let data_str_opt: Option<String>;
    if request_type == HttpType::Request {
        data_str_opt = ctx.get_http_request_header("ferried_data");
    } else if request_type == HttpType::Response {
        data_str_opt = ctx.get_http_response_header("ferried_data");
    } else {
        log::error!("Unsupported http type {:?}", request_type);
        return FerriedData::default();
    }
    if let Some(ferried_data_str) = data_str_opt {
        match serde_json::from_str(&ferried_data_str) {
            Ok(fd) => {
                log::warn!("Successfully parsed ferried_data from header.");
                return fd;
            }
            Err(e) => {
                log::error!("Could not translate stored data to json string: {0}\n", e);
            }
        }
    }
    return FerriedData::default();
}

fn fetch_property(node_name: &str, prop_query: &Vec<&str>, ctx: &HttpHeaders) -> Option<Property> {
    // Insert properties to collect
    let prop_tuple;
    let property_str: String;
    // Seems like we need a copy here, a little bit annoying
    if let Some(property) = ctx.get_property(prop_query.to_vec()) {
        match String::from_utf8(property) {
            Ok(cast_string) => property_str = cast_string,
            Err(_e) => {
                log::error!("Failed to serialize property: {:?}\n", prop_query);
                return None;
            }
        }
    } else {
        log::error!("Failed to retrieve property: {:?}\n", prop_query);
        return None;
    }
    //FIXME Adjust the format of this property
    prop_tuple = Property::new(
        node_name.to_string(),
        join_str(prop_query),
        property_str.clone(),
    );
    return Some(prop_tuple);
}

fn get_shared_data(trace_id: &str, ctx: &HttpHeaders) -> Option<FerriedData> {
    let mut stored_data: FerriedData = FerriedData::default();
    if let (Some(data), _) = ctx.get_shared_data(&trace_id) {
        let msg = format!("Unable to convert {:?} into a string ", data);
        // Add a header on the response.
        // FIXME: There must be  a nicer way to resolve this
        match String::from_utf8(data) {
            Ok(cast_string) => match serde_json::from_str(&cast_string) {
                Ok(d) => {
                    stored_data = d;
                }
                Err(e) => {
                    log::error!("Could not parse envoy shared data: {0}\n", e);
                    return None;
                }
            },
            Err(_e) => log::error!("{}", msg),
        }
    } else {
        log::warn!("Trace key {:?} not found in shared data.", trace_id);
    }
    return Some(stored_data);
}

fn store_data(stored_data_str: &String, trace_id: &str, ctx: &HttpHeaders) {
    let stored_data_bytes = Some(stored_data_str.as_bytes());
    let store_result = ctx.set_shared_data(&trace_id, stored_data_bytes, None);
    if let Err(ref e) = store_result {
        log::error!(
            "Failed to store key {:?} and value {:?}: {:?}",
            trace_id,
            store_result,
            e
        );
    }
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpHeadersRoot {
            target_graph: create_target_graph(),
        })
    });
}

struct HttpHeadersRoot {
    target_graph: Graph<(String, IndexMap<String, String>), String>,
}

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn on_configure(&mut self, _: usize) -> bool {
        true
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        let workload_name: String;
        if let Some(workload) = self.get_property(vec!["node", "metadata", "WORKLOAD_NAME"]) {
            match String::from_utf8(workload) {
                Ok(cast_string) => workload_name = cast_string,
                Err(_e) => workload_name = String::new(),
            }
        } else {
            workload_name = String::new();
        }
        Some(Box::new(HttpHeaders {
            context_id,
            workload_name,
            // FIXME: This should be a reference instead of a copy
            // Extremely annoying but I can not guarantee a life-time here
            target_graph: self.target_graph.clone(),
        }))
    }
}

struct HttpHeaders {
    context_id: u32,
    workload_name: String,
    target_graph: Graph<(String, IndexMap<String, String>), String>,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, num_headers: usize) -> Action {
        let direction = self.get_traffic_direction();
        log::warn!(
            "{}: Request Header Direction {}",
            self.workload_name,
            direction
        );
        self.print_headers(HttpType::Request);
        if direction == TrafficDirection::Inbound {
            self.on_http_request_headers_inbound(num_headers);
        } else if direction == TrafficDirection::Outbound {
            self.on_http_request_headers_outbound(num_headers);
        } else {
            log::error!("Unknown request direction!");
        }
        Action::Continue
    }

    fn on_http_response_headers(&mut self, num_headers: usize) -> Action {
        let direction = self.get_traffic_direction();
        log::warn!(
            "{}: Response Header Direction {}",
            self.workload_name,
            direction
        );
        self.print_headers(HttpType::Response);

        if direction == TrafficDirection::Inbound {
            self.on_http_response_headers_inbound(num_headers);
        } else if direction == TrafficDirection::Outbound {
            self.on_http_response_headers_outbound(num_headers);
        } else {
            log::error!("Unknown request direction!");
        }
        Action::Continue
    }

    fn on_log(&mut self) {
        trace!("#{} completed.", self.context_id);
    }
}

impl HttpHeaders {
    // Retrieves the traffic direction from the configuration context.
    fn get_traffic_direction(&self) -> TrafficDirection {
        if let Some(direction_bytes) = self.get_property(vec!["listener_direction"]) {
            let cast_bytes = <[u8; 8]>::try_from(direction_bytes);
            match cast_bytes {
                Ok(byte_array) => return i64::from_ne_bytes(byte_array).into(),
                Err(_e) => return 0i64.into(),
            }
        }
        return 0i64.into();
    }

    fn print_headers(&self, request_type: HttpType) {
        if request_type == HttpType::Request {
            for (name, value) in &self.get_http_request_headers() {
                log::warn!("#{} -> {}: {}", self.context_id, name, value);
            }
        } else if request_type == HttpType::Response {
            for (name, value) in &self.get_http_response_headers() {
                log::warn!("#{} -> {}: {}", self.context_id, name, value);
            }
        } else {
            log::error!("Unsupported http type {:?}", request_type);
        }
    }

    fn on_http_request_headers_inbound(&mut self, _num_headers: usize) {
        let trace_id: String;
        if let Some(trace_id_) = self.get_http_request_header("x-request-id") {
            trace_id = trace_id_;
            log::warn!("Request inbound: Using trace id {}!", trace_id);
        } else {
            log::error!("Request inbound: x-request-id not found in header!",);
            return;
        }

        // Fetch ferried data
        let mut ferried_data = fetch_data_from_headers(self, HttpType::Request);

        // Handle the properties
        let prop_option_0 = fetch_property(
            &self.workload_name,
            &vec!["node", "metadata", "WORKLOAD_NAME"],
            self,
        );
        if let Some(prop_tuple) = prop_option_0 {
            ferried_data.unassigned_properties.push(prop_tuple);
        } else {
            // We failed to collect the property.
            // This might lead to wrong results, abort
            return;
        }

        // Retrieve the data we have stored
        let stored_data_opt = get_shared_data(&trace_id, self);
        if stored_data_opt.is_none() {
            // We failed to deserialize the shared data.
            // This might lead to wrong results, abort.
            return;
        }
        // Unpack the data we have
        let mut stored_data = stored_data_opt.unwrap();
        // Merge with the new information that is carried forward
        stored_data.merge(ferried_data);
        // Now store the data again after we have merged it
        // First, get a string
        let stored_data_str_opt = data_to_str(&stored_data);
        if stored_data_str_opt.is_none() {
            // We failed to serialize the shared data.
            // This might lead to wrong results, abort.
            return;
        }
        // Unpack the data we have
        let stored_data_str = stored_data_str_opt.unwrap();
        store_data(&stored_data_str, &trace_id, self);
    }

    fn on_http_request_headers_outbound(&mut self, _num_headers: usize) {
        let trace_id: String;
        if let Some(trace_id_) = self.get_http_request_header("x-request-id") {
            trace_id = trace_id_;
            log::warn!("Request outbound: Using trace id {}!", trace_id);
        } else {
            log::error!("Request outbound: x-request-id not found in header!",);
            return;
        }

        // TODO: Okay this does nothing in the original
        // I am not sure about the logic here
        // There may be a bug in the original code

        // // Retrieve the data we have stored
        // let stored_data_opt = get_shared_data(&trace_id, self);

        // if stored_data_opt.is_none() {
        //     // We failed to parse the shared data, this might lead to wrong results
        //     // Abort
        //     return;
        // }
        // // Unpack the data we have
        // let stored_data = stored_data_opt.unwrap();

        // // Now store the data again after we have merged it
        // let stored_data_str_opt = data_to_str(&stored_data);
        // if stored_data_str_opt.is_none() {
        //     // We failed to serialize the shared data.
        //     // This might lead to wrong results, abort.
        //     return;
        // }
        // // Unpack the data we have
        // let stored_data_str = stored_data_str_opt.unwrap();
        // // Set the header
        // log::warn!("Attaching {:?}", stored_data_str);
        // self.set_http_request_header("ferried_data", Some(&stored_data_str));
    }

    fn on_http_response_headers_inbound(&mut self, _num_headers: usize) {
        let trace_id: String;
        if let Some(trace_id_) = self.get_http_response_header("x-request-id") {
            trace_id = trace_id_;
            log::warn!("Response inbound: Using trace id {}!", trace_id);
        } else {
            log::error!("Response inbound: x-request-id not found in header!",);
            return;
        }
        // TODO:  Do not really understand the purpose of this yet
        let mut my_indexmap = IndexMap::new();
        my_indexmap.insert(
            join_str(&vec!["node", "metadata", "WORKLOAD_NAME"]),
            self.workload_name.clone(),
        );

        // Retrieve the data we have stored
        let stored_data_opt = get_shared_data(&trace_id, self);
        if stored_data_opt.is_none() {
            // We failed to deserialize the shared data.
            // This might lead to wrong results, abort.
            return;
        }
        // Unpack the data we have
        let mut stored_data = stored_data_opt.unwrap();

        // Figure out what needs to be done here
        // Also handle case where stored data is fresh?
        // TODO: Make this a function? What is this?
        let mut previous_roots = Vec::new();
        for node in stored_data.trace_graph.node_indices() {
            if stored_data
                .trace_graph
                .neighbors_directed(node, Incoming)
                .count()
                == 0
            {
                previous_roots.push(node);
            }
        }
        let me = stored_data
            .trace_graph
            .add_node((self.workload_name.clone(), my_indexmap));

        for previous_root in previous_roots {
            stored_data
                .trace_graph
                .add_edge(me, previous_root, String::new());
        }
        stored_data.assign_properties();

        // if we are not the root id, return
        // TODO:: Add some diagnostic when we are not the root node
        if self.workload_name == "productpage-v1" {
            // 2. calculate UDFs and store result, and check trace level properties

            let mapping_opt =
                find_mapping_shamir_centralized(&stored_data.trace_graph, &self.target_graph);
            if let Some(mapping) = mapping_opt {
                let node_ptr = get_node_with_id(&self.target_graph, "a".to_string());
                if node_ptr.is_none() {
                    log::error!("Node a not found");
                    // TODO: This should not abort I believe
                    return;
                }
                let mut trace_node_index = None;
                for map in mapping {
                    if self.target_graph.node_weight(map.0).unwrap().0 == "a" {
                        trace_node_index = Some(map.1);
                        break;
                    }
                }
                // TODO: This looks odd, further cleaning required.
                let contains_key = stored_data
                    .trace_graph
                    .node_weight(trace_node_index.unwrap())
                    .unwrap()
                    .1
                    .contains_key(&join_str(&vec!["node", "metadata", "WORKLOAD_NAME"]));
                if trace_node_index == None || !contains_key {
                    // we have not yet collected the return property or have a mapping error
                    log::error!("Mapping error.");
                    // TODO: This should not abort I believe
                    return;
                }
                let ret_service_name = &stored_data
                    .trace_graph
                    .node_weight(trace_node_index.unwrap())
                    .unwrap()
                    .1[&join_str(&vec!["node", "metadata", "WORKLOAD_NAME"])];

                let key = join_str(&vec!["node", "metadata", "WORKLOAD_NAME"]);
                let value = ret_service_name.to_string();

                let call_result = self.dispatch_http_call(
                    "storage-upstream",
                    vec![
                        (":method", "GET"),
                        (":path", "/store"),
                        (":authority", "storage-upstream"),
                        ("key", &key),
                        ("value", &value),
                    ],
                    None,
                    vec![],
                    Duration::from_secs(5),
                );
                if let Err(e) = call_result {
                    log::error!("Failed to make a call to storage {:?}", e);
                }
            } else {
                log::error!("Mapping not found");
            }
        } else {
            log::warn!(
                "Node {:?} is not the expected root node {:?}",
                self.workload_name,
                "productpage-v1"
            );
        }

        // Now store the data again after we have computed over it
        let stored_data_str_opt = data_to_str(&stored_data);
        if stored_data_str_opt.is_none() {
            // We failed to serialize the shared data.
            // This might lead to wrong results, abort.
            return;
        }
        // Unpack the data we have
        let stored_data_str = stored_data_str_opt.unwrap();
        // Set the header
        log::warn!("Attaching {:?}", stored_data_str);
        self.set_http_response_header("ferried_data", Some(&stored_data_str));
    }

    fn on_http_response_headers_outbound(&mut self, _num_headers: usize) {
        let trace_id: String;
        if let Some(trace_id_) = self.get_http_response_header("x-request-id") {
            trace_id = trace_id_;
            log::warn!("Response outbound: Using trace id {}!", trace_id);
        } else {
            log::error!("Response outbound: x-request-id not found in header!",);
            return;
        }
        // Fetch ferried data
        let ferried_data = fetch_data_from_headers(self, HttpType::Response);

        // First, get a string
        let ferried_data_str_opt = data_to_str(&ferried_data);
        if ferried_data_str_opt.is_none() {
            // We failed to serialize the shared data.
            // This might lead to wrong results, abort.
            return;
        }
        // Unpack the data we have
        let ferried_data_str = ferried_data_str_opt.unwrap();
        store_data(&ferried_data_str, &trace_id, self);
    }
}

pub fn create_target_graph() -> Graph<
    (
        std::string::String,
        IndexMap<std::string::String, std::string::String>,
    ),
    std::string::String,
> {
    let vertices = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "c".to_string()),
    ];
    let mut ids_to_properties: IndexMap<String, IndexMap<String, String>> = IndexMap::new();
    ids_to_properties.insert("a".to_string(), IndexMap::new());
    ids_to_properties.insert("b".to_string(), IndexMap::new());
    ids_to_properties.insert("c".to_string(), IndexMap::new());
    let b_hashmap = ids_to_properties.get_mut("b").unwrap();
    b_hashmap.insert("service_name".to_string(), "reviews-v3".to_string());
    return generate_target_graph(vertices, edges, ids_to_properties);
}
