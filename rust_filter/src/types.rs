use std::collections::HashMap;
use std::fs;
use crate::rpc::Rpc;

pub type CodeletType = fn(&Filter, &Rpc) -> Option<Rpc>;


// user defined functions:
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



// This represents a piece of state of the filter
// it either contains a user defined function, or some sort of 
// other persistent state
pub struct State {
    pub type_of_state: Option<String>,
    pub string_data: Option<String>,
    pub udf_count: Option<Count>,
}

pub struct Filter {
    pub filter_state: HashMap<String, State>,
}
