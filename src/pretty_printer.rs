use proto::grammar::*;
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

impl TreeFold for PrettyPrinter {
    fn visit_patterns(&mut self, patterns: &[Pattern]) {
        self.pretty_print_str.push_str("MATCH ");
        for pattern in patterns {
            self.visit_pattern(pattern);
        }
    }

    fn visit_pattern(&mut self, pattern: &Pattern) {
        self.pretty_print_str.push_str(pattern.get_src_id());
        match &pattern.get_rel_typ() {
            Pattern_RelationshipType::PATH => {
                self.pretty_print_str.push_str("-*>");
            }
            Pattern_RelationshipType::EDGE => {
                self.pretty_print_str.push_str("-->");
            }
        };
        self.pretty_print_str.push_str(pattern.get_dst_id());
        self.pretty_print_str.push_str(":");
        self.pretty_print_str.push_str(pattern.get_rel_id());
        self.pretty_print_str.push_str(", ");
    }

    fn visit_filters(&mut self, filters: &[Filter]) {
        self.pretty_print_str.push_str("WHERE ");
        for filter in filters {
            self.visit_filter(filter);
        }
    }

    fn visit_filter(&mut self, filter: &Filter) {
        self.pretty_print_str.push_str(filter.get_id());
        for property in filter.get_properties() {
            self.pretty_print_str.push_str(".");
            self.pretty_print_str.push_str(property);
        }
        self.pretty_print_str.push_str(" == ");
        match &filter.value_oneof {
            Some(Filter_oneof_value_oneof::str(s)) => self.pretty_print_str.push_str(s.as_str()),
            Some(Filter_oneof_value_oneof::u32(v)) => {
                self.pretty_print_str.push_str(v.to_string().as_str())
            }
            None => {}
        }

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
        let input_program = r"MATCH n-->m : a, n-*>m : b, WHERE n.abc == 5,";
        run_pretty_printer_and_reparse(input_program);
    }
}
