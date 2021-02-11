extern crate clap;
extern crate handlebars;
extern crate input_stream;
extern crate serde;

use crate::parser::*;
use crate::visitor::CypherVisitor;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::ParseTreeVisitor;
use antlr_rust::tree::TerminalNode;
use antlr_rust::tree::Visitable;
use std::rc::Rc;

pub struct MyCypherVisitor<'i> {
    struct_filters: Vec<&'i str>,
    prop_filters: Vec<&'i str>,
    return_expr: Vec<&'i str>,
}

fn get_filters<'i>(
    _ctx: &OC_PatternElementContext<'i>,
    _struct_filters: &Vec<&'i str>,
    _prop_filters: &Vec<&'i str>,
) -> Vec<&'i str> {
    return Vec::new();
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for MyCypherVisitor<'i> {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> CypherVisitor<'i> for MyCypherVisitor<'i> {

    // an element of MATCH
    fn visit_oC_PatternElement(&mut self, ctx: &OC_PatternElementContext<'i>) {
        self.visit_children(ctx);
        println!("{:?}", ctx.oC_NodePattern().unwrap().get_text());
        println!("{:?}", ctx.oC_PatternElementChain(0).unwrap().get_text());
        get_filters(ctx, &self.struct_filters, &self.prop_filters);
    }

    // RETURN
    fn visit_oC_ProjectionBody(&mut self, ctx: &OC_ProjectionBodyContext<'i>) {
        self.visit_children(ctx);
        println!("{:?}", ctx.oC_ProjectionItems().unwrap().get_text());
    }
}

pub fn visit_result(result: Rc<OC_CypherContextAll>) {
    let mut visitor = MyCypherVisitor {
        struct_filters: Vec::new(),
        prop_filters: Vec::new(),
        return_expr: Vec::new(),
    };
    let _res = result.accept(&mut visitor);
    println!("{:?}", visitor.struct_filters);
    println!("{:?}", visitor.prop_filters);
    println!("{:?}", visitor.return_expr);
}
