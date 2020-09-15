// udf_type: Aggregation
// id: avg
// return_type: float

class avg {
public:
  std::pair<std::string, float> operator()(int value) {
    avg_  = avg_ + ((float)value - avg_ ) / (count_ + 1);
    count_ += 1;

    return std::make_pair("moving_avg", avg_);
  }

  int count_ =  0;
  float avg_ = 0.0;
};