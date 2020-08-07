use grammar::*;

use tree_fold::TreeFold;

#[derive(Default)]
pub struct CodeGen<'a> {
    pub return_action: Vec<&'a str>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> CodeGen<'a> {
        CodeGen::default()
    }
}

impl<'a> TreeFold<'a> for CodeGen<'a> {
    fn visit_patterns(&mut self, patterns: &'a Patterns) {
        for pattern in &patterns.0 {
            self.visit_pattern(pattern);
        }
    }

    // visit_pattern and visit_filter here are used to generate TreeNode representing user query
    // intent.
    fn visit_pattern(&mut self, _pattern: &'a Pattern) {}

    // visit_pattern and visit_filter here are used to generate TreeNode representing user query
    // intent.
    fn visit_filter(&mut self, _filter: &'a Filter) {}

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
        let _parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let _code_gen = CodeGen::new();
        // code_gen.visit_prog(&parse_tree);
        // assert_eq!(code_gen.nodes.len(), 1);
        // assert_eq!(
        //     code_gen.nodes.get("a"),
        //     Some(&TreeNode {
        //         id: "a".to_string(),
        //         children: RepeatedField::from_vec(vec![TreeNode {
        //             id: "b".to_string(),
        //             children: RepeatedField::from(vec![TreeNode {
        //                 id: "c".to_string(),
        //                 ..Default::default()
        //             }]),
        //             ..Default::default()
        //         }]),
        //         ..Default::default()
        //     })
        // );
    }

    #[test]
    fn test_match_handle_ordering() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        // assert_eq!(code_gen.nodes.len(), 1);
        // assert_eq!(
        //     code_gen.nodes.get("a"),
        //     Some(&TreeNode {
        //         id: "a".to_string(),
        //         children: RepeatedField::from_vec(vec![TreeNode {
        //             id: "b".to_string(),
        //             children: RepeatedField::from(vec![TreeNode {
        //                 id: "c".to_string(),
        //                 ..Default::default()
        //             }]),
        //             ..Default::default()
        //         }]),
        //         ..Default::default()
        //     })
        // );
    }

    #[test]
    fn test_codegen_parallel() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c : x,a-->b : y,a-->d:z,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);
        // assert_eq!(code_gen.nodes.len(), 1);
        // assert_eq!(
        //     code_gen.nodes.get("a"),
        //     Some(&TreeNode {
        //         id: "a".to_string(),
        //         children: RepeatedField::from_vec(vec![
        //             TreeNode {
        //                 id: "b".to_string(),
        //                 children: RepeatedField::from(vec![TreeNode {
        //                     id: "c".to_string(),
        //                     ..Default::default()
        //                 }]),
        //                 ..Default::default()
        //             },
        //             TreeNode {
        //                 id: "d".to_string(),
        //                 ..Default::default()
        //             }
        //         ]),
        //         ..Default::default()
        //     })
        // );
    }

    #[test]
    fn test_codegen_action() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, WHERE n.x ==k, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        // assert_eq!(
        //     code_gen.nodes.get("n"),
        //     Some(&TreeNode {
        //         id: "n".to_string(),
        //         properties: {
        //             let mut map = HashMap::new();
        //             map.insert("x".to_string(), "k".to_string());
        //             map
        //         },
        //         children: RepeatedField::from_vec(vec![TreeNode {
        //             id: "m".to_string(),
        //             ..Default::default()
        //         }]),
        //         ..Default::default()
        //     })
        // );

        assert_eq!(code_gen.return_action, vec!["n", "x"]);
    }

    #[test]
    fn test_codegen_action_without_where() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH n-->m: a, RETURN n.x,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree = parser::parse_prog(&mut token_iter);
        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        // assert_eq!(
        //     code_gen.nodes.get("n"),
        //     Some(&TreeNode {
        //         id: "n".to_string(),
        //         children: RepeatedField::from_vec(vec![TreeNode {
        //             id: "m".to_string(),
        //             ..Default::default()
        //         }]),
        //         ..Default::default()
        //     })
        // );
        assert_eq!(code_gen.return_action, vec!["n", "x"]);
    }
}
