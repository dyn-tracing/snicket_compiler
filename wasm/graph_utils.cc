#include <map>
#include <set>
#include <string>
#include <vector>

#include "graph_utils.h"

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
