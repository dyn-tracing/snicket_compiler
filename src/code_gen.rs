use grammar::*;
use regex::Regex;
use serde::{Serialize, Serializer};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use std::string::ToString;
use strum_macros::EnumString;
use tree_fold::TreeFold;

/// C++ type
#[derive(Clone, Copy, Display, Debug, Eq, Hash, PartialEq, EnumString)]
pub enum CppType {
    // strum crate annotation is used to implement CppType::from_str()
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "int64_t")]
    Int64T,
    #[strum(serialize = "std::string")]
    String,
    #[strum(serialize = "float")]
    Float,
}

impl Default for CppType {
    fn default() -> Self {
        CppType::String
    }
}

// Custom serialization using strum's Display implementation
impl Serialize for CppType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct NodeAttribute<'a> {
    // Service (node) id
    pub id: &'a str,
    // Envoy property parts, e.g. {"node", "metadata", "WORKLOAD_NAME"}
    pub parts: Vec<&'a str>,
    // String representation of value for above property
    // TODO: Use appropriate C++ type instead of string.
    pub value: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum AttributeType {
    Envoy,
    Custom,
}

// This struct is used to represent Envoy property with its corresponding value type.
// They're defined here: https://github.com/istio/envoy/blob/7043f39a2f5f7d072c35b3fe4d50865b5c61a9dc/source/extensions/common/wasm/context.cc#L406
// e.g. typ: CppType::String, parts: {"node", "metadata", "WORKLOAD_NAME"},
// typ: CppType::int64_t, parts: {"repsonse", "total_size"}
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct AttributeDef<'a> {
    pub cpp_type: CppType,
    pub attribute_type: AttributeType,
    pub parts: Vec<&'a str>,
}

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

#[derive(Default, Serialize, PartialEq, Eq, Debug, Clone)]
pub struct Udf {
    pub udf_type: UdfType,
    pub id: String,
    pub func_impl: String,
    pub return_type: CppType,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum CppResult {
    Return { typ: CppType, id: String },
    GroupBy { typ: CppType, id: String },
    None,
}

impl<'a> Default for CppResult {
    fn default() -> Self {
        CppResult::None
    }
}

pub struct CodeGenConfig<'a> {
    // This map is used to keep track of which attribute corresponds to which Envoy property.
    // e.g. service_name -> {"node", "metadata", "WORKLOAD_NAME"}
    // Was introduced to simplify the query language.
    pub attributes_to_property_parts: HashMap<&'static str, AttributeDef<'a>>,
    pub udf_table: HashMap<String, Udf>,
}

impl<'a> CodeGenConfig<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse_udf(&mut self, udf: String) {
        // The regex looks for following pattern
        // // udf_type: <udf_type>
        // // id: <id>
        // // return_type: <return_type>
        // // arg: <arg>
        // TODO: Support parsing multiple arguments.
        let re = Regex::new(
            r".*udf_type:\s+(?P<udf_type>\w+)\n.*id:\s+(?P<id>\w+)\n.*return_type:\s+(?P<return_type>\w+)",
        )
        .unwrap();

        let caps = re.captures(&udf).unwrap();

        let udf_type = UdfType::from_str(caps.name("udf_type").unwrap().as_str()).unwrap();
        let id = String::from(caps.name("id").unwrap().as_str());
        let return_type = CppType::from_str(caps.name("return_type").unwrap().as_str()).unwrap();

        self.udf_table.insert(
            id.clone(),
            Udf {
                udf_type,
                id,
                func_impl: udf,
                return_type,
            },
        );
    }
}

impl<'a> Default for CodeGenConfig<'a> {
    fn default() -> Self {
        let mut attributes_to_property_parts = HashMap::new();
        attributes_to_property_parts.insert(
            "service_name",
            AttributeDef {
                cpp_type: CppType::String,
                attribute_type: AttributeType::Envoy,
                parts: vec!["node", "metadata", "WORKLOAD_NAME"],
            },
        );

        attributes_to_property_parts.insert(
            "response_size",
            AttributeDef {
                cpp_type: CppType::Int64T,
                attribute_type: AttributeType::Envoy,
                parts: vec!["response", "total_size"],
            },
        );

        attributes_to_property_parts.insert(
            "height",
            AttributeDef {
                cpp_type: CppType::Int,
                attribute_type: AttributeType::Custom,
                parts: vec![],
            },
        );

        attributes_to_property_parts.insert(
            "breadth",
            AttributeDef {
                cpp_type: CppType::Int,
                attribute_type: AttributeType::Custom,
                parts: vec![],
            },
        );

        CodeGenConfig {
            attributes_to_property_parts,
            udf_table: HashMap::new(),
        }
    }
}

