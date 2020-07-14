// A tree fold trait. This trait walks through an immutable tree and updates self.
// Implementations of this trait can override
// methods that process specific types of tree nodes, while using default
// methods for other types of tree nodes.

use grammar::*;

pub trait TreeFold<'a> {
    fn visit_prog(&mut self, tree: &'a Prog) {
        self.visit_patterns(&tree.patterns);
        self.visit_filters(&tree.filters);
    }

    fn visit_patterns(&mut self, tree: &'a Patterns) {
        for pattern in &tree.pattern_vector {
            self.visit_pattern(pattern);
        }
    }

    fn visit_pattern(&mut self, tree: &'a Pattern) {
        self.visit_identifier(&tree.from_node);
        self.visit_identifier(&tree.to_node);
    }

    fn visit_filters(&mut self, tree: &'a Filters) {
        for filter in &tree.filter_vector {
            self.visit_filter(filter);
        }
    }

    fn visit_filter(&mut self, tree: &'a Filter) {
        match &tree {
            Filter::Label(ref id, ref label) => {
                self.visit_identifier(id);
                self.visit_identifier(label);
            }
            Filter::Property(ref id, ref properties, ref value) => {
                self.visit_identifier(id);
                for property in properties {
                    self.visit_identifier(property);
                }
                self.visit_value(value);
            }
        }
    }

    fn visit_action(&mut self, tree: &'a Action) {
        match &tree {
            Action::Property(ref id, ref properties) => {
                self.visit_identifier(id);
                for property in properties {
                    self.visit_identifier(property);
                }
            }
        }
    }

    fn visit_identifier(&mut self, _tree: &'a Identifier) {}

    fn visit_value(&mut self, _tree: &'a Value) {}
}
