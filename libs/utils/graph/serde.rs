use super::graph_utils;
use indexmap::map::IndexMap;
use petgraph::Graph;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Property {
    pub entity: String,
    pub property_name: String,
    pub value: String,
}

impl Property {
    pub fn default() -> Property {
        Property {
            entity: String::new(),
            property_name: String::new(),
            value: String::new(),
        }
    }

    pub fn new(entity: String, property_name: String, value: String) -> Property {
        Property {
            entity,
            property_name,
            value,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FerriedData {
    pub trace_graph: Graph<(String, IndexMap<String, String>), ()>,
    pub unassigned_properties: Vec<Property>, // entity property value
}

impl FerriedData {
    pub fn default() -> FerriedData {
        FerriedData {
            trace_graph: Graph::new(),
            unassigned_properties: Vec::new(),
        }
    }

    // take any unassigned properties that apply to nodes in the graph,
    // and associate them with those nodes
    pub fn assign_properties(&mut self) {
        let mut to_delete = Vec::new();
        for property in &mut self.unassigned_properties {
            if let Some(node) = graph_utils::get_node_with_id(&self.trace_graph, property.entity.clone()) {
                self.trace_graph
                    .node_weight_mut(node)
                    .unwrap()
                    .1
                    .insert(property.property_name.clone(), property.value.clone());
                to_delete.push(property.clone());
            }
        }
        self.unassigned_properties
            .retain(|x| !to_delete.contains(&x));
    }
    pub fn merge(&mut self, other_data: FerriedData) {
        //  Merge the graphs by simply adding other data's to self's

        // add nodes
        for node in other_data.trace_graph.node_indices() {
            //let node_name = &other_data.trace_graph.node_weight(node).unwrap().0;
            //if utils::get_node_with_id(&self.trace_graph, node_name.to_string()).is_none() {
                self.trace_graph
                    .add_node(other_data.trace_graph.node_weight(node).unwrap().clone());
            //}
        }
        // add edges
        for edge in other_data.trace_graph.edge_indices() {
            match other_data.trace_graph.edge_endpoints(edge) {
                Some((edge0, edge1)) => {
                    let edge0_weight = &other_data.trace_graph.node_weight(edge0).unwrap().0;
                    let edge1_weight = &other_data.trace_graph.node_weight(edge1).unwrap().0;
                    let edge0_in_stored_graph =
                        graph_utils::get_node_with_id(&self.trace_graph, edge0_weight.to_string())
                            .unwrap();
                    let edge1_in_stored_graph =
                        graph_utils::get_node_with_id(&self.trace_graph, edge1_weight.to_string())
                            .unwrap();
                    self.trace_graph.add_edge(
                        edge0_in_stored_graph,
                        edge1_in_stored_graph,
                        (),
                    );
                }
                None => {
                    log::error!("no edge endpoints found \n");
                    return;
                }
            }
        }

        // 2. merge unassigned properties
        //    these are properties we have collected but are not yet in the graph
        let mut other_properties = other_data.unassigned_properties;
        //FIXME this is odd, why is this necessary?
        self.unassigned_properties.append(&mut other_properties);
        self.unassigned_properties.sort_unstable();
        self.unassigned_properties.dedup(); // remove duplicates
        self.assign_properties();
    }
}
