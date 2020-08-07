#ifndef GRAPH_UTILS_H
#define GRAPH_UTILS_H

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
        ids_to_properties);

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
                                                std::string properties_header);

// If graph_small is sub graph isomorphic, using above property_map_subset,
// returns the sub graph mapping from graph small's node ids to graph large's
// node ids. If not returns nullptr
std::unique_ptr<std::map<std::string, std::string>>
get_sub_graph_mapping(const trace_graph_t &graph_small,
                      const trace_graph_t &graph_large);

// Returns vertex_iterator which has id
const Node *get_node_with_id(const trace_graph_t &g, std::string_view id);

#endif // GRAPH_UTILS_H