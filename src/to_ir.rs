use super::antlr_gen::parser::*;
use super::antlr_gen::visitor::CypherVisitor;
use super::ir::*;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::ParseTreeVisitor;
use antlr_rust::tree::TerminalNode;
use antlr_rust::tree::Tree;
use antlr_rust::tree::Visitable;
use indexmap::map::IndexMap;
use indexmap::set::IndexSet;
use std::process;
use std::rc::Rc;

/***********************************/
// FilterVisitor:  visits tree and fills out structural and property filters
/***********************************/

pub struct PropertyAndUdfVisitor {
    properties: Vec<Property>,
    udf_calls: Vec<UdfCall>,
}

impl Default for PropertyAndUdfVisitor {
    fn default() -> PropertyAndUdfVisitor {
        PropertyAndUdfVisitor {
            properties: Vec::new(),
            udf_calls: Vec::new(),
        }
    }
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for PropertyAndUdfVisitor {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> CypherVisitor<'i> for PropertyAndUdfVisitor {
    fn visit_oC_FunctionInvocation(&mut self, func: &OC_FunctionInvocationContext<'i>) {
        let udf_name: String;
        if let Some(udf_name_) = func.oC_FunctionName() {
            udf_name = udf_name_.get_text();
        } else {
            panic!("Compiler Bug: Missing UDF name.")
        }
        let mut udf_args = vec![];
        // TODO: We should not allocate a property visitor for each call
        for arg in func.oC_Expression_all() {
            arg.accept(self);
            // TODO: These can be complicated expressions
            // For now we expect a simple entity reference
            udf_args.push(arg.get_text());
        }
        log::debug!(
            "Storing UDF with name: {:?} and args {:?}",
            udf_name,
            udf_args
        );
        self.udf_calls.push(UdfCall {
            id: udf_name,
            args: udf_args,
        })
    }

    fn visit_oC_PropertyOrLabelsExpression(
        &mut self,
        prop: &OC_PropertyOrLabelsExpressionContext<'i>,
    ) {
        log::debug!("Generating Property");
        let atom = prop.oC_Atom().unwrap();
        let entity: String;
        if let Some(var) = atom.oC_Variable() {
            entity = var.get_text();
            log::debug!("Storing var: {:?}", entity);
        } else if let Some(func) = atom.oC_FunctionInvocation() {
            func.accept(self);
            // TODO:  Technically, UDFs can return an object
            return;
        } else if let Some(var) = atom.oC_Literal() {
            entity = var.get_text();
            log::debug!("Storing literal: {:?}", entity);
        } else {
            log::error!(
                "Unsupported expression {:?}. Has type {:?}",
                atom.get_text(),
                ruleNames[atom.get_child(0).unwrap().get_rule_index()]
            );
            process::exit(1);
        }

        let mut property_vec = vec![];
        for property in prop.oC_PropertyLookup_all() {
            property_vec.push(property.get_text());
        }
        self.properties.push(Property {
            parent: entity,
            members: property_vec,
        });
    }
}

/***********************************/
// FilterVisitor:  visits tree and fills out structural and property filters
/***********************************/

pub struct FilterVisitor {
    struct_filters: Vec<StructuralFilter>,
    attr_filters: Vec<AttributeFilter>,
    return_items: Vec<IrReturn>,
}

impl Default for FilterVisitor {
    fn default() -> Self {
        FilterVisitor {
            struct_filters: Vec::new(),
            attr_filters: Vec::new(),
            return_items: Vec::new(),
        }
    }
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for FilterVisitor {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> CypherVisitor<'i> for FilterVisitor {
    // we do not want to visit returns in this case, ignore that part of the tree
    // TODO: Apply the visitor directly to the MATCH body instead
    fn visit_oC_Return(&mut self, _ctx: &OC_ReturnContext<'i>) {}

    fn visit_oC_PropertyOrLabelsExpression(
        &mut self,
        ctx: &OC_PropertyOrLabelsExpressionContext<'i>,
    ) {
        log::debug!("Generating IrReturn");
        let atom = ctx.oC_Atom().unwrap();
        let entity: String;
        if let Some(func) = atom.oC_FunctionInvocation() {
            entity = func.get_text();
            // TODO: We can make this UDF more precise
            log::debug!("Storing UDF: {:?}", entity);
        } else if let Some(var) = atom.oC_Variable() {
            entity = var.get_text();
            log::debug!("Storing var: {:?}", entity);
        } else if let Some(var) = atom.oC_Literal() {
            entity = var.get_text();
            log::debug!("Storing literal: {:?}", entity);
        } else {
            log::error!(
                "Unsupported expression {:?}. Has type {:?}",
                atom.get_text(),
                ruleNames[atom.get_child(0).unwrap().get_rule_index()]
            );
            process::exit(1);
        }

        let mut property_str = String::new();
        for property in ctx.oC_PropertyLookup_all() {
            // this includes the dots
            property_str.push_str(&property.get_text())
        }
        log::debug!("Property String {:?}", property_str);
        self.return_items.push(IrReturn {
            entity,
            property: property_str,
        })
    }

    fn visit_oC_ComparisonExpression(&mut self, ctx: &OC_ComparisonExpressionContext<'i>) {
        // get the left node
        ctx.oC_AddOrSubtractExpression().unwrap().accept(self);
        let node = self.return_items[0].entity.clone();
        let property = self.return_items[0].property.clone();
        self.return_items.clear();

        // process the right node
        let value;
        if let Some(right_clause) = ctx.oC_PartialComparisonExpression(0) {
            right_clause.accept(self);
            value = self.return_items[0].entity.clone();
        } else {
            log::error!("Expected a right-hand side expression.");
            process::exit(1);
        }
        self.return_items.clear();
        let attr_filter = AttributeFilter {
            node,
            property,
            value,
        };
        self.attr_filters.push(attr_filter);
    }

    fn visit_oC_PatternElement(&mut self, ctx: &OC_PatternElementContext<'i>) {
        let mut struct_filter = StructuralFilter::default();

        let mut left_node = ctx.oC_NodePattern().unwrap().oC_Variable().unwrap();
        struct_filter.vertices.push(left_node.get_text());
        for pattern_element_i in ctx.oC_PatternElementChain_all() {
            let relationship = pattern_element_i.oC_RelationshipPattern().unwrap();
            let node_pattern = pattern_element_i.oC_NodePattern().unwrap();

            let right_node = node_pattern.oC_Variable().unwrap();
            struct_filter.vertices.push(right_node.get_text());

            // only add right-side edges for now
            if relationship.oC_RightArrowHead().is_some() {
                struct_filter
                    .edges
                    .push((left_node.get_text(), right_node.get_text()));
            } else {
                log::error!("Unsupported direction");
                process::exit(1);
            }
            if let Some(prop) = node_pattern.oC_Properties() {
                let map_literal = prop.oC_MapLiteral().unwrap();
                let mut prop_hashmap = IndexMap::new();
                for (j, prop_key_name) in
                    map_literal.oC_PropertyKeyName_all().into_iter().enumerate()
                {
                    let expression = map_literal.oC_Expression(j).unwrap();
                    prop_hashmap.insert(prop_key_name.get_text(), expression.get_text());
                }
                struct_filter
                    .properties
                    .insert(right_node.get_text(), prop_hashmap);
            }
            // update the left node
            left_node = right_node;
        }
        self.struct_filters.push(struct_filter);
    }

    /// This function visits a match clause.  It extracts the graph inside, complete with any
    /// node attributes, and stores that information in a struct_filter.  It then extracts any information
    /// in the where clause, which pertains to the whole graph, and stores that in an attribute_filter.
    fn visit_oC_Match(&mut self, ctx: &OC_MatchContext<'i>) {
        for p in ctx.oC_Pattern().unwrap().oC_PatternPart_all() {
            p.accept(self);
        }
        if let Some(where_clause) = ctx.oC_Where() {
            where_clause.accept(self);
        }
    }
}

/***********************************/
// ReturnVisitor: Visits tree and fills out the return functions
/***********************************/

pub struct ReturnVisitor {
    return_expr: Option<IrReturn>,
    aggregate: Option<Aggregate>,
    return_items: Vec<IrReturn>,
}

impl Default for ReturnVisitor {
    fn default() -> Self {
        ReturnVisitor {
            return_expr: None,
            aggregate: None,
            return_items: Vec::new(),
        }
    }
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for ReturnVisitor {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> ReturnVisitor {}

impl<'i> CypherVisitor<'i> for ReturnVisitor {
    // we do not want to visit matches in this case, ignore that part of the tree
    // TODO: Apply the visitor directly to the RETURN body instead
    fn visit_oC_Match(&mut self, _ctx: &OC_MatchContext<'i>) {}

    fn visit_oC_PropertyOrLabelsExpression(
        &mut self,
        ctx: &OC_PropertyOrLabelsExpressionContext<'i>,
    ) {
        log::debug!("Generating IrReturn");
        let atom = ctx.oC_Atom().unwrap();
        let entity: String;
        if let Some(func) = atom.oC_FunctionInvocation() {
            entity = func.get_text();
            log::debug!("Storing UDF: {:?}", entity);
        } else if let Some(var) = atom.oC_Variable() {
            entity = var.get_text();
            log::debug!("Storing var: {:?}", entity);
        } else if let Some(var) = atom.oC_Literal() {
            entity = var.get_text();
            log::debug!("Storing literal: {:?}", entity);
        } else {
            log::error!(
                "Unsupported expression {:?}. Has type {:?}",
                atom.get_text(),
                ruleNames[atom.get_child(0).unwrap().get_rule_index()]
            );
            process::exit(1);
        }
        let mut property_str = String::new();
        for property in ctx.oC_PropertyLookup_all() {
            // this includes the dots
            property_str.push_str(&property.get_text());
        }
        log::debug!("Property String {:?}", property_str);
        self.return_items.push(IrReturn {
            entity,
            property: property_str,
        });

        // This is the new code, the section above can be cut
    }

    fn visit_oC_ProjectionItems(&mut self, ctx: &OC_ProjectionItemsContext<'i>) {
        self.visit_children(ctx);
        // For now we assume that a single element implies a return expression
        // Two elements imply an aggregation
        // FIXME: Make this more explicit
        if self.return_items.len() == 1 {
            let return_item = &self.return_items[0];
            // return a value
            self.return_expr = Some(IrReturn::new_with_items(
                return_item.entity.clone(),
                return_item.property.clone(),
            ));
        } else if self.return_items.len() == 2 {
            let return_item = &self.return_items[0];
            let udf = &self.return_items[1];
            self.aggregate = Some(Aggregate::new_with_items(
                return_item.entity.clone(),
                return_item.property.clone(),
                udf.entity.clone(),
            ));
        } else {
            log::error!("More than two return items not supported");
            process::exit(1);
        }
    }

    /// The only two things we allow that have projection bodies are returns and aggregations
    /// In opencypher, an aggregation takes the form of RETURN node.property, aggregation_function(*)
    /// So this function finds the node/property for both return and aggregation, and finds the
    /// aggregation function if applicable.  All this information is stored in self, which is a ReturnVisitor.
    fn visit_oC_ProjectionBody(&mut self, ctx: &OC_ProjectionBodyContext<'i>) {
        ctx.oC_ProjectionItems().unwrap().accept(self);
    }
}

/// This goes through the struct filters, attribute filters, returns, and aggregates of the visitor
/// It finds all the maps, that is, all the attributes given to traces, nodes, etc that are not built in.
/// It records those maps in the visitor's map field.
/// # Arguments
/// * `visitor` - A visitor that already has its filters, returns, and aggregations filled in
pub fn get_map_functions(mut results: VisitorResults) -> VisitorResults {
    let mut unknown_properties: IndexSet<String> = IndexSet::new();
    for struct_filter in &results.struct_filters {
        for property_map in struct_filter.properties.values() {
            for property in property_map.keys() {
                if !unknown_properties.contains(property.as_str()) {
                    unknown_properties.insert(property.to_string());
                }
            }
        }
    }

    for attribute_filter in &results.attr_filters {
        if !unknown_properties.contains(&attribute_filter.property) {
            unknown_properties.insert(attribute_filter.property.to_string());
        }
    }

    // aggregate and return expression are exclusive
    if let Some(ref return_expr) = results.return_expr {
        let prop = &return_expr.property;
        if !unknown_properties.contains(prop) {
            unknown_properties.insert(prop.to_string());
        }
    } else if let Some(ref aggregate) = results.aggregate {
        let prop = &aggregate.property;
        if !unknown_properties.contains(prop) {
            unknown_properties.insert(prop.to_string());
        }
    }
    results.maps.extend(unknown_properties);
    results
}

/// This is a function that aggregates all the functionality above;  it makes a visitor,
/// visits everything in the query via accept, and then finds the map functions.
pub fn visit_result(result: Rc<OC_CypherContextAll>, root_id: String) -> VisitorResults {
    let mut filter_visitor = FilterVisitor::default();
    let mut return_visitor = ReturnVisitor::default();
    let _res = result.accept(&mut filter_visitor);
    // before we return, strip off the extra quotes if there are any
    for attr_filter in &mut filter_visitor.attr_filters {
        attr_filter.value.retain(|c| c != '\'');
    }
    let _res = result.accept(&mut return_visitor);

    let results = VisitorResults {
        struct_filters: filter_visitor.struct_filters,
        attr_filters: filter_visitor.attr_filters,
        return_expr: return_visitor.return_expr,
        aggregate: return_visitor.aggregate,
        maps: Vec::new(),
        root_id,
    };
    get_map_functions(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::antlr_gen::lexer::CypherLexer;
    use crate::antlr_gen::parser::CypherParser;
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
            "MATCH (a) -[]-> (b)-[]->(c) WHERE b.metadata.WORKLOAD_NAME = 'reviews-v1' RETURN a.request.total_size",
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
            "MATCH (a {}) -[]->(b {}) -[]->(c {}) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' RETURN a.request.total_size"
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.attr_filters[0].node == "b");
        assert!(visitor.attr_filters[0].property == ".node.metadata.WORKLOAD_NAME");
        assert!(
            visitor.attr_filters[0].value == "\'reviews-v1\'",
            "value is {:?}",
            visitor.attr_filters[0].value
        );
        assert!(visitor.struct_filters[0].vertices == vec!["a", "b", "c"]);
        assert!(
            visitor.struct_filters[0].edges
                == vec![
                    ("a".to_string(), "b".to_string()),
                    ("b".to_string(), "c".to_string())
                ],
            "edges are {:?}",
            visitor.struct_filters[0].edges
        );
    }

    #[test]
    fn test_match_where() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b {})-[]->(c) WHERE trace.latency = 500 RETURN a.request.total_size",
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        assert!(!visitor.attr_filters.is_empty());
        assert!(visitor.attr_filters[0].node == "trace".to_string());
        assert!(visitor.attr_filters[0].property == ".latency".to_string());
        assert!(visitor.attr_filters[0].value == "500".to_string());
    }

    #[test]
    fn test_return_udf() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {})-[]->(c) RETURN height(a)");
        let mut visitor = PropertyAndUdfVisitor::default();
        let _res = result.accept(&mut visitor);
        // TODO: Kinda ugly. Fix up
        assert!(
            visitor.udf_calls
                == vec![UdfCall {
                    id: "height".to_string(),
                    args: vec!["a".to_string()]
                }]
        );
    }

