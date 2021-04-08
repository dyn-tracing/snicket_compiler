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
pub struct Udf {
    pub udf_type: UdfType,
    pub id: String,
    pub leaf_func: String,
    pub mid_func: String,
    pub func_impl: String,
}

// TODO: Use getters
#[derive(Serialize)]
pub struct CodeGenStruct {
    // the IR, as defined in to_ir.rs
    pub ir: VisitorResults,
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
    pub udf_table: IndexMap<String, Udf>,
    pub envoy_properties: Vec<String>,
    // all the properties we collect
    pub collected_properties: Vec<String>,
}

pub trait CodeGen: Serialize {
    fn generate_code_blocks(ir: VisitorResults, udfs: Vec<String>) -> Self;
    fn parse_udf(&mut self, udf: String);
    fn collect_envoy_property(&mut self, property: String);
    fn collect_udf_property(&mut self, udf_id: String);
    fn get_maps(&mut self);
    fn make_struct_filter_blocks(&mut self);
    fn make_attr_filter_blocks(&mut self);
    fn make_storage_rpc_value_from_trace(&mut self, entity: String, property: String);
    fn make_storage_rpc_value_from_target(&mut self, entity: String, property: String);
    fn make_return_block(&mut self);
    fn make_aggr_block(&mut self);
}
