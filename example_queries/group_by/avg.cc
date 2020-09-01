// udf_type: Aggregation
// id: avg
// return_type: float

class avg {
public:
  float operator()(int value) {
    avg_  = avg_ + ((float)value - avg_ ) / (count_ + 1);
    count_ += 1;

    return avg_;
  }

  int count_ =  0;
  float avg_ = 0.0;
};