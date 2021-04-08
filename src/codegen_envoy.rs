use super::codegen_common::AggregationUdf;
use super::codegen_common::ScalarUdf;
use super::codegen_common::UdfType;
use super::ir::VisitorResults;
use super::CodeGen;
use indexmap::map::IndexMap;
use regex::Regex;
use serde::Serialize;
use std::mem;
use std::str::FromStr;

/********************************/
// Code Generation
/********************************/
#[derive(Serialize)]
pub struct CodeGenEnvoy {
    ir: VisitorResults,                            // the IR, as defined in to_ir.rs
    request_blocks: Vec<String>,                   // code blocks used in incoming requests
    response_blocks: Vec<String>, // code blocks in outgoing responses, after matching
    target_blocks: Vec<String>,   // code blocks to create target graph
    udf_blocks: Vec<String>, // code blocks to be used in outgoing responses, to compute UDF before matching
    trace_lvl_prop_blocks: Vec<String>, // code blocks to be used in outgoing responses, to compute UDF before matching
    scalar_udf_table: IndexMap<String, ScalarUdf>, // where we store udf implementations
    aggregation_udf_table: IndexMap<String, AggregationUdf>, // where we store udf implementations
    envoy_properties: Vec<String>,
    collected_properties: Vec<String>, // all the properties we collect
}

impl CodeGen for CodeGenEnvoy {
    fn generate_code_blocks(ir: VisitorResults, udfs: Vec<String>) -> Self {
        let mut to_return = CodeGenEnvoy {
            ir,
            request_blocks: Vec::new(),
            response_blocks: Vec::new(),
            target_blocks: Vec::new(),
            udf_blocks: Vec::new(),
            trace_lvl_prop_blocks: Vec::new(),
            scalar_udf_table: IndexMap::default(),
            aggregation_udf_table: IndexMap::default(),
            envoy_properties: Vec::new(),
            collected_properties: Vec::new(),
        };
        for udf in &udfs {
            log::info!("udf: {:?}\n\n\n\n", udf);
        }
        for udf in udfs {
            to_return.parse_udf(udf);
        }
        to_return
            .envoy_properties
            .push("request.total_size".to_string());
        to_return
            .envoy_properties
            .push("node.metadata.WORKLOAD_NAME".to_string());
        to_return.get_maps();
        to_return.make_struct_filter_blocks();
        to_return.make_attr_filter_blocks();
        to_return.make_return_block();
        to_return.make_aggr_block();
        to_return
    }

    fn parse_udf(&mut self, udf: String) {
        let udf_clone = udf.clone();
        let scalar_re = Regex::new(
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*leaf_func:\s+(?P<leaf_func>\w+)\n.*mid_func:\s+(?P<mid_func>\w+)\n.*id:\s+(?P<id>\w+)",
        ).unwrap();

        let aggr_re = Regex::new(
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*init_func:\s+(?P<init_func>\w+)\n.*exec_func:\s+(?P<exec_func>\w+)\n.*struct_name:\s+(?P<struct_name>\w+)\n.*id:\s+(?P<id>\w+)",
        ).unwrap();

        let scalar_rust_caps = scalar_re.captures(&udf_clone);
        let aggr_rust_caps = aggr_re.captures(&udf_clone);

        if let Some(rc) = scalar_rust_caps {
            let udf_type = UdfType::from_str(rc.name("udf_type").unwrap().as_str()).unwrap();
            let leaf_func = String::from(rc.name("leaf_func").unwrap().as_str());
            let mid_func = String::from(rc.name("mid_func").unwrap().as_str());
            let id = String::from(rc.name("id").unwrap().as_str());

            self.scalar_udf_table.insert(
                id.clone(),
                ScalarUdf {
                    udf_type,
                    leaf_func,
                    mid_func,
                    func_impl: udf,
                    id,
                },
            );
        } else if let Some(rc) = aggr_rust_caps {
            let udf_type = UdfType::from_str(rc.name("udf_type").unwrap().as_str()).unwrap();
            let init_func = String::from(rc.name("init_func").unwrap().as_str());
            let exec_func = String::from(rc.name("exec_func").unwrap().as_str());
            let struct_name = String::from(rc.name("struct_name").unwrap().as_str());
            let id = String::from(rc.name("id").unwrap().as_str());

            self.aggregation_udf_table.insert(
                id.clone(),
                AggregationUdf {
                    udf_type,
                    init_func,
                    exec_func,
                    struct_name,
                    func_impl: udf,
                    id,
                },
            );
        }
    }