    #[test]
    fn test_match_multiple_where() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b {})-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' AND b.latency = 500 AND trace.client = xyz RETURN a.request.total_size");
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        assert!(visitor.attr_filters.len() == 3);
        assert!(visitor.attr_filters[1].node == "b".to_string());
        assert!(visitor.attr_filters[1].property == ".latency".to_string());
        assert!(visitor.attr_filters[1].value == "500".to_string());

        assert!(visitor.attr_filters[2].node == "trace".to_string());
        assert!(visitor.attr_filters[2].property == ".client".to_string());
        assert!(visitor.attr_filters[2].value == "xyz".to_string());
    }

    #[test]
    fn test_return() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b {})-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' RETURN a.request.total_size",
        );
        let mut visitor = ReturnVisitor::default();
        let _res = result.accept(&mut visitor);
        let ret = visitor.return_expr.unwrap();
        assert!(ret.entity == "a");
        assert!(
            ret.property == ".request.total_size",
            "property is {:?}",
            ret.property
        );
    }

    #[test]
    fn test_aggregate() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b {})-[]->(c) RETURN a.return_code, histogram(a.request.total_size) ",
        );
        let mut visitor = ReturnVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.aggregate.as_ref().unwrap().udf_id == "histogram(a.request.total_size)");
        assert!(visitor.aggregate.as_ref().unwrap().entity == "a");
        assert!(visitor.aggregate.as_ref().unwrap().property == ".return_code");
    }

    #[test]
    fn test_map() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b)-[]->(c) WHERE b.node.metadata.WORKLOAD_NAME = 'reviews-v1' RETURN a.return_code, histogram(a.request.total_size) ");
        let mut filter_visitor = FilterVisitor::default();
        let mut return_visitor = ReturnVisitor::default();
        let _res = result.accept(&mut filter_visitor);
        let _res = result.accept(&mut return_visitor);
        let mut results = VisitorResults {
            struct_filters: filter_visitor.struct_filters,
            attr_filters: filter_visitor.attr_filters,
            return_expr: return_visitor.return_expr,
            aggregate: return_visitor.aggregate,
            maps: Vec::new(),
            root_id: "".to_string(),
        };
        results = get_map_functions(results);
        assert!(results.maps.len() == 2);
        assert!(results
            .maps
            .contains(&".node.metadata.WORKLOAD_NAME".to_string()));
        assert!(results.maps.contains(&".return_code".to_string()));
    }

    #[test]
    fn test_return_whole_trace() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(&tf, "MATCH (a) -[]-> (b)-[]->(c) RETURN trace");
        let mut filter_visitor = FilterVisitor::default();
        let mut return_visitor = ReturnVisitor::default();
        let _res = result.accept(&mut filter_visitor);
        let _res = result.accept(&mut return_visitor);
        let results = VisitorResults {
            struct_filters: filter_visitor.struct_filters,
            attr_filters: filter_visitor.attr_filters,
            return_expr: return_visitor.return_expr,
            aggregate: return_visitor.aggregate,
            maps: Vec::new(),
            root_id: "".to_string(),
        };
        assert!(results.return_expr.is_some());
        assert!(results.return_expr.as_ref().unwrap().entity == "trace");
        assert!(results.return_expr.as_ref().unwrap().property == String::new());
    }
}
