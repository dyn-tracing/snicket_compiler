use indexmap::IndexMap;
use log::trace;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Incoming;
use proxy_wasm::traits::Context;
use proxy_wasm::traits::HttpContext;
use proxy_wasm::traits::RootContext;
use proxy_wasm::types::Action;
use proxy_wasm::types::ContextType;
use proxy_wasm::types::LogLevel;
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;
use serde::{Serialize, Deserialize};

use utils::graph::iso::SetSKey;
use utils::graph::iso::find_mapping_shamir_decentralized;
use utils::graph::iso::find_mapping_shamir_centralized;
use utils::graph::graph_utils::get_node_with_id;
use utils::graph::serde::Property;

// These are generated by the filter
use super::filter::collect_envoy_properties;
use super::filter::create_target_graph;
use super::filter::execute_udfs;
use super::filter::check_trace_lvl_prop;
use super::filter::get_value_for_storage;
use super::filter::get_root_name;

// ---------------------- General Helper Functions ----------------------------
// TODO:  make inheritance work so putting this in a library works
#[derive(Debug, Serialize, Deserialize)]
pub struct FerriedData {
    pub set_s: IndexMap<
        SetSKey,
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
    pub found_match: bool,
    pub trace_graph: Graph<(String, IndexMap<String, String>), ()>,
    pub unassigned_properties: Vec<Property>, // entity property value
}
impl FerriedData {
    fn default() -> FerriedData {
        FerriedData {
            set_s: IndexMap::new(),
            found_match: false,
            trace_graph: Graph::new(),
            unassigned_properties: Vec::new(),
        }
    }
        // take any unassigned properties that apply to nodes in the graph,
    // and associate them with those nodes
    fn assign_properties(&mut self) {
        let mut to_delete = Vec::new();
        for property in &mut self.unassigned_properties {
            let node = get_node_with_id(&self.trace_graph, &property.entity);
            if node.is_some() {
                self.trace_graph.node_weight_mut(node.unwrap()).unwrap().1.insert(property.property_name.clone(), property.value.clone());
                to_delete.push(property.clone());
            }
        }
        self.unassigned_properties.retain(|x| !to_delete.contains(&x));

    }

    fn merge(&mut self, mut other_data: FerriedData) {
        // 1. Merge the graphs by simply adding other data's to self's

        let mut prev_nodes_to_new_nodes = IndexMap::new();
        // add nodes
        for node in other_data.trace_graph.node_indices() {
            let new_node = self.trace_graph.add_node(other_data.trace_graph.node_weight(node).unwrap().clone());
            prev_nodes_to_new_nodes.insert(node, new_node);
        }
        // add edges
        for edge in other_data.trace_graph.edge_indices() {
            match other_data.trace_graph.edge_endpoints(edge) {
                Some((edge0, edge1)) => {
                    let edge0_weight = &other_data.trace_graph.node_weight(edge0).unwrap().0;
                    let edge1_weight = &other_data.trace_graph.node_weight(edge1).unwrap().0;
                    let edge0_in_stored_graph = get_node_with_id(&self.trace_graph, edge0_weight).unwrap();
                    let edge1_in_stored_graph = get_node_with_id(&self.trace_graph, edge1_weight).unwrap();
                    self.trace_graph.add_edge(edge0_in_stored_graph, edge1_in_stored_graph, ());
                }
                None => {
                    panic!("no edge endpoints found \n");
                    return;
                }
            }
        }
        // 2. merge unassigned properties
        //    these are properties we have collected but are not yet in the graph
        self.unassigned_properties.append(&mut other_data.unassigned_properties);
        self.unassigned_properties.sort_unstable();
        self.unassigned_properties.dedup(); // remove duplicates
        self.assign_properties();

        // 3. Merge set s, and found match
        if self.found_match || other_data.found_match {
            self.found_match = true;
            self.set_s = IndexMap::new(); // don't carry around all that baggage
            // esp when you've already sent to storage
        } else {
            for entry in other_data.set_s.keys() {
                // because of our graph merging above, we messed up all the
                // node indices.  So now you've got to rearrange things based on the new
                // index system
                // TODO:  there's gotta be a better way to do this

                // because all target graphs are constructed the same, we don't touch the second entry
                let new_entry = SetSKey { val1: prev_nodes_to_new_nodes[&entry.val1], val2: entry.val2};
                let mut new_indexmap : IndexMap<NodeIndex, _> = IndexMap::new();
                for inner_indexmap_key in other_data.set_s[entry].keys() {
                    if other_data.set_s[entry][inner_indexmap_key].is_none() {
                        new_indexmap.insert(*inner_indexmap_key, None);
                    } else {
                        let mut new_inner_indexmap_vec : Vec<(NodeIndex, NodeIndex)> = Vec::new();
                        for tuple in other_data.set_s[entry][inner_indexmap_key].as_ref().unwrap() {
                            new_inner_indexmap_vec.push(
                                (tuple.0, prev_nodes_to_new_nodes[&tuple.1])
                            );
                        }
                        new_indexmap.insert(*inner_indexmap_key, Some(new_inner_indexmap_vec));
                    }
                }
                self.set_s.insert(new_entry, new_indexmap);
            }
        }
    }
}

