// A tree fold trait. This trait walks through an immutable tree and updates self.
// Implementations of this trait can override
// methods that process specific types of tree nodes, while using default
// methods for other types of tree nodes.

use proto::grammar::*;

pub trait TreeFold {
    fn visit_prog(&mut self, program: &Program) {
        self.visit_patterns(program.get_patterns());
        self.visit_filters(program.get_filters());
        self.visit_actions(program.get_actions());
    }

    fn visit_patterns(&mut self, patterns: &[Pattern]) {
        for pattern in patterns {
            self.visit_pattern(pattern);
        }
    }

    fn visit_pattern(&mut self, pattern: &Pattern) {
        self.visit_id(&pattern.get_src_id());
        self.visit_id(&pattern.get_dst_id());
        self.visit_rel_type(pattern.get_rel_typ());
    }

    fn visit_filters(&mut self, filters: &[Filter]) {
        for filter in filters {
            self.visit_filter(filter);
        }
    }

    fn visit_filter(&mut self, filter: &Filter) {
        self.visit_id(&filter.get_id());
        for property in filter.get_properties() {
            self.visit_id(&property);
        }
    }

    fn visit_value(&mut self, value_oneof: &Option<Filter_oneof_value_oneof>) {
        match value_oneof {
            Some(Filter_oneof_value_oneof::u32(_v)) => {}
            Some(Filter_oneof_value_oneof::str(_s)) => {}
            None => {}
        }
    }

    fn visit_actions(&mut self, actions: &[Action]) {
        for action in actions {
            self.visit_action(&action);
        }
    }

    fn visit_action(&mut self, action: &Action) {
        self.visit_id(action.get_id());
        for property in action.get_properties() {
            self.visit_id(property);
        }
    }

    fn visit_id(&mut self, _id: &str) {}

    fn visit_rel_type(&mut self, _rel_type: Pattern_RelationshipType) {}
}
