use grammar::*;
use tree_fold::TreeFold;
#[derive(Default)]
pub struct CodeGen<'a> {
    pub paths: Vec<Vec<&'a str>>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> CodeGen<'a> {
        CodeGen::default()
    }
}

impl<'a> TreeFold<'a> for CodeGen<'a> {
    fn visit_pattern(&mut self, pattern: &'a Pattern) {
        let from_node = pattern.from_node.id_name;
        let to_node = pattern.to_node.id_name;
        if self.paths.is_empty() {
            self.paths.push(vec![from_node, to_node]);
        } else {
            for path in &mut self.paths {
                if path[0] == to_node {
                    path.insert(0, from_node);
                } else if path[path.len() - 1] == from_node {
                    path.push(to_node);
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
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH a-->b,b-->c,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.paths.len(), 1);
        assert_eq!(code_gen.paths[0], vec!["a", "b", "c"]);
    }

    #[test]
    fn test_match_handle_ordering() {
        let tokens: Vec<Token> = lexer::get_tokens(r"MATCH b-->c,a-->b,");
        let mut token_iter: Peekable<std::slice::Iter<Token>> = tokens.iter().peekable();
        let parse_tree: Prog = parser::parse_prog(&mut token_iter);

        let mut code_gen = CodeGen::new();
        code_gen.visit_prog(&parse_tree);

        assert_eq!(code_gen.paths.len(), 1);
        assert_eq!(code_gen.paths[0], vec!["a", "b", "c"]);
    }
}
