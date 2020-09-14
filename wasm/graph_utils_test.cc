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
  trace_graph_t graph = generate_trace_graph({}, {}, {});
  EXPECT_EQ(graph.num_vertices(), 0);
  EXPECT_EQ(graph.num_edges(), 0);
}

TEST(GenerateTraceGraphFromHeadersTest, SingleNode) {
  trace_graph_t graph = generate_trace_graph_from_headers("a", "");
  EXPECT_EQ(graph.num_vertices(), 1);
  EXPECT_EQ(graph.num_edges(), 0);
}

TEST(GetSubGraphMappingTest, Simple) {
  trace_graph_t graph_small = generate_trace_graph_from_headers("a", "");
  trace_graph_t graph_large = generate_trace_graph_from_headers("b", "");

  auto mapping = get_sub_graph_mapping(graph_small, graph_large);

  ASSERT_NE(mapping, nullptr);
  EXPECT_EQ(mapping->size(), 1);
  EXPECT_THAT(mapping,
              testing::Pointee(testing::Contains(testing::Pair("a", "b"))));
}

TEST(GetSubGraphMappintTest, NoMapping) {
  trace_graph_t graph_small = generate_trace_graph_from_headers("a", "a.x==y");
  trace_graph_t graph_large = generate_trace_graph_from_headers("b", "b.x==z");

  auto mapping = get_sub_graph_mapping(graph_small, graph_large);

  ASSERT_EQ(mapping, nullptr);
}

TEST(GetSubGraphMappingTest, MultipleMapping) {
  // a could be mapped to either b or c.
  trace_graph_t graph_small = generate_trace_graph_from_headers("a", "");
  trace_graph_t graph_large = generate_trace_graph_from_headers("b-c", "");

  auto mapping = get_sub_graph_mapping(graph_small, graph_large);

  ASSERT_NE(mapping, nullptr);
  EXPECT_EQ(mapping->size(), 1);
  EXPECT_TRUE(mapping->at("a") == "b" || mapping->at("a") == "c");
}

TEST(GetSubGraphMappingTest, GetMappingAndProperty) {
  // a could be mapped to either b or c.
  trace_graph_t graph_small = generate_trace_graph_from_headers("a", "a.x==y");
  trace_graph_t graph_large =
      generate_trace_graph_from_headers("b-c", "c.x==y");

  auto mapping = get_sub_graph_mapping(graph_small, graph_large);

  ASSERT_NE(mapping, nullptr);
  EXPECT_EQ(mapping->size(), 1);
  EXPECT_THAT(mapping,
              testing::Pointee(testing::Contains(testing::Pair("a", "c"))));

  const auto *node = get_node_with_id(graph_large, "c");
  ASSERT_NE(node, nullptr);
  ASSERT_EQ(node->properties.at({"x"}), "y");
}

class dfs_max_value_visitor : public boost::default_dfs_visitor {
public:
  dfs_max_value_visitor(std::initializer_list<std::string_view> keys,
                        int *max) {
    key_ = {keys.begin(), keys.end()};
    max_ = max;
  }

  template <typename Vertex, typename Graph>
  void discover_vertex(Vertex u, const Graph &g) {
    auto map = g[u].properties;

    int value = std::atoi(map.at(key_).c_str());

    if (value > *max_) {
      *max_ = value;
    }
  }

  std::vector<std::string> key_;
  // Pointer needs to be passed in. When constructing a visitor using
  // boost::visitor, the function takes a const reference. Any computation
  // result must be stored at a memory location that outlives this object.
  int *max_;
};

TEST(VisitorTest, FindNodeWithMaxValue) {
  int max = INT_MIN;
  dfs_max_value_visitor vis({"start_time"}, &max);
  EXPECT_THAT(vis.key_, testing::ElementsAre("start_time"));

  auto graph = generate_trace_graph_from_headers(
      "a-b-c", "a.start_time==1,b.start_time==2,c.start_time==3");
  EXPECT_EQ(graph.num_vertices(), 3);
  EXPECT_EQ(graph.num_edges(), 2);

  auto a = get_node_with_id(graph, "a");
  EXPECT_EQ(a->properties.at(std::vector<std::string>{"start_time"}), "1");

  boost::depth_first_search(graph, boost::visitor(vis));

  EXPECT_EQ(*vis.max_, 3);
}

