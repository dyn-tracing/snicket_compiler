use grammar::*;
use proto::treenode::TreeNode;
use std::collections::HashMap;

use tree_fold::TreeFold;

#[derive(Default)]
pub struct CodeGen<'a> {
    pub return_action: Vec<&'a str>,
    pub nodes: HashMap<&'a str, TreeNode>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> CodeGen<'a> {
        CodeGen::default()
    }
}

fn find_node_helper<'a>(node: &'a mut TreeNode, id: &'a str) -> Option<&'a mut TreeNode> {
    if node.get_id() == id {
        return Some(node);
    }
    for child in node.mut_children().iter_mut() {
        if let Some(node) = find_node_helper(child, id) {
            return Some(node);
        }
    }

    None
}

fn find_node_with_id<'a>(
    nodes: &'a mut HashMap<&str, TreeNode>,
    id: &'a str,
) -> Option<&'a mut TreeNode> {
    let node_opt = None;
    for (_, v) in nodes.iter_mut() {
        let res = find_node_helper(v, id);
        if res.is_some() {
            return res;
        }
    }

    node_opt
}

impl<'a> TreeFold<'a> for CodeGen<'a> {
    fn visit_patterns(&mut self, patterns: &'a Patterns) {
        for pattern in &patterns.0 {
            self.visit_pattern(pattern);
        }
        assert!(
            self.nodes.len() == 1,
            "Only supports tree pattern, got a forrest with {} roots",
            self.nodes.len()
        );
    }

    fn visit_pattern(&mut self, pattern: &'a Pattern) {
        // TODO: support PATH
        let rel_type = &pattern.relationship_type;
        if let Relationship::Path(_) = rel_type {
            panic!("Path relationship type is not yet supported.");
        }

        let to_id = pattern.to_node.id_name;
        let to_node = match self.nodes.remove(to_id) {
            None => TreeNode {
                id: to_id.to_string(),
                ..Default::default()
            },
            Some(n) => n,
        };

        let from_id = pattern.from_node.id_name;
        let from_node = match find_node_with_id(&mut self.nodes, from_id) {
            Some(n) => n,
            None => self.nodes.entry(from_id).or_insert_with(|| TreeNode {
                id: from_id.to_string(),
                ..Default::default()
            }),
        };

        from_node.mut_children().push(to_node);
    }

    fn visit_filter(&mut self, filter: &'a Filter) {
        let Filter::Property(id, properties, v) = filter;

        let mut node_opt = None;
        for (_, v) in self.nodes.iter_mut() {
            if let Some(node) = find_node_helper(v, id.id_name) {
                node_opt = Some(node);
            }
        }

        match node_opt {
            None => panic!("Couldn't find a node with id {}", id.id_name),
            Some(node) => {
                assert!(
                    properties.len() == 1,
                    "only support top level property in filter"
                );
                node.mut_properties()
                    .insert(properties[0].id_name.to_string(), v.to_string());
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
    use protobuf::RepeatedField;
    use std::iter::Peekable;
    use token::Token;

    #[test]
    fn test_match() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH a-->b : x,b-->c : y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.nodes.len(), 1);
        assert_eq!(
            code_gen.nodes.get("a"),
            Some(&TreeNode {
                id: "a".to_string(),
                children: RepeatedField::from_vec(vec![TreeNode {
                    id: "b".to_string(),
                    children: RepeatedField::from(vec![TreeNode {
                        id: "c".to_string(),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_match_handle_ordering() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.nodes.len(), 1);
        assert_eq!(
            code_gen.nodes.get("a"),
            Some(&TreeNode {
                id: "a".to_string(),
                children: RepeatedField::from_vec(vec![TreeNode {
                    id: "b".to_string(),
                    children: RepeatedField::from(vec![TreeNode {
                        id: "c".to_string(),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }]),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_codegen_parallel() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,a-->d:z,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        assert_eq!(code_gen.nodes.len(), 1);
        assert_eq!(
            code_gen.nodes.get("a"),
            Some(&TreeNode {
                id: "a".to_string(),
                children: RepeatedField::from_vec(vec![
                    TreeNode {
                        id: "b".to_string(),
                        children: RepeatedField::from(vec![TreeNode {
                            id: "c".to_string(),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    },
                    TreeNode {
                        id: "d".to_string(),
                        ..Default::default()
                    }
                ]),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_codegen_action() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(
            code_gen.nodes.get("n"),
            Some(&TreeNode {
                id: "n".to_string(),
                properties: {
                    let mut map = HashMap::new();
                    map.insert("x".to_string(), "k".to_string());
                    map
                },
                children: RepeatedField::from_vec(vec![TreeNode {
                    id: "m".to_string(),
                    ..Default::default()
                }]),
                ..Default::default()
            })
        );

        assert_eq!(code_gen.return_action, vec!["n", "x"]);
    }

    #[test]
    fn test_codegen_action_without_where() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(
            code_gen.nodes.get("n"),
            Some(&TreeNode {
                id: "n".to_string(),
                children: RepeatedField::from_vec(vec![TreeNode {
                    id: "m".to_string(),
                    ..Default::default()
                }]),
                ..Default::default()
            })
        );
        assert_eq!(code_gen.return_action, vec!["n", "x"]);
    }
}
