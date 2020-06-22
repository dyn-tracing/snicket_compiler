use grammar::*;
use tree_fold::TreeFold;

#[derive(Default)]
pub struct PrettyPrinter {
    pretty_print_str: String,
}

impl PrettyPrinter {
    pub fn new() -> PrettyPrinter {
        PrettyPrinter::default()
    }
}

impl<'a> TreeFold<'a> for PrettyPrinter {
    fn visit_patterns(&mut self, tree: &'a Patterns) {
        self.pretty_print_str.push_str("MATCH ");
        for pattern in &tree.pattern_vector {
            self.visit_pattern(pattern);
        }
    }

    fn visit_pattern(&mut self, tree: &'a Pattern) {
        self.pretty_print_str.push_str(tree.from_node.id_name);
        match &tree.relationship_type {
            Relationship::Path() => {
                self.pretty_print_str.push_str("-*>");
            }
            Relationship::Edge() => {
                self.pretty_print_str.push_str("-->");
            }
        };
        self.pretty_print_str.push_str(tree.to_node.id_name);
        self.pretty_print_str.push_str(", ");
    }

    fn visit_filters(&mut self, tree: &'a Filters) {
        self.pretty_print_str.push_str("WHERE ");
        for filter in &tree.filter_vector {
            self.visit_filter(filter);
        }
    }

    fn visit_filter(&mut self, tree: &'a Filter) {
        match &tree {
            Filter::Label(node, label) => {
                self.pretty_print_str.push_str(node.id_name);
                self.pretty_print_str.push_str(" : ");
                self.pretty_print_str.push_str(label.id_name);
            }
            Filter::Property(node, property, val) => {
                self.pretty_print_str.push_str(node.id_name);
                self.pretty_print_str.push_str(".");
                self.pretty_print_str.push_str(property.id_name);
                self.pretty_print_str.push_str(" == ");
                self.pretty_print_str.push_str(&val.to_string());
            }
        };
        self.pretty_print_str.push_str(", ");
    }
}

#[cfg(test)]
mod tests {
    use super::super::lexer;
    use super::super::parser;
    use super::super::tree_fold::TreeFold;
    use super::PrettyPrinter;

    fn run_pretty_printer_and_reparse(input_program: &str) {
        // Lexing
        let tokens = &mut lexer::get_tokens(input_program);

        // parsing
        let token_iter = &mut tokens.iter().peekable();
        let parse_tree = parser::parse_prog(token_iter);
        assert!(token_iter.peek().is_none(), "token_iter is not empty.");
        println!("Parse tree: {:?}\n", parse_tree);

        // Run pretty printer
        let mut pretty_printer = PrettyPrinter::new();
        pretty_printer.visit_prog(&parse_tree);
        println!("Pretty printed code: {}", pretty_printer.pretty_print_str);

        // Reparse pretty printed code
        let new_tokens = &mut lexer::get_tokens(&pretty_printer.pretty_print_str);
        let new_token_iter = &mut new_tokens.iter().peekable();
        let new_parse_tree = parser::parse_prog(new_token_iter);
        assert!(
            new_token_iter.peek().is_none(),
            "new_token_iter is not empty."
        );
        assert!(
            new_parse_tree == parse_tree,
            "Old and new parse trees don't match."
        );
    }

    #[test]
    fn test_pretty_printer() {
        let input_program = r"MATCH n-->m, n-*>m, WHERE n:Node, m:Node, n.abc == 5,";
        run_pretty_printer_and_reparse(input_program);
    }
}
