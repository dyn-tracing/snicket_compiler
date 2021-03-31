/// Implements subgraph isomorphism algorithms two ways:
/// as described in https://www.cs.bgu.ac.il/~dekelts/publications/subtree.pdf
/// Another thing to consider, but is not implemented here, is
/// http://chasewoerner.org/popl87.pdf
///
use super::graph_utils::{find_leaves, find_root, get_node_with_id, has_property_subset};
use indexmap::map::IndexMap;
use pathfinding::directed::edmonds_karp::*;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::DfsPostOrder;
use petgraph::{Incoming, Outgoing};

// -------------- Shamir Isomorphism Algorithm Helper Functions---------------

/// Given two sets of nodes, set x from graph g, and set y from graph h,
/// creates a flow graph with the source connected to all nodes in x and
/// the sink connected to all nodes in y.  Edges between x and y are computed
/// based on if their set (in set_s) contains u_null.  Then we compute
/// the flow of that graph, which is equivalent to the maximum matching.
fn max_matching<EK: EdmondsKarp<i32>>(
    set_x: &Vec<NodeIndex>,
    set_y: &Vec<NodeIndex>,
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
    set_s: &IndexMap<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
    u_null: NodeIndex,
) -> (usize, Vec<(NodeIndex, NodeIndex)>) {
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    vertices.push("SINK".to_string());
    vertices.push("SOURCE".to_string());
    for u in set_x {
        let mut u_str = graph_h.node_weight(*u).unwrap().0.clone();
        u_str.push_str("U");
        vertices.push(u_str.clone());
        edges.push((("SOURCE".to_string(), u_str), 1));
    }
    for v in set_y {
        let mut v_str = graph_g.node_weight(*v).unwrap().0.clone();
        v_str.push_str("V");
        vertices.push(v_str.clone());
        edges.push(((v_str, "SINK".to_string()), 1));
    }

    for u in set_x {
        for v in set_y {
            if set_s[&(*v, *u)].contains_key(&u_null)
                && has_property_subset(
                    &graph_g.node_weight(*v).unwrap().1,
                    &graph_h.node_weight(*u).unwrap().1,
                )
            {
                let mut u_str = graph_h.node_weight(*u).unwrap().0.clone();
                u_str.push_str("U");
                let mut v_str = graph_g.node_weight(*v).unwrap().0.clone();
                v_str.push_str("V");
                // 2. add edge between v and u
                edges.push(((u_str, v_str), 1));
            }
        }
    }

    let vertices_as_str: Vec<&str> = vertices.iter().map(|s| s.as_ref()).collect();
    let edges_as_str: Vec<((&str, &str), i32)> = edges
        .iter()
        .map(|((a, b), cost)| ((a.as_ref(), b.as_ref()), *cost))
        .collect();
    for (edge, _cost) in &edges_as_str {
        assert!(
            vertices_as_str.contains(&edge.0),
            "vertices doesn't have {0}",
            &edge.0
        );
        assert!(
            vertices_as_str.contains(&edge.1),
            "vertices doesn't have {0}",
            &edge.1
        );
    }
    let (edges, _costs) = edmonds_karp::<_, _, _, EK>(
        &vertices_as_str,
        &"SOURCE",
        &"SINK",
        edges_as_str.into_iter(),
    );
    let mut matching = Vec::new();
    for edge_tuple in edges {
        let edge = edge_tuple.0;
        if edge.0 != "SOURCE" && edge.1 != "SINK" {
            let mut node_in_x_set = edge.0.to_string();
            node_in_x_set.pop();
            let mut node_in_y_set = edge.1.to_string();
            node_in_y_set.pop();
            let node_in_x_set_id = get_node_with_id(graph_h, node_in_x_set).unwrap();
            let node_in_y_set_id = get_node_with_id(graph_g, node_in_y_set).unwrap();
            matching.push((node_in_x_set_id, node_in_y_set_id));
        }
    }
    return (matching.len(), matching);
}

