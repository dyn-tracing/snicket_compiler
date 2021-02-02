// udf_type: Aggregation
// init_func: new
// exec_func: execute
// struct_name: Avg
// id: avg

#[derive(Clone, Copy, Debug)]
pub struct Avg {
    avg: u64,
    total: u64,
    num_instances: u64,
}

impl Avg {
    fn new() -> Avg {
        Avg { avg: 0, total: 0 , num_instances: 0}
    }
    fn execute(&mut self, instance: u64) -> u64 {
        self.total += instance;
        self.num_instances += 1;
        self.avg = self.total/self.num_instances;
        self.avg
    }
}
