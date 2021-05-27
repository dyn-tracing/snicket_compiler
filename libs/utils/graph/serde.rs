use super::graph_utils;
use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use super::iso::SetSType;
use super::iso::SetSKey;
use petgraph::Graph;
use petgraph::graph::NodeIndex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct Property {
    pub entity: String,
    pub property_name: u64,
    pub value: String,
}

impl Property {
    pub fn default() -> Property {
        Property {
            entity: String::new(),
            property_name: 0,
            value: String::new(),
        }
    }

    pub fn new(entity: String, property_name: u64, value: String) -> Property {
        Property {
            entity,
            property_name,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FerriedData {
    pub set_s: Option<SetSType>,
    pub found_match: bool,
    pub trace_graph: Graph<(String, IndexMap<u64, String>), ()>,
    pub unassigned_properties: IndexSet<Property>, // entity property value
}

impl FerriedData {
    pub fn default() -> FerriedData {
        FerriedData {
            set_s: None,
            found_match: false,
            trace_graph: Graph::new(),
            unassigned_properties: IndexSet::new(),
        }
    }
    pub fn default_distributed() -> FerriedData {
        FerriedData {
            set_s: Some(IndexMap::new()),
            found_match: false,
            trace_graph: Graph::new(),
            unassigned_properties: IndexSet::new(),
        }
    }

    // take any unassigned properties that apply to nodes in the graph,
    // and associate them with those nodes
    pub fn assign_properties(&mut self) {
        for property in self.unassigned_properties.iter() {
            if let Some(node) = graph_utils::get_node_with_id(&self.trace_graph, &property.entity) {
                self.trace_graph
                    .node_weight_mut(node)
                    .unwrap()
                    .1
                    .insert(property.property_name.clone(), property.value.clone());
            }
        }
        remove_assigned_properties(&mut self.unassigned_properties, &self.trace_graph);
    }

    pub fn merge(&mut self, mut other_data: FerriedData) {
        //  Merge the graphs by simply adding other data's to self's

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
                    let edge0_in_stored_graph =
                        graph_utils::get_node_with_id(&self.trace_graph, edge0_weight)
                            .unwrap();
                    let edge1_in_stored_graph =
                        graph_utils::get_node_with_id(&self.trace_graph, edge1_weight)
                            .unwrap();
                    self.trace_graph
                        .add_edge(edge0_in_stored_graph, edge1_in_stored_graph, ());
                }
                None => {
                    log::error!("no edge endpoints found \n");
                    return;
                }
            }
        }

        // 2. merge unassigned properties
        //    these are properties we have collected but are not yet in the graph

        // 2a. If any of your existing properties can be assigned, do so
        self.assign_properties();
        // 2b. If any of the new properties can be assigned, do so, and add the rest to baggage
        for property in other_data.unassigned_properties {
            if let Some(node) = graph_utils::get_node_with_id(&self.trace_graph, &property.entity) {
                self.trace_graph
                    .node_weight_mut(node)
                    .unwrap()
                    .1
                    .insert(property.property_name.clone(), property.value.clone());
            } else {
                // because it's an indexset, we will not get duplicates
                self.unassigned_properties.insert(property);
            }
        }
        if self.set_s.is_some() && other_data.set_s.is_some() {
            if self.found_match || other_data.found_match {                         
                self.found_match = true;                                            
                self.set_s = Some(IndexMap::new()); // don't carry around all that baggage
                // esp when you've already sent to storage                          
            } else {                                                                
                merge_set_s(self.set_s.as_mut().unwrap(),
                            other_data.set_s.as_mut().unwrap(),
                            &prev_nodes_to_new_nodes);
            }          

        }
    }
}


pub fn remove_assigned_properties(unassigned_properties: &mut IndexSet<Property>,
                                  graph: &Graph<(String, IndexMap<u64, String>), ()>) {
    unassigned_properties.retain(|x|
        graph_utils::get_node_with_id(graph, &x.entity) == None
    );
}


pub fn merge_set_s(my_set_s: &mut SetSType,
                   their_set_s: &mut SetSType,
                   prev_nodes_to_new_nodes: &IndexMap<NodeIndex, NodeIndex>
) {
    for entry in their_set_s.keys() {                              
        // because of our graph merging above, we messed up all the     
        // node indices.  So now you've got to rearrange things based on the new
        // index system                                                 
        // TODO:  there's gotta be a better way to do this              
                                                                        
        // because all target graphs are constructed the same, we don't touch the second entry
        let new_entry = SetSKey { val1: prev_nodes_to_new_nodes[&entry.val1], val2: entry.val2};
        let mut new_indexmap : IndexMap<NodeIndex, _> = IndexMap::new();
        for inner_indexmap_key in their_set_s[entry].keys() {      
            if their_set_s[entry][inner_indexmap_key].is_none() {  
                new_indexmap.insert(*inner_indexmap_key, None);         
            } else {                                                    
                let mut new_inner_indexmap_vec : Vec<(NodeIndex, NodeIndex)> = Vec::new();
                for tuple in their_set_s[entry][inner_indexmap_key].as_ref().unwrap() {
                    new_inner_indexmap_vec.push(                        
                        (tuple.0, prev_nodes_to_new_nodes[&tuple.1])    
                    );                                                  
                }                                                       
                new_indexmap.insert(*inner_indexmap_key, Some(new_inner_indexmap_vec));
            }                                                           
        }                                                               
        my_set_s.insert(new_entry, new_indexmap);                     
    }                                                                   
}
