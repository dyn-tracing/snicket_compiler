use grammar::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use tree_fold::TreeFold;

#[derive(Default, Serialize, PartialEq, Eq, Debug)]
pub struct Property<'a> {
    pub id: &'a str,
    pub paths: Vec<&'a str>,
    pub value: String,
}

#[derive(Default, Serialize, PartialEq, Eq, Debug, Hash, Clone)]
pub struct ToCollect<'a> {
    pub typ: &'a str,
    pub paths: Vec<&'a str>,
}

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

#[derive(Default, Serialize)]
pub struct CodeGen<'a> {
    pub root_id: &'a str,
    pub vertices: HashSet<&'a str>,
    pub edges: Vec<(&'a str, &'a str)>,
    pub ids_to_properties: Vec<Property<'a>>,
    pub properties_to_collect: HashSet<ToCollect<'a>>,
    pub return_stmt: Return<'a>,
    #[serde(skip_serializing)]
    property_map: HashMap<&'static str, ToCollect<'a>>,
    #[serde(skip_serializing)]
    pub udf_table: HashMap<&'static str, Udf<'a>>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> CodeGen<'a> {
        let mut property_map = HashMap::new();
        // TODO: need to specify the type of value returned.
        property_map.insert(
            "service_name",
            ToCollect {
                typ: "std::string",
                paths: vec!["node", "metadata", "WORKLOAD_NAME"],
            },
        );
        property_map.insert(
            "response_size",
            ToCollect {
                typ: "int64_t",
                paths: vec!["response", "total_size"],
            },
        );

        CodeGen {
            property_map,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn new_with_property_map(property_map: HashMap<&'static str, ToCollect<'a>>) -> Self {
        CodeGen {
            property_map,
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
        let to_collect = &self.property_map[p.id_name];
        let value_str = value.to_string();

        self.properties_to_collect.insert(to_collect.clone());

        self.ids_to_properties.push(Property {
            id: vertex_id,
            paths: to_collect.paths.clone(),
            value: value_str,
        });
    }

    fn visit_action(&mut self, action: &'a Action) {
        match action {
            Action::Property(id, p) => {
                self.properties_to_collect
                    .insert(self.property_map[p.id_name].clone());

                self.return_stmt = Return::Property(ReturnProperty {
                    id: id.id_name,
                    paths: self.property_map[p.id_name].paths.clone(),
                });
            }
            Action::None => {}
            Action::CallUdf(id) => {
                if !self.udf_table.contains_key(id.id_name) {
                    panic!("Can't find udf function: {}", id.id_name);
                }
                let func = self.udf_table[id.id_name].clone();
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
        assert!(code_gen.ids_to_properties.is_empty());
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
        assert!(code_gen.ids_to_properties.is_empty());
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
        assert!(code_gen.ids_to_properties.is_empty());
    }

    #[test]
    fn test_codegen_action() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_property_map(
            [(
                "x",
                ToCollect {
                    typ: "string",
                    paths: vec!["x"],
                },
            )]
            .iter()
            .cloned()
            .collect(),
        );
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.ids_to_properties,
            vec![Property {
                id: "n",
                paths: vec!["x"],
                value: String::from("k"),
            }]
        );
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
        let mut code_gen = CodeGen::new_with_property_map(
            [(
                "x",
                ToCollect {
                    typ: "string",
                    paths: vec!["x"],
                },
            )]
            .iter()
            .cloned()
            .collect(),
        );

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
    fn test_codegen_multiple_properties_to_collect() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_property_map(
            [
                (
                    "x",
                    ToCollect {
                        typ: "string",
                        paths: vec!["x"],
                    },
                ),
                (
                    "y",
                    ToCollect {
                        typ: "int64_t",
                        paths: vec!["y"],
                    },
                ),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.ids_to_properties,
            vec![Property {
                id: "n",
                paths: vec!["x"],
                value: String::from("k"),
            }]
        );
        assert_eq!(
            code_gen.properties_to_collect,
            [
                ToCollect {
                    typ: "string",
                    paths: vec!["x"],
                },
                ToCollect {
                    typ: "int64_t",
                    paths: vec!["y"],
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
    fn test_codegen_udf() {}
}
