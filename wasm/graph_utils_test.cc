#include <map>
#include <string>

#include "boost/graph/adjacency_list.hpp"
#include "boost/graph/directed_graph.hpp"
#include "boost/graph/vf2_sub_graph_iso.hpp"
#include "gmock/gmock.h"
#include "gtest/gtest.h"

#include "graph_utils.h"

TEST(GraphUtilsTest, DirectedGraph) {
  typedef boost::directed_graph<> graph_type;

  graph_type graph1;
  auto v0 = graph1.add_vertex();
  auto v1 = graph1.add_vertex();
  auto v2 = graph1.add_vertex();
  graph1.add_edge(v2, v0);
  graph1.add_edge(v2, v1);

  graph_type graph2;
  v0 = graph2.add_vertex();
  v1 = graph2.add_vertex();
  v2 = graph2.add_vertex();
  auto v3 = graph2.add_vertex();
  graph2.add_edge(v0, v1);
  graph2.add_edge(v1, v2);
  graph2.add_edge(v0, v3);

  boost::vf2_print_callback<graph_type, graph_type> callback(graph1, graph2);

  ASSERT_TRUE(vf2_subgraph_iso(graph1, graph2, callback));
}

TEST(GraphUtilsTest, DirectedGraphWithProperties) {
  typedef boost::directed_graph<Node> graph_type;

  graph_type graph1;

  Node node2;
  node2.properties.insert({{"workload_name"}, "productpagev1"});
  auto v0 = graph1.add_vertex();
  auto v1 = graph1.add_vertex();
  auto v2 = graph1.add_vertex(node2);
  graph1.add_edge(v2, v0);
  graph1.add_edge(v2, v1);

  graph_type graph2;
  v0 = graph2.add_vertex();
  v1 = graph2.add_vertex();
  v2 = graph2.add_vertex();
  auto v3 = graph2.add_vertex();
  graph2.add_edge(v0, v1);
  graph2.add_edge(v1, v2);
  graph2.add_edge(v0, v3);

  boost::vf2_print_callback<graph_type, graph_type> callback(graph1, graph2);

  auto vertex_comp = boost::make_property_map_equivalent(
      boost::get(&Node::properties, graph1),
      boost::get(&Node::properties, graph2));

  // When called without vertex_property_map
  ASSERT_FALSE(boost::vf2_subgraph_iso(
      graph1, graph2, callback, boost::vertex_order_by_mult(graph1),
      edges_equivalent(boost::always_equivalent())
          .vertices_equivalent(vertex_comp)));

  graph_type graph3;
  Node node0;
  node0.properties.insert({{"workload_name"}, "productpagev1"});

  v0 = graph3.add_vertex(node0);
  v1 = graph3.add_vertex();
  v2 = graph3.add_vertex();
  v3 = graph3.add_vertex();
  graph3.add_edge(v0, v1);
  graph3.add_edge(v1, v2);
  graph3.add_edge(v0, v3);

  auto vertex_comp2 = boost::make_property_map_equivalent(
      boost::get(&Node::properties, graph1),
      boost::get(&Node::properties, graph3));

  ASSERT_TRUE(boost::vf2_subgraph_iso(
      graph1, graph3, callback, boost::vertex_order_by_mult(graph1),
      edges_equivalent(boost::always_equivalent())
          .vertices_equivalent(vertex_comp2)));
}

TEST(GraphUtilsTest, DirectedGraphPropertySubset) {
  typedef boost::directed_graph<Node> graph_type;

  graph_type graph1;
  Node node0;
  node0.properties.insert({{"workload_name"}, "productpagev1"});
  graph1.add_vertex(node0);

  graph_type graph2;
  Node node1;
  node1.properties.insert({{"workload_name"}, "productpagev1"});
  node1.properties.insert({{"x", "y", "z"}, "abc"});
  graph2.add_vertex(node1);

  auto vertex_comp = boost::make_property_map_equivalent(
      boost::get(&Node::properties, graph1),
      boost::get(&Node::properties, graph2));

  boost::vf2_print_callback<graph_type, graph_type> callback(graph1, graph2);

  ASSERT_FALSE(boost::vf2_subgraph_iso(
      graph1, graph2, callback, boost::vertex_order_by_mult(graph1),
      edges_equivalent(boost::always_equivalent())
          .vertices_equivalent(vertex_comp)));

  auto vertex_comp2 =
      make_property_map_subset(boost::get(&Node::properties, graph1),
                               boost::get(&Node::properties, graph2));

  ASSERT_TRUE(boost::vf2_subgraph_iso(
      graph1, graph2, callback, boost::vertex_order_by_mult(graph1),
      edges_equivalent(boost::always_equivalent())
          .vertices_equivalent(vertex_comp2)));
}

TEST(GenerateTraceGraphFromHeadersTest, ReturnsGraph) {
  std::string paths_header = "a-b-c,a-d";
  std::string properties_header = "a.x.y.z==123";

  trace_graph_t graph =
      generate_trace_graph_from_headers(paths_header, properties_header);
  EXPECT_EQ(graph.num_vertices(), 4);
  EXPECT_EQ(graph.num_edges(), 3);

  trace_graph_t expected_graph;
  auto v0 = expected_graph.add_vertex();
  auto v1 = expected_graph.add_vertex();
  auto v2 = expected_graph.add_vertex();
  auto v3 = expected_graph.add_vertex();

  expected_graph.add_edge(v0, v1);
  expected_graph.add_edge(v1, v2);
  expected_graph.add_edge(v0, v3);

  boost::vf2_print_callback<trace_graph_t, trace_graph_t> callback(
      graph, expected_graph);

  auto vertex_comp =
      make_property_map_subset(boost::get(&Node::properties, graph),
                               boost::get(&Node::properties, expected_graph));

  EXPECT_FALSE(boost::vf2_subgraph_iso(
      graph, expected_graph, callback, boost::vertex_order_by_mult(graph),
      edges_equivalent(boost::always_equivalent())
          .vertices_equivalent(vertex_comp)));

  // Add expected property
  boost::put(&Node::properties, expected_graph, v0,
             std::map<std::vector<std::string>, std::string>{
                 {{"x", "y", "z"}, "123"}});

  EXPECT_TRUE(boost::vf2_subgraph_iso(
      graph, expected_graph, callback, boost::vertex_order_by_mult(graph),
      edges_equivalent(boost::always_equivalent())
          .vertices_equivalent(vertex_comp)));
}

TEST(GenerateTraceGraphFromHeadersTest, EmptyInputs) {
  trace_graph_t graph = generate_trace_graph_from_headers("", "");
  EXPECT_EQ(graph.num_vertices(), 0);
  EXPECT_EQ(graph.num_edges(), 0);
}

TEST(GenerateTraceGraphTest, EmptyInputs) {
  trace_graph_t graph = generate_trace_graph({}, {});
  EXPECT_EQ(graph.num_vertices(), 0);
  EXPECT_EQ(graph.num_edges(), 0);
}