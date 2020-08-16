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

#[derive(Default, Serialize, PartialEq, Eq, Debug)]
pub struct Return<'a> {
    pub id: &'a str,
    pub paths: Vec<&'a str>,
}

#[derive(Default, Serialize)]
pub struct CodeGen<'a> {
    pub root_id: &'a str,
    pub vertices: HashSet<&'a str>,
    pub edges: Vec<(&'a str, &'a str)>,
    pub ids_to_properties: Vec<Property<'a>>,
    pub properties_to_collect: HashSet<Vec<&'a str>>,
    pub return_action: Return<'a>,
    #[serde(skip_serializing)]
    property_map: HashMap<&'static str, Vec<&'static str>>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> CodeGen<'a> {
        let mut property_map = HashMap::new();
        property_map.insert("service_name", vec!["node", "metadata", "WORKLOAD_NAME"]);
        property_map.insert("request_dur", vec!["request", "duration"]);

        CodeGen {
            property_map,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn new_with_property_map(property_map: HashMap<&'static str, Vec<&'static str>>) -> Self {
        CodeGen {
            property_map,
            ..Default::default()
        }
    }
}

impl<'a> TreeFold<'a> for CodeGen<'a> {
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
        let property_paths: Vec<&'a str> = self.property_map[p.id_name].clone();
        let value_str = value.to_string();

        self.properties_to_collect.insert(property_paths.clone());

        self.ids_to_properties.push(Property {
            id: vertex_id,
            paths: property_paths,
            value: value_str,
        });
    }

    fn visit_action(&mut self, action: &'a Action) {
        let Action::Property(id, p) = action;

        self.properties_to_collect
            .insert(self.property_map[p.id_name].clone());

        self.return_action = Return {
            id: id.id_name,
            paths: self.property_map[p.id_name].clone(),
        };
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
        let mut code_gen =
            CodeGen::new_with_property_map([("x", vec!["x"])].iter().cloned().collect());
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
            code_gen.return_action,
            Return {
                id: "n",
                paths: vec!["x"]
            }
        );
    }

    #[test]
    fn test_codegen_action_without_where() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen =
            CodeGen::new_with_property_map([("x", vec!["x"])].iter().cloned().collect());

        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.vertices, ["n", "m"].iter().cloned().collect());
        assert_eq!(code_gen.edges, vec![("n", "m")]);
        assert_eq!(
            code_gen.return_action,
            Return {
                id: "n",
                paths: vec!["x"]
            }
        );
    }

    #[test]
    fn test_codegen_multiple_properties_to_collect() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new_with_property_map(
            [("x", vec!["x"]), ("y", vec!["y"])]
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
            [vec!["x",], vec!["y",],].iter().cloned().collect()
        );
        assert_eq!(
            code_gen.return_action,
            Return {
                id: "n",
                paths: vec!["y"]
            }
        );
    }
}
