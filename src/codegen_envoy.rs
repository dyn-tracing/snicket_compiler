use super::codegen_common::AggregationUdf;
use super::codegen_common::ScalarUdf;
use super::codegen_common::UdfType;
use super::ir::IrReturnEnum;
use super::ir::VisitorResults;
use crate::codegen_common::CodeStruct;
use crate::ir::Aggregate;
use crate::ir::AttributeFilter;
use crate::ir::Property;
use crate::ir::PropertyOrUDF;
use crate::ir::StructuralFilter;
use crate::ir::UdfCall;
use indexmap::IndexMap;
use indexmap::IndexSet;
use regex::Regex;
use std::str::FromStr;

/********************************/
// Code Generation
/********************************/

fn make_struct_filter_blocks(
    attr_filters: &[AttributeFilter],
    struct_filters: &[StructuralFilter],
) -> Vec<String> {
    let mut target_blocks = Vec::new();
    for struct_filter in struct_filters {
        target_blocks.push(" let vertices = vec!( ".to_string());
        for vertex in &struct_filter.vertices {
            target_blocks.push(format!("\"{vertex}\".to_string(),", vertex = vertex));
        }
        target_blocks.push(" );\n".to_string());

        target_blocks.push("        let edges = vec!( ".to_string());
        for edge in &struct_filter.edges {
            target_blocks.push(format!(
                " (\"{edge1}\".to_string(), \"{edge2}\".to_string() ), ",
                edge1 = edge.0,
                edge2 = edge.1
            ));
        }
        target_blocks.push(" );\n".to_string());

        let ids_to_prop_block = "        let mut ids_to_properties: IndexMap<String, IndexMap<String, String>> = IndexMap::new();\n".to_string();
        target_blocks.push(ids_to_prop_block);

        for vertex in &struct_filter.vertices {
            let ids_to_properties_hashmap_init = format!(
                "        ids_to_properties.insert(\"{node}\".to_string(), IndexMap::new());\n",
                node = vertex
            );
            target_blocks.push(ids_to_properties_hashmap_init);
        }
        for node in struct_filter.properties.keys() {
            let get_hashmap = format!(
                    "        let mut {node}_hashmap = ids_to_properties.get_mut(\"{node}\").unwrap();\n",
                    node = node
                );
            target_blocks.push(get_hashmap);
            for property_name in struct_filter.properties[node].keys() {
                let fill_in_hashmap = format!("        {node}_hashmap.insert(\"{property_name}\".to_string(), \"{property_value}\".to_string());\n",
                                                   node=node,
                                                   property_name=property_name,
                                                   property_value=struct_filter.properties[node][property_name]);
                target_blocks.push(fill_in_hashmap);
            }
            for property_filter in attr_filters {
                if property_filter.node != "trace" {
                    let mut property_name_without_period = property_filter.property.clone();
                    if property_name_without_period.starts_with('.') {
                        property_name_without_period.remove(0);
                    }
                    let fill_in_hashmap = format!("        {node}_hashmap.insert(\"{property_name}\".to_string(), \"{property_value}\".to_string());\n",
                                                       node=property_filter.node,
                                                       property_name=property_name_without_period,
                                                       property_value=property_filter.value);
                    target_blocks.push(fill_in_hashmap);
                }
            }
        }
        let make_graph =
            "        return generate_target_graph(vertices, edges, ids_to_properties);\n"
                .to_string();

        target_blocks.push(make_graph);
    }
    target_blocks
}

