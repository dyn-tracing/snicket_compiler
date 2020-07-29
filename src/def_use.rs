use proto::grammar::*;
use std::collections::HashSet;
use tree_fold::TreeFold;

#[derive(Default)]
pub struct DefUse {
    pub known_nodes: HashSet<String>,
    pub known_edges: HashSet<String>,
}

impl DefUse {
    pub fn new() -> DefUse {
        DefUse::default()
    }
    fn is_undefined(&self, id: &str) -> bool {
        self.known_edges.get(id).is_none() && self.known_nodes.get(id).is_none()
    }
}

impl TreeFold for DefUse {
    fn visit_pattern(&mut self, pattern: &Pattern) {
        let src_id = pattern.get_src_id().to_string();
        let dst_id = pattern.get_dst_id().to_string();
        self.known_nodes.insert(src_id);
        self.known_nodes.insert(dst_id);
        let rel_id = pattern.get_rel_id();

        assert!(
            self.is_undefined(rel_id),
            "Edge {:?} already defined.",
            rel_id
        );
        self.known_edges.insert(rel_id.to_string());
    }

    fn visit_filter(&mut self, filter: &Filter) {
        if !self.known_nodes.contains(filter.get_id())
            && !self.known_edges.contains(filter.get_id())
        {
            panic!("Edge/Node {:?} not defined.", filter.get_id());
        }
    }
    fn visit_action(&mut self, action: &Action) {
        if !self.known_nodes.contains(action.get_id())
            && !self.known_edges.contains(action.get_id())
        {
            panic!("Edge/Node {:?} not defined.", action.get_id());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefUse;
    use lexer;
    use parser;
    use tree_fold::TreeFold;

    fn run_def_use(input_program: &str) {
        // Lexing
        let tokens = &mut lexer::get_tokens(input_program);

        // parsing
        let token_iter = &mut tokens.iter().peekable();
        let parse_tree = parser::parse_prog(token_iter);
        assert!(token_iter.peek().is_none(), "token_iter is not empty.");
        println!("Parse tree: {:?}\n", parse_tree);

        // Check that identifiers are defined before use
        let mut def_use = DefUse::new();
        def_use.visit_prog(&parse_tree);
    }

    macro_rules! test_pass {
        ($input_code:expr,$test_name:ident) => {
            #[test]
            fn $test_name() {
                let input_program = $input_code;
                run_def_use(input_program);
            }
        };
    }

    macro_rules! test_fail {
        ($input_code:expr,$test_name:ident,$panic_msg:expr) => {
            #[test]
            #[should_panic(expected=$panic_msg)]
            fn $test_name() {
                let input_program = $input_code;
                run_def_use(input_program);
            }
        };
    }

    test_pass!(r"MATCH n-->m : a, n-*>m : b,", test_def_use_no_filter);

    test_pass!(
        r"MATCH n-->m : a, n-*>m : b, WHERE a.x == 5,",
        test_def_use_prop_filter
    );
    test_fail!(
        r"MATCH n-->m : a, n-*>m : b, WHERE x.a == 5,",
        test_fail_def_use_prop_filter,
        "Edge/Node \"x\" not defined."
    );
}
