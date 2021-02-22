#![allow(nonstandard_style)]
// Generated from Cypher.g4 by ANTLR 4.8
use super::parser::*;
use antlr_rust::tree::ParseTreeVisitor;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link CypherParser}.
 */
pub trait CypherVisitor<'input>: ParseTreeVisitor<'input, CypherParserContextType> {
    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Cypher}.
     * @param ctx the parse tree
     */
    fn visit_oC_Cypher(&mut self, ctx: &OC_CypherContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Statement}.
     * @param ctx the parse tree
     */
    fn visit_oC_Statement(&mut self, ctx: &OC_StatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Query}.
     * @param ctx the parse tree
     */
    fn visit_oC_Query(&mut self, ctx: &OC_QueryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RegularQuery}.
     * @param ctx the parse tree
     */
    fn visit_oC_RegularQuery(&mut self, ctx: &OC_RegularQueryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Union}.
     * @param ctx the parse tree
     */
    fn visit_oC_Union(&mut self, ctx: &OC_UnionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_SingleQuery}.
     * @param ctx the parse tree
     */
    fn visit_oC_SingleQuery(&mut self, ctx: &OC_SingleQueryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_SinglePartQuery}.
     * @param ctx the parse tree
     */
    fn visit_oC_SinglePartQuery(&mut self, ctx: &OC_SinglePartQueryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_MultiPartQuery}.
     * @param ctx the parse tree
     */
    fn visit_oC_MultiPartQuery(&mut self, ctx: &OC_MultiPartQueryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_UpdatingClause}.
     * @param ctx the parse tree
     */
    fn visit_oC_UpdatingClause(&mut self, ctx: &OC_UpdatingClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ReadingClause}.
     * @param ctx the parse tree
     */
    fn visit_oC_ReadingClause(&mut self, ctx: &OC_ReadingClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Match}.
     * @param ctx the parse tree
     */
    fn visit_oC_Match(&mut self, ctx: &OC_MatchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Unwind}.
     * @param ctx the parse tree
     */
    fn visit_oC_Unwind(&mut self, ctx: &OC_UnwindContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Merge}.
     * @param ctx the parse tree
     */
    fn visit_oC_Merge(&mut self, ctx: &OC_MergeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_MergeAction}.
     * @param ctx the parse tree
     */
    fn visit_oC_MergeAction(&mut self, ctx: &OC_MergeActionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Create}.
     * @param ctx the parse tree
     */
    fn visit_oC_Create(&mut self, ctx: &OC_CreateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Set}.
     * @param ctx the parse tree
     */
    fn visit_oC_Set(&mut self, ctx: &OC_SetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_SetItem}.
     * @param ctx the parse tree
     */
    fn visit_oC_SetItem(&mut self, ctx: &OC_SetItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Delete}.
     * @param ctx the parse tree
     */
    fn visit_oC_Delete(&mut self, ctx: &OC_DeleteContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Remove}.
     * @param ctx the parse tree
     */
    fn visit_oC_Remove(&mut self, ctx: &OC_RemoveContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RemoveItem}.
     * @param ctx the parse tree
     */
    fn visit_oC_RemoveItem(&mut self, ctx: &OC_RemoveItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_InQueryCall}.
     * @param ctx the parse tree
     */
    fn visit_oC_InQueryCall(&mut self, ctx: &OC_InQueryCallContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_StandaloneCall}.
     * @param ctx the parse tree
     */
    fn visit_oC_StandaloneCall(&mut self, ctx: &OC_StandaloneCallContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_YieldItems}.
     * @param ctx the parse tree
     */
    fn visit_oC_YieldItems(&mut self, ctx: &OC_YieldItemsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_YieldItem}.
     * @param ctx the parse tree
     */
    fn visit_oC_YieldItem(&mut self, ctx: &OC_YieldItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_With}.
     * @param ctx the parse tree
     */
    fn visit_oC_With(&mut self, ctx: &OC_WithContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Return}.
     * @param ctx the parse tree
     */
    fn visit_oC_Return(&mut self, ctx: &OC_ReturnContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ProjectionBody}.
     * @param ctx the parse tree
     */
    fn visit_oC_ProjectionBody(&mut self, ctx: &OC_ProjectionBodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ProjectionItems}.
     * @param ctx the parse tree
     */
    fn visit_oC_ProjectionItems(&mut self, ctx: &OC_ProjectionItemsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ProjectionItem}.
     * @param ctx the parse tree
     */
    fn visit_oC_ProjectionItem(&mut self, ctx: &OC_ProjectionItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Order}.
     * @param ctx the parse tree
     */
    fn visit_oC_Order(&mut self, ctx: &OC_OrderContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Skip}.
     * @param ctx the parse tree
     */
    fn visit_oC_Skip(&mut self, ctx: &OC_SkipContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Limit}.
     * @param ctx the parse tree
     */
    fn visit_oC_Limit(&mut self, ctx: &OC_LimitContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_SortItem}.
     * @param ctx the parse tree
     */
    fn visit_oC_SortItem(&mut self, ctx: &OC_SortItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Where}.
     * @param ctx the parse tree
     */
    fn visit_oC_Where(&mut self, ctx: &OC_WhereContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Pattern}.
     * @param ctx the parse tree
     */
    fn visit_oC_Pattern(&mut self, ctx: &OC_PatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PatternPart}.
     * @param ctx the parse tree
     */
    fn visit_oC_PatternPart(&mut self, ctx: &OC_PatternPartContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_AnonymousPatternPart}.
     * @param ctx the parse tree
     */
    fn visit_oC_AnonymousPatternPart(&mut self, ctx: &OC_AnonymousPatternPartContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PatternElement}.
     * @param ctx the parse tree
     */
    fn visit_oC_PatternElement(&mut self, ctx: &OC_PatternElementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_NodePattern}.
     * @param ctx the parse tree
     */
    fn visit_oC_NodePattern(&mut self, ctx: &OC_NodePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PatternElementChain}.
     * @param ctx the parse tree
     */
    fn visit_oC_PatternElementChain(&mut self, ctx: &OC_PatternElementChainContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RelationshipPattern}.
     * @param ctx the parse tree
     */
    fn visit_oC_RelationshipPattern(&mut self, ctx: &OC_RelationshipPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RelationshipDetail}.
     * @param ctx the parse tree
     */
    fn visit_oC_RelationshipDetail(&mut self, ctx: &OC_RelationshipDetailContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Properties}.
     * @param ctx the parse tree
     */
    fn visit_oC_Properties(&mut self, ctx: &OC_PropertiesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RelationshipTypes}.
     * @param ctx the parse tree
     */
    fn visit_oC_RelationshipTypes(&mut self, ctx: &OC_RelationshipTypesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_NodeLabels}.
     * @param ctx the parse tree
     */
    fn visit_oC_NodeLabels(&mut self, ctx: &OC_NodeLabelsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_NodeLabel}.
     * @param ctx the parse tree
     */
    fn visit_oC_NodeLabel(&mut self, ctx: &OC_NodeLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RangeLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_RangeLiteral(&mut self, ctx: &OC_RangeLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_LabelName}.
     * @param ctx the parse tree
     */
    fn visit_oC_LabelName(&mut self, ctx: &OC_LabelNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RelTypeName}.
     * @param ctx the parse tree
     */
    fn visit_oC_RelTypeName(&mut self, ctx: &OC_RelTypeNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Expression}.
     * @param ctx the parse tree
     */
    fn visit_oC_Expression(&mut self, ctx: &OC_ExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_OrExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_OrExpression(&mut self, ctx: &OC_OrExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_XorExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_XorExpression(&mut self, ctx: &OC_XorExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_AndExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_AndExpression(&mut self, ctx: &OC_AndExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_NotExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_NotExpression(&mut self, ctx: &OC_NotExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ComparisonExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_ComparisonExpression(&mut self, ctx: &OC_ComparisonExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_AddOrSubtractExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_AddOrSubtractExpression(
        &mut self,
        ctx: &OC_AddOrSubtractExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_MultiplyDivideModuloExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_MultiplyDivideModuloExpression(
        &mut self,
        ctx: &OC_MultiplyDivideModuloExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PowerOfExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_PowerOfExpression(&mut self, ctx: &OC_PowerOfExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_UnaryAddOrSubtractExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_UnaryAddOrSubtractExpression(
        &mut self,
        ctx: &OC_UnaryAddOrSubtractExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_StringListNullOperatorExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_StringListNullOperatorExpression(
        &mut self,
        ctx: &OC_StringListNullOperatorExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ListOperatorExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_ListOperatorExpression(&mut self, ctx: &OC_ListOperatorExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_StringOperatorExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_StringOperatorExpression(
        &mut self,
        ctx: &OC_StringOperatorExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_NullOperatorExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_NullOperatorExpression(&mut self, ctx: &OC_NullOperatorExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PropertyOrLabelsExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_PropertyOrLabelsExpression(
        &mut self,
        ctx: &OC_PropertyOrLabelsExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Atom}.
     * @param ctx the parse tree
     */
    fn visit_oC_Atom(&mut self, ctx: &OC_AtomContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Literal}.
     * @param ctx the parse tree
     */
    fn visit_oC_Literal(&mut self, ctx: &OC_LiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_BooleanLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_BooleanLiteral(&mut self, ctx: &OC_BooleanLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ListLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_ListLiteral(&mut self, ctx: &OC_ListLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PartialComparisonExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_PartialComparisonExpression(
        &mut self,
        ctx: &OC_PartialComparisonExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ParenthesizedExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_ParenthesizedExpression(
        &mut self,
        ctx: &OC_ParenthesizedExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RelationshipsPattern}.
     * @param ctx the parse tree
     */
    fn visit_oC_RelationshipsPattern(&mut self, ctx: &OC_RelationshipsPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_FilterExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_FilterExpression(&mut self, ctx: &OC_FilterExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_IdInColl}.
     * @param ctx the parse tree
     */
    fn visit_oC_IdInColl(&mut self, ctx: &OC_IdInCollContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_FunctionInvocation}.
     * @param ctx the parse tree
     */
    fn visit_oC_FunctionInvocation(&mut self, ctx: &OC_FunctionInvocationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_FunctionName}.
     * @param ctx the parse tree
     */
    fn visit_oC_FunctionName(&mut self, ctx: &OC_FunctionNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ExplicitProcedureInvocation}.
     * @param ctx the parse tree
     */
    fn visit_oC_ExplicitProcedureInvocation(
        &mut self,
        ctx: &OC_ExplicitProcedureInvocationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ImplicitProcedureInvocation}.
     * @param ctx the parse tree
     */
    fn visit_oC_ImplicitProcedureInvocation(
        &mut self,
        ctx: &OC_ImplicitProcedureInvocationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ProcedureResultField}.
     * @param ctx the parse tree
     */
    fn visit_oC_ProcedureResultField(&mut self, ctx: &OC_ProcedureResultFieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ProcedureName}.
     * @param ctx the parse tree
     */
    fn visit_oC_ProcedureName(&mut self, ctx: &OC_ProcedureNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Namespace}.
     * @param ctx the parse tree
     */
    fn visit_oC_Namespace(&mut self, ctx: &OC_NamespaceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ListComprehension}.
     * @param ctx the parse tree
     */
    fn visit_oC_ListComprehension(&mut self, ctx: &OC_ListComprehensionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PatternComprehension}.
     * @param ctx the parse tree
     */
    fn visit_oC_PatternComprehension(&mut self, ctx: &OC_PatternComprehensionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PropertyLookup}.
     * @param ctx the parse tree
     */
    fn visit_oC_PropertyLookup(&mut self, ctx: &OC_PropertyLookupContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_CaseExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_CaseExpression(&mut self, ctx: &OC_CaseExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_CaseAlternatives}.
     * @param ctx the parse tree
     */
    fn visit_oC_CaseAlternatives(&mut self, ctx: &OC_CaseAlternativesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Variable}.
     * @param ctx the parse tree
     */
    fn visit_oC_Variable(&mut self, ctx: &OC_VariableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_NumberLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_NumberLiteral(&mut self, ctx: &OC_NumberLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_MapLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_MapLiteral(&mut self, ctx: &OC_MapLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Parameter}.
     * @param ctx the parse tree
     */
    fn visit_oC_Parameter(&mut self, ctx: &OC_ParameterContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PropertyExpression}.
     * @param ctx the parse tree
     */
    fn visit_oC_PropertyExpression(&mut self, ctx: &OC_PropertyExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_PropertyKeyName}.
     * @param ctx the parse tree
     */
    fn visit_oC_PropertyKeyName(&mut self, ctx: &OC_PropertyKeyNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_IntegerLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_IntegerLiteral(&mut self, ctx: &OC_IntegerLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_DoubleLiteral}.
     * @param ctx the parse tree
     */
    fn visit_oC_DoubleLiteral(&mut self, ctx: &OC_DoubleLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_SchemaName}.
     * @param ctx the parse tree
     */
    fn visit_oC_SchemaName(&mut self, ctx: &OC_SchemaNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_ReservedWord}.
     * @param ctx the parse tree
     */
    fn visit_oC_ReservedWord(&mut self, ctx: &OC_ReservedWordContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_SymbolicName}.
     * @param ctx the parse tree
     */
    fn visit_oC_SymbolicName(&mut self, ctx: &OC_SymbolicNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_LeftArrowHead}.
     * @param ctx the parse tree
     */
    fn visit_oC_LeftArrowHead(&mut self, ctx: &OC_LeftArrowHeadContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_RightArrowHead}.
     * @param ctx the parse tree
     */
    fn visit_oC_RightArrowHead(&mut self, ctx: &OC_RightArrowHeadContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link CypherParser#oC_Dash}.
     * @param ctx the parse tree
     */
    fn visit_oC_Dash(&mut self, ctx: &OC_DashContext<'input>) {
        self.visit_children(ctx)
    }
}
