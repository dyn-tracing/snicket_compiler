#include <algorithm>
#include <map>
#include <memory>
#include <regex>
#include <set>
#include <string_view>

#include "boost/graph/directed_graph.hpp"
#include "boost/graph/vf2_sub_graph_iso.hpp"

#include "str_utils.h"

struct Node {
  // ID of the node, either specified by user query, or service_name from trace.
  std::string id;
  // Map from property names to values.
  std::map<std::vector<std::string>, std::string> properties;
};

typedef boost::directed_graph<Node> trace_graph_t;

// Binary function object that returns true if the values in item1 (a map)
// in property_map1 are contained in item2 (a map) in property_map2.
template <typename PropertyMapFirst, typename PropertyMapSecond>
struct property_map_subset {

  property_map_subset(const PropertyMapFirst property_map1,
                      const PropertyMapSecond property_map2)
      : m_property_map1(property_map1), m_property_map2(property_map2) {}

  template <typename ItemFirst, typename ItemSecond>
  bool operator()(const ItemFirst item1, const ItemSecond item2) {
    const auto &map1 = get(m_property_map1, item1);
    const auto &map2 = get(m_property_map2, item2);

    for (const auto &pair : map1) {
      if (map2.find(pair.first) == map2.end() ||
          map2.at(pair.first) != pair.second) {
        return false;
      }
    }

    return true;
  }

private:
  const PropertyMapFirst m_property_map1;
  const PropertyMapSecond m_property_map2;
};

// Returns a property_map_subset object that compares the values
// of property_map1 and property_map2.
template <typename PropertyMapFirst, typename PropertyMapSecond>
property_map_subset<PropertyMapFirst, PropertyMapSecond>
make_property_map_subset(const PropertyMapFirst property_map1,
                         const PropertyMapSecond property_map2) {

  return (property_map_subset<PropertyMapFirst, PropertyMapSecond>(
      property_map1, property_map2));
}

trace_graph_t generate_trace_graph(
    std::set<std::string> vertices,
    std::vector<std::pair<std::string, std::string>> edges,
    std::map<std::string, std::map<std::vector<std::string>, std::string>>
        ids_to_properties) {
  trace_graph_t graph;

  std::map<std::string, void *> ids_to_vertex_descriptors;

  for (const auto &vertex : vertices) {
    trace_graph_t::vertex_descriptor v;
    if (ids_to_properties.find(vertex) != ids_to_properties.end()) {
      v = graph.add_vertex(Node{vertex, ids_to_properties[vertex]});
    } else {
      v = graph.add_vertex();
    }
    ids_to_vertex_descriptors.insert({vertex, v});
  }

  for (const auto &edge : edges) {
    const auto &src_id = edge.first;
    auto *src_vertex = ids_to_vertex_descriptors[src_id];

    const auto &dst_id = edge.second;
    auto *dst_vertex = ids_to_vertex_descriptors[dst_id];

    graph.add_edge(src_vertex, dst_vertex);
  }

  return graph;
}

// Generate trace graph from a string representing paths and properties
// paths header
// a-b-c,a-d
// Above means following
// a has directed edge to b
// b has directed edge to c
// a has directed edge to d
//
// properties header
// a.x.y.z==123,b.y.z==456
trace_graph_t generate_trace_graph_from_headers(std::string paths_header,
                                                std::string properties_header) {
  std::vector<std::string> paths = str_split(paths_header, ",", true);

  std::set<std::string> vertices;

  std::vector<std::pair<std::string, std::string>> edges;
  for (const std::string &path : paths) {
    std::vector<std::string> vertices_vec = str_split(path, "-");
    for (int i = 0; i < vertices_vec.size(); ++i) {
      vertices.insert(vertices_vec[i]);
      if (i + 1 < vertices_vec.size()) {
        edges.push_back(std::make_pair(vertices_vec[i], vertices_vec[i + 1]));
      }
    }
  }

  std::map<std::string, std::map<std::vector<std::string>, std::string>>
      vertices_to_properties;
  std::vector<std::string> properties = str_split(properties_header, ",", true);
  for (const auto &property : properties) {
    // Given a.x.y.z == 123, the vector will have a, x, y, z, 123
    std::vector<std::string> property_split =
        str_split(property, R"([.]|(==)|(\s+))", /*filter_empty=*/true);
    const auto &vertex_id = property_split.front();
    const auto &value = property_split.back();

    vertices_to_properties[vertex_id].insert(
        {std::vector<std::string>{property_split.begin() + 1,
                                  property_split.end() - 1},
         value});
  }

  return generate_trace_graph(vertices, edges, vertices_to_properties);
}
