// udf_type: Aggregation
// init_func: new
// exec_func: execute
// struct_name: Histogram
// id: histogram

use petgraph::Graph;

#[derive(Clone, Debug)]
pub struct Histogram {
    buckets: HashMap<u64, u64>
}

impl Histogram {
    fn new() -> Histogram {
        Histogram { buckets: HashMap::new() }
    }
    fn execute(&mut self, value: u64) -> String {
        if self.buckets.contains_key(&value) {
            *self.buckets.get_mut(&value).unwrap() += 1;
        }
        else {
            self.buckets.insert(value, 1);
        }
        // now we get a string representation
        let mut to_return = String::from("Hist: ");
        for key in self.buckets.keys() {
            let mut pair = format!(" ({key}, {value}) ", key=key.to_string(), value=self.buckets[key].to_string());
            to_return.push_str(&pair);
        }
        to_return.push_str("\n");
        return to_return;
    }
}
