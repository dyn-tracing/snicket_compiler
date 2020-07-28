use std::collections::HashMap;

lazy_static! {
    static ref PROPERTY_MAP: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("service_name", vec!["node", "metadata", "WORKLOAD_NAME"]);
        m
    };
}
