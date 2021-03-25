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