// For debugging only
fn print_set_s(
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
    set_s: &IndexMap<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
) {
    for key in set_s.keys() {
        print!(
            "key: {:?} {:?} ",
            graph_g.node_weight(key.0).unwrap(),
            graph_h.node_weight(key.1).unwrap()
        );
        for value_key in set_s[key].keys() {
            print!("inner key: {:?} ", graph_h.node_weight(*value_key).unwrap());
            for mapping in &set_s[key][value_key] {
                for map in mapping {
                    print!(
                        "maps {:?} to {:?} ",
                        graph_h.node_weight(map.0).unwrap(),
                        graph_g.node_weight(map.1).unwrap()
                    );
                }
            }
        }
        print!("\n\n");
    }
}

fn get_mapping_from_set_s(
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
    set_s: &IndexMap<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
    root_in_g: &NodeIndex,
) -> Vec<(NodeIndex, NodeIndex)> {
    let root_h = find_root(graph_h);
    let mut to_return = Vec::new();
    let mut set_to_find_mapping = vec![(root_h, *root_in_g)];
    while !set_to_find_mapping.is_empty() {
        let key = set_to_find_mapping.pop().unwrap();
        to_return.push(key);

        if set_s[&(key.1, key.0)].contains_key(&key.0) {
            for map in set_s[&(key.1, key.0)][&key.0].as_ref() {
                for mapping in map {
                    if !to_return.contains(&(mapping.1, mapping.0)) {
                        to_return.push((mapping.1, mapping.0));
                        set_to_find_mapping.push(*mapping);
                    }
                }
            }
        }
    }
    return to_return;
}

fn find_mapping_shamir_inner_loop(
    v: NodeIndex,
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
    set_s: &mut IndexMap<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
) -> (bool, Option<NodeIndex>) {
    let root_h = find_root(&graph_h);
    let v_neighbors: Vec<NodeIndex> = graph_g.neighbors_undirected(v).collect();
    for u in graph_h.node_indices() {
        let u_neighbors: Vec<NodeIndex> = graph_h.neighbors_undirected(u).collect();
        // all vertices of degree at most t+1
        if u_neighbors.len() > v_neighbors.len() + 1 {
            continue;
        }

        // maximum matching where X0 = X
        let (cost, path) = max_matching::<DenseCapacity<_>>(
            &u_neighbors,
            &v_neighbors,
            graph_g,
            graph_h,
            set_s,
            u,
        );
        if cost == u_neighbors.len() {
            if set_s[&(v, u)].contains_key(&u) {
            } else {
                set_s.get_mut(&(v, u)).unwrap().insert(u, Some(path));
            }
        }

        // maximum matching where X0 is X minus an element
        for vertex in 0..u_neighbors.len() {
            let mut new_x_set = u_neighbors.clone();
            let vertex_id = new_x_set.remove(vertex);
            let (cost, path) = max_matching::<DenseCapacity<_>>(
                &new_x_set,
                &v_neighbors,
                graph_g,
                graph_h,
                set_s,
                u,
            );
            if cost == new_x_set.len() {
                if set_s[&(v, u)].contains_key(&vertex_id) {
                } else {
                    set_s
                        .get_mut(&(v, u))
                        .unwrap()
                        .insert(vertex_id, Some(path));
                }
            }
        }

        // lines 12-14
        if set_s[&(v, root_h)].contains_key(&root_h) {
            if has_property_subset(
                &graph_g.node_weight(v).unwrap().1,
                &graph_h.node_weight(root_h).unwrap().1,
            ) {
                return (true, Some(v));
            }
        }
    }
    return (false, None);
}

// ----------------- Shamir Isomorphism Algorithm Centralized ----------------

