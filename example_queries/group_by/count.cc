// udf_type: Aggregation
// id: count
// return_type: int

class count {
public:
  int operator()(std::string height) {

    buckets_[height] += 1;

    return buckets_[height];
  }

  std::map<std::string, int> buckets_;
};