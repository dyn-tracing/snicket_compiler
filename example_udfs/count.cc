// udf_type: Scalar
// id: count
// return_type: int

class count : public user_func<int> {
public:
  int operator()(const trace_graph_t &graph) {
    counter_ += 1;
    return counter_;
  }

private:
  int counter_ = 0;
};