// this performs lines 0-4 in the Shamir paper figure 3
fn initialize_s(
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
) -> IndexMap<(NodeIndex, NodeIndex), IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>> {
    let mut s = IndexMap::<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >::new();
    for node_g in graph_g.node_indices() {
        for u in graph_h.node_indices() {
            // initialize S entry as empty set
            s.insert((node_g, u), IndexMap::new());
        }
    }
    let root_g = find_root(&graph_g);
    let root_h = find_root(&graph_h);
    for leaf_g in find_leaves(root_g, &graph_g) {
        for leaf_h in find_leaves(root_h, &graph_h) {
            s.get_mut(&(leaf_g, leaf_h))
                .unwrap()
                .insert(leaf_h, Some(vec![(leaf_h, leaf_g)]));
            for neighbor in graph_h.neighbors_directed(leaf_h, Incoming) {
                s.get_mut(&(leaf_g, leaf_h))
                    .unwrap()
                    .insert(neighbor, Some(vec![(leaf_h, leaf_g)]));
            }
        }
    }
    return s;
}

pub fn find_mapping_shamir_centralized(
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
) -> Option<Vec<(NodeIndex, NodeIndex)>> {
    // TODO:  before even dealing with isomorphism, ask if breadth,
    // height, num nodes match up
    if graph_g.node_count() < graph_h.node_count() {
        return None;
    }

    // initialize S with all N(u) sets, lines 1-4
    let mut set_s = initialize_s(graph_g, graph_h);
    let root_g = find_root(graph_g);

    // postorder traversal and filtering of children for degrees, lines 5-8;
    let mut post_order = DfsPostOrder::new(graph_g, root_g);
    while let Some(node) = post_order.next(graph_g) {
        let (mapping_found, mapping_root) =
            find_mapping_shamir_inner_loop(node, graph_g, graph_h, &mut set_s);
        if mapping_found {
            return Some(get_mapping_from_set_s(
                graph_g,
                graph_h,
                &set_s,
                &mapping_root.unwrap(),
            ));
        }
    }
    // line 15
    return None;
}

// ---------------- Shamir Isomorphism Algorithm Decentralized ---------------
fn initialize_s_for_node(
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
    set_s: &mut IndexMap<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
    node: NodeIndex,
) {
    for u in graph_h.node_indices() {
        // initialize S entry as empty set
        set_s.insert((node, u), IndexMap::new());
    }
    let root_h = find_root(&graph_h);

    // if I am a leaf
    if graph_g.neighbors_directed(node, Outgoing).count() == 0 {
        for leaf_h in find_leaves(root_h, &graph_h) {
            set_s
                .get_mut(&(node, leaf_h))
                .unwrap()
                .insert(leaf_h, Some(vec![(leaf_h, node)]));
            for neighbor in graph_h.neighbors_directed(leaf_h, Incoming) {
                set_s
                    .get_mut(&(node, leaf_h))
                    .unwrap()
                    .insert(neighbor, Some(vec![(leaf_h, node)]));
            }
        }
    }
}

