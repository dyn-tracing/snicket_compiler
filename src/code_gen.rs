use grammar::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::fmt;
use tree_fold::TreeFold;

#[derive(Serialize, PartialEq, Eq, Debug, Hash, Clone)]
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

#[derive(Serialize)]
pub struct CppDefVar {
    typ: CppType,
    id: String,
    rvalue: String,
}

#[derive(Default, Serialize, PartialEq, Eq, Debug, Hash, Clone)]
pub struct Attribute<'a> {
    pub typ: CppType,
    pub parts: Vec<&'a str>,
    pub value: Option<String>,
}

// pub struct Attribute {
//     pub id: String,
//     pub value: String,
// }

// // Run the code to retrieve Envoy property.
// // {{typ}} {{parts.join(_)}};
// // if (!getValue({ {{#each parts}} "{{this}}", {{/each}} }, &{{parts.join(_)}})) {
// //   logWarn("Failed to retrieve property. {{parts.join(_}}");
// // }
// pub struct EnvoyProperty {
//     pub typ: CPPType,
//     pub parts: Vec<String>,
// }

// // Call function `func_name` which returns a value of type `return_type` using `args`.
// // {{return_type}}
// pub struct Callfunc {
//     pub return_type: CPPType,
//     pub func_name: String,
//     pub args: Vec<String>,
// }

#[derive(Default, Serialize, PartialEq, Eq, Debug)]
pub struct ReturnProperty<'a> {
    pub id: &'a str,
    pub paths: Vec<&'a str>,
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
    pub return_type: &'a str,
    pub arg: Vec<&'a str>,
}

#[derive(Serialize, PartialEq, Eq, Debug)]
pub enum Return<'a> {
    Property(ReturnProperty<'a>),
    CallUdf(Udf<'a>),
    None,
}

impl<'a> Default for Return<'a> {
    fn default() -> Self {
        Return::None
    }
}

#[derive(Serialize)]
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

    // For running code to get node/graph attributes.
    pub node_attributes_to_collect: HashSet<Attribute<'a>>,
    pub graph_attributes_to_collect: Vec<Attribute<'a>>,

    // Final computation result
    pub return_stmt: Return<'a>,
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

        self.node_attributes_to_collect.insert(attribute.clone());

        attribute.value = Some(value.to_string());

        self.ids_to_attributes
            .entry(vertex_id)
            .or_default()
            .push(attribute.clone());
    }

    fn visit_action(&mut self, action: &'a Action) {
        match action {
            Action::GetProperty(id, p) => {
                if id.id_name == "target" {
                    if p.id_name == "height" {
                        self.graph_attributes_to_collect.push(Attribute {
                            typ: CppType::Int,
                            parts: vec!["get_tree_height"],
                            value: None,
                        });
                    } else {
                        panic!("{} graph property not supported", p.id_name)
                    }
                } else {
                    self.node_attributes_to_collect
                        .insert(self.config.attributes_to_property_parts[p.id_name].clone());

                    self.return_stmt = Return::Property(ReturnProperty {
                        id: id.id_name,
                        paths: self.config.attributes_to_property_parts[p.id_name]
                            .parts
                            .clone(),
                    });
                }
            }
            Action::None => {}
            Action::CallUdf(id) => {
                if !self.config.udf_table.contains_key(id.id_name) {
                    panic!("Can't find udf function: {}", id.id_name);
                }
                let func = self.config.udf_table[id.id_name].clone();
                self.return_stmt = Return::CallUdf(func);
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
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.x,");
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
        // assert_eq!(
        //     code_gen.ids_to_properties,
        //     vec![Property {
        //         id: "n",
        //         parts: vec!["x"],
        //         value: String::from("k"),
        //     }]
        // );
        assert_eq!(
            code_gen.return_stmt,
            Return::Property(ReturnProperty {
                id: "n",
                paths: vec!["x"]
            })
        );
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
            code_gen.return_stmt,
            Return::Property(ReturnProperty {
                id: "n",
                paths: vec!["x"]
            })
        );
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
        // assert_eq!(
        //     code_gen.ids_to_properties,
        //     vec![Property {
        //         id: "n",
        //         parts: vec!["x"],
        //         value: String::from("k"),
        //     }]
        // );
        assert_eq!(
            code_gen.node_attributes_to_collect,
            [
                Attribute {
                    typ: CppType::String,
                    parts: vec!["x"],
                    value: None,
                },
                Attribute {
                    typ: CppType::Int64T,
                    parts: vec!["y"],
                    value: None,
                },
            ]
            .iter()
            .cloned()
            .collect()
        );
        assert_eq!(
            code_gen.return_stmt,
            Return::Property(ReturnProperty {
                id: "n",
                paths: vec!["y"]
            })
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
            code_gen.graph_attributes_to_collect,
            vec![Attribute {
                typ: CppType::Int,
                parts: vec!["get_tree_height"],
                value: None,
            }]
        );
    }
}
