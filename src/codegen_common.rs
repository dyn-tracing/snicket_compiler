use super::ir::VisitorResults;
use indexmap::IndexMap;
use serde::Serialize;
use strum_macros::EnumString;

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
    // code blocks used in incoming requests
    pub request_blocks: Vec<String>,
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
            request_blocks: Vec::new(),
            response_blocks: Vec::new(),
            target_blocks: Vec::new(),
            udf_blocks: Vec::new(),
            trace_lvl_prop_blocks: Vec::new(),
            scalar_udf_table: IndexMap::default(),
            aggregation_udf_table: IndexMap::default(),
        }
    }
}

pub trait CodeGen: Serialize {
    fn generate_code_blocks(ir: VisitorResults, udfs: Vec<String>) -> CodeStruct;

}