pub fn find_mapping_shamir_decentralized(
    graph_g: &Graph<(String, IndexMap<String, String>), String>,
    graph_h: &Graph<(String, IndexMap<String, String>), String>,
    set_s: &mut IndexMap<
        (NodeIndex, NodeIndex),
        IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
    >,
    cur_node: NodeIndex, // what node we are in graph_g
    am_root: bool,
) -> Option<Vec<(NodeIndex, NodeIndex)>> {
    // 1. Add yourself (that is, all your entries) to set S
    initialize_s_for_node(graph_g, graph_h, set_s, cur_node);

    // 2. For all your children, run inner loop
    let mut mapping_root_for_children = None;
    for child in graph_g.neighbors_directed(cur_node, Outgoing) {
        let (mapping_found, mapping_root) =
            find_mapping_shamir_inner_loop(child, graph_g, graph_h, set_s);
        if !am_root && mapping_found {
            mapping_root_for_children = mapping_root;
        }
    }

    // 2a. If one of your children matched all of graph_h, return that matching
    if mapping_root_for_children.is_some() {
        return Some(get_mapping_from_set_s(
            graph_g,
            graph_h,
            &set_s,
            &mapping_root_for_children.unwrap(),
        ));
    }

    // 3. If you are the root, run the inner loop for yourself as well
    if am_root {
        let (mapping_found, mapping_root) =
            find_mapping_shamir_inner_loop(cur_node, graph_g, graph_h, set_s);
        if mapping_found {
            return Some(get_mapping_from_set_s(
                graph_g,
                graph_h,
                &set_s,
                &mapping_root.unwrap(),
            ));
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_node_with_id;

    /// --------------- Graph Creation Helper functions -------------------
    fn three_node_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a = graph.add_node(("a".to_string(), IndexMap::new()));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));
        let c = graph.add_node(("c".to_string(), IndexMap::new()));
        graph.add_edge(a, b, String::new());
        graph.add_edge(a, c, String::new());
        return graph;
    }

    fn three_node_chain_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a = graph.add_node(("a".to_string(), IndexMap::new()));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));
        let c = graph.add_node(("c".to_string(), IndexMap::new()));
        graph.add_edge(a, b, String::new());
        graph.add_edge(b, c, String::new());
        return graph;
    }

    fn two_node_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a = graph.add_node(("a".to_string(), IndexMap::new()));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));

        graph.add_edge(a, b, String::new());
        return graph;
    }

    fn three_node_graph_with_properties() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "100".to_string()),
            ("breadth".to_string(), "5".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let a = graph.add_node(("a".to_string(), a_hashmap));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));
        let c = graph.add_node(("c".to_string(), IndexMap::new()));
        graph.add_edge(a, b, String::new());
        graph.add_edge(a, c, String::new());
        return graph;
    }

    fn two_node_graph_with_properties() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a_hashmap: IndexMap<String, String> = [("height".to_string(), "100".to_string())]
            .iter()
            .cloned()
            .collect();
        let a = graph.add_node(("a".to_string(), a_hashmap));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));

        graph.add_edge(a, b, String::new());
        return graph;
    }

    fn two_node_graph_with_wrong_properties() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a_hashmap: IndexMap<String, String> = [("height".to_string(), "1".to_string())]
            .iter()
            .cloned()
            .collect();
        let a = graph.add_node(("a".to_string(), a_hashmap));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));

        graph.add_edge(a, b, String::new());
        return graph;
    }

    fn chain_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let a = graph.add_node(("a".to_string(), IndexMap::new()));
        let b = graph.add_node(("b".to_string(), IndexMap::new()));
        let c = graph.add_node(("c".to_string(), IndexMap::new()));
        let star = graph.add_node(("*".to_string(), IndexMap::new()));

        graph.add_edge(a, b, String::new());
        graph.add_edge(b, c, String::new());
        graph.add_edge(c, star, String::new());
        return graph;
    }

    // from figure 2 in shamir paper
    fn g_figure_2() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let r = graph.add_node((String::from("r"), IndexMap::new()));
        let v = graph.add_node((String::from("v"), IndexMap::new()));
        let v1 = graph.add_node((String::from("v1"), IndexMap::new()));
        let v2 = graph.add_node((String::from("v2"), IndexMap::new()));
        let v3 = graph.add_node((String::from("v3"), IndexMap::new()));

        let left_unnamed_child = graph.add_node((String::from("leftchild"), IndexMap::new()));
        let right_unnamed_child = graph.add_node((String::from("rightchild"), IndexMap::new()));

        graph.add_edge(r, v, String::new());
        graph.add_edge(v, v1, String::new());
        graph.add_edge(v, v2, String::new());
        graph.add_edge(v, v3, String::new());
        graph.add_edge(v1, left_unnamed_child, String::new());
        graph.add_edge(v1, right_unnamed_child, String::new());

        return graph;
    }

    // from figure 2 in shamir paper
    fn h_figure_2() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let u = graph.add_node((String::from("u"), IndexMap::new()));
        let u1 = graph.add_node((String::from("u1"), IndexMap::new()));
        let u2 = graph.add_node((String::from("u2"), IndexMap::new()));
        let u3 = graph.add_node((String::from("u3"), IndexMap::new()));
        let u1_left_child = graph.add_node((String::from("u1left"), IndexMap::new()));
        let u1_right_child = graph.add_node((String::from("u1right"), IndexMap::new()));
        let u3_child = graph.add_node((String::from("u3child"), IndexMap::new()));

        graph.add_edge(u, u1, String::new());
        graph.add_edge(u, u2, String::new());
        graph.add_edge(u, u3, String::new());
        graph.add_edge(u1, u1_left_child, String::new());
        graph.add_edge(u1, u1_right_child, String::new());
        graph.add_edge(u3, u3_child, String::new());

        return graph;
    }

    fn three_child_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let root = graph.add_node((String::from("root"), IndexMap::new()));
        let child1 = graph.add_node((String::from("child1"), IndexMap::new()));
        let child2 = graph.add_node((String::from("child2"), IndexMap::new()));
        let child3 = graph.add_node((String::from("child3"), IndexMap::new()));

        graph.add_edge(root, child1, String::new());
        graph.add_edge(root, child2, String::new());
        graph.add_edge(root, child3, String::new());

        return graph;
    }

    fn four_child_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let root = graph.add_node((String::from("root"), IndexMap::new()));
        let child1 = graph.add_node((String::from("child1"), IndexMap::new()));
        let child2 = graph.add_node((String::from("child2"), IndexMap::new()));
        let child3 = graph.add_node((String::from("child3"), IndexMap::new()));
        let child4 = graph.add_node((String::from("child4"), IndexMap::new()));

        graph.add_edge(root, child1, String::new());
        graph.add_edge(root, child2, String::new());
        graph.add_edge(root, child3, String::new());
        graph.add_edge(root, child4, String::new());

        return graph;
    }

    fn bookinfo_trace_graph() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph = Graph::<(String, IndexMap<String, String>), String>::new();
        let productpage = graph.add_node((String::from("productpage-v1"), IndexMap::new()));
        let reviews = graph.add_node((String::from("reviews-v1"), IndexMap::new()));
        let ratings = graph.add_node((String::from("ratings-v1"), IndexMap::new()));
        let details = graph.add_node((String::from("details-v1"), IndexMap::new()));

        graph.add_edge(productpage, reviews, String::new());
        graph.add_edge(productpage, details, String::new());
        graph.add_edge(reviews, ratings, String::new());

        return graph;
    }

    fn simulation_example() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph_g = Graph::<(String, IndexMap<String, String>), String>::new();
        let prod_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "2".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "productpage-v1".to_string(),
            ),
            ("service_name".to_string(), "productpage-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let prod = graph_g.add_node(("productpage-v1".to_string(), prod_hashmap));

        let ratings_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "0".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "ratings-v1".to_string(),
            ),
            ("service_name".to_string(), "ratings-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let ratings = graph_g.add_node(("ratings-v1".to_string(), ratings_hashmap));

        let reviews_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "1".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "reviews-v1".to_string(),
            ),
            ("service_name".to_string(), "reviews-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let reviews = graph_g.add_node(("reviews-v1".to_string(), reviews_hashmap));

        let details_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "0".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "details-v1".to_string(),
            ),
            ("service_name".to_string(), "details-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let details = graph_g.add_node(("details-v1".to_string(), details_hashmap));

        graph_g.add_edge(prod, reviews, String::new());
        graph_g.add_edge(reviews, ratings, String::new());
        graph_g.add_edge(prod, details, String::new());
        return graph_g;
    }

    fn simulation_example_no_properties() -> Graph<(String, IndexMap<String, String>), String> {
        let mut graph_g = Graph::<(String, IndexMap<String, String>), String>::new();
        let prod = graph_g.add_node(("productpage-v1".to_string(), IndexMap::new()));

        let ratings = graph_g.add_node(("ratings-v1".to_string(), IndexMap::new()));

        let reviews = graph_g.add_node(("reviews-v1".to_string(), IndexMap::new()));

        let details = graph_g.add_node(("details-v1".to_string(), IndexMap::new()));

        graph_g.add_edge(prod, reviews, String::new());
        graph_g.add_edge(reviews, ratings, String::new());
        graph_g.add_edge(prod, details, String::new());
        return graph_g;
    }

    // ---------------------- Shamir Tests -------------------------

    #[test]
    fn test_initialize_s() {
        let graph_g = three_node_graph();
        let graph_h = two_node_graph();
        let s = initialize_s(&graph_g, &graph_h);
        assert!(s.keys().count() == 6);

        // useful debugging if this fails
        for key in s.keys() {
            print!(
                "key: {:?} weight: {:?}, {:?}\n",
                key,
                graph_g.node_weight(key.0),
                graph_h.node_weight(key.1)
            );
        }

        let aa = (
            get_node_with_id(&graph_g, "a".to_string()).unwrap(),
            get_node_with_id(&graph_h, "a".to_string()).unwrap(),
        );
        let ab = (
            get_node_with_id(&graph_g, "a".to_string()).unwrap(),
            get_node_with_id(&graph_h, "b".to_string()).unwrap(),
        );

        let ba = (
            get_node_with_id(&graph_g, "b".to_string()).unwrap(),
            get_node_with_id(&graph_h, "a".to_string()).unwrap(),
        );
        let bb = (
            get_node_with_id(&graph_g, "b".to_string()).unwrap(),
            get_node_with_id(&graph_h, "b".to_string()).unwrap(),
        );

        let ca = (
            get_node_with_id(&graph_g, "c".to_string()).unwrap(),
            get_node_with_id(&graph_h, "a".to_string()).unwrap(),
        );
        let cb = (
            get_node_with_id(&graph_g, "c".to_string()).unwrap(),
            get_node_with_id(&graph_h, "b".to_string()).unwrap(),
        );

        assert!(s.contains_key(&aa));
        assert!(s.contains_key(&ab));

        assert!(s.contains_key(&ba));
        assert!(s.contains_key(&bb));

        assert!(s.contains_key(&ca));
        assert!(s.contains_key(&cb));

        assert!(s[&aa].len() == 0);
        assert!(s[&ba].len() == 0);
        assert!(s[&ca].len() == 0);

        assert!(s[&bb].len() == 2, "bb len is {:?}", s[&bb].len());
        assert!(s[&cb].len() == 2, "cb len is {:?}", s[&cb].len());
    }

    #[test]
    fn test_shamir_small_graphs() {
        let graph_g = three_node_graph();
        let graph_h = two_node_graph();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_some());
    }
    #[test]
    fn test_shamir_figure_2() {
        let graph_g = g_figure_2();
        let graph_h = h_figure_2();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_none());
    }

    #[test]
    fn test_shamir_chain_graphs() {
        let graph_g = chain_graph();
        let graph_h_1 = two_node_graph();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h_1).is_some());
    }

    #[test]
    fn test_shamir_branching_graphs() {
        let graph_g = four_child_graph();
        let graph_h = three_child_graph();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_some());

        let graph_g_2 = three_child_graph();
        let graph_h_2 = four_child_graph();
        assert!(find_mapping_shamir_centralized(&graph_g_2, &graph_h_2).is_none());
    }

    #[test]
    fn test_shamir_on_bookinfo() {
        let graph_g = bookinfo_trace_graph();
        let graph_h = three_node_graph();
        let mut mapping_wrapped = find_mapping_shamir_centralized(&graph_g, &graph_h);
        assert!(mapping_wrapped.is_some());
        let mapping = mapping_wrapped.unwrap();
        let a = get_node_with_id(&graph_h, "a".to_string()).unwrap();
        let b = get_node_with_id(&graph_h, "b".to_string()).unwrap();
        let c = get_node_with_id(&graph_h, "c".to_string()).unwrap();
        let prod = get_node_with_id(&graph_g, "productpage-v1".to_string()).unwrap();
        let det = get_node_with_id(&graph_g, "details-v1".to_string()).unwrap();
        let rev = get_node_with_id(&graph_g, "reviews-v1".to_string()).unwrap();
        assert!(mapping.contains(&(a, prod)));
        assert!(mapping.contains(&(b, det)) || mapping.contains(&(c, det)));
        assert!(mapping.contains(&(b, rev)) || mapping.contains(&(c, rev)));

        let graph_g_2 = bookinfo_trace_graph();
        let graph_h_2 = three_node_chain_graph();
        let mut mapping_wrapped_2 = find_mapping_shamir_centralized(&graph_g_2, &graph_h_2);
        assert!(mapping_wrapped_2.is_some());
        let mapping_2 = mapping_wrapped_2.unwrap();
        let a_2 = get_node_with_id(&graph_h_2, "a".to_string()).unwrap();
        let b_2 = get_node_with_id(&graph_h_2, "b".to_string()).unwrap();
        let c_2 = get_node_with_id(&graph_h_2, "c".to_string()).unwrap();
        let prod_2 = get_node_with_id(&graph_g_2, "productpage-v1".to_string()).unwrap();
        let rev_2 = get_node_with_id(&graph_g_2, "reviews-v1".to_string()).unwrap();
        assert!(mapping_2.contains(&(a_2, prod_2)));
        assert!(mapping_2.contains(&(b_2, rev_2)));
    }

    #[test]
    fn test_shamir_full_match() {
        let graph_g = three_node_graph();
        let graph_h = three_node_graph();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_some());
    }

    #[test]
    fn test_property_matches() {
        let graph_g = three_node_graph_with_properties();
        let graph_h = two_node_graph_with_properties();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_some());
        let graph_h_2 = two_node_graph();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h_2).is_some());
    }

    #[test]
    fn test_property_does_not_match() {
        let graph_g = three_node_graph_with_properties();
        let graph_h = two_node_graph_with_wrong_properties();
        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_none());

        let graph_g_2 = three_node_graph();
        let graph_h_2 = two_node_graph_with_wrong_properties();
        assert!(find_mapping_shamir_centralized(&graph_g_2, &graph_h_2).is_none());
    }

    #[test]
    fn test_simulation_example() {
        let graph_g = simulation_example_no_properties();
        let mut graph_h = Graph::<(String, IndexMap<String, String>), String>::new();
        let a = graph_h.add_node(("a".to_string(), IndexMap::new()));
        let b = graph_h.add_node(("b".to_string(), IndexMap::new()));
        graph_h.add_edge(a, b, String::new());

        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_some());

        let graph_g_2 = simulation_example();
        assert!(find_mapping_shamir_centralized(&graph_g_2, &graph_h).is_some());
    }

    #[test]
    fn test_simulation_example_no_match() {
        let graph_g = simulation_example();

        let mut graph_h = Graph::<(String, IndexMap<String, String>), String>::new();
        let a_hashmap: IndexMap<String, String> = [("height".to_string(), "0".to_string())]
            .iter()
            .cloned()
            .collect();
        let a = graph_h.add_node((String::from("productpage-v1"), a_hashmap));
        let b = graph_h.add_node((String::from("reviews-v1"), IndexMap::new()));
        let c = graph_h.add_node((String::from("ratings-v1"), IndexMap::new()));

        graph_h.add_edge(a, b, String::new());
        graph_h.add_edge(b, c, String::new());

        assert!(find_mapping_shamir_centralized(&graph_g, &graph_h).is_none());
    }

    #[test]
    fn test_decentralized() {
        let mut set_s = IndexMap::<
            (NodeIndex, NodeIndex),
            IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
        >::new();
        let graph_h = three_node_chain_graph();

        let mut graph_g = Graph::<(String, IndexMap<String, String>), String>::new();
        let a = graph_g.add_node((String::from("a"), IndexMap::new()));
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, a, false);
        assert!(ret.is_none());

        let b = graph_g.add_node((String::from("b"), IndexMap::new()));
        graph_g.add_edge(b, a, String::new());
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, b, false);
        assert!(ret.is_none());

        let c = graph_g.add_node((String::from("c"), IndexMap::new()));
        graph_g.add_edge(c, b, String::new());
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, c, false);
    }

    #[test]
    fn test_decentralized_complex() {
        let mut set_s = IndexMap::<
            (NodeIndex, NodeIndex),
            IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
        >::new();

        // create graph h
        let mut graph_h = Graph::<(String, IndexMap<String, String>), String>::new();
        let a_hashmap: IndexMap<String, String> = [("height".to_string(), "2".to_string())]
            .iter()
            .cloned()
            .collect();
        let a = graph_h.add_node((String::from("productpage-v1"), a_hashmap));
        let b = graph_h.add_node((String::from("reviews-v1"), IndexMap::new()));
        let c = graph_h.add_node((String::from("ratings-v1"), IndexMap::new()));

        graph_h.add_edge(a, b, String::new());
        graph_h.add_edge(b, c, String::new());

        //create graph g
        let mut graph_g = Graph::<(String, IndexMap<String, String>), String>::new();
        let ratings_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "0".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "ratings-v1".to_string(),
            ),
            ("service_name".to_string(), "ratings-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let ratings = graph_g.add_node(("ratings-v1".to_string(), ratings_hashmap));
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, ratings, false);
        assert!(ret.is_none());

        let reviews_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "1".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "reviews-v1".to_string(),
            ),
            ("service_name".to_string(), "reviews-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let reviews = graph_g.add_node(("reviews-v1".to_string(), reviews_hashmap));
        graph_g.add_edge(reviews, ratings, String::new());
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, reviews, false);
        assert!(ret.is_none());

        let prod_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "2".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "productpage-v1".to_string(),
            ),
            ("service_name".to_string(), "productpage-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let prod = graph_g.add_node(("productpage-v1".to_string(), prod_hashmap));

        graph_g.add_edge(prod, reviews, String::new());
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, prod, true);
        assert!(ret.is_some());
    }

    #[test]
    fn test_decentralized_complex_wrong_properties() {
        let mut set_s = IndexMap::<
            (NodeIndex, NodeIndex),
            IndexMap<NodeIndex, Option<Vec<(NodeIndex, NodeIndex)>>>,
        >::new();

        // create graph h
        let mut graph_h = Graph::<(String, IndexMap<String, String>), String>::new();
        let a_hashmap: IndexMap<String, String> = [("height".to_string(), "2".to_string())]
            .iter()
            .cloned()
            .collect();
        let a = graph_h.add_node((String::from("productpage-v1"), a_hashmap));
        let b = graph_h.add_node((String::from("reviews-v1"), IndexMap::new()));
        let c = graph_h.add_node((String::from("ratings-v1"), IndexMap::new()));

        graph_h.add_edge(a, b, String::new());
        graph_h.add_edge(b, c, String::new());

        //create graph g
        let mut graph_g = Graph::<(String, IndexMap<String, String>), String>::new();
        let ratings_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "0".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "ratings-v1".to_string(),
            ),
            ("service_name".to_string(), "ratings-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let ratings = graph_g.add_node(("ratings-v1".to_string(), ratings_hashmap));
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, ratings, false);
        assert!(ret.is_none());

        let reviews_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "1".to_string()),
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "reviews-v1".to_string(),
            ),
            ("service_name".to_string(), "reviews-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let reviews = graph_g.add_node(("reviews-v1".to_string(), reviews_hashmap));
        graph_g.add_edge(reviews, ratings, String::new());
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, reviews, false);
        assert!(ret.is_none());

        let prod_hashmap: IndexMap<String, String> = [
            ("height".to_string(), "0".to_string()), // WRONG PROPERTY, should make this fail
            (
                "node.metadata.WORKLOAD_NAME".to_string(),
                "productpage-v1".to_string(),
            ),
            ("service_name".to_string(), "productpage-v1".to_string()),
        ]
        .iter()
        .cloned()
        .collect();
        let prod = graph_g.add_node(("productpage-v1".to_string(), prod_hashmap));

        graph_g.add_edge(prod, reviews, String::new());
        let ret = find_mapping_shamir_decentralized(&graph_g, &graph_h, &mut set_s, prod, true);
        assert!(ret.is_none());
    }
}
