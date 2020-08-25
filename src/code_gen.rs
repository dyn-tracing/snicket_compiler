use grammar::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tree_fold::TreeFold;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub enum CppType {
    Int,
    Int64T,
    String,
}

impl Default for CppType {
    fn default() -> Self {
        CppType::String
    }
}

impl fmt::Display for CppType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CppType::Int => write!(f, "int"),
            CppType::Int64T => write!(f, "int64_t"),
            CppType::String => write!(f, "std::string"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub enum CppCodeBlock<'a> {
    NodeEnvoyProperty {
        typ: CppType,
        id: String,
        parts: Vec<&'a str>,
    },
    CallFunc {
        typ: CppType,
        udf_id: &'a str,
        args: Vec<&'a str>,
    },
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Attribute<'a> {
    pub typ: CppType,
    pub parts: Vec<&'a str>,
    pub value: Option<String>,
}

#[derive(Serialize, PartialEq, Eq, Debug, Clone)]
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
pub struct Udf<'a> {
    pub udf_type: UdfType,
    pub id: &'a str,
    pub func_impl: &'a str,
    pub return_type: CppType,
    pub arg: Vec<&'a str>,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum Result {
    Return { typ: CppType, id: String },
    GroupBy { typ: CppType, id: String },
    None,
}

impl<'a> Default for Result {
    fn default() -> Self {
        Result::None
    }
}

pub struct CodeGenConfig<'a> {
    pub attributes_to_property_parts: HashMap<&'static str, Attribute<'a>>,
    pub udf_table: HashMap<&'static str, Udf<'a>>,
}

impl<'a> CodeGenConfig<'a> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'a> Default for CodeGenConfig<'a> {
    fn default() -> Self {
        let mut attributes_to_property_parts = HashMap::new();
        attributes_to_property_parts.insert(
            "service_name",
            Attribute {
                typ: CppType::String,
                parts: vec!["node", "metadata", "WORKLOAD_NAME"],
                value: None,
            },
        );

        attributes_to_property_parts.insert(
            "response_size",
            Attribute {
                typ: CppType::Int64T,
                parts: vec!["response", "total_size"],
                value: None,
            },
        );

        CodeGenConfig {
            attributes_to_property_parts,
            udf_table: HashMap::new(),
        }
    }
}

