use grammar::*;
use std::collections::HashSet;
use tree_fold::TreeFold;

#[derive(Default)]
pub struct DefUse<'a> {
    pub known_nodes: HashSet<&'a str>,
    pub known_edges: HashSet<&'a str>,
}

impl<'a> DefUse<'a> {
    pub fn new() -> DefUse<'a> {
        DefUse::default()
    }
    fn is_undefined(&self, id: &'a str) -> bool {
        self.known_edges.get(id).is_none() && self.known_nodes.get(id).is_none()
    }
}

impl<'a> TreeFold<'a> for DefUse<'a> {
    fn visit_pattern(&mut self, tree: &'a Pattern) {
        let from_node = &tree.from_node;
        let to_node = &tree.to_node;
        self.known_nodes.insert(from_node.id_name);
        self.known_nodes.insert(to_node.id_name);
        let edge_name = match &tree.relationship_type {
            Relationship::Path(id) | Relationship::Edge(id) => id,
        }
        .id_name;
        assert!(
            self.is_undefined(edge_name),
            "Edge {:?} already defined.",
            edge_name
        );
        self.known_edges.insert(edge_name);
    }

    fn visit_filter(&mut self, tree: &'a Filter) {
        match &tree {
            Filter::Property(id, _, _) => {
                if !self.known_nodes.contains(id.id_name) && !self.known_edges.contains(id.id_name)
                {
                    panic!("Edge/Node {:?} not defined.", id.id_name);
                }
            }
        }
    }
    fn visit_action(&mut self, tree: &'a Action) {
        match &tree {
            Action::GetProperty(id, _) => {
                if !self.known_nodes.contains(id.id_name) && !self.known_edges.contains(id.id_name)
                {
                    panic!("Edge/Node {:?} not defined.", id.id_name);
                }
            }
            Action::CallUdf(_id) => {}
            Action::None => {}
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
