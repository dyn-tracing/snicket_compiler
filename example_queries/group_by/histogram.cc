// udf_type: Aggregation
// id: count
// return_type: int

class count {
public:
  int operator()(int height) {

    buckets_[height] += 1;

    return buckets_[height];
  }

  std::map<int, int> buckets_;
};