// TODO: Split this struct into inputs and outputs, for example CodeGenerator, and CodeGenOutput or
// HBarsInput.
#[derive(Default, Serialize)]
pub struct CodeGen<'a> {
    // Graph structures
    pub root_id: &'a str,
    pub vertices: HashSet<&'a str>,
    pub edges: Vec<(&'a str, &'a str)>,
    pub nodes_to_attributes: Vec<NodeAttribute<'a>>,

    // Envoy node attribute initializer lists.
    pub node_attributes_to_fetch: HashSet<AttributeDef<'a>>,

    // Intermediate computations necessary for computing result
    pub blocks: Vec<String>,
    pub udfs: Vec<Udf>,
    // Final computation result
    pub result: CppResult,

    // The original command used to invoke dyn_tracing.
    pub cmd: String,

    #[serde(skip_serializing)]
    pub config: CodeGenConfig<'a>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn new_with_config(config: CodeGenConfig<'a>) -> Self {
        CodeGen {
            config,
            ..Default::default()
        }
    }

    pub fn get_attribute_def(&self, attribute_id: &str) -> &AttributeDef<'a> {
        &self
            .config
            .attributes_to_property_parts
            .get(attribute_id)
            .unwrap_or_else(|| panic!("Don't support attribute {}", attribute_id))
    }

    fn codegen_envoy_property(&mut self, id: &Identifier, attribute: AttributeDef<'a>) -> String {
        let property_var_id: String =
            String::from(id.id_name) + "_" + &attribute.parts.join("_") + "_str";

        let mut parts = String::from("{");
        parts.push_str(
            &attribute
                .parts
                .iter()
                .map(|s| String::from("\"") + s + "\"")
                .collect::<Vec<String>>()
                .join(", "),
        );
        parts.push('}');

        let block = format!(
            "node_ptr = get_node_with_id(target, mapping->at(\"{node_id}\"));
if (node_ptr == nullptr || node_ptr->properties.find({parts}) == node_ptr->properties.end()) {{
    LOG_WARN(\"Node {node_id} not found\");
    return;
}}
std::string {cpp_var_id} = node_ptr->properties.at({parts});",
            node_id = id.id_name,
            parts = parts,
            cpp_var_id = property_var_id,
        );
        self.blocks.push(block);

        self.result = CppResult::Return {
            typ: attribute.cpp_type,
            id: property_var_id.clone(),
        };
        self.node_attributes_to_fetch.insert(attribute);

        property_var_id
    }

    fn codegen_get_property(&mut self, id: &Identifier, p: &Identifier) -> String {
        let attribute = self.get_attribute_def(p.id_name).clone();

        match attribute.attribute_type {
            AttributeType::Envoy => self.codegen_envoy_property(id, attribute),
            AttributeType::Custom => match p.id_name {
                "height" => {
                    let cpp_var_id = String::from(id.id_name) + "_height";
                    let block = format!(
                            "std::string {cpp_var_id} = std::to_string(get_tree_height(target, mapping->at(\"{node_id}\")));",
                            cpp_var_id = cpp_var_id,
                            node_id = id.id_name,
                        );
                    self.blocks.push(block);
                    self.result = CppResult::Return {
                        typ: attribute.cpp_type,
                        id: cpp_var_id.clone(),
                    };

                    cpp_var_id
                }
                "breadth" => {
                    let cpp_var_id = String::from(id.id_name) + "_height";
                    let block = format!(
                        "std::string {cpp_var_id} = std::to_string(get_out_degree(target, mapping->at(\"{node_id}\")));",
                        cpp_var_id = cpp_var_id,
                        node_id = id.id_name,
                    );

                    self.blocks.push(block);
                    self.result = CppResult::Return {
                        typ: attribute.cpp_type,
                        id: cpp_var_id.clone(),
                    };

                    cpp_var_id
                }
                _ => unreachable!(),
            },
        }
    }

    fn codegen_call_func(&mut self, func_name: &str, arg: &str) {
        if !self.config.udf_table.contains_key(func_name) {
            panic!("can't find udf function: {}", func_name);
        }

        let func = &self.config.udf_table[func_name];
        self.udfs.push(func.clone());
        let result_cpp_var_id = String::from(func_name) + "_udf_result";

        let block = format!(
            "auto {result_cpp_var_id} = root_->{func_name}_udf_({arg});",
            result_cpp_var_id = result_cpp_var_id,
            func_name = func_name,
            arg = arg
        );

        self.blocks.push(block);

        let key_value_block = match func.udf_type {
            UdfType::Scalar => {
                if func.return_type != CppType::String {
                    format!(
                        "value = std::to_string({result_cpp_var_id});",
                        result_cpp_var_id = result_cpp_var_id
                    )
                } else {
                    format!(
                        "value = {result_cpp_var_id};",
                        result_cpp_var_id = result_cpp_var_id
                    )
                }
            }
            UdfType::Aggregation => {
                if func.return_type != CppType::String {
                    format!(
                        "std::tie(key, value) = std::make_pair({result_cpp_var_id}.first, std::to_string({result_cpp_var_id}.second));",
                        result_cpp_var_id = result_cpp_var_id
                    )
                } else {
                    format!(
                        "std::tie(key, value) = {result_cpp_var_id};",
                        result_cpp_var_id = result_cpp_var_id
                    )
                }
            }
        };

        self.blocks.push(key_value_block);
    }
}

