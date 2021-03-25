use indexmap::IndexMap;
use log::trace;
use petgraph::Graph;
use petgraph::Incoming;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;
use utils::graph::iso::find_mapping_shamir_centralized;
use utils::graph::serde::put_ferried_data_in_hdrs;
use utils::graph::serde::FerriedData;
use utils::graph::serde::Property;
use utils::graph::utils::generate_target_graph;
use utils::graph::utils::get_node_with_id;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        let target_graph = create_target_graph();
        Box::new(HttpHeadersRoot {
            target_graph,
        })
    });
}

#[repr(i64)]
#[derive(Debug, PartialEq)]
enum TrafficDirection {
    Unspecified = 0,
    Inbound = 1,
    Outbound = 2,
}
impl From<i64> for TrafficDirection {
    fn from(orig: i64) -> Self {
        match orig {
            0x1 => return TrafficDirection::Inbound,
            0x2 => return TrafficDirection::Outbound,
            _ => return TrafficDirection::Unspecified,
        };
    }
}

impl fmt::Display for TrafficDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TrafficDirection::Unspecified => write!(f, "unspecified"),
            TrafficDirection::Inbound => write!(f, "inbound"),
            TrafficDirection::Outbound => write!(f, "outbound"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum HttpType {
    Unspecified = 0,
    Request = 1,
    Response = 2,
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
            target_graph: self.target_graph,
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

        let key = "This";
        let value = "works";

        let call_result = self.dispatch_http_call(
            "storage-upstream",
            vec![
                (":method", "GET"),
                (":path", "/store"),
                (":authority", "storage-upstream"),
                ("key", key),
                ("value", value),
            ],
            None,
            vec![],
            Duration::from_secs(5),
        );
        if let Err(e) = call_result {
            log::error!("Failed to make a call to storage {:?}", e);
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

    fn print_headers(&self, direction: HttpType) {
        if direction == HttpType::Request {
            for (name, value) in &self.get_http_request_headers() {
                log::warn!("#{} -> {}: {}", self.context_id, name, value);
            }
        } else if direction == HttpType::Response {
            for (name, value) in &self.get_http_response_headers() {
                log::warn!("#{} -> {}: {}", self.context_id, name, value);
            }
        } else {
            log::error!("Unsupported http type {:?}", direction);
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
        let mut ferried_data: FerriedData = FerriedData::default();
        if let Some(ferried_data_str) = self.get_http_request_header("ferried_data") {
            match serde_json::from_str(&ferried_data_str) {
                Ok(fd) => {
                    log::warn!("Successfully parsed ferried_data from header.");
                    ferried_data = fd;
                }
                Err(e) => {
                    log::error!("Could not translate stored data to json string: {0}\n", e);
                }
            }
        }

        // Insert properties to collect
        let prop_tuple;
        let property_str: String;
        if let Some(property) = self.get_property(vec!["node", "metadata", "WORKLOAD_NAME"]) {
            match String::from_utf8(property) {
                Ok(cast_string) => property_str = cast_string,
                Err(_e) => {
                    log::error!(
                        "Failed to serialize property: {:?}\n",
                        vec!["node", "metadata", "WORKLOAD_NAME"]
                    );
                    return;
                }
            }
        } else {
            log::error!(
                "Failed to retrieve property: {:?}\n",
                vec!["node", "metadata", "WORKLOAD_NAME"]
            );
            return;
        }
        prop_tuple = Property::new(
            self.workload_name.clone(),
            "service_name".to_string(),
            property_str.clone(),
        );
        ferried_data.unassigned_properties.push(prop_tuple);

        // Return ferried data to x, and store headers
        let mut hdrs = IndexMap::<String, String>::new();
        put_ferried_data_in_hdrs(&mut ferried_data, &mut hdrs);

        let mut stored_data: FerriedData = FerriedData::default();

        // Try to access some result we have stored before and attach it
        if let (Some(data), _) = self.get_shared_data(&trace_id) {
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
                        return;
                    }
                },
                Err(_e) => log::error!("{}", msg),
            }
        } else {
            log::warn!("Trace key {:?} not found in shared data.", trace_id);
        }

        stored_data.merge(ferried_data);

        let stored_data_str: String;
        match serde_json::to_string(&stored_data) {
            Ok(stored_data_str_) => {
                stored_data_str = stored_data_str_;
            }
            Err(e) => {
                log::error!("Could not translate stored data to json string: {0}\n", e);
                return;
            }
        }
        let stored_data_bytes = Some(stored_data_str.as_bytes());
        let store_result = self.set_shared_data(&trace_id, stored_data_bytes, None);
        if let Err(ref e) = store_result {
            log::error!(
                "Failed to store key {:?} and value {:?}: {:?}",
                trace_id,
                store_result,
                e
            );
        }
    }

    fn on_http_request_headers_outbound(&mut self, _num_headers: usize) {
        let trace_id: String;
        if let Some(trace_id_) = self.get_http_request_header("x-request-id") {
            trace_id = trace_id_;
            log::warn!("Request outbound:Using trace id {}!", trace_id);
        } else {
            log::error!("Request outbound: x-request-id not found in header!",);
            return;
        }
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
        let mut my_indexmap = IndexMap::new();
        my_indexmap.insert(
            "node.metadata.WORKLOAD_NAME".to_string(),
            self.workload_name.clone(),
        );
        let mut ferried_data: FerriedData = FerriedData::default();
        if let (Some(data), _) = self.get_shared_data(&trace_id) {
            let msg = format!("Unable to convert {:?} into a string ", data);
            // Add a header on the response.
            // FIXME: There must be  a nicer way to resolve this
            match String::from_utf8(data) {
                Ok(cast_string) => match serde_json::from_str(&cast_string) {
                    Ok(d) => {
                        let mut data: FerriedData = d;
                        let mut previous_roots = Vec::new();
                        for node in data.trace_graph.node_indices() {
                            if data.trace_graph.neighbors_directed(node, Incoming).count() == 0 {
                                previous_roots.push(node);
                            }
                        }
                        let me = data
                            .trace_graph
                            .add_node((self.workload_name.clone(), my_indexmap));

                        for previous_root in previous_roots {
                            data.trace_graph.add_edge(me, previous_root, String::new());
                        }
                        data.assign_properties();
                    }
                    Err(e) => {
                        log::error!("Could not parse envoy shared data: {0}\n", e);
                        return;
                    }
                },
                Err(_e) => log::error!("{}", msg),
            }
        } else {
            log::warn!("Trace key {:?} not found in shared data.", trace_id);
            ferried_data
                .trace_graph
                .add_node((self.workload_name.clone(), my_indexmap));
        }

        // Finally set the header with the newly merged data structure
        let stored_data_str: String;
        match serde_json::to_string(&ferried_data) {
            Ok(stored_data_str_) => {
                stored_data_str = stored_data_str_;
            }
            Err(e) => {
                log::error!("Could not translate stored data to json string: {0}\n", e);
                return;
            }
        }
        // if we are not the root id, return
        if self.workload_name != "productpage-v1" {
            self.set_http_response_header("ferried_data", Some(&stored_data_str));
            return;
        }
        // 2. calculate UDFs and store result, and check trace level properties

        let mapping =
            find_mapping_shamir_centralized(&ferried_data.trace_graph, &self.target_graph);
        if mapping.is_some() {
            let m = mapping.unwrap();
            let mut value: String;
            let mut value: String;

            let node_ptr = get_node_with_id(&self.target_graph, "a".to_string());
            if node_ptr.is_none() {
                log::warn!("Node a not found");
                return;
            }
            let mut trace_node_index = None;
            for map in m {
                if self.target_graph.node_weight(map.0).unwrap().0 == "a" {
                    trace_node_index = Some(map.1);
                    break;
                }
            }
            if trace_node_index == None
                || !&ferried_data
                    .trace_graph
                    .node_weight(trace_node_index.unwrap())
                    .unwrap()
                    .1
                    .contains_key("service_name")
            {
                // we have not yet collected the return property or have a mapping error
                return;
            }
            let mut ret_service_name = &ferried_data
                .trace_graph
                .node_weight(trace_node_index.unwrap())
                .unwrap()
                .1["service_name"];

            value = ret_service_name.to_string();
        }

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

        // This is where we merge the path and send it out
        if let Some(_x_wasm) = self.get_http_response_header("x-wasm") {
            let trace_key = trace_id + "path";
            let _data = self.get_shared_data(&trace_key);
        } else {
            log::error!("Response outbound: x-wasm not found!",);
            return;
        }
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
    let mut b_hashmap = ids_to_properties.get_mut("b").unwrap();
    b_hashmap.insert("service_name".to_string(), "reviews-v1".to_string());
    return generate_target_graph(vertices, edges, ids_to_properties);
}