    fn collect_envoy_property(&mut self, property: String) {
        // FIXME: Properties should be lists already...
        let get_prop_block = format!(
            "prop_tuple_wrapped = fetch_property(&http_headers.workload_name,
                                        &\"{property}\".split(\".\").collect(),
                                        http_headers);
                                            ",
            property = property,
        );
        self.request_blocks.push(get_prop_block);
        let push_block = "if let Some(prop_tuple) = prop_tuple_wrapped {
            fd.unassigned_properties.push(prop_tuple);
            } else {
                return Err(());
            }"
        .to_string();
        self.request_blocks.push(push_block);
    }

    fn collect_udf_property(&mut self, udf_id: String) {
        let get_udf_vals = format!(
            "let my_{id}_value;
            let child_iterator = fd.trace_graph.neighbors_directed(
                get_node_with_id(&fd.trace_graph, http_headers.workload_name.clone()).unwrap(),
                petgraph::Outgoing);
            let mut child_values = Vec::new();
            for child in child_iterator {{
                child_values.push(fd.trace_graph.node_weight(child).unwrap().1[\"{id}\"].clone());
            }}
            if child_values.len() == 0 {{
                my_{id}_value = {leaf_func}(&fd.trace_graph).to_string();
            }} else {{
                my_{id}_value = {mid_func}(&fd.trace_graph, child_values).to_string();
            }}

        ",
            id = udf_id,
            leaf_func = self.scalar_udf_table[&udf_id].leaf_func,
            mid_func = self.scalar_udf_table[&udf_id].mid_func
        );
        self.udf_blocks.push(get_udf_vals);

        let save_udf_vals = format!(
            "
        let node = get_node_with_id(&fd.trace_graph, http_headers.workload_name.clone()).unwrap();
        // if we already have the property, don't add it
        if !( fd.trace_graph.node_weight(node).unwrap().1.contains_key(\"{id}\") &&
               fd.trace_graph.node_weight(node).unwrap().1[\"{id}\"] == my_{id}_value ) {{
           fd.trace_graph.node_weight_mut(node).unwrap().1.insert(
               \"{id}\".to_string(), my_{id}_value);
        }}
        ",
            id = udf_id
        );

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
                if !self.scalar_udf_table.contains_key(&map_name)
                    && !map_name.is_empty()
                    && !self.envoy_properties.contains(&map_name)
                {
                    panic!("unrecognized UDF {:?}", map_name);
                }
                self.collected_properties.push(map_name.clone());
                if self.envoy_properties.contains(&map_name) {
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

            let ids_to_prop_block = "        let mut ids_to_properties: IndexMap<String, IndexMap<String, String>> = IndexMap::new();\n".to_string();
            self.target_blocks.push(ids_to_prop_block);

            for vertex in &struct_filter.vertices {
                let ids_to_properties_hashmap_init = format!(
                    "        ids_to_properties.insert(\"{node}\".to_string(), IndexMap::new());\n",
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
                for property_filter in &self.ir.attr_filters {
                    if property_filter.node != "trace" {
                        let mut property_name_without_period = property_filter.property.clone();
                        if property_name_without_period.starts_with('.') {
                            property_name_without_period.remove(0);
                        }
                        let fill_in_hashmap = format!("        {node}_hashmap.insert(\"{property_name}\".to_string(), \"{property_value}\".to_string());\n",
                                                       node=property_filter.node,
                                                       property_name=property_name_without_period,
                                                       property_value=property_filter.value);
                        self.target_blocks.push(fill_in_hashmap);
                    }
                }
            }
            let make_graph =
                "        return generate_target_graph(vertices, edges, ids_to_properties);\n"
                    .to_string();
            self.target_blocks.push(make_graph);
        }
    }

    fn make_attr_filter_blocks(&mut self) {
        // for everything except trace level attributes, the UDF/envoy property
        // collection will make the attribute filtering happen at the same time as
        // the struct filtering.  This is not the case for trace-level attributes

        let if_root_block = "
            if &http_headers.workload_name == root_id {"
            .to_string();
        self.trace_lvl_prop_blocks.push(if_root_block);
        let init_trace_prop_str = "        let mut trace_prop_str : String;\n".to_string();
        self.trace_lvl_prop_blocks.push(init_trace_prop_str);

        for attr_filter in &self.ir.attr_filters {
            if attr_filter.node == "trace" {
                let mut prop = attr_filter.property.clone();
                if prop.starts_with('.') {
                    prop.remove(0);
                }
                let trace_filter_block = format!(
                "
                let root_node = get_node_with_id(&fd.trace_graph, \"{root_id}\".to_string()).unwrap();
                if ! ( fd.trace_graph.node_weight(root_node).unwrap().1.contains_key(\"{prop_name}\") &&
                    fd.trace_graph.node_weight(root_node).unwrap().1[\"{prop_name}\"] == \"{value}\" ){{
                    // TODO:  replace fd
                    match serde_yaml::to_string(&fd) {{
                        Ok(fd_str) => {{
                            return false;
                        }}
                        Err(e) => {{
                            log::error!(\"could not serialize baggage {{0}}\n\", e);
                            return false;
                        }}
                     }}
                     return false;
                }}
                ", root_id=self.ir.root_id, prop_name=prop, value=attr_filter.value);
                self.trace_lvl_prop_blocks.push(trace_filter_block);
            }
        }

        let end_root_block = "       }".to_string();
        self.trace_lvl_prop_blocks.push(end_root_block);
    }
    fn make_trace_rpc_value(&mut self) {
        let ret_block = "                                                       
        match serde_yaml::to_string(fd) {                                       
            Ok(trace_str) => { value = trace_str; }                             
            Err(e) => { log::error!(\"Error:  could not translate ferried data to string\"); }\
        }                                                                       
        "
        .to_string();
        self.response_blocks.push(ret_block);
    }

    fn make_storage_rpc_value_from_trace(&mut self, entity: String, property: String) {
        let mut prop_wo_periods = property.clone();
        prop_wo_periods.retain(|c| c != '.');
        let ret_block = format!(
        "let trace_node_idx = get_node_with_id(&fd.trace_graph, \"{node_id}\".to_string());
        if trace_node_idx.is_none() {{
           log::error!(\"Node {node_id} not found\");
                return None;
        }}
        let ret_{prop_var} = &fd.trace_graph.node_weight(trace_node_idx.unwrap()).unwrap().1[ \"{prop}\" ];\n
        value = ret_{prop_var}.to_string();\n",
                node_id = entity,
                prop_var = prop_wo_periods,
                prop = property
        );

        self.response_blocks.push(ret_block);
    }
    fn make_storage_rpc_value_from_target(&mut self, entity: String, property: String) {
        let mut prop_wo_periods = property.clone();
        prop_wo_periods.retain(|c| c != '.');
        let ret_block = format!(
        "let node_ptr = get_node_with_id(target_graph, \"{node_id}\".to_string());
        if node_ptr.is_none() {{
           log::error!(\"Node {node_id} not found\");
                return None;
        }}
        let mut trace_node_idx_opt = None;
        for map in mapping {{
            if target_graph.node_weight(map.0).unwrap().0 == \"{node_id}\" {{
                trace_node_idx_opt = Some(map.1);
                break;
            }}
        }}
        if trace_node_idx_opt.is_none() {{
            log::error!(\"Node index {node_id} not found.\");
            // we have not yet collected the return property or have a mapping error
            return None;
        }}
        let trace_node_idx = trace_node_idx_opt.unwrap();
        if !&stored_data
            .trace_graph
            .node_weight(trace_node_idx)
            .unwrap()
            .1
            .contains_key(\"{prop}\")
        {{
            // we have not yet collected the return property
            log::error!(\"Missing return property {prop}\");
            return None;
        }}
        let ret_{prop_var} = &stored_data.trace_graph.node_weight(trace_node_idx).unwrap().1[ \"{prop}\" ];\n
        value = ret_{prop_var}.to_string();\n",
                node_id = entity,
                prop_var = prop_wo_periods,
                prop = property
        );

        self.response_blocks.push(ret_block);
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
            self.make_storage_rpc_value_from_trace(self.ir.root_id.clone(), property);
        } else {
            let num_struct_filters = self.ir.struct_filters.len();
            if !self.ir.struct_filters[num_struct_filters - 1]
                .vertices
                .contains(&entity)
            {
                panic!("Unknown entity in return expression");
            }
            self.make_storage_rpc_value_from_target(entity, property);
        }
    }

    fn make_aggr_block(&mut self) {
        if self.ir.aggregate.is_none() {
            return;
        }
        let entity = self.ir.aggregate.as_ref().unwrap().clone().entity;
        let mut property = self.ir.aggregate.as_ref().unwrap().clone().property;
        if property.chars().next().unwrap() == ".".chars().next().unwrap() {
            property.remove(0);
        }

        if entity == "trace" {
            self.make_storage_rpc_value_from_trace(self.ir.root_id.clone(), property);
        } else {
            let num_struct_filters = self.ir.struct_filters.len();
            if !self.ir.struct_filters[num_struct_filters - 1]
                .vertices
                .contains(&entity)
            {
                panic!("Unknown entity in return expression");
            }
            self.make_storage_rpc_value_from_target(entity, property);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::antlr_gen::lexer::CypherLexer;
    use crate::antlr_gen::parser::CypherParser;
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

    static AVG: &str = "
        // udf_type: Scalar
    // init_func: init
    // exec_func: exec
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
    ";
    fn get_codegen_from_query(input: String) -> VisitorResults {
        let tf = CommonTokenFactory::default();
        let query_stream = InputStream::new_owned(input.to_string().into_boxed_str());
        let mut _lexer = CypherLexer::new_with_token_factory(query_stream, &tf);
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = CypherParser::new(token_source);
        let result = parser.oC_Cypher().expect("parsed unsuccessfully");
        visit_result(result, "".to_string())
    }

    #[test]
    fn get_codegen_doesnt_throw_error() {
        let result =
            get_codegen_from_query("MATCH (a) -[]-> (b {})-[]->(c) RETURN a.count".to_string());
        assert!(!result.struct_filters.is_empty());
        let _codegen = CodeGenEnvoy::generate_code_blocks(result, [COUNT.to_string()].to_vec());
    }

    #[test]
    fn get_codegen_doesnt_throw_error_with_mult_periods() {
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b {})-[]->(c) RETURN a.node.metadata.WORKLOAD_NAME".to_string(),
        );
        assert!(!result.struct_filters.is_empty());
        let _codegen = CodeGenEnvoy::generate_code_blocks(result, [COUNT.to_string()].to_vec());
    }

    #[test]
    fn get_group_by() {
        let result = get_codegen_from_query(
            "MATCH (a {}) WHERE a.node.metadata.WORKLOAD_NAME = 'productpage-v1' RETURN a.count, agg".to_string(),
        );
        assert!(!result.struct_filters.is_empty());
        let _codegen = CodeGenEnvoy::generate_code_blocks(result, [COUNT.to_string()].to_vec());
        assert!(!_codegen.target_blocks.is_empty());
        assert!(!_codegen.ir.struct_filters.is_empty());
        assert!(!_codegen.ir.aggregate.is_none());
    }

    #[test]
    fn test_where() {
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b)-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' AND trace.request.total_size = 1 RETURN a.request.total_size, avg(a.request.total_size)".to_string(),
        );
        assert!(!result.struct_filters.is_empty());
        let _codegen = CodeGenEnvoy::generate_code_blocks(result, [COUNT.to_string()].to_vec());
        assert!(!_codegen.ir.attr_filters.is_empty());
    }

    #[test]
    fn test_aggr_udf() {
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b)-[]->(c) RETURN a.request.total_size, avg".to_string(),
        );
        let _codegen = CodeGenEnvoy::generate_code_blocks(
            result,
            [COUNT.to_string(), AVG.to_string()].to_vec(),
        );
        assert!(_codegen.aggregation_udf_table.keys().count() == 1);
    }
}
