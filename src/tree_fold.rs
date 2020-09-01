// A tree fold trait. This trait walks through an immutable tree and updates self.
// Implementations of this trait can override
// methods that process specific types of tree nodes, while using default
// methods for other types of tree nodes.

use grammar::*;

pub trait TreeFold<'a> {
    fn visit_prog(&mut self, tree: &'a Prog) {
        self.visit_patterns(&tree.patterns);
        self.visit_filters(&tree.filters);
        self.visit_action(&tree.action);
    }

    fn visit_patterns(&mut self, patterns: &'a Patterns) {
        for pattern in &patterns.0 {
            self.visit_pattern(pattern);
        }
    }

    fn visit_pattern(&mut self, tree: &'a Pattern) {
        self.visit_identifier(&tree.from_node);
        self.visit_identifier(&tree.to_node);
        self.visit_relationship(&tree.relationship_type);
    }

    fn visit_relationship(&mut self, _rel_type: &'a Relationship) {}

    fn visit_filters(&mut self, filters: &'a Filters) {
        for filter in &filters.0 {
            self.visit_filter(filter);
        }
    }

    fn visit_filter(&mut self, tree: &'a Filter) {
        match &tree {
            Filter::Property(ref id, ref p, ref value) => {
                self.visit_identifier(id);
                self.visit_identifier(p);
                self.visit_value(value);
            }
        }
    }

    fn visit_action(&mut self, tree: &'a Action) {
        match &tree {
            Action::GetProperty(ref id, ref p) => {
                self.visit_identifier(id);
                self.visit_identifier(p);
            }
            Action::CallUdf(ref id) => {
                self.visit_identifier(id);
            }
            Action::GroupBy(ref id, ref p, ref func) => {
                self.visit_identifier(id);
                self.visit_identifier(p);
                self.visit_identifier(func);
            }
            Action::None => {}
        }
    }

    fn visit_identifier(&mut self, _tree: &'a Identifier) {}

    fn visit_value(&mut self, _tree: &'a Value) {}
}
