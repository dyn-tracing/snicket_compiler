use super::utils;
use indexmap::map::IndexMap;
use petgraph::Graph;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Property {
    entity: String,
    property_name: String,
    value: String,
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
    pub trace_graph: Graph<(String, IndexMap<String, String>), String>,
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
            let node = utils::get_node_with_id(&self.trace_graph, property.entity.clone());
            if node.is_some() {
                self.trace_graph
                    .node_weight_mut(node.unwrap())
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
            self.trace_graph
                .add_node(other_data.trace_graph.node_weight(node).unwrap().clone());
        }
        // add edges
        for edge in other_data.trace_graph.edge_indices() {
            match other_data.trace_graph.edge_endpoints(edge) {
                Some((edge0, edge1)) => {
                    let edge0_weight = &other_data.trace_graph.node_weight(edge0).unwrap().0;
                    let edge1_weight = &other_data.trace_graph.node_weight(edge1).unwrap().0;
                    let edge0_in_stored_graph =
                        utils::get_node_with_id(&self.trace_graph, edge0_weight.to_string())
                            .unwrap();
                    let edge1_in_stored_graph =
                        utils::get_node_with_id(&self.trace_graph, edge1_weight.to_string())
                            .unwrap();
                    self.trace_graph.add_edge(
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

        // 2. merge unassigned properties
        //    these are properties we have collected but are not yet in the graph
        self.unassigned_properties
            .append(&mut other_data.unassigned_properties);
        self.unassigned_properties.sort_unstable();
        self.unassigned_properties.dedup(); // remove duplicates
        self.assign_properties();
    }
}

pub fn put_ferried_data_in_hdrs(fd: &mut FerriedData, hdr: &mut IndexMap<String, String>) {
    match serde_json::to_string(fd) {
        Ok(stored_data_string) => {
            hdr.insert("ferried_data".to_string(), stored_data_string);
        }
        Err(e) => {
            log::error!(
                "ERROR:  could not translate stored data to json string: {0}\n",
                e
            );
        }
    }
}
