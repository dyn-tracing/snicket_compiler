use crate::ir::VisitorResults;
use indexmap::map::IndexMap;
use regex::Regex;
use serde::Serialize;
use std::mem;
use std::str::FromStr;
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

#[derive(Serialize, Debug, Clone)]
struct Udf {
    udf_type: UdfType,
    id: String,
    leaf_func: String,
    mid_func: String,
    func_impl: String,
}

/********************************/
// Code Generation
/********************************/
#[derive(Serialize)]
pub struct CodeGenSimulator {
    ir: VisitorResults,               // the IR, as defined in to_ir.rs
    request_blocks: Vec<String>, // code blocks used in incoming requests
    response_blocks: Vec<String>, // code blocks in outgoing responses, after matching
    target_blocks: Vec<String>,   // code blocks to create target graph
    udf_blocks: Vec<String>, // code blocks to be used in outgoing responses, to compute UDF before matching
    udf_table: IndexMap<String, Udf>, // where we store udf implementations
    envoy_properties_to_access_names: IndexMap<String, String>,
    collected_properties: Vec<String>, // all the properties we collect
}

impl CodeGenSimulator {
    pub fn generate_code_blocks(ir: VisitorResults, udfs: Vec<String>) -> Self {
        let mut to_return = CodeGenSimulator {
            ir,
            request_blocks: Vec::new(),
            response_blocks: Vec::new(),
            target_blocks: Vec::new(),
            udf_blocks: Vec::new(),
            udf_table: IndexMap::default(),
            envoy_properties_to_access_names: IndexMap::new(),
            collected_properties: Vec::new(),
        };
        for udf in udfs {
            to_return.parse_udf(udf);
        }
        to_return
            .envoy_properties_to_access_names
            .insert("request_size".to_string(), "request.total_size".to_string());
        to_return.envoy_properties_to_access_names.insert(
            "service_name".to_string(),
            "node.metadata.WORKLOAD_NAME".to_string(),
        );
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
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*leaf_func:\s+(?P<leaf_func>\w+)\n.*mid_func:\s+(?P<mid_func>\w+)\n.*id:\s+(?P<id>\w+)",
        ).unwrap();

        let rust_caps = re
            .captures(&udf_clone)
            .expect("Rust UDF did not have proper header");

        let udf_type = UdfType::from_str(rust_caps.name("udf_type").unwrap().as_str()).unwrap();
        let leaf_func = String::from(rust_caps.name("leaf_func").unwrap().as_str());
        let mid_func = String::from(rust_caps.name("mid_func").unwrap().as_str());
        let id = String::from(rust_caps.name("id").unwrap().as_str());

