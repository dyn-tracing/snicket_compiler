// udf_type: Scalar
// id: max_response_size
// return_type: int
// arg: target

class max_response_size : public user_func<int> {
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
