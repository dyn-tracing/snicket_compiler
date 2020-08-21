class request_count : public user_func<int> {
  int operator()(const trace_graph_t &graph) { return ++num_requests_; }

private
  int num_requests_ = 0;
};