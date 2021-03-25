#![feature(static_nobundle)]
use indexmap::IndexMap;
use log::trace;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::convert::TryFrom;
use std::fmt;
use std::time::Duration;
use utils::graph::serde::put_ferried_data_in_hdrs;
use utils::graph::serde::FerriedData;
use utils::graph::serde::Property;
use utils::graph::utils::get_node_with_id;

pub fn merged_ferried_data(data: &mut FerriedData, stored_data: &mut FerriedData) {
    // 2. Merge the graphs by simply adding it - later, when we merge, we will
    //    make a root

    // add node
    for node in data.trace_graph.node_indices() {
        stored_data
            .trace_graph
            .add_node(data.trace_graph.node_weight(node).unwrap().clone());
    }
    // add edges
    for edge in data.trace_graph.edge_indices() {
        match data.trace_graph.edge_endpoints(edge) {
            Some((edge0, edge1)) => {
                let edge0_weight = &data.trace_graph.node_weight(edge0).unwrap().0;
                let edge1_weight = &data.trace_graph.node_weight(edge1).unwrap().0;
                let edge0_in_stored_graph =
                    get_node_with_id(&stored_data.trace_graph, edge0_weight.to_string()).unwrap();
                let edge1_in_stored_graph =
                    get_node_with_id(&stored_data.trace_graph, edge1_weight.to_string()).unwrap();
                stored_data.trace_graph.add_edge(
                    edge0_in_stored_graph,
                    edge1_in_stored_graph,
                    String::new(),
                );
            }
            None => {
                log::error!("no edge endpoints found \n");
                return;
            }
        }
    }

    // 3. merge unassigned properties
    //    these are properties we have collected but are not yet in the graph
    stored_data
        .unassigned_properties
        .append(&mut data.unassigned_properties);
    stored_data.unassigned_properties.sort_unstable();
    stored_data.unassigned_properties.dedup();
    stored_data.assign_properties();
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
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

struct HttpHeadersRoot;

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
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
        }))
    }
}

struct HttpHeaders {
    context_id: u32,
    workload_name: String,
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
        merged_ferried_data(&mut ferried_data, &mut stored_data);
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
        log::warn!("STORED DATA STRING {:?}", stored_data_str);
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
        // Try to access some result we have stored before and attach it
        let trace_key = trace_id + "path";
        if let (Some(data), _) = self.get_shared_data(&trace_key) {
            let msg = format!("Unable to convert {:?} into a string ", data);
            // Add a header on the response.
            // FIXME: There must be  a nicer way to resolve this
            match String::from_utf8(data) {
                Ok(cast_string) => self.set_http_response_header("Hello", Some(&cast_string)),
                Err(_e) => log::error!("{}", msg),
            }
        } else {
            log::warn!("Trace key {:?} not found in shared data.", trace_key);
        }
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
