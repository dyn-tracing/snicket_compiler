use rpc_lib::rpc::Rpc;
use indexmap::map::IndexMap;
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use serde::{Serialize, Deserialize};
extern crate serde_yaml;

pub type CodeletType = fn(&Filter, &Rpc) -> Option<Rpc>;

fn log_setup() {                                                                
    // Build a stderr logger.                                                   
    let stderr = ConsoleAppender::builder()                                     
        .encoder(Box::new(PatternEncoder::new("{h({l})}: {m}\n")))              
        .target(Target::Stderr)                                                 
        .build();                                                               
    // Logging to log file.                                                     
    let logfile = FileAppender::builder()                                       
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html   
        .encoder(Box::new(PatternEncoder::new("{l}: {m}\n")))                   
        .append(false)                                                          
        .build("sim.log")                                                       
        .unwrap();                                                              
    // Log Trace level output to file where trace is the default level          
    // and the programmatically specified level to stderr.                      
    let config = Config::builder()                                              
        .appender(Appender::builder().build("logfile", Box::new(logfile)))      
        .appender(                                                              
            Appender::builder()                                                 
                .filter(Box::new(ThresholdFilter::new(log::LevelFilter::Info))) 
                .build("stderr", Box::new(stderr)),                             
        )                                                                       
        .build(                                                                 
            Root::builder()                                                     
                .appender("logfile")                                            
                .appender("stderr")                                             
                .build(log::LevelFilter::Trace),                                
        )                                                                       
        .unwrap();                                                              
    // Use this to change log levels at runtime.                                
    // This means you can change the default log level to trace                 
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.                                                       
    let _handle = log4rs::init_config(config);                                  
}    


// user defined functions:

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
    fn execute(&mut self, _trace_id: u64, instance: String) {
        self.total += instance.parse::<u64>().unwrap();
        self.num_instances += 1;
        self.avg = self.total/self.num_instances;
        self.avg.to_string()
    }
}



#[derive(Clone, Debug)]
pub struct Filter {
    
    avg: Avg,
    
}

impl Filter {
    #[no_mangle]
    pub fn new() -> *mut Filter {
         log_setup();
         Box::into_raw(Box::new(Filter {
            
            avg: Avg::new(),
            
         }))
    }

    #[no_mangle]
    pub fn new_with_envoy_properties(string_data: IndexMap<String, String>) -> *mut Filter {
        log_setup();
        Box::into_raw(Box::new(Filter {
            
            avg: Avg::new(),
            
        }))
     }

    pub fn on_incoming_requests(&mut self, mut x: Rpc) -> Vec<Rpc> {
        let mut to_return = vec![x];
        
        let mut avg_str = "avg".to_string();
        avg_str.push_str(& self.avg.execute(x.uid, x.data.clone()) );
        to_return.push(Rpc::new(avg_str));
        
        return to_return;
    }

    pub fn on_outgoing_responses(&mut self, mut x: Rpc) -> Vec<Rpc> {
        // we shouldn't do anything
        return vec![x];


    }

    pub fn on_outgoing_requests(&mut self, mut x: Rpc) -> Vec<Rpc>{
        // this should never happen to storage
        return vec![x];
    }

    pub fn on_incoming_responses(&mut self, mut x: Rpc) -> Vec<Rpc> {
        // this should never happen to storage
        return vec![x];
    }


    #[no_mangle]
    pub fn execute(&mut self, x: &Rpc) -> Vec<Rpc> {
        match x.headers["direction"].as_str() {
            "request" => {
                 match x.headers["location"].as_str() {
                 "ingress" => { return self.on_incoming_requests(x.clone()); }
                 "egress" => { return self.on_outgoing_requests(x.clone()); }
                 _ => { panic!("Filter got an rpc with no location\n"); }
                 }
             }
             "response" => {
                 match x.headers["location"].as_str() {
                 "ingress" => { return self.on_incoming_responses(x.clone()); }
                 "egress" => { return self.on_outgoing_responses(x.clone()); }
                 _ => { panic!("Filter got an rpc with no location\n"); }
                 }
             }
             _ => { panic!("Filter got an rpc with no direction\n"); }
        }
    }

}
