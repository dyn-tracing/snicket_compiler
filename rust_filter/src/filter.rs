use rpc_lib::rpc::Rpc;
use std::collections::HashMap;
use std::fs;
use petgraph::algo::isomorphic_subgraph_matching;
use petgraph::graph::NodeIndex;
use crate::graph_utils;

pub type CodeletType = fn(&Filter, &Rpc) -> Option<Rpc>;


// user defined functions:



// This represents a piece of state of the filter
// it either contains a user defined function, or some sort of
// other persistent state
#[derive(Clone, Debug)]
pub struct State {
    pub type_of_state: Option<String>,
    pub string_data: Option<String>,
    
}

impl State {
    pub fn new() -> State {
        State {
            type_of_state: None,
            string_data: None,
            
        }
    }

    pub fn new_with_str(str_data: String) -> State {
        State {
            type_of_state: Some(String::from("String")),
            string_data: Some(str_data),
            
        }
    }
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub filter_state: HashMap<String, State>,
}

impl Filter {
    #[no_mangle]
    pub fn new() -> *mut Filter {
         Box::into_raw(Box::new(Filter {
            filter_state: HashMap::new(),
        }))
    }

    #[no_mangle]
    pub fn new_with_envoy_properties(string_data: HashMap<String, String>) -> *mut Filter {
        let mut hash = HashMap::new();
        for key in string_data.keys() {
            hash.insert(key.clone(), State::new_with_str(string_data[key].clone()));
        }
        Box::into_raw(Box::new(Filter { filter_state: hash }))
    }

    #[no_mangle]
    pub fn execute(&mut self, x: &Rpc) -> Option<Rpc> {
        let mut to_return = Rpc { data: x.data, uid: x.uid, path: x.path.clone(), headers: x.headers.clone() };
        // 0. Who am I?
        let my_node_wrapped = self
            .filter_state
            .get("node.metadata.WORKLOAD_NAME");
        if my_node_wrapped.is_none() {
            print!("WARNING: filter was initialized without envoy properties and thus cannot function");
            return Some(to_return);
        }
        let my_node = my_node_wrapped
            .unwrap()
            .string_data
            .clone()
            .unwrap();


        // 1. Do I need to put any udf variables/objects in?
        

        // 2. Include any relevant node attributes
        let mut data_to_append: String;
        let mut data_key: String;
        let mut me;
        

        me = my_node.clone();
        me.insert_str(0, " ");
        me.push_str(".");

        data_key = vec![  String::from("node"),  String::from("metadata"),  String::from("WORKLOAD_NAME"),  ].join(".");
        data_to_append = self.filter_state[&data_key].string_data.as_ref().unwrap().to_string();
        me.push_str(&data_key);
        me.push_str("==");
        me.push_str(&data_to_append);
        me.push_str(",");

        if to_return.headers.contains_key(&"properties".to_string()) {
            to_return.headers.get_mut(&"properties".to_string()).unwrap().push_str(&me);
        }
        else {
            to_return.headers.insert("properties".to_string(), me);
        }
        
        

        me = my_node.clone();
        me.insert_str(0, " ");
        me.push_str(".");

        data_key = vec![  String::from("response"),  String::from("total_size"),  ].join(".");
        data_to_append = self.filter_state[&data_key].string_data.as_ref().unwrap().to_string();
        me.push_str(&data_key);
        me.push_str("==");
        me.push_str(&data_to_append);
        me.push_str(",");

        if to_return.headers.contains_key(&"properties".to_string()) {
            to_return.headers.get_mut(&"properties".to_string()).unwrap().push_str(&me);
        }
        else {
            to_return.headers.insert("properties".to_string(), me);
        }
        
        

        // 3.  Make a subgraph representing the query, check isomorphism compared to the
        //     observed trace, and do return calls based on that info
        if my_node == String::from("0") {
            // we need to create the graph given by the query
            let vertices = vec![ String::from("c"), String::from("b"), String::from("d"), String::from("a"),   ];
            let edges = vec![  ( String::from("a"), String::from("b"),  ),  ( String::from("b"), String::from("c"),  ),  ( String::from("a"), String::from("d"),  ),  ];
            // ids_to_properties is a HashMap taking <(NodeName, Properties), Desired Value>, so if the query says
            // a.service_name == productpagev1"
            // in ids_to_properties we have
            // ids_to_properties("a", { node.metadata.WORKLOAD_NAME = "productpage-v1" } )
            let mut ids_to_properties: HashMap<String, HashMap<String, String>> = HashMap::new();

            
            let mut a_property_hashmap = HashMap::new();
            a_property_hashmap.insert(vec! [  String::from("node"),  String::from("metadata"),  String::from("WORKLOAD_NAME"),  ].join("."), "productpage-v1".to_string());
            ids_to_properties.insert("a".to_string(), a_property_hashmap.clone());
            
            let mut b_property_hashmap = HashMap::new();
            b_property_hashmap.insert(vec! [  String::from("node"),  String::from("metadata"),  String::from("WORKLOAD_NAME"),  ].join("."), "reviewsv2".to_string());
            ids_to_properties.insert("b".to_string(), b_property_hashmap.clone());
            
            let mut c_property_hashmap = HashMap::new();
            c_property_hashmap.insert(vec! [  String::from("node"),  String::from("metadata"),  String::from("WORKLOAD_NAME"),  ].join("."), "ratingsv1".to_string());
            ids_to_properties.insert("c".to_string(), c_property_hashmap.clone());
            
            let mut d_property_hashmap = HashMap::new();
            d_property_hashmap.insert(vec! [  String::from("node"),  String::from("metadata"),  String::from("WORKLOAD_NAME"),  ].join("."), "detailsv1".to_string());
            ids_to_properties.insert("d".to_string(), d_property_hashmap.clone());
            

            let target_graph = graph_utils::generate_target_graph(vertices, edges, ids_to_properties);
            let trace_graph;
            if x.headers.contains_key(&"properties".to_string()) {
                trace_graph = graph_utils::generate_trace_graph_from_headers(x.path.clone(), to_return.headers.get_mut(&"properties".to_string()).unwrap().to_string());
            }
            else {
                trace_graph = graph_utils::generate_trace_graph_from_headers(x.path.clone(), String::new());

            }
            let mapping = isomorphic_subgraph_matching(
                &target_graph,
                &trace_graph,
                |x, y| {
                    for property in y.1.keys() {
                        if property != &"node.metadata.WORKLOAD_NAME".to_string() && &(x.1[property]) != &(y.1[property]) { return false; }
                    }
                return true;
                },
                |x, y| x == y,
            );
            if !mapping.is_none() {
                let m = mapping.unwrap();
                // In the non-simulator version, we will send the result to storage.  Given this is
                // a simulation, we will write it to a file for now
                // TODO:  make a storage node in the simulator
                let node_ptr = graph_utils::get_node_with_id(&target_graph, "a".to_string());
               if node_ptr.is_none() {
                   print!("WARNING Node a not found");
                   return  Some(to_return);
               }
               let trace_node_index = NodeIndex::new(m[node_ptr.unwrap().index()]);
               let a_response_total_size_str = &trace_graph.node_weight(trace_node_index).unwrap().1[ &vec!["response", "total_size"].join(".") ];

                
                
                fs::write("result.txt", a_response_total_size_str).expect("Unable to write file");
                
                


                

            }
        }
        // 4.  Allow udfs to execute
        


        // 5.  Pass the rpc on
        Some(to_return)
    }

}
