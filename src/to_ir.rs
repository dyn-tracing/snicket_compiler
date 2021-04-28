use super::antlr_gen::parser::*;
use super::antlr_gen::visitor::CypherVisitor;
use super::ir::IrReturnEnum;
use super::ir::PropertyOrUDF;
use super::ir::*;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::ParseTreeVisitor;
use antlr_rust::tree::TerminalNode;
use antlr_rust::tree::Tree;
use antlr_rust::tree::Visitable;
use indexmap::map::IndexMap;
use indexmap::IndexSet;
use std::process;
use std::rc::Rc;

/***********************************/
// FilterVisitor:  visits tree and fills out structural and property filters
/***********************************/
pub struct PropertyAndUdfVisitor {
    properties: IndexSet<Property>,
    udf_calls: IndexSet<UdfCall>,
}

impl Default for PropertyAndUdfVisitor {
    fn default() -> PropertyAndUdfVisitor {
        PropertyAndUdfVisitor {
            properties: IndexSet::new(),
            udf_calls: IndexSet::new(),
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
        self.udf_calls.insert(UdfCall {
            id: udf_name,
            args: udf_args,
        });
    }

    fn visit_oC_PropertyOrLabelsExpression(
        &mut self,
        prop: &OC_PropertyOrLabelsExpressionContext<'i>,
    ) {
        log::debug!("Generating Property");
        let atom = prop.oC_Atom().unwrap();
        let entity: String;
        if let Some(func) = atom.oC_FunctionInvocation() {
            func.accept(self);
            // TODO:  Technically, UDFs can return an object
            return;
        } else if let Some(var) = atom.oC_Variable() {
            entity = var.get_text();
            log::debug!("Storing var: {:?}", entity);
        } else if let Some(var) = atom.oC_Literal() {
            // TODO:  Literals are values, not property. How do we treat them?
            log::debug!("Storing literal: {:?}", var);
            return;
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
            if let Some(prop_key) = property.oC_PropertyKeyName() {
                property_vec.push(prop_key.get_text());
            } else {
                panic!("Expected identifer to follow property notation.")
            }
        }
        // if it's 0, that means you likely just found a variable inside a UDF, like height(c)
        if property_vec.len() > 0 {
            self.properties.insert(Property {
                parent: entity,
                members: property_vec,
            });
        }
    }
}

/***********************************/
// FilterVisitor:  visits tree and fills out structural and property filters
/***********************************/

pub struct FilterVisitor {
    struct_filters: Vec<StructuralFilter>,
    attr_filters: Vec<AttributeFilter>,
    return_items: Vec<IrReturn>,
    property_references: Vec<EntityReference>,
}

impl Default for FilterVisitor {
    fn default() -> Self {
        FilterVisitor {
            struct_filters: Vec::new(),
            attr_filters: Vec::new(),
            return_items: Vec::new(),
            property_references: Vec::new(),
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
        let mut entity: String;
        let mut property_str = String::new();
        for property in ctx.oC_PropertyLookup_all() {
            // this includes the dots
            property_str.push_str(&property.get_text())
        }
        print!("property string: {:?}\n", property_str);
        log::debug!("Property String {:?}", property_str);
        
        if let Some(func) = atom.oC_FunctionInvocation() {
            entity = func.get_text();
            log::debug!("Storing UDF: {:?}", entity);
            // this is not the correct "parse-y" way to do it.  But I can't figure out another way
            if let Some(index) = entity.find('(') {
                property_str = entity[0..index].to_string();
                entity = entity[index+1..entity.len()-1].to_string();
            } else { log::error!("could not find parenthesis in UDF"); }
        } else if let Some(var) = atom.oC_Variable() {
            entity = var.get_text();
            print!("entity: {:?}\n", entity);
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

        self.return_items.push(IrReturn {
            entity,
            property: property_str,
        });

        if atom.oC_Literal().is_some() {
            //TODO: This check can be removed once we are done with the transition
            return;
        }
        self.property_references.push(ctx.get_text());
    }

    fn visit_oC_ComparisonExpression(&mut self, ctx: &OC_ComparisonExpressionContext<'i>) {
        // get the left node
        ctx.oC_AddOrSubtractExpression().unwrap().accept(self);
        let node = self.return_items[0].entity.clone();
        let property = self.return_items[0].property.clone();
        self.return_items.clear();

        if property.is_empty() {
            // skip
            return;
        }

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
        if self.struct_filters.is_empty() {
            let mut new_struct_filter = StructuralFilter::default();
            self.struct_filters.push(new_struct_filter);
        }
        let mut struct_filter = &mut self.struct_filters[0];

        let mut left_node = ctx.oC_NodePattern().unwrap().oC_Variable().unwrap();
        struct_filter.vertices.insert(left_node.get_text());
        for pattern_element_i in ctx.oC_PatternElementChain_all() {
            let relationship = pattern_element_i.oC_RelationshipPattern().unwrap();
            let node_pattern = pattern_element_i.oC_NodePattern().unwrap();

            let right_node = node_pattern.oC_Variable().unwrap();
            struct_filter.vertices.insert(right_node.get_text());

            // only add right-side edges for now
            if relationship.oC_RightArrowHead().is_some() {
                struct_filter
                    .edges
                    .insert((left_node.get_text(), right_node.get_text()));
            } else {
                log::error!("Unsupported direction");
                process::exit(1);
            }
            // update the left node
            left_node = right_node;
        }
        if let Some(extra_struct) = ctx.oC_PatternElement() {
            extra_struct.accept(self);
        }
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
    return_expr: IrReturnEnum,
    obj_references: Vec<PropertyOrUDF>,
}

impl Default for ReturnVisitor {
    fn default() -> Self {
        ReturnVisitor {
            return_expr: IrReturnEnum::default(),
            obj_references: Vec::new(),
        }
    }
}

impl<'i> ParseTreeVisitor<'i, CypherParserContextType> for ReturnVisitor {
    fn visit_terminal(&mut self, _node: &TerminalNode<'i, CypherParserContextType>) {}
}

impl<'i> ReturnVisitor {
    fn get_udf_function(&self, func: &OC_FunctionInvocationContext<'i>) -> UdfCall {
        let udf_name: String;
        if let Some(udf_name_) = func.oC_FunctionName() {
            udf_name = udf_name_.get_text();
        } else {
            panic!("Compiler Bug: Missing UDF name.")
        }
        let mut udf_args = vec![];
        // TODO: We should not allocate a property visitor for each call
        for arg in func.oC_Expression_all() {
            // TODO: These can be complicated expressions
            // For now we expect a simple entity reference
            udf_args.push(arg.get_text());
        }
        log::debug!(
            "Storing UDF with name: {:?} and args {:?}",
            udf_name,
            udf_args
        );
        UdfCall {
            id: udf_name,
            args: udf_args,
        }
    }
}

impl<'i> CypherVisitor<'i> for ReturnVisitor {
    // we do not want to visit matches in this case, ignore that part of the tree
    // TODO: Apply the visitor directly to the RETURN body instead
    fn visit_oC_Match(&mut self, _ctx: &OC_MatchContext<'i>) {}

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
            // TODO: Technically, UDFs can return an object
            self.obj_references
                .push(PropertyOrUDF::UdfCall(self.get_udf_function(&func)));
            return;
        } else if let Some(var) = atom.oC_Literal() {
            // TODO:  Literals are values, not property. How do we treat them?
            log::debug!("Storing literal: {:?}", var);
            return;
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
            if let Some(prop_key) = property.oC_PropertyKeyName() {
                property_vec.push(prop_key.get_text());
            } else {
                panic!("Expected identifer to follow property notation.")
            }
        }
        self.obj_references.push(PropertyOrUDF::Property(Property {
            parent: entity,
            members: property_vec,
        }));
    }

    fn visit_oC_ProjectionItems(&mut self, ctx: &OC_ProjectionItemsContext<'i>) {
        // For now we distinguish between aggregation and UDFs in the codegen
        // FIXME: Make this more explicit with a dedicated visit tree
        self.obj_references.clear();
        let proj_items = ctx.oC_ProjectionItem_all();
        let proj_items_len = proj_items.len();
        if proj_items_len == 1 {
            proj_items[0].accept(self);
            if self.obj_references.is_empty() {
                log::error!("Return term has unexpected format or was not found.");
                process::exit(1)
            }
            let return_item = self.obj_references[0].clone();
            // return a value
            self.return_expr = IrReturnEnum::PropertyOrUDF(return_item);
        } else if proj_items_len == 2 {
            proj_items[0].accept(self);
            if self.obj_references.is_empty() {
                log::error!("Group term has unexpected format or was not found.");
                process::exit(1)
            }
            let return_item = self.obj_references[0].clone();
            self.obj_references.clear();
            proj_items[1].accept(self);
            if self.obj_references.is_empty() {
                log::error!("Agg term has unexpected format or was not found.");
                process::exit(1)
            }
            if let PropertyOrUDF::UdfCall(udf) = self.obj_references[0].clone() {
                self.return_expr = IrReturnEnum::Aggregate(Aggregate::new_with_items(udf, vec![return_item]));

            } else {
                log::error!("Could not parse aggregation function");
                process::exit(1);
            }
            
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

/// This is a function that aggregates all the functionality above;  it makes a visitor,
/// visits everything in the query via accept, and then finds the map functions.
pub fn visit_result(result: Rc<OC_CypherContextAll>, root_id: String) -> VisitorResults {
    let mut filter_visitor = FilterVisitor::default();
    let mut return_visitor = ReturnVisitor::default();
    let mut prop_visitor = PropertyAndUdfVisitor::default();
    let _res = result.accept(&mut filter_visitor);
    // before we return, strip off the extra quotes if there are any
    for attr_filter in &mut filter_visitor.attr_filters {
        attr_filter.value.retain(|c| c != '\'');
    }
    let _res = result.accept(&mut return_visitor);
    let _res = result.accept(&mut prop_visitor);

    VisitorResults {
        struct_filters: filter_visitor.struct_filters,
        attr_filters: filter_visitor.attr_filters,
        return_expr: return_visitor.return_expr,
        root_id,
        properties: prop_visitor.properties,
        udf_calls: prop_visitor.udf_calls,
    }
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
    fn test_structural_filter() {
        // non branching
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b)-[]->(c) RETURN a.request.total_size",
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        let mut correct_vertices = IndexSet::new();
        correct_vertices.insert("a".to_string());
        correct_vertices.insert("b".to_string());
        correct_vertices.insert("c".to_string());
        let mut correct_edges = IndexSet::new();
        correct_edges.insert(("a".to_string(), "b".to_string()));
        correct_edges.insert(("b".to_string(), "c".to_string()));
        assert!(visitor.struct_filters[0].vertices == correct_vertices);
        assert!(visitor.struct_filters[0].edges == correct_edges);

        // branching
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b), (a) -[]-> (c)  RETURN height(a)"
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(!visitor.struct_filters.is_empty());
        let mut correct_vertices = IndexSet::new();
        correct_vertices.insert("a".to_string());
        correct_vertices.insert("b".to_string());
        correct_vertices.insert("c".to_string());
        let mut correct_edges = IndexSet::new();
        correct_edges.insert(("a".to_string(), "b".to_string()));
        correct_edges.insert(("a".to_string(), "c".to_string()));
        assert!(visitor.struct_filters[0].vertices == correct_vertices,
            "vertices are {:?}", visitor.struct_filters[0].vertices);
        assert!(visitor.struct_filters[0].edges == correct_edges);
    }


    #[test]
    fn test_attribute_filter() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b)-[]->(c) WHERE a.request.total_size = 5 RETURN a.workload_name",
        );
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.attr_filters.len()==1);
        assert!(visitor.attr_filters.contains(&AttributeFilter { node: "a".to_string(), property: ".request.total_size".to_string(), value: "5".to_string() } ));
    }


