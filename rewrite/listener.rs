#![allow(nonstandard_style)]
// Generated from Cypher.g4 by ANTLR 4.8
use super::parser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait CypherListener<'input>: ParseTreeListener<'input, CypherParserContextType> {
    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Cypher}.
     * @param ctx the parse tree
     */
    fn enter_oC_Cypher(&mut self, _ctx: &OC_CypherContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Cypher}.
     * @param ctx the parse tree
     */
    fn exit_oC_Cypher(&mut self, _ctx: &OC_CypherContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Statement}.
     * @param ctx the parse tree
     */
    fn enter_oC_Statement(&mut self, _ctx: &OC_StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Statement}.
     * @param ctx the parse tree
     */
    fn exit_oC_Statement(&mut self, _ctx: &OC_StatementContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Query}.
     * @param ctx the parse tree
     */
    fn enter_oC_Query(&mut self, _ctx: &OC_QueryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Query}.
     * @param ctx the parse tree
     */
    fn exit_oC_Query(&mut self, _ctx: &OC_QueryContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RegularQuery}.
     * @param ctx the parse tree
     */
    fn enter_oC_RegularQuery(&mut self, _ctx: &OC_RegularQueryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RegularQuery}.
     * @param ctx the parse tree
     */
    fn exit_oC_RegularQuery(&mut self, _ctx: &OC_RegularQueryContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Union}.
     * @param ctx the parse tree
     */
    fn enter_oC_Union(&mut self, _ctx: &OC_UnionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Union}.
     * @param ctx the parse tree
     */
    fn exit_oC_Union(&mut self, _ctx: &OC_UnionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_SingleQuery}.
     * @param ctx the parse tree
     */
    fn enter_oC_SingleQuery(&mut self, _ctx: &OC_SingleQueryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_SingleQuery}.
     * @param ctx the parse tree
     */
    fn exit_oC_SingleQuery(&mut self, _ctx: &OC_SingleQueryContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_SinglePartQuery}.
     * @param ctx the parse tree
     */
    fn enter_oC_SinglePartQuery(&mut self, _ctx: &OC_SinglePartQueryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_SinglePartQuery}.
     * @param ctx the parse tree
     */
    fn exit_oC_SinglePartQuery(&mut self, _ctx: &OC_SinglePartQueryContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_MultiPartQuery}.
     * @param ctx the parse tree
     */
    fn enter_oC_MultiPartQuery(&mut self, _ctx: &OC_MultiPartQueryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_MultiPartQuery}.
     * @param ctx the parse tree
     */
    fn exit_oC_MultiPartQuery(&mut self, _ctx: &OC_MultiPartQueryContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_UpdatingClause}.
     * @param ctx the parse tree
     */
    fn enter_oC_UpdatingClause(&mut self, _ctx: &OC_UpdatingClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_UpdatingClause}.
     * @param ctx the parse tree
     */
    fn exit_oC_UpdatingClause(&mut self, _ctx: &OC_UpdatingClauseContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ReadingClause}.
     * @param ctx the parse tree
     */
    fn enter_oC_ReadingClause(&mut self, _ctx: &OC_ReadingClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ReadingClause}.
     * @param ctx the parse tree
     */
    fn exit_oC_ReadingClause(&mut self, _ctx: &OC_ReadingClauseContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Match}.
     * @param ctx the parse tree
     */
    fn enter_oC_Match(&mut self, _ctx: &OC_MatchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Match}.
     * @param ctx the parse tree
     */
    fn exit_oC_Match(&mut self, _ctx: &OC_MatchContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Unwind}.
     * @param ctx the parse tree
     */
    fn enter_oC_Unwind(&mut self, _ctx: &OC_UnwindContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Unwind}.
     * @param ctx the parse tree
     */
    fn exit_oC_Unwind(&mut self, _ctx: &OC_UnwindContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Merge}.
     * @param ctx the parse tree
     */
    fn enter_oC_Merge(&mut self, _ctx: &OC_MergeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Merge}.
     * @param ctx the parse tree
     */
    fn exit_oC_Merge(&mut self, _ctx: &OC_MergeContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_MergeAction}.
     * @param ctx the parse tree
     */
    fn enter_oC_MergeAction(&mut self, _ctx: &OC_MergeActionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_MergeAction}.
     * @param ctx the parse tree
     */
    fn exit_oC_MergeAction(&mut self, _ctx: &OC_MergeActionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Create}.
     * @param ctx the parse tree
     */
    fn enter_oC_Create(&mut self, _ctx: &OC_CreateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Create}.
     * @param ctx the parse tree
     */
    fn exit_oC_Create(&mut self, _ctx: &OC_CreateContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Set}.
     * @param ctx the parse tree
     */
    fn enter_oC_Set(&mut self, _ctx: &OC_SetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Set}.
     * @param ctx the parse tree
     */
    fn exit_oC_Set(&mut self, _ctx: &OC_SetContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_SetItem}.
     * @param ctx the parse tree
     */
    fn enter_oC_SetItem(&mut self, _ctx: &OC_SetItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_SetItem}.
     * @param ctx the parse tree
     */
    fn exit_oC_SetItem(&mut self, _ctx: &OC_SetItemContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Delete}.
     * @param ctx the parse tree
     */
    fn enter_oC_Delete(&mut self, _ctx: &OC_DeleteContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Delete}.
     * @param ctx the parse tree
     */
    fn exit_oC_Delete(&mut self, _ctx: &OC_DeleteContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Remove}.
     * @param ctx the parse tree
     */
    fn enter_oC_Remove(&mut self, _ctx: &OC_RemoveContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Remove}.
     * @param ctx the parse tree
     */
    fn exit_oC_Remove(&mut self, _ctx: &OC_RemoveContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RemoveItem}.
     * @param ctx the parse tree
     */
    fn enter_oC_RemoveItem(&mut self, _ctx: &OC_RemoveItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RemoveItem}.
     * @param ctx the parse tree
     */
    fn exit_oC_RemoveItem(&mut self, _ctx: &OC_RemoveItemContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_InQueryCall}.
     * @param ctx the parse tree
     */
    fn enter_oC_InQueryCall(&mut self, _ctx: &OC_InQueryCallContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_InQueryCall}.
     * @param ctx the parse tree
     */
    fn exit_oC_InQueryCall(&mut self, _ctx: &OC_InQueryCallContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_StandaloneCall}.
     * @param ctx the parse tree
     */
    fn enter_oC_StandaloneCall(&mut self, _ctx: &OC_StandaloneCallContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_StandaloneCall}.
     * @param ctx the parse tree
     */
    fn exit_oC_StandaloneCall(&mut self, _ctx: &OC_StandaloneCallContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_YieldItems}.
     * @param ctx the parse tree
     */
    fn enter_oC_YieldItems(&mut self, _ctx: &OC_YieldItemsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_YieldItems}.
     * @param ctx the parse tree
     */
    fn exit_oC_YieldItems(&mut self, _ctx: &OC_YieldItemsContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_YieldItem}.
     * @param ctx the parse tree
     */
    fn enter_oC_YieldItem(&mut self, _ctx: &OC_YieldItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_YieldItem}.
     * @param ctx the parse tree
     */
    fn exit_oC_YieldItem(&mut self, _ctx: &OC_YieldItemContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_With}.
     * @param ctx the parse tree
     */
    fn enter_oC_With(&mut self, _ctx: &OC_WithContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_With}.
     * @param ctx the parse tree
     */
    fn exit_oC_With(&mut self, _ctx: &OC_WithContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Return}.
     * @param ctx the parse tree
     */
    fn enter_oC_Return(&mut self, _ctx: &OC_ReturnContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Return}.
     * @param ctx the parse tree
     */
    fn exit_oC_Return(&mut self, _ctx: &OC_ReturnContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ProjectionBody}.
     * @param ctx the parse tree
     */
    fn enter_oC_ProjectionBody(&mut self, _ctx: &OC_ProjectionBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ProjectionBody}.
     * @param ctx the parse tree
     */
    fn exit_oC_ProjectionBody(&mut self, _ctx: &OC_ProjectionBodyContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ProjectionItems}.
     * @param ctx the parse tree
     */
    fn enter_oC_ProjectionItems(&mut self, _ctx: &OC_ProjectionItemsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ProjectionItems}.
     * @param ctx the parse tree
     */
    fn exit_oC_ProjectionItems(&mut self, _ctx: &OC_ProjectionItemsContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ProjectionItem}.
     * @param ctx the parse tree
     */
    fn enter_oC_ProjectionItem(&mut self, _ctx: &OC_ProjectionItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ProjectionItem}.
     * @param ctx the parse tree
     */
    fn exit_oC_ProjectionItem(&mut self, _ctx: &OC_ProjectionItemContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Order}.
     * @param ctx the parse tree
     */
    fn enter_oC_Order(&mut self, _ctx: &OC_OrderContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Order}.
     * @param ctx the parse tree
     */
    fn exit_oC_Order(&mut self, _ctx: &OC_OrderContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Skip}.
     * @param ctx the parse tree
     */
    fn enter_oC_Skip(&mut self, _ctx: &OC_SkipContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Skip}.
     * @param ctx the parse tree
     */
    fn exit_oC_Skip(&mut self, _ctx: &OC_SkipContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Limit}.
     * @param ctx the parse tree
     */
    fn enter_oC_Limit(&mut self, _ctx: &OC_LimitContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Limit}.
     * @param ctx the parse tree
     */
    fn exit_oC_Limit(&mut self, _ctx: &OC_LimitContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_SortItem}.
     * @param ctx the parse tree
     */
    fn enter_oC_SortItem(&mut self, _ctx: &OC_SortItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_SortItem}.
     * @param ctx the parse tree
     */
    fn exit_oC_SortItem(&mut self, _ctx: &OC_SortItemContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Where}.
     * @param ctx the parse tree
     */
    fn enter_oC_Where(&mut self, _ctx: &OC_WhereContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Where}.
     * @param ctx the parse tree
     */
    fn exit_oC_Where(&mut self, _ctx: &OC_WhereContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Pattern}.
     * @param ctx the parse tree
     */
    fn enter_oC_Pattern(&mut self, _ctx: &OC_PatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Pattern}.
     * @param ctx the parse tree
     */
    fn exit_oC_Pattern(&mut self, _ctx: &OC_PatternContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PatternPart}.
     * @param ctx the parse tree
     */
    fn enter_oC_PatternPart(&mut self, _ctx: &OC_PatternPartContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PatternPart}.
     * @param ctx the parse tree
     */
    fn exit_oC_PatternPart(&mut self, _ctx: &OC_PatternPartContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_AnonymousPatternPart}.
     * @param ctx the parse tree
     */
    fn enter_oC_AnonymousPatternPart(&mut self, _ctx: &OC_AnonymousPatternPartContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_AnonymousPatternPart}.
     * @param ctx the parse tree
     */
    fn exit_oC_AnonymousPatternPart(&mut self, _ctx: &OC_AnonymousPatternPartContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PatternElement}.
     * @param ctx the parse tree
     */
    fn enter_oC_PatternElement(&mut self, _ctx: &OC_PatternElementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PatternElement}.
     * @param ctx the parse tree
     */
    fn exit_oC_PatternElement(&mut self, _ctx: &OC_PatternElementContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_NodePattern}.
     * @param ctx the parse tree
     */
    fn enter_oC_NodePattern(&mut self, _ctx: &OC_NodePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_NodePattern}.
     * @param ctx the parse tree
     */
    fn exit_oC_NodePattern(&mut self, _ctx: &OC_NodePatternContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PatternElementChain}.
     * @param ctx the parse tree
     */
    fn enter_oC_PatternElementChain(&mut self, _ctx: &OC_PatternElementChainContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PatternElementChain}.
     * @param ctx the parse tree
     */
    fn exit_oC_PatternElementChain(&mut self, _ctx: &OC_PatternElementChainContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RelationshipPattern}.
     * @param ctx the parse tree
     */
    fn enter_oC_RelationshipPattern(&mut self, _ctx: &OC_RelationshipPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RelationshipPattern}.
     * @param ctx the parse tree
     */
    fn exit_oC_RelationshipPattern(&mut self, _ctx: &OC_RelationshipPatternContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RelationshipDetail}.
     * @param ctx the parse tree
     */
    fn enter_oC_RelationshipDetail(&mut self, _ctx: &OC_RelationshipDetailContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RelationshipDetail}.
     * @param ctx the parse tree
     */
    fn exit_oC_RelationshipDetail(&mut self, _ctx: &OC_RelationshipDetailContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Properties}.
     * @param ctx the parse tree
     */
    fn enter_oC_Properties(&mut self, _ctx: &OC_PropertiesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Properties}.
     * @param ctx the parse tree
     */
    fn exit_oC_Properties(&mut self, _ctx: &OC_PropertiesContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RelationshipTypes}.
     * @param ctx the parse tree
     */
    fn enter_oC_RelationshipTypes(&mut self, _ctx: &OC_RelationshipTypesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RelationshipTypes}.
     * @param ctx the parse tree
     */
    fn exit_oC_RelationshipTypes(&mut self, _ctx: &OC_RelationshipTypesContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_NodeLabels}.
     * @param ctx the parse tree
     */
    fn enter_oC_NodeLabels(&mut self, _ctx: &OC_NodeLabelsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_NodeLabels}.
     * @param ctx the parse tree
     */
    fn exit_oC_NodeLabels(&mut self, _ctx: &OC_NodeLabelsContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_NodeLabel}.
     * @param ctx the parse tree
     */
    fn enter_oC_NodeLabel(&mut self, _ctx: &OC_NodeLabelContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_NodeLabel}.
     * @param ctx the parse tree
     */
    fn exit_oC_NodeLabel(&mut self, _ctx: &OC_NodeLabelContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RangeLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_RangeLiteral(&mut self, _ctx: &OC_RangeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RangeLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_RangeLiteral(&mut self, _ctx: &OC_RangeLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_LabelName}.
     * @param ctx the parse tree
     */
    fn enter_oC_LabelName(&mut self, _ctx: &OC_LabelNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_LabelName}.
     * @param ctx the parse tree
     */
    fn exit_oC_LabelName(&mut self, _ctx: &OC_LabelNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RelTypeName}.
     * @param ctx the parse tree
     */
    fn enter_oC_RelTypeName(&mut self, _ctx: &OC_RelTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RelTypeName}.
     * @param ctx the parse tree
     */
    fn exit_oC_RelTypeName(&mut self, _ctx: &OC_RelTypeNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Expression}.
     * @param ctx the parse tree
     */
    fn enter_oC_Expression(&mut self, _ctx: &OC_ExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Expression}.
     * @param ctx the parse tree
     */
    fn exit_oC_Expression(&mut self, _ctx: &OC_ExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_OrExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_OrExpression(&mut self, _ctx: &OC_OrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_OrExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_OrExpression(&mut self, _ctx: &OC_OrExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_XorExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_XorExpression(&mut self, _ctx: &OC_XorExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_XorExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_XorExpression(&mut self, _ctx: &OC_XorExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_AndExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_AndExpression(&mut self, _ctx: &OC_AndExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_AndExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_AndExpression(&mut self, _ctx: &OC_AndExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_NotExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_NotExpression(&mut self, _ctx: &OC_NotExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_NotExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_NotExpression(&mut self, _ctx: &OC_NotExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ComparisonExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_ComparisonExpression(&mut self, _ctx: &OC_ComparisonExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ComparisonExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_ComparisonExpression(&mut self, _ctx: &OC_ComparisonExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_AddOrSubtractExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_AddOrSubtractExpression(
        &mut self,
        _ctx: &OC_AddOrSubtractExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_AddOrSubtractExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_AddOrSubtractExpression(
        &mut self,
        _ctx: &OC_AddOrSubtractExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_MultiplyDivideModuloExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_MultiplyDivideModuloExpression(
        &mut self,
        _ctx: &OC_MultiplyDivideModuloExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_MultiplyDivideModuloExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_MultiplyDivideModuloExpression(
        &mut self,
        _ctx: &OC_MultiplyDivideModuloExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PowerOfExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_PowerOfExpression(&mut self, _ctx: &OC_PowerOfExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PowerOfExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_PowerOfExpression(&mut self, _ctx: &OC_PowerOfExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_UnaryAddOrSubtractExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_UnaryAddOrSubtractExpression(
        &mut self,
        _ctx: &OC_UnaryAddOrSubtractExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_UnaryAddOrSubtractExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_UnaryAddOrSubtractExpression(
        &mut self,
        _ctx: &OC_UnaryAddOrSubtractExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_StringListNullOperatorExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_StringListNullOperatorExpression(
        &mut self,
        _ctx: &OC_StringListNullOperatorExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_StringListNullOperatorExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_StringListNullOperatorExpression(
        &mut self,
        _ctx: &OC_StringListNullOperatorExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ListOperatorExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_ListOperatorExpression(&mut self, _ctx: &OC_ListOperatorExpressionContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ListOperatorExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_ListOperatorExpression(&mut self, _ctx: &OC_ListOperatorExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_StringOperatorExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_StringOperatorExpression(
        &mut self,
        _ctx: &OC_StringOperatorExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_StringOperatorExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_StringOperatorExpression(
        &mut self,
        _ctx: &OC_StringOperatorExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_NullOperatorExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_NullOperatorExpression(&mut self, _ctx: &OC_NullOperatorExpressionContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_NullOperatorExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_NullOperatorExpression(&mut self, _ctx: &OC_NullOperatorExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PropertyOrLabelsExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_PropertyOrLabelsExpression(
        &mut self,
        _ctx: &OC_PropertyOrLabelsExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PropertyOrLabelsExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_PropertyOrLabelsExpression(
        &mut self,
        _ctx: &OC_PropertyOrLabelsExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Atom}.
     * @param ctx the parse tree
     */
    fn enter_oC_Atom(&mut self, _ctx: &OC_AtomContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Atom}.
     * @param ctx the parse tree
     */
    fn exit_oC_Atom(&mut self, _ctx: &OC_AtomContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Literal}.
     * @param ctx the parse tree
     */
    fn enter_oC_Literal(&mut self, _ctx: &OC_LiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Literal}.
     * @param ctx the parse tree
     */
    fn exit_oC_Literal(&mut self, _ctx: &OC_LiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_BooleanLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_BooleanLiteral(&mut self, _ctx: &OC_BooleanLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_BooleanLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_BooleanLiteral(&mut self, _ctx: &OC_BooleanLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ListLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_ListLiteral(&mut self, _ctx: &OC_ListLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ListLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_ListLiteral(&mut self, _ctx: &OC_ListLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PartialComparisonExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_PartialComparisonExpression(
        &mut self,
        _ctx: &OC_PartialComparisonExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PartialComparisonExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_PartialComparisonExpression(
        &mut self,
        _ctx: &OC_PartialComparisonExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ParenthesizedExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_ParenthesizedExpression(
        &mut self,
        _ctx: &OC_ParenthesizedExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ParenthesizedExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_ParenthesizedExpression(
        &mut self,
        _ctx: &OC_ParenthesizedExpressionContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RelationshipsPattern}.
     * @param ctx the parse tree
     */
    fn enter_oC_RelationshipsPattern(&mut self, _ctx: &OC_RelationshipsPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RelationshipsPattern}.
     * @param ctx the parse tree
     */
    fn exit_oC_RelationshipsPattern(&mut self, _ctx: &OC_RelationshipsPatternContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_FilterExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_FilterExpression(&mut self, _ctx: &OC_FilterExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_FilterExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_FilterExpression(&mut self, _ctx: &OC_FilterExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_IdInColl}.
     * @param ctx the parse tree
     */
    fn enter_oC_IdInColl(&mut self, _ctx: &OC_IdInCollContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_IdInColl}.
     * @param ctx the parse tree
     */
    fn exit_oC_IdInColl(&mut self, _ctx: &OC_IdInCollContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_FunctionInvocation}.
     * @param ctx the parse tree
     */
    fn enter_oC_FunctionInvocation(&mut self, _ctx: &OC_FunctionInvocationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_FunctionInvocation}.
     * @param ctx the parse tree
     */
    fn exit_oC_FunctionInvocation(&mut self, _ctx: &OC_FunctionInvocationContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_FunctionName}.
     * @param ctx the parse tree
     */
    fn enter_oC_FunctionName(&mut self, _ctx: &OC_FunctionNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_FunctionName}.
     * @param ctx the parse tree
     */
    fn exit_oC_FunctionName(&mut self, _ctx: &OC_FunctionNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ExplicitProcedureInvocation}.
     * @param ctx the parse tree
     */
    fn enter_oC_ExplicitProcedureInvocation(
        &mut self,
        _ctx: &OC_ExplicitProcedureInvocationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ExplicitProcedureInvocation}.
     * @param ctx the parse tree
     */
    fn exit_oC_ExplicitProcedureInvocation(
        &mut self,
        _ctx: &OC_ExplicitProcedureInvocationContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ImplicitProcedureInvocation}.
     * @param ctx the parse tree
     */
    fn enter_oC_ImplicitProcedureInvocation(
        &mut self,
        _ctx: &OC_ImplicitProcedureInvocationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ImplicitProcedureInvocation}.
     * @param ctx the parse tree
     */
    fn exit_oC_ImplicitProcedureInvocation(
        &mut self,
        _ctx: &OC_ImplicitProcedureInvocationContext<'input>,
    ) {
    }

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ProcedureResultField}.
     * @param ctx the parse tree
     */
    fn enter_oC_ProcedureResultField(&mut self, _ctx: &OC_ProcedureResultFieldContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ProcedureResultField}.
     * @param ctx the parse tree
     */
    fn exit_oC_ProcedureResultField(&mut self, _ctx: &OC_ProcedureResultFieldContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ProcedureName}.
     * @param ctx the parse tree
     */
    fn enter_oC_ProcedureName(&mut self, _ctx: &OC_ProcedureNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ProcedureName}.
     * @param ctx the parse tree
     */
    fn exit_oC_ProcedureName(&mut self, _ctx: &OC_ProcedureNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Namespace}.
     * @param ctx the parse tree
     */
    fn enter_oC_Namespace(&mut self, _ctx: &OC_NamespaceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Namespace}.
     * @param ctx the parse tree
     */
    fn exit_oC_Namespace(&mut self, _ctx: &OC_NamespaceContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ListComprehension}.
     * @param ctx the parse tree
     */
    fn enter_oC_ListComprehension(&mut self, _ctx: &OC_ListComprehensionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ListComprehension}.
     * @param ctx the parse tree
     */
    fn exit_oC_ListComprehension(&mut self, _ctx: &OC_ListComprehensionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PatternComprehension}.
     * @param ctx the parse tree
     */
    fn enter_oC_PatternComprehension(&mut self, _ctx: &OC_PatternComprehensionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PatternComprehension}.
     * @param ctx the parse tree
     */
    fn exit_oC_PatternComprehension(&mut self, _ctx: &OC_PatternComprehensionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PropertyLookup}.
     * @param ctx the parse tree
     */
    fn enter_oC_PropertyLookup(&mut self, _ctx: &OC_PropertyLookupContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PropertyLookup}.
     * @param ctx the parse tree
     */
    fn exit_oC_PropertyLookup(&mut self, _ctx: &OC_PropertyLookupContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_CaseExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_CaseExpression(&mut self, _ctx: &OC_CaseExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_CaseExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_CaseExpression(&mut self, _ctx: &OC_CaseExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_CaseAlternatives}.
     * @param ctx the parse tree
     */
    fn enter_oC_CaseAlternatives(&mut self, _ctx: &OC_CaseAlternativesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_CaseAlternatives}.
     * @param ctx the parse tree
     */
    fn exit_oC_CaseAlternatives(&mut self, _ctx: &OC_CaseAlternativesContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Variable}.
     * @param ctx the parse tree
     */
    fn enter_oC_Variable(&mut self, _ctx: &OC_VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Variable}.
     * @param ctx the parse tree
     */
    fn exit_oC_Variable(&mut self, _ctx: &OC_VariableContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_NumberLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_NumberLiteral(&mut self, _ctx: &OC_NumberLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_NumberLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_NumberLiteral(&mut self, _ctx: &OC_NumberLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_MapLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_MapLiteral(&mut self, _ctx: &OC_MapLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_MapLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_MapLiteral(&mut self, _ctx: &OC_MapLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Parameter}.
     * @param ctx the parse tree
     */
    fn enter_oC_Parameter(&mut self, _ctx: &OC_ParameterContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Parameter}.
     * @param ctx the parse tree
     */
    fn exit_oC_Parameter(&mut self, _ctx: &OC_ParameterContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PropertyExpression}.
     * @param ctx the parse tree
     */
    fn enter_oC_PropertyExpression(&mut self, _ctx: &OC_PropertyExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PropertyExpression}.
     * @param ctx the parse tree
     */
    fn exit_oC_PropertyExpression(&mut self, _ctx: &OC_PropertyExpressionContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_PropertyKeyName}.
     * @param ctx the parse tree
     */
    fn enter_oC_PropertyKeyName(&mut self, _ctx: &OC_PropertyKeyNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_PropertyKeyName}.
     * @param ctx the parse tree
     */
    fn exit_oC_PropertyKeyName(&mut self, _ctx: &OC_PropertyKeyNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_IntegerLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_IntegerLiteral(&mut self, _ctx: &OC_IntegerLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_IntegerLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_IntegerLiteral(&mut self, _ctx: &OC_IntegerLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_DoubleLiteral}.
     * @param ctx the parse tree
     */
    fn enter_oC_DoubleLiteral(&mut self, _ctx: &OC_DoubleLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_DoubleLiteral}.
     * @param ctx the parse tree
     */
    fn exit_oC_DoubleLiteral(&mut self, _ctx: &OC_DoubleLiteralContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_SchemaName}.
     * @param ctx the parse tree
     */
    fn enter_oC_SchemaName(&mut self, _ctx: &OC_SchemaNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_SchemaName}.
     * @param ctx the parse tree
     */
    fn exit_oC_SchemaName(&mut self, _ctx: &OC_SchemaNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_ReservedWord}.
     * @param ctx the parse tree
     */
    fn enter_oC_ReservedWord(&mut self, _ctx: &OC_ReservedWordContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_ReservedWord}.
     * @param ctx the parse tree
     */
    fn exit_oC_ReservedWord(&mut self, _ctx: &OC_ReservedWordContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_SymbolicName}.
     * @param ctx the parse tree
     */
    fn enter_oC_SymbolicName(&mut self, _ctx: &OC_SymbolicNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_SymbolicName}.
     * @param ctx the parse tree
     */
    fn exit_oC_SymbolicName(&mut self, _ctx: &OC_SymbolicNameContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_LeftArrowHead}.
     * @param ctx the parse tree
     */
    fn enter_oC_LeftArrowHead(&mut self, _ctx: &OC_LeftArrowHeadContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_LeftArrowHead}.
     * @param ctx the parse tree
     */
    fn exit_oC_LeftArrowHead(&mut self, _ctx: &OC_LeftArrowHeadContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_RightArrowHead}.
     * @param ctx the parse tree
     */
    fn enter_oC_RightArrowHead(&mut self, _ctx: &OC_RightArrowHeadContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_RightArrowHead}.
     * @param ctx the parse tree
     */
    fn exit_oC_RightArrowHead(&mut self, _ctx: &OC_RightArrowHeadContext<'input>) {}

    /**
     * Enter a parse tree produced by {@link CypherParser#oC_Dash}.
     * @param ctx the parse tree
     */
    fn enter_oC_Dash(&mut self, _ctx: &OC_DashContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link CypherParser#oC_Dash}.
     * @param ctx the parse tree
     */
    fn exit_oC_Dash(&mut self, _ctx: &OC_DashContext<'input>) {}
}
