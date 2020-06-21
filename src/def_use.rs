use grammar::*;
use std::collections::HashSet;
use std::collections::HashMap;
use tree_fold::TreeFold;

pub struct DefUse<'a> {
  pub known_nodes     : HashSet<&'a str>
}

impl<'a> DefUse<'a> {
  pub fn new() -> DefUse<'a> {
    DefUse {
      known_nodes  : HashSet::new(),
    }
  }
}

impl<'a> TreeFold<'a> for DefUse<'a> {
  fn visit_pattern(&mut self, tree : &'a Pattern) {
    let from_node = &tree.from_node;
    let to_node   = &tree.to_node;
    self.known_nodes.insert(from_node.get_str());
    self.known_nodes.insert(to_node.get_str());
  }

  fn visit_filter(&mut self, tree : &'a Filter) {
    match &tree {
      Filter::Label(node, _) | Filter::Property(node, _, _)
      =>
      { if (!self.known_nodes.contains(node.get_str())) { panic!("Node {:?} not defined.", node.get_str()); } }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::lexer;
  use super::super::parser;
  use super::DefUse;
  use super::super::tree_fold::TreeFold;

  fn run_def_use(input_program : &str) {
    // Lexing
    let tokens = & mut lexer::get_tokens(input_program);
  
    // parsing
    let token_iter = & mut tokens.iter().peekable();
    let parse_tree = parser::parse_prog(token_iter);
    assert!(token_iter.peek().is_none(), "token_iter is not empty.");
    println!("Parse tree: {:?}\n", parse_tree);
  
    // Check that identifiers are defined before use
    let mut def_use = DefUse::new();
    def_use.visit_prog(&parse_tree);
  }

  macro_rules! test_pass {
    ($input_code:expr,$test_name:ident) => (
      #[test]
      fn $test_name() {
        let input_program = $input_code;
        run_def_use(input_program);
      }
    )
  }

  macro_rules! test_fail {
    ($input_code:expr,$test_name:ident,$panic_msg:expr) => (
      #[test]
      #[should_panic(expected=$panic_msg)]
      fn $test_name() {
        let input_program = $input_code;
        run_def_use(input_program);
      }
    )
  }

  test_pass!(r"MATCH n-->m MATCH n-*>m", test_def_use_no_filter);
  test_pass!(r"MATCH n-->m MATCH n-*>m WHERE n : Node", test_def_use_label_filter);
  test_pass!(r"MATCH n-->m MATCH n-*>m WHERE n.a == 5", test_def_use_prop_filter);
  test_fail!(r"MATCH n-->m MATCH n-*>m WHERE x.a == 5", test_fail_def_use_prop_filter,
             "Node \"x\" not defined.");
  test_fail!(r"MATCH n-->m MATCH n-*>m WHERE x : a", test_fail_def_use_label_filter,
             "Node \"x\" not defined.");
}
