// udf_type: Scalar
// init_func: new
// exec_func: execute
// struct_name: Count
// id: count

use petgraph::Graph;

#[derive(Clone, Copy, Debug)]
pub struct Count {
    counter: u32
}

impl Count {
    fn new() -> Count {
        Count { counter: 0 }
    }
    fn execute(&mut self, target: Graph<(String, HashMap<String, String>), String>) -> u32 {
        self.counter = self.counter + 1;
        self.counter
    }
}