#[repr(i64)]
#[derive(Debug, PartialEq)]
pub enum TrafficDirection {
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
pub enum HttpType {
    Unspecified = 0,
    Request = 1,
    Response = 2,
}

pub fn data_to_str(stored_data: &FerriedData) -> Option<String> {
    let stored_data_str: String;
    match serde_json::to_string(&stored_data) {
        Ok(stored_data_str_) => {
            stored_data_str = stored_data_str_;
        }
        Err(e) => {
            log::error!("Could not translate stored data to json string in data_to_str: {:?}\n", e);
            return None;
        }
    }
    return Some(stored_data_str);
}

pub fn join_str(str_vec: &Vec<&str>) -> String {
    return str_vec.join(".");
}

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
                log::warn!("this is the distributed version\n");
                return fd;
            }
            Err(e) => {
                log::error!("Could not translate stored data to json string in fetch_data_from_headers: {:?}\n", e);
            }
        }
    }
    return FerriedData::default();
}

pub fn fetch_property(
    node_name: &str,
    prop_query: &Vec<&str>,
    ctx: &HttpHeaders,
) -> Option<Property> {
    // Insert properties to collect
    let prop_tuple;
    let property_str: String;
    // Seems like we need a copy here, a little bit annoying
    if let Some(property) = ctx.get_property(prop_query.to_vec()) {
        property_str = String::from_utf8_lossy(&property).to_string();
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
        // Add a header on the response.
        // FIXME: There must be  a nicer way to resolve this
        let cast_string = String::from_utf8_lossy(&data).to_string();
        match serde_json::from_str(&cast_string) {
            Ok(d) => {
                stored_data = d;
            }
            Err(e) => {
                log::error!("Could not parse envoy shared data: {:?}\n", e);
                return None;
            }
        }
    } else {
        log::warn!("Trace key {:?} not found in shared data.", trace_id);
    }
    return Some(stored_data);
}

fn store_data(data_to_store: &mut FerriedData, trace_id: &str, ctx: &HttpHeaders) {
    // Merge with data that is already present.
    let stored_data_opt = get_shared_data(&trace_id, ctx);
    if stored_data_opt.is_some() {
        let stored_data_old = stored_data_opt.unwrap();
        data_to_store.merge(stored_data_old);
    }

    // Convert to string, then to bytes.
    let stored_data_str_opt = data_to_str(&data_to_store);
    if stored_data_str_opt.is_none() {
        // We failed to serialize the shared data.
        // This might lead to wrong results, abort.
        return;
    }
    let stored_data_str = stored_data_str_opt.unwrap();
    let stored_data_bytes = Some(stored_data_str.as_bytes());

    // Update the stored result.
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

// ---------------------------- Filter ------------------------------------

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Info);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpHeadersRoot {
            target_graph: create_target_graph(),
        })
    });
}

struct HttpHeadersRoot {
    target_graph: Graph<(String, IndexMap<String, String>), ()>,
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

pub struct HttpHeaders {
    pub context_id: u32,
    pub workload_name: String,
    pub target_graph: Graph<(String, IndexMap<String, String>), ()>,
}

impl Context for HttpHeaders {
    /// Process the callback from any http calls the filter makes for debugging.
    /// This is usually from storage.
    /// TODO: This is not working reliably yet. Needs some investigating.
    fn on_http_call_response(
        &mut self,
        _token_id: u32,
        _num_headers: usize,
        body_size: usize,
        _: usize,
    ) {
        log::warn!("Received response from storage");
        if let Some(body) = self.get_http_call_response_body(0, body_size) {
            log::warn!("Storage body: {:?}", body);
        }
        for (name, value) in &self.get_http_response_headers() {
            log::warn!("Storage Header - {:?}: {:?}", name, value);
        }
    }
}

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

        match collect_envoy_properties(self, &mut ferried_data) {
            Ok(_) => {}
            Err(_) => {
                return;
            }
        }

        store_data(&mut ferried_data, &trace_id, self);
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
            log::warn!("returning early because could not deserialize stored data");
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
                .add_edge(me, previous_root, ());
        }
        stored_data.assign_properties();
        execute_udfs(self, &mut stored_data);

        if !stored_data.found_match {
            let am_root = self.workload_name == get_root_name();
            log::warn!("am root is {:?}", am_root);
            log::warn!("I am {:?}", self.workload_name);
            log::warn!("root id is {:?}", get_root_name());
            let new_am_root = self.workload_name == get_root_name();
            log::warn!("new am root is {:?}", new_am_root);

            let mapping_opt =
                find_mapping_shamir_decentralized(
                    &stored_data.trace_graph,
                    &self.target_graph,
                    &mut stored_data.set_s,
                    get_node_with_id(&stored_data.trace_graph, &self.workload_name).unwrap(),
                    am_root);
            let cent_mapping_opt = find_mapping_shamir_centralized(&stored_data.trace_graph, &self.target_graph);
            if cent_mapping_opt.is_some() {
                log::warn!("centralized found mapping");
            } else {
                log::warn!("centralized did not find mapping");

            }
            if mapping_opt.is_some() && check_trace_lvl_prop(self, &mut stored_data) {
                let mapping = mapping_opt.unwrap();
                stored_data.found_match = true;
                let key = join_str(&vec!["node", "metadata", "WORKLOAD_NAME"]);
                let value_wrapped =
                    get_value_for_storage(&self.target_graph, &mapping, &stored_data);
                if value_wrapped.is_none() {
                    return;
                }
                let value = value_wrapped.unwrap();
                log::warn!("Sending to storage based on distributed isomorphism");
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
                match call_result {
                    Ok(_ok) => {}
                    Err(e) => log::error!("Failed to make a call to storage {:?}", e),
                }
            } else {
                log::warn!("Mapping not found at {:?}", self.workload_name);
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
        let mut ferried_data = fetch_data_from_headers(self, HttpType::Response);

        store_data(&mut ferried_data, &trace_id, self);
    }
}
