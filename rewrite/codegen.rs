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
pub struct AttributeFilter {
    attributes: Vec<(String, String)>
}

impl AttributeFilter {
    pub fn new() -> Self {
       AttributeFilter { attributes : Vec::new() }
    }
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
pub struct Aggregate {
    udf_id: String,
    property_to_aggregate: String,
}
impl Aggregate {
    pub fn new() -> Self {
        Aggregate { udf_id: String::new(), property_to_aggregate: String::new()}
    }
    pub fn new_with_items(property: String, udf: String) -> Self {
        Aggregate { udf_id: udf, property_to_aggregate: property }
    }
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

pub struct MyCypherVisitor <'i>{
    struct_filters: Vec<StructuralFilter>,
    prop_filters: Vec<AttributeFilter>,
    return_expr: String,
    aggregate: Aggregate,
    other_data: Vec<&'i str>,
}
impl <'i> MyCypherVisitor<'i> {
    pub fn new() -> MyCypherVisitor<'i> {
        MyCypherVisitor {
            struct_filters: Vec::new(),
            prop_filters: Vec::new(),
            return_expr: String::new(),
            aggregate: Aggregate::new(),
            other_data: Vec::new(),
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
            for pattern_element_i in pattern_element.oC_PatternElementChain_all() {
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
                    while !map_literal.oC_PropertyKeyName(j).is_none() && !map_literal.oC_Expression(j).is_none() {
                        let property_key_name = map_literal.oC_PropertyKeyName(j).unwrap();
                        let expression = map_literal.oC_Expression(j).unwrap();
                        prop_hashmap.insert(property_key_name.get_text(), expression.get_text());
                        j += 1;
                    }
                    struct_filter.properties.insert(first_node.clone(), prop_hashmap);
                }
            }
        }
        self.struct_filters.push(struct_filter);

        if !where_clause.is_none() {
            let mut prop_filter = AttributeFilter::new();
            let exp = where_clause.unwrap().oC_Expression().unwrap();
            let or = exp.oC_OrExpression().unwrap();
            // we do not have any xors, etc, in the language.  So we ignore them for now
            for xor in or.oC_XorExpression_all() {
                for and in xor.oC_AndExpression_all() {
                    for not in and.oC_NotExpression_all() {
                        let comparison = not.oC_ComparisonExpression().unwrap();
                        let key = comparison.oC_AddOrSubtractExpression().unwrap();
                        for partial in comparison.oC_PartialComparisonExpression_all() {
                            let value = partial.oC_AddOrSubtractExpression().unwrap();
                            prop_filter.attributes.push((key.get_text(), value.get_text()));
                        }

                    }
                }
            }
            self.prop_filters.push(prop_filter);
        }
    }

    // RETURN and group by/aggregate, which in opencypher is just RETURN value, func
    fn visit_oC_ProjectionBody(&mut self, ctx: &OC_ProjectionBodyContext<'i>) {
        self.visit_children(ctx);
        println!("RETURN BEGIN");
        let return_items = ctx.oC_ProjectionItems().unwrap().oC_ProjectionItem_all();
        if return_items.len() == 1 {
            // return a value
            self.return_expr = return_items[0].get_text();
        }
        else if return_items.len() == 2 {
            // make a group by
            self.aggregate = Aggregate::new_with_items(return_items[0].get_text(), return_items[1].get_text());

        }
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
    let mut visitor = MyCypherVisitor::new();
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

    #[test]
    fn test_match_where() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) WHERE trace.latency = 500 RETURN a.request_size");
        let mut visitor = MyCypherVisitor::new();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        assert!(!visitor.prop_filters.is_empty());
        assert!(visitor.prop_filters[0].attributes[0] == ("trace.latency".to_string(), "500".to_string()));
    }

    #[test]
    fn test_match_multiple_where() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) WHERE trace.latency = 500 AND trace.client = xyz RETURN a.request_size");
        let mut visitor = MyCypherVisitor::new();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        assert!(visitor.prop_filters[0].attributes[0] == ("trace.latency".to_string(), "500".to_string()));
        assert!(visitor.prop_filters[0].attributes[1] == ("trace.client".to_string(), "xyz".to_string()));
    }

    #[test]
    fn test_return() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size");
        let mut visitor = MyCypherVisitor::new();
        let _res = result.accept(&mut visitor);
        assert!(visitor.return_expr == "a.request_size");
    }

    #[test]
    fn test_aggregate() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size, histogram(*)");
        let mut visitor = MyCypherVisitor::new();
        let _res = result.accept(&mut visitor);
        assert!(visitor.aggregate.udf_id == "histogram(*)");
        assert!(visitor.aggregate.property_to_aggregate == "a.request_size");
    }

}
