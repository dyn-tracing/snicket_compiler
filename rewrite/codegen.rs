extern crate clap;
extern crate handlebars;
extern crate input_stream;
extern crate serde;
use crate::parser::CypherParserContextType;
use crate::parser::OC_CypherContextAll;
use crate::parser::OC_MatchContext;
use crate::visitor::CypherVisitor;
use antlr_rust::tree::ParseTreeVisitor;
use antlr_rust::tree::TerminalNode;
use antlr_rust::tree::Visitable;
use std::rc::Rc;

pub struct MyCypherVisitor<'i> {
    data: Vec<&'i str>,
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for MyCypherVisitor<'i> {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {
        println!("{:?}", _node);
    }
}

impl<'i> CypherVisitor<'i> for MyCypherVisitor<'i> {
    fn visit_oC_Match(&mut self, ctx: &OC_MatchContext<'i>) {
        self.visit_children(ctx);
    }
}

pub fn visit_result(result: Rc<OC_CypherContextAll>) {
    let mut visitor = MyCypherVisitor {
        data: Vec::new(),
    };
    let _res = result.accept(&mut visitor);
    println!("{:?}", visitor.data);
}