fn make_attr_filter_blocks(root_id: &str, attr_filters: &[AttributeFilter]) -> Vec<String> {
    // for everything except trace level attributes, the UDF/envoy property
    // collection will make the attribute filtering happen at the same time as
    // the struct filtering.  This is not the case for trace-level attributes
    let mut trace_lvl_prop_blocks = Vec::new();

    let if_root_block = "
            if &http_headers.workload_name == root_id {"
        .to_string();
    trace_lvl_prop_blocks.push(if_root_block);
    let init_trace_prop_str = "        let mut trace_prop_str : String;\n".to_string();
    trace_lvl_prop_blocks.push(init_trace_prop_str);

    for attr_filter in attr_filters {
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
                    match serde_json::to_string(&fd) {{
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
                ", root_id=root_id, prop_name=prop, value=attr_filter.value);
            trace_lvl_prop_blocks.push(trace_filter_block);
        }
    }

    let end_root_block = "       }".to_string();
    trace_lvl_prop_blocks.push(end_root_block);
    trace_lvl_prop_blocks
}

#[allow(dead_code)]
fn make_trace_rpc_value(code_struct: &mut CodeStruct) {
    let ret_block = "
        match serde_json::to_string(fd) {
            Ok(trace_str) => { value = trace_str; }
            Err(e) => { log::error!(\"Error:  could not translate ferried data to string\"); return; }\
        }
        "
        .to_string();
    code_struct.response_blocks.push(ret_block);
}

fn make_storage_rpc_value_from_trace(entity: String, property: &str) -> String {
    return format!(
        "let trace_node_idx = get_node_with_id(&fd.trace_graph, \"{node_id}\".to_string());
        if trace_node_idx.is_none() {{
           log::error!(\"Node {node_id} not found\");
                return None;
        }}
        let ret = &fd.trace_graph.node_weight(trace_node_idx.unwrap()).unwrap().1[\"{prop}\"];\n
        value = ret.to_string();\n",
        node_id = entity,
        prop = property
    );
}

fn make_storage_rpc_value_from_target(entity: &str, property: &str) -> String {
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
            .contains_key(\"{property}\")
        {{
            // we have not yet collected the return property
            log::error!(\"Missing return property {property}\");
            return None;
        }}
        let ret = &stored_data.trace_graph.node_weight(trace_node_idx).unwrap().1[ \"{property}\" ];\n
        value = ret.to_string();\n",
                node_id = entity,
                property = property
        );

    ret_block
}

fn make_return_block(entity_ref: &PropertyOrUDF, query_data: &VisitorResults) -> String {
    match entity_ref {
        PropertyOrUDF::Property(prop) => match prop.parent.as_str() {
            "trace" => make_storage_rpc_value_from_trace(query_data.root_id.clone(), &prop.parent),
            _ => make_storage_rpc_value_from_target(&prop.parent, &prop.to_dot_string()),
        },
        PropertyOrUDF::UdfCall(call) => {
            // Because of quirky design we need to get the first arg
            if call.args.len() != 1 {
                panic!("We currently only implement very specific arguments for UDFs!");
            }
            let node = &call.args[0];
            match node.as_str() {
                "trace" => make_storage_rpc_value_from_trace(query_data.root_id.clone(), &node),
                _ => make_storage_rpc_value_from_target(&node, &call.to_ref_str()),
            }
        }
    }
}

fn make_aggr_block(agg: &Aggregate, query_data: &VisitorResults) -> String {
    make_return_block(&agg.property, query_data)
}

fn generate_property_blocks(properties: &IndexSet<Property>) -> Vec<String> {
    let mut property_blocks = Vec::new();
    for property in properties {
        // There is nothing to fetch so ignore.
        // TODO: What do we actually need here?
        if property.members.is_empty() {
            continue;
        }
        let get_prop_block = format!(
            "prop_tuple_wrapped = fetch_property(&http_headers.workload_name,
                                        &{property},
                                        http_headers);
                                            ",
            property = property.as_vec_str(),
        );
        property_blocks.push(get_prop_block);
        let push_block = "if let Some(prop_tuple) = prop_tuple_wrapped {
            fd.unassigned_properties.push(prop_tuple);
            } else {
                return Err(());
            }"
        .to_string();
        property_blocks.push(push_block);
    }
    property_blocks
}

fn generate_udf_blocks(
    scalar_udf_table: &IndexMap<String, ScalarUdf>,
    aggregation_udf_table: &IndexMap<String, AggregationUdf>,
    udf_calls: &IndexSet<UdfCall>,
) -> Vec<String> {
    let mut udf_blocks = Vec::new();
    for call in udf_calls {
        let udf_ref = call.to_ref_str();
        if aggregation_udf_table.contains_key(&call.id) {
            // TODO: Aggregations are handled separately, where do they go?
            continue;
        }
        if !scalar_udf_table.contains_key(&call.id) {
            log::error!("ID {:?} not found in the scalar UDF map!", call.id);
            std::process::exit(1);
        }
        let get_udf_vals = format!(
            "let my_{id}_value;
            let child_iterator = fd.trace_graph.neighbors_directed(
                get_node_with_id(&fd.trace_graph, http_headers.workload_name.clone()).unwrap(),
                petgraph::Outgoing);
            let mut child_values = Vec::new();
            for child in child_iterator {{
                child_values.push(fd.trace_graph.node_weight(child).unwrap().1[\"{udf_ref}\"].clone());
            }}
            if child_values.len() == 0 {{
                my_{id}_value = {leaf_func}(&fd.trace_graph).to_string();
            }} else {{
                my_{id}_value = {mid_func}(&fd.trace_graph, child_values).to_string();
            }}

        ",
            id = call.id,
            udf_ref = udf_ref,
            leaf_func = scalar_udf_table[&call.id].leaf_func,
            mid_func = scalar_udf_table[&call.id].mid_func
        );
        udf_blocks.push(get_udf_vals);

        let save_udf_vals = format!(
            "
        let node = get_node_with_id(&fd.trace_graph, http_headers.workload_name.clone()).unwrap();
        // if we already have the property, don't add it
        if !( fd.trace_graph.node_weight(node).unwrap().1.contains_key(\"{udf_ref}\") &&
               fd.trace_graph.node_weight(node).unwrap().1[\"{udf_ref}\"] == my_{id}_value ) {{
           fd.trace_graph.node_weight_mut(node).unwrap().1.insert(
               \"{udf_ref}\".to_string(), my_{id}_value);
        }}
        ",
            id = call.id,
            udf_ref = udf_ref
        );
        udf_blocks.push(save_udf_vals);
    }
    udf_blocks
}

enum ScalarOrAggregationUdf {
    ScalarUdf(ScalarUdf),
    AggregationUdf(AggregationUdf),
}

fn parse_udf(udf: String) -> ScalarOrAggregationUdf {
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

pub fn generate_code_blocks(query_data: VisitorResults, udf_paths: Vec<String>) -> CodeStruct {
    let mut code_struct = CodeStruct::new(&query_data.root_id);
    let mut scalar_udf_table: IndexMap<String, ScalarUdf> = IndexMap::new();
    // where we store udf implementations
    let mut aggregation_udf_table: IndexMap<String, AggregationUdf> = IndexMap::new();
    for udf_path in udf_paths {
        log::debug!("UDF: {:?}", udf_path);
        match parse_udf(udf_path) {
            ScalarOrAggregationUdf::ScalarUdf(udf) => {
                scalar_udf_table.insert(udf.id.clone(), udf);
            }
            ScalarOrAggregationUdf::AggregationUdf(udf) => {
                aggregation_udf_table.insert(udf.id.clone(), udf);
            }
        }
    }
    // all the properties we collect
    code_struct.request_blocks = generate_property_blocks(&query_data.properties);
    code_struct.udf_blocks = generate_udf_blocks(
        &scalar_udf_table,
        &aggregation_udf_table,
        &query_data.udf_calls,
    );
    code_struct.target_blocks =
        make_struct_filter_blocks(&query_data.attr_filters, &query_data.struct_filters);
    code_struct.trace_lvl_prop_blocks =
        make_attr_filter_blocks(&query_data.root_id, &query_data.attr_filters);

    let resp_block = match query_data.return_expr {
        IrReturnEnum::PropertyOrUDF(ref entity_ref) => make_return_block(entity_ref, &query_data),
        IrReturnEnum::Aggregate(ref agg) => make_aggr_block(&agg, &query_data),
    };
    code_struct.response_blocks.push(resp_block);
    code_struct.aggregation_udf_table = aggregation_udf_table;
    code_struct.scalar_udf_table = scalar_udf_table;
    code_struct
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
    // udf_type: Aggregation
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
        let _codegen = generate_code_blocks(result, [COUNT.to_string()].to_vec());
    }

    #[test]
    fn get_codegen_doesnt_throw_error_with_mult_periods() {
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b {})-[]->(c) RETURN a.node.metadata.WORKLOAD_NAME".to_string(),
        );
        assert!(!result.struct_filters.is_empty());
        let _codegen = generate_code_blocks(result, [COUNT.to_string()].to_vec());
    }

    #[test]
    fn get_group_by() {
        // TODO: These tests are odd, not sure what we want to test here
        let result = get_codegen_from_query(
            "MATCH (a {}) WHERE a.node.metadata.WORKLOAD_NAME = 'productpage-v1' RETURN count(a), agg".to_string(),
        );
        assert!(!result.struct_filters.is_empty());
        // Do not throw an error parsing this expression.
        let _codegen = generate_code_blocks(result, [COUNT.to_string()].to_vec());
    }

    #[test]
    fn test_where() {
        // TODO: These tests are odd, not sure what we want to test here
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b)-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' AND trace.request.total_size = 1 RETURN a.request.total_size, avg(a.request.total_size)".to_string(),
        );
        assert!(!result.struct_filters.is_empty());
        assert!(!result.attr_filters.is_empty());
        // Do not throw an error parsing this expression.
        let _codegen = generate_code_blocks(result, [AVG.to_string()].to_vec());
    }

    #[test]
    fn test_aggr_udf() {
        let result = get_codegen_from_query(
            "MATCH (a) -[]-> (b)-[]->(c) RETURN a.request.total_size, avg".to_string(),
        );
        // Do not throw an error parsing this expression.
        let codegen = generate_code_blocks(result, [AVG.to_string()].to_vec());
        assert!(codegen.aggregation_udf_table.keys().count() == 1);
    }
}