        self.udf_table.insert(
            id.clone(),
            Udf {
                udf_type,
                leaf_func,
                mid_func,
                func_impl: udf,
                id,
            },
        );
    }

    fn collect_envoy_property(&mut self, property: String) {
        let get_prop_block = format!("prop_str = format!(\"{{whoami}}.{{property}}=={{value}}\",
                                                      whoami=&self.whoami.as_ref().unwrap(),
                                                      property=\"{property}\",
                                                      value=self.filter_state[\"{envoy_property}\"].string_data.as_ref().unwrap().to_string());
                                            ", property=property, envoy_property=self.envoy_properties_to_access_names[&property]);
        let insert_hdr_block = format!("
        if x.headers.contains_key(\"properties_{property}\") {{
            if !x.headers[\"properties_{property}\"].contains(&prop_str) {{ // don't add our properties if they have already been added
                x.headers.get_mut(&\"properties_{property}\".to_string()).unwrap().push_str(\",\");
                x.headers.get_mut(&\"properties_{property}\".to_string()).unwrap().push_str(&prop_str);
            }}
        }}
        else {{
            x.headers.insert(\"properties_{property}\".to_string(), prop_str);
        }}
        ", property=property);
        self.request_blocks.push(get_prop_block);
        self.request_blocks.push(insert_hdr_block);
    }

    fn collect_udf_property(&mut self, udf_id: String) {
        let get_udf_vals = format!("let my_{id}_value;
    if x.headers.contains_key(\"properties_path\") {{
            // TODO:  only create trace graph once and then add to it
            let graph = self.create_trace_graph(x.clone());
            let child_iterator = graph.neighbors_directed(
                utils::get_node_with_id(&graph, self.whoami.as_ref().unwrap().clone()).unwrap(),
                petgraph::Outgoing);
            let mut child_values = Vec::new();
            for child in child_iterator {{
                child_values.push(graph.node_weight(child).unwrap().1[\"properties_{id}\"].clone());
            }}
            if child_values.len() == 0 {{
                my_{id}_value = {leaf_func}(graph);
            }} else {{
                my_{id}_value = {mid_func}(graph, child_values);
            }}
         }}
         else {{
             print!(\"WARNING: no path header\");
             return vec!(x);
         }}

        ",
        id=udf_id,
        leaf_func=self.udf_table[&udf_id].leaf_func,
        mid_func=self.udf_table[&udf_id].mid_func
        );
        self.udf_blocks.push(get_udf_vals);

        let save_udf_vals = format!("let {udf_id}_str = format!(\"{{whoami}}.{{udf_id}}=={{value}}\",
                                                      whoami=&self.whoami.as_ref().unwrap(),
                                                      udf_id=\"properties_{udf_id}\",
                                                      value=my_{udf_id}_value);
        if x.headers.contains_key(\"properties_{udf_id}\") {{
            if !x.headers[\"properties_{udf_id}\"].contains(&{udf_id}_str) {{ // don't add a udf property twice
                x.headers.get_mut(&\"properties_{udf_id}\".to_string()).unwrap().push_str(\",\");
                x.headers.get_mut(&\"properties_{udf_id}\".to_string()).unwrap().push_str(&{udf_id}_str);
            }}
        }}
        else {{
            x.headers.insert(\"properties_{udf_id}\".to_string(), {udf_id}_str);
        }}

        ", udf_id=udf_id);

        self.udf_blocks.push(save_udf_vals);
    }

    fn get_maps(&mut self) {
        let mut maps = Vec::new();
        mem::swap(&mut maps, &mut self.ir.maps);
        for map in &maps {
            let mut map_name = map.clone();
            let mut has_period = false;
            if map_name.starts_with('.') {
                map_name.remove(0);
                has_period = true;
            }
            if !has_period || !self.ir.maps.contains(&map_name) {
                // we might have duplicates bc some have preceding periods
                if !self.udf_table.contains_key(&map_name)
                    && !map_name.is_empty()
                    && !self
                        .envoy_properties_to_access_names
                        .contains_key(&map_name)
                {
                    panic!("unrecognized UDF");
                }
                self.collected_properties.push(map_name.clone());
                if self
                    .envoy_properties_to_access_names
                    .contains_key(&map_name)
                {
                    self.collect_envoy_property(map_name);
                } else {
                    self.collect_udf_property(map_name);
                }
            }
        }
        mem::swap(&mut maps, &mut self.ir.maps);
    }

    fn make_struct_filter_blocks(&mut self) {
        for struct_filter in &self.ir.struct_filters {
            self.target_blocks
                .push(" let vertices = vec!( ".to_string());
            for vertex in &struct_filter.vertices {
                self.target_blocks
                    .push(format!("\"{vertex}\".to_string(),", vertex = vertex));
            }
            self.target_blocks.push(" );\n".to_string());

            self.target_blocks
                .push("        let edges = vec!( ".to_string());
            for edge in &struct_filter.edges {
                self.target_blocks.push(format!(
                    " (\"{edge1}\".to_string(), \"{edge2}\".to_string() ), ",
                    edge1 = edge.0,
                    edge2 = edge.1
                ));
            }
            self.target_blocks.push(" );\n".to_string());

            let ids_to_prop_block = "        let mut ids_to_properties: HashMap<String, HashMap<String, String>> = HashMap::new();\n".to_string();
            self.target_blocks.push(ids_to_prop_block);

            for vertex in &struct_filter.vertices {
                let ids_to_properties_hashmap_init = format!(
                    "        ids_to_properties.insert(\"{node}\".to_string(), HashMap::new());\n",
                    node = vertex
                );
                self.target_blocks.push(ids_to_properties_hashmap_init);
            }
            for node in struct_filter.properties.keys() {
                let get_hashmap = format!(
                    "        let mut {node}_hashmap = ids_to_properties.get_mut(\"{node}\").unwrap();\n",
                    node = node
                );
                self.target_blocks.push(get_hashmap);
                for property_name in struct_filter.properties[node].keys() {
                    let fill_in_hashmap = format!("        {node}_hashmap.insert(\"{property_name}\".to_string(), \"{property_value}\".to_string());\n",
                                                   node=node,
                                                   property_name=property_name,
                                                   property_value=struct_filter.properties[node][property_name]);
                    self.target_blocks.push(fill_in_hashmap);
                }
            }
            let make_graph = "        self.target_graph = Some(utils::generate_target_graph(vertices, edges, ids_to_properties));\n".to_string();
            self.target_blocks.push(make_graph);
        }
    }

    fn make_attr_filter_blocks(&mut self) {
        // TODO
    }

    fn make_return_block(&mut self) {
        if self.ir.return_expr.is_none() {
            return;
        }
        let entity = self.ir.return_expr.as_ref().unwrap().clone().entity;
        let mut property = self.ir.return_expr.as_ref().unwrap().clone().property;
        if property.chars().next().unwrap() == ".".chars().next().unwrap() {
            property.remove(0);
        }

        if entity == "trace" {
            // TODO
        } else {
            let num_struct_filters = self.ir.struct_filters.len();
            if !self.ir.struct_filters[num_struct_filters - 1]
                .vertices
                .contains(&entity)
            {
                panic!("Unknown entity in return expression");
            }

            let ret_block = format!(
               "let node_ptr = utils::get_node_with_id(&self.target_graph.as_ref().unwrap(), \"{node_id}\".to_string());
               if node_ptr.is_none() {{
                   print!(\"WARNING Node {node_id} not found\");
                   return vec!(x);
               }}
               let mut trace_node_index = None;
               for map in m {{
                   if self.target_graph.as_ref().unwrap().node_weight(map.0).unwrap().0 == \"{node_id}\" {{
                       trace_node_index = Some(map.1);
                       break;
                   }}
               }}
               if trace_node_index == None || !&trace_graph.node_weight(trace_node_index.unwrap()).unwrap().1.contains_key(\"{prop}\") {{
                   // we have not yet collected the return property or have a mapping error
                   return vec!(x);
               }}
               let mut ret_{prop} = &trace_graph.node_weight(trace_node_index.unwrap()).unwrap().1[ \"{prop}\" ];\n
               value = ret_{prop}.to_string();\n",
                   node_id = entity,
                   prop = property
                );

            self.response_blocks.push(ret_block);
        }
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
        let result =
            get_codegen_from_query("MATCH (a) -[]-> (b)-[]->(c) RETURN a.count".to_string());
        assert!(!result.struct_filters.is_empty());
        let codegen = CodeGenSimulator::generate_code_blocks(result, [COUNT.to_string()].to_vec());
    }
}
