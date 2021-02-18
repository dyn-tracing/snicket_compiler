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
use antlr_rust::tree::Tree;
use std::rc::Rc;
use std::collections::HashMap;

/***********************************/
// IR Structs
/***********************************/
#[derive(Clone, Debug)]
pub struct StructuralFilter {
    vertices: Vec<String>,
    edges: Vec<(String, String)>,
    properties: HashMap<String, HashMap<String, String>>,
}

impl StructuralFilter {
    pub fn new() -> Self {
        StructuralFilter { vertices: Vec::new(), edges: Vec::new(), properties: HashMap::new() }
    }

}
#[derive(Clone, Debug)]
pub struct AttributeFilter<'a> {
    attributes: Vec<(&'a str, &'a str)>
}

#[derive(Clone, Debug)]
pub struct Map<'a> {
    new_attributes: Vec<&'a str>,
}

#[derive(Clone, Debug)]
pub struct IRReturn<'a> {
    obj_to_return_on: &'a str,
    property_to_return: &'a str,
}

#[derive(Clone, Debug)]
pub struct Aggregate<'a> {
    udf_id: &'a str,
    property_to_aggregate: &'a str,
}

#[derive(Clone, Debug)]
pub enum IrFunction {
    AttributeFilter,
    StructuralFilter,
    Map,
    IRReturn, // can't just be return bc that's a keyword
    Aggregate,
}

/***********************************/
// Code Gen:  holds all relevant information for generating code
/***********************************/
pub struct CodeGen {
    ir_blocks: Vec<IrFunction>
}

pub struct MyCypherVisitor<'i> {
    struct_filters: Vec<StructuralFilter>,
    prop_filters: Vec<&'i str>,
    return_expr: Vec<&'i str>,
}
impl <'i> MyCypherVisitor<'i> {
    pub fn new() -> MyCypherVisitor<'i> {
        MyCypherVisitor {
            struct_filters: Vec::new(),
            prop_filters: Vec::new(),
            return_expr: Vec::new(),
        }
    }
}

fn get_filters<'i>(
    ctx: &OC_PatternElementContext<'i>,
    _struct_filters: &Vec<&'i str>,
    _prop_filters: &Vec<&'i str>,
) -> Vec<&'i str> {
    println!("in get filters: {:?}", ctx.oC_PatternElementChain(0).unwrap().get_text());
    return Vec::new();
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for MyCypherVisitor<'i> {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> CypherVisitor<'i> for MyCypherVisitor<'i> {

    fn visit_oC_Match(&mut self, ctx: &OC_MatchContext<'i>) {
        self.visit_children(ctx);
        let pattern = ctx.oC_Pattern().unwrap();
        let mut struct_filter = StructuralFilter::new();
        let where_clause = ctx.oC_Where(); //TODO

        pattern.oC_PatternPart_all();
        for p in pattern.oC_PatternPart_all() {
            let pattern_element = p.oC_AnonymousPatternPart().unwrap().oC_PatternElement().unwrap();
            let mut first_node = pattern_element.oC_NodePattern().unwrap().oC_Variable().unwrap().get_text();
            struct_filter.vertices.push(first_node.clone());
            let mut i = 0;
            while !pattern_element.oC_PatternElementChain(i).is_none() {
                let pattern_element_i = pattern_element.oC_PatternElementChain(i).unwrap();

                let relationship = pattern_element_i.oC_RelationshipPattern().unwrap();
                let node_pattern = pattern_element_i.oC_NodePattern().unwrap();

                let var = node_pattern.oC_Variable().unwrap().get_text();
                struct_filter.vertices.push(var.clone());

                if !relationship.oC_RightArrowHead().is_none() {
                    struct_filter.edges.push((first_node.clone(), var.clone()));
                }
                first_node = var.clone();
                let prop = node_pattern.oC_Properties();
                if !prop.is_none() {
                    let map_literal = prop.clone().unwrap().oC_MapLiteral().unwrap();
                    let mut prop_hashmap = HashMap::new();
                    let mut j = 0;
                    while !map_literal.oC_PropertyKeyName(j).is_none() {
                        let property_key_name = map_literal.oC_PropertyKeyName(j as usize).unwrap();
                        let expression = map_literal.oC_Expression(j as usize).unwrap();
                        prop_hashmap.insert(property_key_name.get_text(), expression.get_text());
                        j += 1;
                    }
                    struct_filter.properties.insert(first_node.clone(), prop_hashmap);
                }
                i += 1;
            }
        }
        self.struct_filters.push(struct_filter);
        assert!(!self.struct_filters.is_empty());
    }

    // RETURN
    fn visit_oC_ProjectionBody(&mut self, ctx: &OC_ProjectionBodyContext<'i>) {
        self.visit_children(ctx);
        println!("RETURN BEGIN");
        //println!("ret: {:?}\n", ctx.oC_NodePattern().unwrap().get_text());
        println!("{:?}", ctx.oC_ProjectionItems().unwrap().get_text());
        println!("RETURN END");
    }

    // Properties
    fn visit_oC_PropertyLookup(&mut self, ctx: &OC_PropertyLookupContext<'i>) {
        self.visit_children(ctx);
        println!("PROPERTY BEGIN");
        println!("{:?}", ctx.get_text());
        println!("{:?}", ctx.get_parent().unwrap().get_text());
        println!("PROPERTY END");
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






#[cfg(test)]
mod tests {
    use super::*;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::token_factory::CommonTokenFactory;
    use antlr_rust::InputStream;
    use crate::lexer::CypherLexer;
    use crate::parser::CypherParser;
    
    fn run_parser<'a>(tf: &'a CommonTokenFactory, input: &'a str) -> Rc<OC_CypherContextAll<'a>> {
        //let tf = CommonTokenFactory::default();
        let query_stream = InputStream::new_owned(input.to_string().into_boxed_str());
        let mut _lexer = CypherLexer::new_with_token_factory(query_stream, tf);
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = CypherParser::new(token_source);
        let result = parser.oC_Cypher().expect("parsed unsuccessfully");
        return result;
    }

    #[test]
    fn test_parser_finds_match() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size");
        let mut visitor = MyCypherVisitor::new();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
    }

    #[test]
    fn test_parser_finds_vertices_edges_properties() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size");
        let mut visitor = MyCypherVisitor::new();
        let _res = result.accept(&mut visitor);
        assert!(visitor.struct_filters[0].vertices == vec!["a", "b", "c"]);
        assert!(visitor.struct_filters[0].edges == vec![("a".to_string(), "b".to_string()), ("b".to_string(), "c".to_string())]);
        assert!(visitor.struct_filters[0].properties.contains_key("b"));
        assert!(visitor.struct_filters[0].properties["b"].contains_key("service_name"));
        assert!(visitor.struct_filters[0].properties["b"]["service_name"] == "reviews-v1");

    }

}
