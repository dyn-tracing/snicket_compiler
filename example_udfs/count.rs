// init_func: new
// exec_func: execute
// struct_name: Count
// id: count

#[derive(Clone, Copy, Debug)]
pub struct Count {
    counter: u32
}

impl Count {
    fn new() -> Count {
        Count { counter: 0 }
    }
    fn execute(mut self) -> u32 {
        self.counter = self.counter + 1;
        self.counter
    }
}
