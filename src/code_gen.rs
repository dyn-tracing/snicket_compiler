use grammar::*;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use tree_fold::TreeFold;

#[derive(Debug, PartialEq, Eq)]
pub struct Node<'a> {
    pub id: &'a str,
    pub property_filters: HashMap<&'a str, String>,
    pub children: Vec<&'a str>,
}

impl<'a> Node<'a> {
    fn new(id: &'a str) -> Self {
        Self {
            id,
            property_filters: HashMap::new(),
            children: Vec::new(),
        }
    }
}

impl<'a> Hash for Node<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Default)]
pub struct CodeGen<'a> {
    pub return_action: Vec<&'a str>,
    pub nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> CodeGen<'a> {
        CodeGen::default()
    }
}

impl<'a> TreeFold<'a> for CodeGen<'a> {
    fn visit_pattern(&mut self, pattern: &'a Pattern) {
        let from_id = pattern.from_node.id_name;
        let to_id = pattern.to_node.id_name;

        let from_node = self
            .nodes
            .entry(from_id)
            .or_insert_with(|| Node::new(from_id));
        from_node.children.push(to_id);

        let _to_node = self.nodes.entry(to_id).or_insert_with(|| Node::new(to_id));

        // TODO: Handle relationship type.
        let _rel_type = &pattern.relationship_type;
    }

    fn visit_filter(&mut self, filter: &'a Filter) {
        let Filter::Property(id, properties, v) = filter;
        match self.nodes.get_mut(id.id_name) {
            None => {
                panic!("Can't find node with name {}", id.id_name);
            }
            Some(node) => {
                assert!(
                    properties.len() == 1,
                    "only support top level property in filter."
                );
                node.property_filters
                    .insert(properties[0].id_name, v.to_string());
            }
        }
    }

    fn visit_action(&mut self, action: &'a Action) {
        let Action::Property(id, p) = action;
        self.return_action.push(id.id_name);
        for i in p {
            self.return_action.push(i.id_name);
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
        assert_eq!(code_gen.nodes.len(), 3);
        assert_eq!(
            code_gen.nodes.get("a"),
            Some(&Node {
                id: "a",
                property_filters: HashMap::new(),
                children: vec!["b"],
            })
        );
        assert_eq!(
            code_gen.nodes.get("b"),
            Some(&Node {
                id: "b",
                property_filters: HashMap::new(),
                children: vec!["c"],
            })
        );
        assert_eq!(code_gen.nodes.get("c"), Some(&Node::new("c")))
    }

    #[test]
    fn test_match_handle_ordering() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
    }

    #[test]
    fn test_codegen_parallel() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,a-->d:z,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
    }

    #[test]
    fn test_codegen_action() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.return_action, vec!["n", "x"]);
    }

    #[test]
    fn test_codegen_action_without_where() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.return_action, vec!["n", "x"]);
    }
}
