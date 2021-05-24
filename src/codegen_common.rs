use indexmap::IndexMap;
use regex::Regex;
use serde::Serialize;
use std::str::FromStr;
use strum_macros::EnumString;
use super::ir::Property;
use indexmap::IndexSet;

/********************************/
// Helper structs
/********************************/
#[derive(Serialize, PartialEq, Eq, Debug, Clone, EnumString)]
pub enum UdfType {
    Scalar,
    Aggregation,
}

impl Default for UdfType {
    fn default() -> Self {
        UdfType::Scalar
    }
}

// TODO: Use getters
#[derive(Serialize, Debug, Clone)]
pub struct ScalarUdf {
    pub udf_type: UdfType,
    pub id: String,
    pub leaf_func: String,
    pub mid_func: String,
    pub func_impl: String,
}

#[derive(Serialize, Debug, Clone)]
pub struct AggregationUdf {
    pub udf_type: UdfType,
    pub id: String,
    pub init_func: String,
    pub exec_func: String,
    pub struct_name: String,
    pub func_impl: String,
}

#[derive(Serialize)]
pub struct CodeStruct {
    // the IR, as defined in to_ir.rs
    pub root_id: String,
    // code blocks used in incoming requests to collect properties
    pub collect_properties_blocks: Vec<String>,
    // map of numbers to properties in order to compress the messages
    pub id_to_property: IndexMap<String, u64>,
    // code blocks in outgoing responses, after matching
    pub response_blocks: Vec<String>,
    // code blocks to create target graph
    pub target_blocks: Vec<String>,
    // code blocks to be used in outgoing responses, to compute UDF before matching
    pub udf_blocks: Vec<String>,
    // code blocks to be used in outgoing responses, to compute UDF before matching
    pub trace_lvl_prop_blocks: Vec<String>,
    // where we store udf implementations
    pub scalar_udf_table: IndexMap<String, ScalarUdf>,
    // where we store udf implementations
    pub aggregation_udf_table: IndexMap<String, AggregationUdf>,
}

impl CodeStruct {
    pub fn new(root_id: &str) -> CodeStruct {
        CodeStruct {
            root_id: root_id.to_string(),
            collect_properties_blocks: Vec::new(),
            id_to_property: IndexMap::default(),
            response_blocks: Vec::new(),
            target_blocks: Vec::new(),
            udf_blocks: Vec::new(),
            trace_lvl_prop_blocks: Vec::new(),
            scalar_udf_table: IndexMap::default(),
            aggregation_udf_table: IndexMap::default(),
        }
    }
}

pub enum ScalarOrAggregationUdf {
    ScalarUdf(ScalarUdf),
    AggregationUdf(AggregationUdf),
}

pub fn parse_udf(udf: String) -> ScalarOrAggregationUdf {
    let scalar_re = Regex::new(
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*leaf_func:\s+(?P<leaf_func>\w+)\n.*mid_func:\s+(?P<mid_func>\w+)\n.*id:\s+(?P<id>\w+)",
        ).unwrap();

    if let Some(rc) = scalar_re.captures(&udf) {
        let udf_type = UdfType::from_str(rc.name("udf_type").unwrap().as_str()).unwrap();
        let leaf_func = String::from(rc.name("leaf_func").unwrap().as_str());
        let mid_func = String::from(rc.name("mid_func").unwrap().as_str());
        let id = String::from(rc.name("id").unwrap().as_str());

        return ScalarOrAggregationUdf::ScalarUdf(ScalarUdf {
            udf_type,
            leaf_func,
            mid_func,
            func_impl: udf,
            id,
        });
    }
    let aggr_re = Regex::new(
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*init_func:\s+(?P<init_func>\w+)\n.*exec_func:\s+(?P<exec_func>\w+)\n.*struct_name:\s+(?P<struct_name>\w+)\n.*id:\s+(?P<id>\w+)",
        ).unwrap();
    if let Some(rc) = aggr_re.captures(&udf) {
        let udf_type = UdfType::from_str(rc.name("udf_type").unwrap().as_str()).unwrap();
        let init_func = String::from(rc.name("init_func").unwrap().as_str());
        let exec_func = String::from(rc.name("exec_func").unwrap().as_str());
        let struct_name = String::from(rc.name("struct_name").unwrap().as_str());
        let id = String::from(rc.name("id").unwrap().as_str());

        return ScalarOrAggregationUdf::AggregationUdf(AggregationUdf {
            udf_type,
            init_func,
            exec_func,
            struct_name,
            func_impl: udf,
            id,
        });
    }
    log::error!("Unable to parse input udf {:?}", udf);
    std::process::exit(1);
}

pub fn assign_id_to_property(properties: &IndexSet<Property>) -> IndexMap<String, u64> {
    let mut id_to_property = IndexMap::new();
    let mut i : u64 = 0;
    for property in properties {
        id_to_property.insert(property.to_dot_string(), i);
        i += 1;
    }
    id_to_property

}
