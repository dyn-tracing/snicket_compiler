use crate::ir::VisitorResults;
use regex::Regex;
use indexmap::map::IndexMap;
use strum_macros::EnumString;
use serde::Serialize;
use std::str::FromStr;
use std::collections::HashMap;

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

#[derive(Serialize, Debug, Clone)]
struct Udf {
    udf_type: UdfType,
    id: String,
    leaf_func: String,
    mid_func: String,
    struct_name: String,
    func_impl: String,
}

/********************************/
// Code Generation
/********************************/
#[derive(Serialize)]
pub struct CodeGenSimulator {
    ir: VisitorResults,  // the IR, as defined in to_ir.rs
    request_blocks: Vec<String>,  // code blocks to be used in the handlebars
    response_blocks: Vec<String>,  // code blocks to be used in the handlebars
    udf_table: IndexMap<String, Udf>, // where we store udf implementations
    envoy_properties_to_access_names: HashMap<String, String>
}

impl CodeGenSimulator {
    pub fn generate_code_blocks(ir: VisitorResults, udfs: Vec<String>) -> Self{
        let mut to_return = CodeGenSimulator {
            ir,
            request_blocks: Vec::new(),
            response_blocks: Vec::new(),
            udf_table: IndexMap::default(),
            envoy_properties_to_access_names: HashMap::new(),
        };
        for udf in udfs {
            to_return.parse_udf(udf);
        }
        to_return.envoy_properties_to_access_names.insert("request_size".to_string(), "request.total_size".to_string());
        to_return.envoy_properties_to_access_names.insert("service_name".to_string(), "node.metadata.WORKLOAD_NAME".to_string());
        to_return.get_maps();
        to_return.make_struct_filter_blocks();
        to_return.make_attr_filter_blocks();
        to_return.make_return_block();
        to_return.make_aggr_block();
        to_return
    }

    fn parse_udf(&mut self, udf: String) {
        let udf_clone = udf.clone();
        let re = Regex::new(
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*leaf_func:\s+(?P<leaf_func>\w+)\n.*mid_func:\s+(?P<mid_func>\w+)\n.*struct_name:\s+(?P<struct_name>\w+)\n.*id:\s+(?P<id>\w+)",
        ).unwrap();
        let rust_caps = re.captures(&udf_clone);

        match rust_caps {
            Some(caps) => {
                let udf_type = UdfType::from_str(caps.name("udf_type").unwrap().as_str()).unwrap();
                let leaf_func = String::from(caps.name("leaf_func").unwrap().as_str());
                let mid_func = String::from(caps.name("mid_func").unwrap().as_str());
                let struct_name = String::from(caps.name("struct_name").unwrap().as_str());
                let id = String::from(caps.name("id").unwrap().as_str());

                self.udf_table.insert(
                    id.clone(),
                    Udf {
                        udf_type,
                        leaf_func,
                        mid_func,
                        struct_name,
                        func_impl: udf,
                        id,
                    },
                );
            }
            None => panic!("Rust UDF did not have proper header"),
        }
    }

    fn collect_envoy_property(&mut self, property: &String) {
        let block = format!("let prop_str = format!(\"{{whoami}}.{{property}}=={{value}},\",
                                                      whoami=&self.whoami,
                                                      property=\"{property}\",
                                                      value=self.filter_state&[{property}].string_data.as_ref().unwrap().to_string());
                                            ", property=property);
        self.request_blocks.push(block);
    }

    fn get_maps(&mut self) {
        for map in &mut self.ir.maps.clone() {
            let mut map_name = map.clone();
            if map_name.chars().next().unwrap() == ".".chars().next().unwrap() {
                 map_name.remove(0);
            }
            if !self.udf_table.contains_key(&map_name) && map_name != "" && !self.envoy_properties_to_access_names.contains_key(&map_name) {
                panic!("unrecognized UDF");
            }
            if self.envoy_properties_to_access_names.contains_key(&map_name) {
                self.collect_envoy_property(&map_name);
            }
        }
    }

    fn make_struct_filter_blocks(&mut self) {
        for struct_filter in &self.ir.struct_filters {
            

        }
    }

    fn make_attr_filter_blocks(&mut self) {
        // TODO
    }

    fn make_return_block(&mut self) {
        // TODO
    }

    fn make_aggr_block(&mut self) {
        // TODO
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::CypherLexer;
    use crate::parser::CypherParser;
    use crate::to_ir::visit_result;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::token_factory::CommonTokenFactory;
    use antlr_rust::InputStream;
   
    static COUNT: &str = "
        // udf_type: Scalar
	// leaf_func: leaf
	// mid_func: mid
	// struct_name: Count
	// id: count

	use petgraph::Graph;

	struct ServiceName {
	    fn leaf(my_node: String, graph: Graph) {
		return 0;
	    }

	    fn mid(my_node: String, graph: Graph) {
		return 1;
	    }
	}
    ";
    fn get_codegen_from_query(input: String) -> VisitorResults {
        let tf = CommonTokenFactory::default();
        let query_stream = InputStream::new_owned(input.to_string().into_boxed_str());
        let mut _lexer = CypherLexer::new_with_token_factory(query_stream, &tf);
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = CypherParser::new(token_source);
        let result = parser.oC_Cypher().expect("parsed unsuccessfully");
        visit_result(result)
    }

    #[test]
    fn get_codegen_doesnt_throw_error() {
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b)-[]->(c) RETURN a.count".to_string());
        assert!(!result.struct_filters.is_empty());
        let codegen = CodeGenSimulator::generate_code_blocks(result, [COUNT.to_string()].to_vec());
        
    }


}