impl<'a> TreeFold<'a> for CodeGen<'a> {
    fn visit_prog(&mut self, prog: &'a Prog) {
        self.visit_patterns(&prog.patterns);
        self.visit_filters(&prog.filters);
        self.visit_action(&prog.action);
    }

    fn visit_pattern(&mut self, pattern: &'a Pattern) {
        let src_id = pattern.from_node.id_name;
        let dst_id = pattern.to_node.id_name;

        // TODO: Handle edge ids and edge properties.
        let _edge_id = match &pattern.relationship_type {
            Relationship::Edge(id) => String::from(id.id_name),
            Relationship::Path(_) => panic!("TODO: support EDGE relatipnship type"),
        };

        self.vertices.insert(src_id);
        self.vertices.insert(dst_id);
        self.edges.push((src_id, dst_id));
    }

    fn visit_filter(&mut self, filter: &'a Filter) {
        let Filter::Property(id, p, value) = filter;

        let node_id = id.id_name;

        let attribute_def = &self.config.attributes_to_property_parts[p.id_name];

        self.nodes_to_attributes.push(NodeAttribute {
            id: node_id,
            parts: attribute_def.parts.clone(),
            value: value.to_string(),
        });
        self.node_attributes_to_fetch.insert(attribute_def.clone());
    }

    fn visit_action(&mut self, action: &'a Action) {
        match action {
            Action::GetProperty(id, p) => {
                self.codegen_get_property(id, p);
            }
            Action::None => {}
            Action::CallUdf(id) => {
                self.codegen_call_func(id.id_name, "target");
            }
            Action::GroupBy(id, p, fid) => {
                let property_var_id = self.codegen_get_property(id, p);

                let attribute = self.get_attribute_def(p.id_name);

                let converted_id = property_var_id.clone() + "_conv";
                // C++ code for type conversion for the property
                let conv = match &attribute.cpp_type {
                    CppType::Float => format!(
                        "float {converted_id} = std::atof({cpp_var_id}.c_str());",
                        converted_id = converted_id,
                        cpp_var_id = property_var_id
                    ),
                    CppType::Int => format!(
                        "int {converted_id} = std::atoi({cpp_var_id}.c_str());",
                        converted_id = converted_id,
                        cpp_var_id = property_var_id
                    ),
                    CppType::Int64T => format!(
                        "int64_t {converted_id} = std::atoll({cpp_var_id}.c_str());",
                        converted_id = converted_id,
                        cpp_var_id = property_var_id
                    ),
                    CppType::String => format!(
                        "std::string {converted_id} = {cpp_var_id};",
                        converted_id = converted_id,
                        cpp_var_id = property_var_id
                    ),
                };

                self.blocks.push(conv);

                // Now generate code for calling user function specified with the value retrieved
                // above.
                self.codegen_call_func(fid.id_name, &converted_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer;
    use parser;
    use std::iter::Peekable;
    use token::Token;

    #[test]
    fn test_match() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH a-->b : x,b-->c : y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["a", "b", "c"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("a", "b"), ("b", "c")]);
        assert!(code_gen.nodes_to_attributes.is_empty());
    }

    #[test]
    fn test_match_handle_ordering() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["a", "b", "c"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("b", "c"), ("a", "b")]);
        assert!(code_gen.nodes_to_attributes.is_empty());
    }

    #[test]
    fn test_codegen_parallel() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,a-->d:z,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(
            code_gen.vertices,
            ["a", "b", "c", "d"].iter().cloned().collect()
        );
        assert_eq!(code_gen.edges, vec![("b", "c"), ("a", "b"), ("a", "d")]);
        assert!(code_gen.nodes_to_attributes.is_empty());
    }

    #[test]
    fn test_codegen_action() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x == k, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_config(CodeGenConfig {
            attributes_to_property_parts: [(
                "x",
                AttributeDef {
                    cpp_type: CppType::String,
                    attribute_type: AttributeType::Envoy,
                    parts: vec!["x"],
                },
            )]
            .iter()
            .cloned()
            .collect(),
            ..Default::default()
        });
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.blocks,
            vec![
                (String::from(
                    "node_ptr = get_node_with_id(target, mapping->at(\"n\"));
if (node_ptr == nullptr || node_ptr->properties.find({\"x\"}) == node_ptr->properties.end()) {
    LOG_WARN(\"Node n not found\");
    return;
}
std::string n_x_str = node_ptr->properties.at({\"x\"});"
                ))
            ]
        );
        assert_eq!(
            code_gen.nodes_to_attributes,
            vec![NodeAttribute {
                id: "n",
                parts: vec!["x"],
                value: String::from("k"),
            }]
        );
        assert_eq!(
            code_gen.result,
            CppResult::Return {
                typ: CppType::String,
                id: String::from("n_x_str"),
            }
        )
    }

    #[test]
    fn test_codegen_action_without_where() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_config(CodeGenConfig {
            attributes_to_property_parts: [(
                "x",
                AttributeDef {
                    cpp_type: CppType::String,
                    attribute_type: AttributeType::Envoy,
                    parts: vec!["x"],
                },
            )]
            .iter()
            .cloned()
            .collect(),
            ..Default::default()
        });

        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.blocks,
            vec![
                (String::from(
                    "node_ptr = get_node_with_id(target, mapping->at(\"n\"));
if (node_ptr == nullptr || node_ptr->properties.find({\"x\"}) == node_ptr->properties.end()) {
    LOG_WARN(\"Node n not found\");
    return;
}
std::string n_x_str = node_ptr->properties.at({\"x\"});"
                ))
            ]
        );
        assert_eq!(
            code_gen.result,
            CppResult::Return {
                typ: CppType::String,
                id: String::from("n_x_str"),
            }
        )
    }

    #[test]
    fn test_codegen_multiple_node_properties_to_collect() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_config(CodeGenConfig {
            attributes_to_property_parts: [
                (
                    "x",
                    AttributeDef {
                        cpp_type: CppType::String,
                        attribute_type: AttributeType::Envoy,
                        parts: vec!["x"],
                    },
                ),
                (
                    "y",
                    AttributeDef {
                        cpp_type: CppType::Int64T,
                        attribute_type: AttributeType::Envoy,
                        parts: vec!["y"],
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
            ..Default::default()
        });

        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.nodes_to_attributes,
            vec![NodeAttribute {
                id: "n",
                parts: vec!["x"],
                value: String::from("k"),
            }]
        );
        assert_eq!(
            code_gen.node_attributes_to_fetch,
            vec![
                AttributeDef {
                    cpp_type: CppType::String,
                    attribute_type: AttributeType::Envoy,
                    parts: vec!["x"]
                },
                AttributeDef {
                    cpp_type: CppType::Int64T,
                    attribute_type: AttributeType::Envoy,
                    parts: vec!["y"]
                }
            ]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_codegen_call_udf() {
        let tokens = lexer::get_tokens(r"MATCH n-->m: a, RETURN max_response_size,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.config.udf_table.insert(
            String::from("max_response_size"),
            Udf {
                udf_type: UdfType::Scalar,
                id: String::from("max_response_size"),
                func_impl: String::from("function_impl"),
                return_type: CppType::Int64T,
            },
        );
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);

        assert_eq!(
            code_gen.blocks,
            vec![
                String::from(
                    "auto max_response_size_udf_result = root_->max_response_size_udf_(target);"
                ),
                String::from("value = std::to_string(max_response_size_udf_result);")
            ]
        );
        assert_eq!(
            code_gen.udfs,
            vec![Udf {
                udf_type: UdfType::Scalar,
                id: String::from("max_response_size"),
                func_impl: String::from("function_impl"),
                return_type: CppType::Int64T,
            }],
        );
    }

    #[test]
    fn test_group_by() {
        let tokens = lexer::get_tokens(r"MATCH n-->m: a, GROUP a.response_size BY max,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();

        code_gen.config.udf_table.insert(
            String::from("max"),
            Udf {
                udf_type: UdfType::Aggregation,
                id: String::from("max"),
                func_impl: String::from("function_impl"),
                return_type: CppType::Int,
            },
        );
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.node_attributes_to_fetch,
            vec![AttributeDef {
                cpp_type: CppType::Int64T,
                attribute_type: AttributeType::Envoy,
                parts: vec!["response", "total_size"]
            }]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_init_udf_table() {
        let mut config = CodeGenConfig::new();
        config.parse_udf(String::from(
            "// udf_type: Scalar
                      // id: max_response_size
                      // return_type: int

                      class max_response_size",
        ));

        let parsed_udf = config
            .udf_table
            .get(&String::from("max_response_size"))
            .unwrap();
        assert_eq!(parsed_udf.udf_type, UdfType::Scalar);
        assert_eq!(parsed_udf.id, String::from("max_response_size"));
        assert_eq!(parsed_udf.return_type, CppType::Int);
        assert!(parsed_udf.func_impl.contains("class max_response_size"));
    }
}