class scalar_func : public user_func<int> {
public:
  int operator()(const trace_graph_t &graph) { return 0; }
};

class scalar_dfs_max_value : public user_func<int> {
public:
  int operator()(const trace_graph_t &graph) {
    int max = INT_MIN;
    scalar_visitor vis(&max);
    boost::depth_first_search(graph, boost::visitor(vis));
    return max;
  }

private:
  class scalar_visitor : public boost::default_dfs_visitor {
  public:
    scalar_visitor(int *max) { max_ = max; }

    template <typename Vertex, typename Graph>
    void discover_vertex(Vertex u, const Graph &g) {
      auto map = g[u].properties;

      int value = std::atoi(map.at(key_).c_str());

      if (value > *max_) {
        *max_ = value;
      }
    }

    std::vector<std::string> key_{"start_time"};
    int *max_;
  };
};

class aggr_func : public user_func<int> {
public:
  int operator()(const trace_graph_t &graph) {
    num_vertices += graph.num_vertices();
    return num_vertices;
  }

private:
  int num_vertices = 0;
};

TEST(UserFuncTest, ScalarFunc) {
  auto graph = generate_trace_graph_from_headers("a", "");
  EXPECT_EQ(graph.num_vertices(), 1);
  EXPECT_EQ(graph.num_edges(), 0);

  scalar_func f;

  EXPECT_EQ(f(graph), 0);

  aggr_func f2;
  EXPECT_EQ(f2(graph), 1);
  EXPECT_EQ(f2(graph), 2);
}

TEST(UserFuncTest, ScalarInnerClassDef) {
  auto graph = generate_trace_graph_from_headers(
      "a-b-c", "a.start_time==1,b.start_time==2,c.start_time==3");

  scalar_dfs_max_value f;

  EXPECT_EQ(f(graph), 3);
}

class max_respons_size : public user_func<int> {
public:
  int operator()(const trace_graph_t &graph) {
    int max = INT_MIN;
    scalar_visitor vis(&max);
    boost::depth_first_search(graph, boost::visitor(vis));
    return max;
  }

private:
  class scalar_visitor : public boost::default_dfs_visitor {
  public:
    scalar_visitor(int *max) { max_ = max; }

    template <typename Vertex, typename Graph>
    void discover_vertex(Vertex u, const Graph &g) {
      auto map = g[u].properties;

      int value = std::atoi(map.at(key_).c_str());

      if (value > *max_) {
        *max_ = value;
      }
    }

    std::vector<std::string> key_{"response", "total_size"};
    int *max_;
  };
};

TEST(GraphPropertiesTest, Height) {
  auto graph = generate_trace_graph_from_headers("a-b-c,a-d", "");
  EXPECT_EQ(get_tree_height(graph), 2);
  EXPECT_EQ(get_tree_height(graph, "b"), 1);
  graph = generate_trace_graph_from_headers("", "");
  EXPECT_EQ(get_tree_height(graph), 0);
  graph = generate_trace_graph_from_headers("a-b-c-d,d-e", "");
  EXPECT_EQ(get_tree_height(graph), 4);
  graph = generate_trace_graph_from_headers("a-b-c-d,b-e,c-f,f-g,g-h", "");
  EXPECT_EQ(get_tree_height(graph), 5);
  graph = generate_trace_graph_from_headers("b-a,b-c", "");
  EXPECT_EQ(get_tree_height(graph), 1);
  graph = generate_trace_graph_from_headers("a", "");
  EXPECT_EQ(get_tree_height(graph), 0);
}

TEST(GraphUtilsTest, GetOutDegree) {
  auto graph = generate_trace_graph_from_headers("a-b-c,a-d", "");
  EXPECT_EQ(get_out_degree(graph, "a"), 2);
  EXPECT_EQ(get_out_degree(graph, "f"), -1);
  EXPECT_EQ(get_out_degree(graph, "b"), 1);
  EXPECT_EQ(get_out_degree(graph, "d"), 0);
}