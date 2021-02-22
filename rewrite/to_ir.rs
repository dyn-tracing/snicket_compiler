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
// use antlr_rust::tree::Tree; // TODO: do we need this import?
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

/***********************************/
// IR Structs
/***********************************/
#[derive(Clone, Debug)]
pub struct StructuralFilter {
    vertices: Vec<String>,
    edges: Vec<(String, String)>,
    properties: HashMap<String, HashMap<String, String>>, // attribute, value
}
impl Default for StructuralFilter {
    fn default() -> Self {
        StructuralFilter {
            vertices: Vec::new(),
            edges: Vec::new(),
            properties: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AttributeFilter {
    node: String,
    property: String,
    value: String,
}
impl Default for AttributeFilter {
    fn default() -> Self {
        AttributeFilter {
            node: String::new(),
            property: String::new(),
            value: String::new(),
        }
    }
}

impl AttributeFilter {
    pub fn insert_values(&mut self, node: String, property: String, value: String) {
        self.node = node;
        self.property = property;
        self.value = value;
    }
}

#[derive(Clone, Debug)]
pub struct IrReturn {
    entity: String,
    property: String,
}

impl IrReturn {
    pub fn new_with_items(entity: String, property: String) -> Self {
        IrReturn { entity, property }
    }
}

#[derive(Clone, Debug)]
pub struct Aggregate {
    udf_id: String,
    entity: String,
    property: String,
}
impl Aggregate {
    pub fn new_with_items(entity: String, property: String, udf: String) -> Self {
        Aggregate {
            udf_id: udf,
            entity,
            property,
        }
    }
}

pub struct VisitorResults {
    struct_filters: Vec<StructuralFilter>,
    prop_filters: Vec<AttributeFilter>,
    return_expr: Option<IrReturn>,
    aggregate: Option<Aggregate>,
    maps: Vec<String>,
}

/***********************************/
// FilterVisitor:  visits tree and fills out structs with information for code gen
/***********************************/

pub struct FilterVisitor {
    struct_filters: Vec<StructuralFilter>,
    prop_filters: Vec<AttributeFilter>,
}

impl Default for FilterVisitor {
    fn default() -> Self {
        FilterVisitor {
            struct_filters: Vec::new(),
            prop_filters: Vec::new(),
        }
    }
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for FilterVisitor {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> CypherVisitor<'i> for FilterVisitor {
    /// This function visits a match clause.  It extracts the graph inside, complete with any
    /// node attributes, and stores that information in a struct_filter.  It then extracts any information
    /// in the where clause, which pertains to the whole graph, and stores that in an attribute_filter.
    fn visit_oC_Match(&mut self, ctx: &OC_MatchContext<'i>) {
        self.visit_children(ctx);
        let pattern = ctx.oC_Pattern().unwrap();
        let mut struct_filter = StructuralFilter::default();

        pattern.oC_PatternPart_all();
        for p in pattern.oC_PatternPart_all() {
            let pattern_element = p
                .oC_AnonymousPatternPart()
                .unwrap()
                .oC_PatternElement()
                .unwrap();
            let mut first_node = pattern_element
                .oC_NodePattern()
                .unwrap()
                .oC_Variable()
                .unwrap()
                .get_text();
            struct_filter.vertices.push(first_node.clone());
            for pattern_element_i in pattern_element.oC_PatternElementChain_all() {
                let relationship = pattern_element_i.oC_RelationshipPattern().unwrap();
                let node_pattern = pattern_element_i.oC_NodePattern().unwrap();

                let var = node_pattern.oC_Variable().unwrap().get_text();
                struct_filter.vertices.push(var.clone());

                if relationship.oC_RightArrowHead().is_some() {
                    struct_filter.edges.push((first_node.clone(), var.clone()));
                }
                first_node = var.clone();
                let prop = node_pattern.oC_Properties();
                if prop.is_some() {
                    let map_literal = prop.clone().unwrap().oC_MapLiteral().unwrap();
                    let mut prop_hashmap = HashMap::new();
                    let mut j = 0;
                    while map_literal.oC_PropertyKeyName(j).is_some()
                        && map_literal.oC_Expression(j).is_some()
                    {
                        let property_key_name = map_literal.oC_PropertyKeyName(j).unwrap();
                        let expression = map_literal.oC_Expression(j).unwrap();
                        prop_hashmap.insert(property_key_name.get_text(), expression.get_text());
                        j += 1;
                    }
                    struct_filter
                        .properties
                        .insert(first_node.clone(), prop_hashmap);
                }
            }
        }
        self.struct_filters.push(struct_filter);

        if let Some(where_clause) = ctx.oC_Where() {
            let mut prop_filter = AttributeFilter::default();
            let exp = where_clause.oC_Expression().unwrap();
            let or = exp.oC_OrExpression().unwrap();
            // we do not have any xors, etc, in the language.  So we ignore them for now
            for xor in or.oC_XorExpression_all() {
                for and in xor.oC_AndExpression_all() {
                    for not in and.oC_NotExpression_all() {
                        let comparison = not.oC_ComparisonExpression().unwrap();
                        let add_sub = comparison.oC_AddOrSubtractExpression().unwrap();
                        for mod_div in add_sub.oC_MultiplyDivideModuloExpression_all() {
                            for power in mod_div.oC_PowerOfExpression_all() {
                                for unary in power.oC_UnaryAddOrSubtractExpression_all() {
                                    let prop_exp = unary
                                        .oC_StringListNullOperatorExpression()
                                        .unwrap()
                                        .oC_PropertyOrLabelsExpression()
                                        .unwrap();
                                    let node = prop_exp.oC_Atom().unwrap().get_text();
                                    let property =
                                        prop_exp.oC_PropertyLookup(0).unwrap().get_text();
                                    for partial in comparison.oC_PartialComparisonExpression_all() {
                                        let value = partial.oC_AddOrSubtractExpression().unwrap();
                                        prop_filter.insert_values(
                                            node.clone(),
                                            property.clone(),
                                            value.get_text(),
                                        );
                                        self.prop_filters.push(prop_filter.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct ReturnItem {
    node: String,
    property: String,
}

pub struct ReturnVisitor {
    return_expr: Option<IrReturn>,
    aggregate: Option<Aggregate>,
    return_items: Vec<ReturnItem>,
}

impl Default for ReturnVisitor {
    fn default() -> Self {
        ReturnVisitor {
            return_expr: None,
            aggregate: None,
            return_items : Vec::new(),
        }
    }
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for ReturnVisitor {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> CypherVisitor<'i> for ReturnVisitor {
    fn visit_oC_UnaryAddOrSubtractExpression(
        &mut self,
        ctx: &OC_UnaryAddOrSubtractExpressionContext<'i>,
    ) {
        let prop_exp = ctx
            .oC_StringListNullOperatorExpression()
            .unwrap()
            .oC_PropertyOrLabelsExpression()
            .unwrap();
        println!("{:?}", ctx.get_text() );
        let node = prop_exp.oC_Atom().unwrap().get_text();
        let property = prop_exp.oC_PropertyLookup(0).unwrap().get_text();
        self.return_items.push(ReturnItem {node, property})
    }

    fn visit_oC_ProjectionItems(&mut self, ctx: &OC_ProjectionItemsContext<'i>) {
        println!("SDAASDASDASDASDASDASDa");
        self.visit_children(ctx);
    }

    /// The only two things we allow that have projection bodies are returns and aggregations
    /// In opencypher, an aggregation takes the form of RETURN node.property, aggregation_function(*)
    /// So this function finds the node/property for both return and aggregation, and finds the
    /// aggregation function if applicable.  All this information is stored in self, which is a ReturnVisitor.
    fn visit_oC_ProjectionBody(&mut self, ctx: &OC_ProjectionBodyContext<'i>) {
        ctx.oC_ProjectionItems().unwrap().accept(self);
        // let return_items = ctx.oC_ProjectionItems().unwrap().oC_ProjectionItem_all();
        // let exp = return_items[0].oC_Expression().unwrap();
        // let or = exp.oC_OrExpression().unwrap();
        // we do not have any xors, etc, in the language.  So we ignore them for now, if needed can come back later
        if self.return_items.len() == 1 {
            let return_item = &self.return_items[0];
            // return a value
            self.return_expr = Some(IrReturn::new_with_items(return_item.node.clone(), return_item.property.clone()));
        } else if self.return_items.len() == 2 {
            let return_item = &self.return_items[0];
            self.return_expr = Some(IrReturn::new_with_items(return_item.node.clone(), return_item.property.clone()));
            self.aggregate = Some(Aggregate::new_with_items(
                return_item.node.clone(),
                return_item.property.clone(),
                "".to_string(),
            ));
        }
    }
}

/// This goes through the struct filters, attribute filters, returns, and aggregates of the visitor
/// It finds all the maps, that is, all the attributes given to traces, nodes, etc that are not built in.
/// It records those maps in the visitor's map field.
/// # Arguments
/// * `visitor` - A visitor that already has its filters, returns, and aggregations filled in
pub fn get_map_functions(mut results: VisitorResults) -> VisitorResults {
    let mut unknown_properties: HashSet<String> = HashSet::new();
    let mut known_properties: HashSet<String> = HashSet::new();
    known_properties.insert(".id".to_string()); // TODO:  are there any other built in properties besides id?
    for struct_filter in &results.struct_filters {
        for node in struct_filter.properties.keys() {
            for property in struct_filter.properties[node].keys() {
                if !known_properties.contains(property.as_str())
                    && !unknown_properties.contains(property.as_str())
                {
                    unknown_properties.insert(property.to_string());
                }
            }
        }
    }
    for attribute_filter in &results.prop_filters {
        if !known_properties.contains(attribute_filter.property.as_str())
            && !unknown_properties.contains(attribute_filter.property.as_str())
        {
            unknown_properties.insert(attribute_filter.property.to_string());
        }
    }

    if results.return_expr.is_some() {
        let prop: &str = results.return_expr.as_ref().unwrap().property.as_str();
        if !known_properties.contains(prop) && !unknown_properties.contains(prop) {
            unknown_properties.insert(prop.to_string());
        }
    }
    if results.aggregate.is_some() {
        let prop: &str = results.aggregate.as_ref().unwrap().property.as_str();
        if !known_properties.contains(prop) && !unknown_properties.contains(prop) {
            unknown_properties.insert(prop.to_string());
        }
    }
    results.maps.extend(unknown_properties);
    results
}

/// This is a function that aggregates all the functionality above;  it makes a visitor,
/// visits everything in the query via accept, and then finds the map functions.
pub fn visit_result(result: Rc<OC_CypherContextAll>) -> VisitorResults {
    let mut filter_visitor = FilterVisitor::default();
    let mut return_visitor = ReturnVisitor::default();
    let _res = result.accept(&mut filter_visitor);
    let _res = result.accept(&mut return_visitor);
    let results = VisitorResults {
        struct_filters: filter_visitor.struct_filters,
        prop_filters: filter_visitor.prop_filters,
        return_expr: return_visitor.return_expr,
        aggregate: return_visitor.aggregate,
        maps: Vec::new(),
    };
    get_map_functions(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::CypherLexer;
    use crate::parser::CypherParser;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::token_factory::CommonTokenFactory;
    use antlr_rust::InputStream;

    fn run_parser<'a>(tf: &'a CommonTokenFactory, input: &'a str) -> Rc<OC_CypherContextAll<'a>> {
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
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size",
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
    }

    #[test]
    fn test_parser_finds_vertices_edges_properties() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size",
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.struct_filters[0].vertices == vec!["a", "b", "c"]);
        assert!(
            visitor.struct_filters[0].edges
                == vec![
                    ("a".to_string(), "b".to_string()),
                    ("b".to_string(), "c".to_string())
                ]
        );
        assert!(visitor.struct_filters[0].properties.contains_key("b"));
        assert!(visitor.struct_filters[0].properties["b"].contains_key("service_name"));
        assert!(visitor.struct_filters[0].properties["b"]["service_name"] == "reviews-v1");
    }

    #[test]
    fn test_match_where() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) WHERE trace.latency = 500 RETURN a.request_size");
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        assert!(!visitor.prop_filters.is_empty());
        assert!(visitor.prop_filters[0].node == "trace".to_string());
        assert!(visitor.prop_filters[0].property == ".latency".to_string());
        assert!(visitor.prop_filters[0].value == "500".to_string());
    }

    #[test]
    fn test_match_multiple_where() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) WHERE trace.latency = 500 AND trace.client = xyz RETURN a.request_size");
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        assert!(visitor.prop_filters.len() == 2);
        assert!(visitor.prop_filters[0].node == "trace".to_string());
        assert!(visitor.prop_filters[0].property == ".latency".to_string());
        assert!(visitor.prop_filters[0].value == "500".to_string());

        assert!(visitor.prop_filters[1].node == "trace".to_string());
        assert!(visitor.prop_filters[1].property == ".client".to_string());
        assert!(visitor.prop_filters[1].value == "xyz".to_string());
    }

    #[test]
    fn test_return() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size",
        );
        let mut visitor = ReturnVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.return_expr.unwrap().entity == "a");
    }

    #[test]
    fn test_aggregate() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size, histogram(*) ");
        let mut visitor = ReturnVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.aggregate.as_ref().unwrap().udf_id == "histogram(*)");
        assert!(visitor.aggregate.as_ref().unwrap().entity == "a");
        assert!(visitor.aggregate.as_ref().unwrap().property == ".request_size");
    }

    #[test]
    fn test_map() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {service_name: reviews-v1})-[]->(c) RETURN a.request_size, histogram(*) ");
        let mut filter_visitor = FilterVisitor::default();
        let mut return_visitor = ReturnVisitor::default();
        let _res = result.accept(&mut filter_visitor);
        let _res = result.accept(&mut return_visitor);
        let mut results = VisitorResults {
            struct_filters: filter_visitor.struct_filters,
            prop_filters: filter_visitor.prop_filters,
            return_expr: return_visitor.return_expr,
            aggregate: return_visitor.aggregate,
            maps: Vec::new(),
        };
        results = get_map_functions(results);
        assert!(results.maps.len() == 2);
        assert!(results.maps.contains(&"service_name".to_string()));
        assert!(results.maps.contains(&".request_size".to_string()));
    }
}
