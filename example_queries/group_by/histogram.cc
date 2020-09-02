// udf_type: Aggregation
// id: count
// return_type: int

class count {
public:
  std::pair<std::string, int> operator()(int height) {

    buckets_[height] += 1;

    return std::make_pair(std::to_string(height), buckets_[height]);
  }

  std::map<int, int> buckets_;
};