#[derive(Default, Serialize)]
pub struct CodeGen<'a> {
    // Graph structures
    pub root_id: &'a str,
    pub vertices: HashSet<&'a str>,
    pub edges: Vec<(&'a str, &'a str)>,
    pub ids_to_attributes: HashMap<&'a str, Vec<Attribute<'a>>>,

    // Intermediate computations necessary for computing result
    pub blocks: Vec<CppCodeBlock<'a>>,
    pub udf_impls: Vec<&'a str>,
    // Final computation result
    pub result: Result,

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

        let vertex_id = id.id_name;

        let mut attribute = self.config.attributes_to_property_parts[p.id_name].clone();
        attribute.value = Some(value.to_string());

        self.ids_to_attributes
            .entry(vertex_id)
            .or_default()
            .push(attribute);
    }

    fn visit_action(&mut self, action: &'a Action) {
        match action {
            Action::GetProperty(id, p) => {
                if id.id_name == "target" {
                    if p.id_name == "height" {
                        let cpp_var_id = "get_tree_height_target";
                        self.blocks.push(CppCodeBlock::CallFunc {
                            typ: CppType::Int,
                            udf_id: "get_tree_height",
                            args: vec!["target"],
                        });

                        self.result = Result::Return {
                            typ: CppType::Int,
                            id: String::from(cpp_var_id),
                        };
                    } else {
                        panic!("{} graph property not supported", p.id_name)
                    }
                } else {
                    let attribute = &self.config.attributes_to_property_parts[p.id_name];

                    let cpp_var_id: String =
                        String::from(id.id_name) + "_" + &attribute.parts.join("_");
                    self.blocks.push(CppCodeBlock::NodeEnvoyProperty {
                        typ: attribute.typ,
                        id: cpp_var_id.clone(),
                        parts: attribute.parts.clone(),
                    });

                    self.result = Result::Return {
                        typ: attribute.typ,
                        id: cpp_var_id,
                    };
                }
            }
            Action::None => {}
            Action::CallUdf(id) => {
                if !self.config.udf_table.contains_key(id.id_name) {
                    panic!("Can't find udf function: {}", id.id_name);
                }
                let func = &self.config.udf_table[id.id_name];

                self.blocks.push(CppCodeBlock::CallFunc {
                    typ: func.return_type,
                    udf_id: func.id,
                    args: vec![],
                });

                self.udf_impls.push(func.func_impl);

                self.result = Result::Return {
                    typ: func.return_type,
                    id: String::from(func.id) + "_result",
                }
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
        assert!(code_gen.ids_to_attributes.is_empty());
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
        assert!(code_gen.ids_to_attributes.is_empty());
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
        assert!(code_gen.ids_to_attributes.is_empty());
    }

    #[test]
    fn test_codegen_action() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x == k, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_config(CodeGenConfig {
            attributes_to_property_parts: [(
                "x",
                Attribute {
                    typ: CppType::String,
                    parts: vec!["x"],
                    value: None,
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
            vec![CppCodeBlock::NodeEnvoyProperty {
                typ: CppType::String,
                id: String::from("n_x"),
                parts: vec!["x"]
            }]
        );
        assert_eq!(
            code_gen.ids_to_attributes,
            [(
                "n",
                vec![Attribute {
                    typ: CppType::String,
                    parts: vec!["x"],
                    value: Some(String::from("k")),
                }]
            )]
            .iter()
            .cloned()
            .collect()
        );
        assert_eq!(
            code_gen.result,
            Result::Return {
                typ: CppType::String,
                id: String::from("n_x"),
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
                Attribute {
                    typ: CppType::String,
                    parts: vec!["x"],
                    value: None,
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
            vec![CppCodeBlock::NodeEnvoyProperty {
                typ: CppType::String,
                id: String::from("n_x"),
                parts: vec!["x"]
            }]
        );
        assert_eq!(
            code_gen.result,
            Result::Return {
                typ: CppType::String,
                id: String::from("n_x"),
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
                    Attribute {
                        typ: CppType::String,
                        parts: vec!["x"],
                        value: None,
                    },
                ),
                (
                    "y",
                    Attribute {
                        typ: CppType::Int64T,
                        parts: vec!["y"],
                        value: None,
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
            code_gen.ids_to_attributes,
            [(
                "n",
                vec![Attribute {
                    typ: CppType::String,
                    parts: vec!["x"],
                    value: Some(String::from("k")),
                }]
            )]
            .iter()
            .cloned()
            .collect()
        );
    }

    #[test]
    fn test_codegen_graph_properties() {
        let tokens = lexer::get_tokens(r"MATCH n-->m: a, RETURN target.height,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.blocks,
            vec![CppCodeBlock::CallFunc {
                typ: CppType::Int,
                udf_id: "get_tree_height",
                args: vec!["target"],
            }]
        );
        assert_eq!(
            code_gen.result,
            Result::Return {
                typ: CppType::Int,
                id: String::from("get_tree_height_target")
            }
        );
    }

    #[test]
    fn test_codegen_call_udf() {
        let tokens = lexer::get_tokens(r"MATCH n-->m: a, RETURN max_response_size,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.config.udf_table.insert(
            "max_response_size",
            Udf {
                udf_type: UdfType::Scalar,
                id: "max_response_size",
                func_impl: "function_impl",
                return_type: CppType::Int64T,
                arg: vec![],
            },
        );
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);

        assert_eq!(
            code_gen.blocks,
            vec![CppCodeBlock::CallFunc {
                typ: CppType::Int64T,
                udf_id: "max_response_size",
                args: vec![],
            }]
        );
        assert_eq!(code_gen.udf_impls, vec!["function_impl"]);
        assert_eq!(
            code_gen.result,
            Result::Return {
                typ: CppType::Int64T,
                id: String::from("max_response_size_result"),
            }
        );
    }
}