    #[test]
    fn test_properties_and_udf_calls() {
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b)-[]->(c) WHERE height(a) = 2 RETURN a.workload_name",
        );
        let mut visitor = PropertyAndUdfVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.properties.len()==1);
        assert!(visitor.properties.contains(
            &Property {
                parent: "a".to_string(),
                members: vec!["workload_name".to_string()]
            }
        ));
        assert!(visitor.udf_calls.len()==1);
        assert!(visitor.udf_calls.contains(
            &UdfCall {
                id: "height".to_string(),
                args: vec!["a".to_string()]
            }
        ));

        // check that attribute part of regular visitor is working properly too 
        let mut visitor = FilterVisitor::default();
        let _res = result.accept(&mut visitor);
        assert!(visitor.attr_filters.len()==1, "attr filters are {:?}", visitor.attr_filters);
        assert!(visitor.attr_filters.contains(
            &AttributeFilter {
                node: "a".to_string(),
                property: "height".to_string(),
                value: "2".to_string()
            }),
            "attr filter is {:?}", visitor.attr_filters[0]
        );
    }

    #[test]
    fn test_return_expr() {
        // regular return
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b)-[]->(c) WHERE height(a) = 2 RETURN a.request.total_size",
        );
        let mut return_visitor = ReturnVisitor::default();
        let _res = result.accept(&mut return_visitor);
        assert!(return_visitor.return_expr
                == IrReturnEnum::PropertyOrUDF(PropertyOrUDF::Property({
                    Property {
                        parent: "a".to_string(),
                        members: vec!["request".to_string(), "total_size".to_string()],
                    }
                }))
        );

        // aggregation
        let tf = CommonTokenFactory::default();
        let result = run_parser(
            &tf,
            "MATCH (a) -[]-> (b)-[]->(c) WHERE height(a) = 2 RETURN a.request.total_size, avg(a.request.total_size)",
        );
        let mut return_visitor = ReturnVisitor::default();
        let _res = result.accept(&mut return_visitor);

        if let IrReturnEnum::Aggregate(agg) = return_visitor.return_expr {
            assert!(agg.udf_reference.id == "avg".to_string());
            assert!(agg.udf_reference.args.len() == 1);
            assert!(agg.udf_reference.args[0] == "a.request.total_size".to_string());

            assert!(agg.args.len() == 1);
            if let PropertyOrUDF::Property(prop) = &agg.args[0] {
                assert!(prop.parent == "a".to_string());
                assert!(prop.members == vec!["request".to_string(), "total_size".to_string()] );
            } else {
                assert!(false, "could not get property in aggregation function");
            }
        } else {
            assert!(false, "did not recognize aggregation function as aggregation");
        }
    }
}
