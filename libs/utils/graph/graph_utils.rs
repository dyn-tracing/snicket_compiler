/* This file contains functions relating to creating and comparing trace and target (user-given) graphs */

use indexmap::map::IndexMap;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use petgraph::Incoming;
type GraphType = Graph<(String, IndexMap<String, String>), ()>;

/* This function creates a petgraph graph representing the query given by the user.
 * For example, if the cql query were MATCH n -> m, e WHERE ... the input to this function
 * would be vertices = [n, m], edges = [(n,m)].
 *
 * Arguments:
 * @vertices:  the vertices of the graph to construct
 * @edges:  the edges of the graph to construct
 *
 * Return Value:
 * @graph: the constructed graph reprsenting the inputs
 */

pub fn generate_target_graph(
    vertices: Vec<String>,
    edges: Vec<(String, String)>,
    ids_to_properties: IndexMap<String, IndexMap<String, String>>,
) -> GraphType {
    let mut graph = Graph::new();

    // In order to make edges, we have to know the handles of the nodes, and you
    // get the handles of the nodes by adding them to the graph

    let mut nodes_to_node_handles: IndexMap<String, NodeIndex> = IndexMap::new();
    for node in vertices {
        if ids_to_properties.contains_key(&node) {
            nodes_to_node_handles.insert(
                node.clone(),
                graph.add_node((node.clone(), ids_to_properties[&node].clone())),
            );
        } else {
            nodes_to_node_handles.insert(
                node.clone(),
                graph.add_node((node.clone(), IndexMap::new())),
            );
        }
    }

    // Make edges with handles instead of the vertex names
    let mut edge_handles = Vec::new();
    for edge in edges {
        let node0 = nodes_to_node_handles[&edge.0];
        let node1 = nodes_to_node_handles[&edge.1];
        let new_edge = (node0, node1);
        edge_handles.push(new_edge);
    }
    graph.extend_with_edges(edge_handles);
    graph
}


pub fn get_node_with_id(
    graph: &GraphType,
    node_name: &str,
) -> Option<NodeIndex> {
    for index in graph.node_indices() {
        if graph.node_weight(index).unwrap().0 == node_name {
            return Some(index);
        }
    }
    None
}

pub fn find_leaves(
    node: NodeIndex,
    graph: &GraphType,
) -> Vec<NodeIndex> {
    let mut post_order = DfsPostOrder::new(&graph, node);
    let mut to_return = Vec::new();
    while let Some(visited) = post_order.next(&graph) {
        if graph.neighbors(visited).count() == 0 {
            to_return.push(visited);
        }
    }
    to_return
}

pub fn find_root(graph: &GraphType) -> NodeIndex {
    for node in graph.node_indices() {
        if graph.neighbors_directed(node, Incoming).count() == 0 {
            return node;
        }
    }
    panic!("no root found");
}

pub fn has_property_subset(
    property_set_1: &IndexMap<String, String>, // set
    property_set_2: &IndexMap<String, String>, // subset
) -> bool {
    for property in property_set_2.keys() {
        if !property_set_1.contains_key(property) {
            return false;
        }
        if property_set_1[property] != property_set_2[property] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;


    fn make_small_target_graph() -> GraphType {
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        let vertices = vec![a.clone(), b.clone(), c.clone()];
        let edges = vec![(a.clone(), b.clone()), (b.clone(), c.clone())];
        let mut ids_to_properties = IndexMap::new();

        let mut a_hashmap = IndexMap::new();
        a_hashmap.insert("node.metadata.WORKLOAD_NAME".to_string(), "a".to_string());
        ids_to_properties.insert("a".to_string(), a_hashmap);

        let mut b_hashmap = IndexMap::new();
        b_hashmap.insert("node.metadata.WORKLOAD_NAME".to_string(), "b".to_string());
        ids_to_properties.insert("b".to_string(), b_hashmap);

        let mut c_hashmap = IndexMap::new();
        c_hashmap.insert("node.metadata.WORKLOAD_NAME".to_string(), "c".to_string());
        ids_to_properties.insert("c".to_string(), c_hashmap);

        assert!(ids_to_properties.keys().len() == 3);
        assert!(ids_to_properties.contains_key(&"a".to_string()));
        assert!(ids_to_properties.contains_key(&"b".to_string()));
        assert!(ids_to_properties.contains_key(&"c".to_string()));
        for vertex in &vertices {
            assert!(ids_to_properties.contains_key(vertex));
        }
        let graph = generate_target_graph(vertices, edges, ids_to_properties);
        graph
    }
    fn little_branching_graph() -> GraphType {
        let mut graph: GraphType = Graph::new();
        graph.extend_with_edges(&[(0, 1), (0, 2), (0, 3), (1, 4), (3, 5)]);
        return graph;
    }

    fn little_graph() -> GraphType {
        let mut graph: GraphType = Graph::new();
        let a = graph.add_node(("a".to_string(), IndexMap::new()));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));
        let c = graph.add_node(("c".to_string(), IndexMap::new()));
        graph.add_edge(a,b,());
        graph.add_edge(b,c,());
        graph
    }


    #[test]
    fn test_generate_target_graph() {
        let graph = make_small_target_graph();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_get_node_with_id() {
        let graph = little_graph();
        let ret = get_node_with_id(&graph, "a");
        assert!(ret.is_some());
    }

    #[test]
    fn test_find_leaves() {
        let graph = little_branching_graph();
        let leaves = find_leaves(NodeIndex::new(0), &graph);
        let correct_leaves = vec![2, 4, 5];
        for leaf in &leaves {
            assert!(correct_leaves.contains(&leaf.index()));
        }
    }

    #[test]
    fn test_find_root() {
        let graph = little_graph();
        let root = find_root(&graph);
        assert!(graph.node_weight(root).unwrap().0 == "a", "root label is {:?}", graph.node_weight(root).unwrap());
    }
}
