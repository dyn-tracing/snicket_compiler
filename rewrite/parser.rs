// Generated from Cypher.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use super::listener::*;
use super::visitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::PredictionContextCache;
use antlr_rust::TokenSource;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const T__0: isize = 1;
pub const T__1: isize = 2;
pub const T__2: isize = 3;
pub const T__3: isize = 4;
pub const T__4: isize = 5;
pub const T__5: isize = 6;
pub const T__6: isize = 7;
pub const T__7: isize = 8;
pub const T__8: isize = 9;
pub const T__9: isize = 10;
pub const T__10: isize = 11;
pub const T__11: isize = 12;
pub const T__12: isize = 13;
pub const T__13: isize = 14;
pub const T__14: isize = 15;
pub const T__15: isize = 16;
pub const T__16: isize = 17;
pub const T__17: isize = 18;
pub const T__18: isize = 19;
pub const T__19: isize = 20;
pub const T__20: isize = 21;
pub const T__21: isize = 22;
pub const T__22: isize = 23;
pub const T__23: isize = 24;
pub const T__24: isize = 25;
pub const T__25: isize = 26;
pub const T__26: isize = 27;
pub const T__27: isize = 28;
pub const T__28: isize = 29;
pub const T__29: isize = 30;
pub const T__30: isize = 31;
pub const T__31: isize = 32;
pub const T__32: isize = 33;
pub const T__33: isize = 34;
pub const T__34: isize = 35;
pub const T__35: isize = 36;
pub const T__36: isize = 37;
pub const T__37: isize = 38;
pub const T__38: isize = 39;
pub const T__39: isize = 40;
pub const T__40: isize = 41;
pub const T__41: isize = 42;
pub const T__42: isize = 43;
pub const T__43: isize = 44;
pub const T__44: isize = 45;
pub const UNION: isize = 46;
pub const ALL: isize = 47;
pub const OPTIONAL: isize = 48;
pub const MATCH: isize = 49;
pub const UNWIND: isize = 50;
pub const AS: isize = 51;
pub const MERGE: isize = 52;
pub const ON: isize = 53;
pub const CREATE: isize = 54;
pub const SET: isize = 55;
pub const DETACH: isize = 56;
pub const DELETE: isize = 57;
pub const REMOVE: isize = 58;
pub const CALL: isize = 59;
pub const YIELD: isize = 60;
pub const WITH: isize = 61;
pub const RETURN: isize = 62;
pub const DISTINCT: isize = 63;
pub const ORDER: isize = 64;
pub const BY: isize = 65;
pub const L_SKIP: isize = 66;
pub const LIMIT: isize = 67;
pub const ASCENDING: isize = 68;
pub const ASC: isize = 69;
pub const DESCENDING: isize = 70;
pub const DESC: isize = 71;
pub const WHERE: isize = 72;
pub const OR: isize = 73;
pub const XOR: isize = 74;
pub const AND: isize = 75;
pub const NOT: isize = 76;
pub const IN: isize = 77;
pub const STARTS: isize = 78;
pub const ENDS: isize = 79;
pub const CONTAINS: isize = 80;
pub const IS: isize = 81;
pub const NULL: isize = 82;
pub const COUNT: isize = 83;
pub const ANY: isize = 84;
pub const NONE: isize = 85;
pub const SINGLE: isize = 86;
pub const TRUE: isize = 87;
pub const FALSE: isize = 88;
pub const EXISTS: isize = 89;
pub const CASE: isize = 90;
pub const ELSE: isize = 91;
pub const END: isize = 92;
pub const WHEN: isize = 93;
pub const THEN: isize = 94;
pub const StringLiteral: isize = 95;
pub const EscapedChar: isize = 96;
pub const HexInteger: isize = 97;
pub const DecimalInteger: isize = 98;
pub const OctalInteger: isize = 99;
pub const HexLetter: isize = 100;
pub const HexDigit: isize = 101;
pub const Digit: isize = 102;
pub const NonZeroDigit: isize = 103;
pub const NonZeroOctDigit: isize = 104;
pub const OctDigit: isize = 105;
pub const ZeroDigit: isize = 106;
pub const ExponentDecimalReal: isize = 107;
pub const RegularDecimalReal: isize = 108;
pub const CONSTRAINT: isize = 109;
pub const DO: isize = 110;
pub const FOR: isize = 111;
pub const REQUIRE: isize = 112;
pub const UNIQUE: isize = 113;
pub const MANDATORY: isize = 114;
pub const SCALAR: isize = 115;
pub const OF: isize = 116;
pub const ADD: isize = 117;
pub const DROP: isize = 118;
pub const FILTER: isize = 119;
pub const EXTRACT: isize = 120;
pub const UnescapedSymbolicName: isize = 121;
pub const IdentifierStart: isize = 122;
pub const IdentifierPart: isize = 123;
pub const EscapedSymbolicName: isize = 124;
pub const SP: isize = 125;
pub const WHITESPACE: isize = 126;
pub const Comment: isize = 127;
pub const RULE_oC_Cypher: usize = 0;
pub const RULE_oC_Statement: usize = 1;
pub const RULE_oC_Query: usize = 2;
pub const RULE_oC_RegularQuery: usize = 3;
pub const RULE_oC_Union: usize = 4;
pub const RULE_oC_SingleQuery: usize = 5;
pub const RULE_oC_SinglePartQuery: usize = 6;
pub const RULE_oC_MultiPartQuery: usize = 7;
pub const RULE_oC_UpdatingClause: usize = 8;
pub const RULE_oC_ReadingClause: usize = 9;
pub const RULE_oC_Match: usize = 10;
pub const RULE_oC_Unwind: usize = 11;
pub const RULE_oC_Merge: usize = 12;
pub const RULE_oC_MergeAction: usize = 13;
pub const RULE_oC_Create: usize = 14;
pub const RULE_oC_Set: usize = 15;
pub const RULE_oC_SetItem: usize = 16;
pub const RULE_oC_Delete: usize = 17;
pub const RULE_oC_Remove: usize = 18;
pub const RULE_oC_RemoveItem: usize = 19;
pub const RULE_oC_InQueryCall: usize = 20;
pub const RULE_oC_StandaloneCall: usize = 21;
pub const RULE_oC_YieldItems: usize = 22;
pub const RULE_oC_YieldItem: usize = 23;
pub const RULE_oC_With: usize = 24;
pub const RULE_oC_Return: usize = 25;
pub const RULE_oC_ProjectionBody: usize = 26;
pub const RULE_oC_ProjectionItems: usize = 27;
pub const RULE_oC_ProjectionItem: usize = 28;
pub const RULE_oC_Order: usize = 29;
pub const RULE_oC_Skip: usize = 30;
pub const RULE_oC_Limit: usize = 31;
pub const RULE_oC_SortItem: usize = 32;
pub const RULE_oC_Where: usize = 33;
pub const RULE_oC_Pattern: usize = 34;
pub const RULE_oC_PatternPart: usize = 35;
pub const RULE_oC_AnonymousPatternPart: usize = 36;
pub const RULE_oC_PatternElement: usize = 37;
pub const RULE_oC_NodePattern: usize = 38;
pub const RULE_oC_PatternElementChain: usize = 39;
pub const RULE_oC_RelationshipPattern: usize = 40;
pub const RULE_oC_RelationshipDetail: usize = 41;
pub const RULE_oC_Properties: usize = 42;
pub const RULE_oC_RelationshipTypes: usize = 43;
pub const RULE_oC_NodeLabels: usize = 44;
pub const RULE_oC_NodeLabel: usize = 45;
pub const RULE_oC_RangeLiteral: usize = 46;
pub const RULE_oC_LabelName: usize = 47;
pub const RULE_oC_RelTypeName: usize = 48;
pub const RULE_oC_Expression: usize = 49;
pub const RULE_oC_OrExpression: usize = 50;
pub const RULE_oC_XorExpression: usize = 51;
pub const RULE_oC_AndExpression: usize = 52;
pub const RULE_oC_NotExpression: usize = 53;
pub const RULE_oC_ComparisonExpression: usize = 54;
pub const RULE_oC_AddOrSubtractExpression: usize = 55;
pub const RULE_oC_MultiplyDivideModuloExpression: usize = 56;
pub const RULE_oC_PowerOfExpression: usize = 57;
pub const RULE_oC_UnaryAddOrSubtractExpression: usize = 58;
pub const RULE_oC_StringListNullOperatorExpression: usize = 59;
pub const RULE_oC_ListOperatorExpression: usize = 60;
pub const RULE_oC_StringOperatorExpression: usize = 61;
pub const RULE_oC_NullOperatorExpression: usize = 62;
pub const RULE_oC_PropertyOrLabelsExpression: usize = 63;
pub const RULE_oC_Atom: usize = 64;
pub const RULE_oC_Literal: usize = 65;
pub const RULE_oC_BooleanLiteral: usize = 66;
pub const RULE_oC_ListLiteral: usize = 67;
pub const RULE_oC_PartialComparisonExpression: usize = 68;
pub const RULE_oC_ParenthesizedExpression: usize = 69;
pub const RULE_oC_RelationshipsPattern: usize = 70;
pub const RULE_oC_FilterExpression: usize = 71;
pub const RULE_oC_IdInColl: usize = 72;
pub const RULE_oC_FunctionInvocation: usize = 73;
pub const RULE_oC_FunctionName: usize = 74;
pub const RULE_oC_ExplicitProcedureInvocation: usize = 75;
pub const RULE_oC_ImplicitProcedureInvocation: usize = 76;
pub const RULE_oC_ProcedureResultField: usize = 77;
pub const RULE_oC_ProcedureName: usize = 78;
pub const RULE_oC_Namespace: usize = 79;
pub const RULE_oC_ListComprehension: usize = 80;
pub const RULE_oC_PatternComprehension: usize = 81;
pub const RULE_oC_PropertyLookup: usize = 82;
pub const RULE_oC_CaseExpression: usize = 83;
pub const RULE_oC_CaseAlternatives: usize = 84;
pub const RULE_oC_Variable: usize = 85;
pub const RULE_oC_NumberLiteral: usize = 86;
pub const RULE_oC_MapLiteral: usize = 87;
pub const RULE_oC_Parameter: usize = 88;
pub const RULE_oC_PropertyExpression: usize = 89;
pub const RULE_oC_PropertyKeyName: usize = 90;
pub const RULE_oC_IntegerLiteral: usize = 91;
pub const RULE_oC_DoubleLiteral: usize = 92;
pub const RULE_oC_SchemaName: usize = 93;
pub const RULE_oC_ReservedWord: usize = 94;
pub const RULE_oC_SymbolicName: usize = 95;
pub const RULE_oC_LeftArrowHead: usize = 96;
pub const RULE_oC_RightArrowHead: usize = 97;
pub const RULE_oC_Dash: usize = 98;
pub const ruleNames: [&'static str; 99] = [
    "oC_Cypher",
    "oC_Statement",
    "oC_Query",
    "oC_RegularQuery",
    "oC_Union",
    "oC_SingleQuery",
    "oC_SinglePartQuery",
    "oC_MultiPartQuery",
    "oC_UpdatingClause",
    "oC_ReadingClause",
    "oC_Match",
    "oC_Unwind",
    "oC_Merge",
    "oC_MergeAction",
    "oC_Create",
    "oC_Set",
    "oC_SetItem",
    "oC_Delete",
    "oC_Remove",
    "oC_RemoveItem",
    "oC_InQueryCall",
    "oC_StandaloneCall",
    "oC_YieldItems",
    "oC_YieldItem",
    "oC_With",
    "oC_Return",
    "oC_ProjectionBody",
    "oC_ProjectionItems",
    "oC_ProjectionItem",
    "oC_Order",
    "oC_Skip",
    "oC_Limit",
    "oC_SortItem",
    "oC_Where",
    "oC_Pattern",
    "oC_PatternPart",
    "oC_AnonymousPatternPart",
    "oC_PatternElement",
    "oC_NodePattern",
    "oC_PatternElementChain",
    "oC_RelationshipPattern",
    "oC_RelationshipDetail",
    "oC_Properties",
    "oC_RelationshipTypes",
    "oC_NodeLabels",
    "oC_NodeLabel",
    "oC_RangeLiteral",
    "oC_LabelName",
    "oC_RelTypeName",
    "oC_Expression",
    "oC_OrExpression",
    "oC_XorExpression",
    "oC_AndExpression",
    "oC_NotExpression",
    "oC_ComparisonExpression",
    "oC_AddOrSubtractExpression",
    "oC_MultiplyDivideModuloExpression",
    "oC_PowerOfExpression",
    "oC_UnaryAddOrSubtractExpression",
    "oC_StringListNullOperatorExpression",
    "oC_ListOperatorExpression",
    "oC_StringOperatorExpression",
    "oC_NullOperatorExpression",
    "oC_PropertyOrLabelsExpression",
    "oC_Atom",
    "oC_Literal",
    "oC_BooleanLiteral",
    "oC_ListLiteral",
    "oC_PartialComparisonExpression",
    "oC_ParenthesizedExpression",
    "oC_RelationshipsPattern",
    "oC_FilterExpression",
    "oC_IdInColl",
    "oC_FunctionInvocation",
    "oC_FunctionName",
    "oC_ExplicitProcedureInvocation",
    "oC_ImplicitProcedureInvocation",
    "oC_ProcedureResultField",
    "oC_ProcedureName",
    "oC_Namespace",
    "oC_ListComprehension",
    "oC_PatternComprehension",
    "oC_PropertyLookup",
    "oC_CaseExpression",
    "oC_CaseAlternatives",
    "oC_Variable",
    "oC_NumberLiteral",
    "oC_MapLiteral",
    "oC_Parameter",
    "oC_PropertyExpression",
    "oC_PropertyKeyName",
    "oC_IntegerLiteral",
    "oC_DoubleLiteral",
    "oC_SchemaName",
    "oC_ReservedWord",
    "oC_SymbolicName",
    "oC_LeftArrowHead",
    "oC_RightArrowHead",
    "oC_Dash",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 107] = [
    None,
    Some("';'"),
    Some("','"),
    Some("'='"),
    Some("'+='"),
    Some("'*'"),
    Some("'('"),
    Some("')'"),
    Some("'['"),
    Some("']'"),
    Some("':'"),
    Some("'|'"),
    Some("'..'"),
    Some("'+'"),
    Some("'-'"),
    Some("'/'"),
    Some("'%'"),
    Some("'^'"),
    Some("'<>'"),
    Some("'<'"),
    Some("'>'"),
    Some("'<='"),
    Some("'>='"),
    Some("'.'"),
    Some("'{'"),
    Some("'}'"),
    Some("'$'"),
    Some("'\u{27E8}'"),
    Some("'\u{3008}'"),
    Some("'\u{FE64}'"),
    Some("'\u{FF1C}'"),
    Some("'\u{27E9}'"),
    Some("'\u{3009}'"),
    Some("'\u{FE65}'"),
    Some("'\u{FF1E}'"),
    Some("'\u{00AD}'"),
    Some("'\u{2010}'"),
    Some("'\u{2011}'"),
    Some("'\u{2012}'"),
    Some("'\u{2013}'"),
    Some("'\u{2014}'"),
    Some("'\u{2015}'"),
    Some("'\u{2212}'"),
    Some("'\u{FE58}'"),
    Some("'\u{FE63}'"),
    Some("'\u{FF0D}'"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'0'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 128] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("UNION"),
    Some("ALL"),
    Some("OPTIONAL"),
    Some("MATCH"),
    Some("UNWIND"),
    Some("AS"),
    Some("MERGE"),
    Some("ON"),
    Some("CREATE"),
    Some("SET"),
    Some("DETACH"),
    Some("DELETE"),
    Some("REMOVE"),
    Some("CALL"),
    Some("YIELD"),
    Some("WITH"),
    Some("RETURN"),
    Some("DISTINCT"),
    Some("ORDER"),
    Some("BY"),
    Some("L_SKIP"),
    Some("LIMIT"),
    Some("ASCENDING"),
    Some("ASC"),
    Some("DESCENDING"),
    Some("DESC"),
    Some("WHERE"),
    Some("OR"),
    Some("XOR"),
    Some("AND"),
    Some("NOT"),
    Some("IN"),
    Some("STARTS"),
    Some("ENDS"),
    Some("CONTAINS"),
    Some("IS"),
    Some("NULL"),
    Some("COUNT"),
    Some("ANY"),
    Some("NONE"),
    Some("SINGLE"),
    Some("TRUE"),
    Some("FALSE"),
    Some("EXISTS"),
    Some("CASE"),
    Some("ELSE"),
    Some("END"),
    Some("WHEN"),
    Some("THEN"),
    Some("StringLiteral"),
    Some("EscapedChar"),
    Some("HexInteger"),
    Some("DecimalInteger"),
    Some("OctalInteger"),
    Some("HexLetter"),
    Some("HexDigit"),
    Some("Digit"),
    Some("NonZeroDigit"),
    Some("NonZeroOctDigit"),
    Some("OctDigit"),
    Some("ZeroDigit"),
    Some("ExponentDecimalReal"),
    Some("RegularDecimalReal"),
    Some("CONSTRAINT"),
    Some("DO"),
    Some("FOR"),
    Some("REQUIRE"),
    Some("UNIQUE"),
    Some("MANDATORY"),
    Some("SCALAR"),
    Some("OF"),
    Some("ADD"),
    Some("DROP"),
    Some("FILTER"),
    Some("EXTRACT"),
    Some("UnescapedSymbolicName"),
    Some("IdentifierStart"),
    Some("IdentifierPart"),
    Some("EscapedSymbolicName"),
    Some("SP"),
    Some("WHITESPACE"),
    Some("Comment"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    CypherParserExt,
    I,
    CypherParserContextType,
    dyn CypherListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type CypherTreeWalker<'input, 'a> =
    ParseTreeWalker<'input, 'a, CypherParserContextType, dyn CypherListener<'input> + 'a>;

/// Parser for Cypher grammar
pub struct CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "2");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(input, Arc::clone(&interpreter), CypherParserExt {}),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> CypherParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> CypherParser<'input, I, DefaultErrorStrategy<'input, CypherParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for CypherParser
pub trait CypherParserContext<'input>:
    for<'x> Listenable<dyn CypherListener<'input> + 'x>
    + for<'x> Visitable<dyn CypherVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = CypherParserContextType>
{
}

impl<'input, 'x, T> VisitableDyn<T> for dyn CypherParserContext<'input> + 'input
where
    T: CypherVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn CypherVisitor<'input> + 'x))
    }
}

impl<'input> CypherParserContext<'input> for TerminalNode<'input, CypherParserContextType> {}
impl<'input> CypherParserContext<'input> for ErrorNode<'input, CypherParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn CypherParserContext<'input> + 'input {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn CypherListener<'input> + 'input {}

pub struct CypherParserContextType;
antlr_rust::type_id! {CypherParserContextType}

impl<'input> ParserNodeType<'input> for CypherParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn CypherParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct CypherParserExt {}

impl CypherParserExt {}

impl<'input> TokenAware<'input> for CypherParserExt {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for CypherParserExt
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for CypherParserExt
{
    fn get_grammar_file_name(&self) -> &str {
        "Cypher.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
}
//------------------- oC_Cypher ----------------
pub type OC_CypherContextAll<'input> = OC_CypherContext<'input>;

pub type OC_CypherContext<'input> = BaseParserRuleContext<'input, OC_CypherContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CypherContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_CypherContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CypherContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Cypher(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Cypher(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_CypherContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Cypher(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_CypherContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Cypher
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Cypher }
}
antlr_rust::type_id! {OC_CypherContextExt<'a>}

impl<'input> OC_CypherContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_CypherContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_CypherContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_CypherContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_CypherContextExt<'input>>
{
    fn oC_Statement(&self) -> Option<Rc<OC_StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EOF, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_CypherContextAttrs<'input> for OC_CypherContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Cypher(&mut self) -> Result<Rc<OC_CypherContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_CypherContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_oC_Cypher);
        let mut _localctx: Rc<OC_CypherContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(199);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(198);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Statement*/
                recog.base.set_state(201);
                recog.oC_Statement()?;

                recog.base.set_state(206);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(2, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(203);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(202);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(205);
                        recog.base.match_token(T__0, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(209);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(208);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(211);
                recog.base.match_token(EOF, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Statement ----------------
pub type OC_StatementContextAll<'input> = OC_StatementContext<'input>;

pub type OC_StatementContext<'input> =
    BaseParserRuleContext<'input, OC_StatementContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_StatementContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_StatementContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Statement(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_StatementContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_StatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Statement }
}
antlr_rust::type_id! {OC_StatementContextExt<'a>}

impl<'input> OC_StatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_StatementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_StatementContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_StatementContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_StatementContextExt<'input>>
{
    fn oC_Query(&self) -> Option<Rc<OC_QueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_StatementContextAttrs<'input> for OC_StatementContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Statement(&mut self) -> Result<Rc<OC_StatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 2, RULE_oC_Statement);
        let mut _localctx: Rc<OC_StatementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_Query*/
                recog.base.set_state(213);
                recog.oC_Query()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Query ----------------
pub type OC_QueryContextAll<'input> = OC_QueryContext<'input>;

pub type OC_QueryContext<'input> = BaseParserRuleContext<'input, OC_QueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_QueryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_QueryContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_QueryContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Query(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Query(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_QueryContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Query(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_QueryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Query
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Query }
}
antlr_rust::type_id! {OC_QueryContextExt<'a>}

impl<'input> OC_QueryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_QueryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_QueryContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_QueryContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_QueryContextExt<'input>>
{
    fn oC_RegularQuery(&self) -> Option<Rc<OC_RegularQueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_StandaloneCall(&self) -> Option<Rc<OC_StandaloneCallContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_QueryContextAttrs<'input> for OC_QueryContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Query(&mut self) -> Result<Rc<OC_QueryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_QueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_oC_Query);
        let mut _localctx: Rc<OC_QueryContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(217);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(4, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_RegularQuery*/
                        recog.base.set_state(215);
                        recog.oC_RegularQuery()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_StandaloneCall*/
                        recog.base.set_state(216);
                        recog.oC_StandaloneCall()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RegularQuery ----------------
pub type OC_RegularQueryContextAll<'input> = OC_RegularQueryContext<'input>;

pub type OC_RegularQueryContext<'input> =
    BaseParserRuleContext<'input, OC_RegularQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RegularQueryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RegularQueryContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RegularQueryContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RegularQuery(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RegularQuery(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RegularQueryContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RegularQuery(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RegularQueryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RegularQuery
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RegularQuery }
}
antlr_rust::type_id! {OC_RegularQueryContextExt<'a>}

impl<'input> OC_RegularQueryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RegularQueryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RegularQueryContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RegularQueryContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RegularQueryContextExt<'input>>
{
    fn oC_SingleQuery(&self) -> Option<Rc<OC_SingleQueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Union_all(&self) -> Vec<Rc<OC_UnionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Union(&self, i: usize) -> Option<Rc<OC_UnionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_RegularQueryContextAttrs<'input> for OC_RegularQueryContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RegularQuery(&mut self) -> Result<Rc<OC_RegularQueryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RegularQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 6, RULE_oC_RegularQuery);
        let mut _localctx: Rc<OC_RegularQueryContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_SingleQuery*/
                recog.base.set_state(219);
                recog.oC_SingleQuery()?;

                recog.base.set_state(226);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(6, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(221);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(220);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_Union*/
                                recog.base.set_state(223);
                                recog.oC_Union()?;
                            }
                        }
                    }
                    recog.base.set_state(228);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(6, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Union ----------------
pub type OC_UnionContextAll<'input> = OC_UnionContext<'input>;

pub type OC_UnionContext<'input> = BaseParserRuleContext<'input, OC_UnionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UnionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_UnionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UnionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Union(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Union(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_UnionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Union(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_UnionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Union
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Union }
}
antlr_rust::type_id! {OC_UnionContextExt<'a>}

impl<'input> OC_UnionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_UnionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_UnionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_UnionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_UnionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UNION
    /// Returns `None` if there is no child corresponding to token UNION
    fn UNION(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNION, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token ALL
    /// Returns `None` if there is no child corresponding to token ALL
    fn ALL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ALL, 0)
    }
    fn oC_SingleQuery(&self) -> Option<Rc<OC_SingleQueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_UnionContextAttrs<'input> for OC_UnionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Union(&mut self) -> Result<Rc<OC_UnionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_UnionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_oC_Union);
        let mut _localctx: Rc<OC_UnionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(241);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(9, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(229);
                            recog.base.match_token(UNION, &mut recog.err_handler)?;

                            recog.base.set_state(230);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(231);
                            recog.base.match_token(ALL, &mut recog.err_handler)?;

                            recog.base.set_state(233);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(232);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_SingleQuery*/
                            recog.base.set_state(235);
                            recog.oC_SingleQuery()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(236);
                            recog.base.match_token(UNION, &mut recog.err_handler)?;

                            recog.base.set_state(238);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(237);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_SingleQuery*/
                            recog.base.set_state(240);
                            recog.oC_SingleQuery()?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_SingleQuery ----------------
pub type OC_SingleQueryContextAll<'input> = OC_SingleQueryContext<'input>;

pub type OC_SingleQueryContext<'input> =
    BaseParserRuleContext<'input, OC_SingleQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SingleQueryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SingleQueryContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SingleQueryContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_SingleQuery(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_SingleQuery(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SingleQueryContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_SingleQuery(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SingleQueryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_SingleQuery
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_SingleQuery }
}
antlr_rust::type_id! {OC_SingleQueryContextExt<'a>}

impl<'input> OC_SingleQueryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SingleQueryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SingleQueryContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SingleQueryContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SingleQueryContextExt<'input>>
{
    fn oC_SinglePartQuery(&self) -> Option<Rc<OC_SinglePartQueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_MultiPartQuery(&self) -> Option<Rc<OC_MultiPartQueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_SingleQueryContextAttrs<'input> for OC_SingleQueryContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_SingleQuery(&mut self) -> Result<Rc<OC_SingleQueryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_SingleQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 10, RULE_oC_SingleQuery);
        let mut _localctx: Rc<OC_SingleQueryContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(245);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(10, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_SinglePartQuery*/
                        recog.base.set_state(243);
                        recog.oC_SinglePartQuery()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_MultiPartQuery*/
                        recog.base.set_state(244);
                        recog.oC_MultiPartQuery()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_SinglePartQuery ----------------
pub type OC_SinglePartQueryContextAll<'input> = OC_SinglePartQueryContext<'input>;

pub type OC_SinglePartQueryContext<'input> =
    BaseParserRuleContext<'input, OC_SinglePartQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SinglePartQueryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SinglePartQueryContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SinglePartQueryContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_SinglePartQuery(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_SinglePartQuery(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SinglePartQueryContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_SinglePartQuery(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SinglePartQueryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_SinglePartQuery
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_SinglePartQuery }
}
antlr_rust::type_id! {OC_SinglePartQueryContextExt<'a>}

impl<'input> OC_SinglePartQueryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SinglePartQueryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SinglePartQueryContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SinglePartQueryContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SinglePartQueryContextExt<'input>>
{
    fn oC_Return(&self) -> Option<Rc<OC_ReturnContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_ReadingClause_all(&self) -> Vec<Rc<OC_ReadingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_ReadingClause(&self, i: usize) -> Option<Rc<OC_ReadingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_UpdatingClause_all(&self) -> Vec<Rc<OC_UpdatingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_UpdatingClause(&self, i: usize) -> Option<Rc<OC_UpdatingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_SinglePartQueryContextAttrs<'input> for OC_SinglePartQueryContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_SinglePartQuery(
        &mut self,
    ) -> Result<Rc<OC_SinglePartQueryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_SinglePartQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_oC_SinglePartQuery);
        let mut _localctx: Rc<OC_SinglePartQueryContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            recog.base.set_state(282);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(19, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(253);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while (((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << OPTIONAL)
                                        | (1usize << MATCH)
                                        | (1usize << UNWIND)
                                        | (1usize << CALL)))
                                    != 0)
                            {
                                {
                                    {
                                        /*InvokeRule oC_ReadingClause*/
                                        recog.base.set_state(247);
                                        recog.oC_ReadingClause()?;

                                        recog.base.set_state(249);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        if _la == SP {
                                            {
                                                recog.base.set_state(248);
                                                recog
                                                    .base
                                                    .match_token(SP, &mut recog.err_handler)?;
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(255);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            /*InvokeRule oC_Return*/
                            recog.base.set_state(256);
                            recog.oC_Return()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(263);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while (((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << OPTIONAL)
                                        | (1usize << MATCH)
                                        | (1usize << UNWIND)
                                        | (1usize << CALL)))
                                    != 0)
                            {
                                {
                                    {
                                        /*InvokeRule oC_ReadingClause*/
                                        recog.base.set_state(257);
                                        recog.oC_ReadingClause()?;

                                        recog.base.set_state(259);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        if _la == SP {
                                            {
                                                recog.base.set_state(258);
                                                recog
                                                    .base
                                                    .match_token(SP, &mut recog.err_handler)?;
                                            }
                                        }
                                    }
                                }
                                recog.base.set_state(265);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            /*InvokeRule oC_UpdatingClause*/
                            recog.base.set_state(266);
                            recog.oC_UpdatingClause()?;

                            recog.base.set_state(273);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(16, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(268);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(267);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_UpdatingClause*/
                                            recog.base.set_state(270);
                                            recog.oC_UpdatingClause()?;
                                        }
                                    }
                                }
                                recog.base.set_state(275);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(16, &mut recog.base)?;
                            }
                            recog.base.set_state(280);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(18, &mut recog.base)? {
                                x if x == 1 => {
                                    {
                                        recog.base.set_state(277);
                                        recog.err_handler.sync(&mut recog.base)?;
                                        _la = recog.base.input.la(1);
                                        if _la == SP {
                                            {
                                                recog.base.set_state(276);
                                                recog
                                                    .base
                                                    .match_token(SP, &mut recog.err_handler)?;
                                            }
                                        }

                                        /*InvokeRule oC_Return*/
                                        recog.base.set_state(279);
                                        recog.oC_Return()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_MultiPartQuery ----------------
pub type OC_MultiPartQueryContextAll<'input> = OC_MultiPartQueryContext<'input>;

pub type OC_MultiPartQueryContext<'input> =
    BaseParserRuleContext<'input, OC_MultiPartQueryContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MultiPartQueryContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_MultiPartQueryContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MultiPartQueryContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_MultiPartQuery(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_MultiPartQuery(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_MultiPartQueryContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_MultiPartQuery(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_MultiPartQueryContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_MultiPartQuery
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_MultiPartQuery }
}
antlr_rust::type_id! {OC_MultiPartQueryContextExt<'a>}

impl<'input> OC_MultiPartQueryContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_MultiPartQueryContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_MultiPartQueryContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_MultiPartQueryContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_MultiPartQueryContextExt<'input>>
{
    fn oC_SinglePartQuery(&self) -> Option<Rc<OC_SinglePartQueryContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_With_all(&self) -> Vec<Rc<OC_WithContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_With(&self, i: usize) -> Option<Rc<OC_WithContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_ReadingClause_all(&self) -> Vec<Rc<OC_ReadingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_ReadingClause(&self, i: usize) -> Option<Rc<OC_ReadingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_UpdatingClause_all(&self) -> Vec<Rc<OC_UpdatingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_UpdatingClause(&self, i: usize) -> Option<Rc<OC_UpdatingClauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_MultiPartQueryContextAttrs<'input> for OC_MultiPartQueryContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_MultiPartQuery(
        &mut self,
    ) -> Result<Rc<OC_MultiPartQueryContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_MultiPartQueryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_oC_MultiPartQuery);
        let mut _localctx: Rc<OC_MultiPartQueryContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(306);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1;
                loop {
                    match _alt {
                        x if x == 1 => {
                            {
                                recog.base.set_state(290);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while (((_la) & !0x3f) == 0
                                    && ((1usize << _la)
                                        & ((1usize << OPTIONAL)
                                            | (1usize << MATCH)
                                            | (1usize << UNWIND)
                                            | (1usize << CALL)))
                                        != 0)
                                {
                                    {
                                        {
                                            /*InvokeRule oC_ReadingClause*/
                                            recog.base.set_state(284);
                                            recog.oC_ReadingClause()?;

                                            recog.base.set_state(286);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(285);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }
                                        }
                                    }
                                    recog.base.set_state(292);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                                recog.base.set_state(299);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                while (((_la) & !0x3f) == 0
                                    && ((1usize << _la)
                                        & ((1usize << MERGE)
                                            | (1usize << CREATE)
                                            | (1usize << SET)
                                            | (1usize << DETACH)
                                            | (1usize << DELETE)
                                            | (1usize << REMOVE)))
                                        != 0)
                                {
                                    {
                                        {
                                            /*InvokeRule oC_UpdatingClause*/
                                            recog.base.set_state(293);
                                            recog.oC_UpdatingClause()?;

                                            recog.base.set_state(295);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(294);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }
                                        }
                                    }
                                    recog.base.set_state(301);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                }
                                /*InvokeRule oC_With*/
                                recog.base.set_state(302);
                                recog.oC_With()?;

                                recog.base.set_state(304);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(303);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(308);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(25, &mut recog.base)?;
                    if _alt == 2 || _alt == INVALID_ALT {
                        break;
                    }
                }
                /*InvokeRule oC_SinglePartQuery*/
                recog.base.set_state(310);
                recog.oC_SinglePartQuery()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_UpdatingClause ----------------
pub type OC_UpdatingClauseContextAll<'input> = OC_UpdatingClauseContext<'input>;

pub type OC_UpdatingClauseContext<'input> =
    BaseParserRuleContext<'input, OC_UpdatingClauseContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UpdatingClauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_UpdatingClauseContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UpdatingClauseContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_UpdatingClause(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_UpdatingClause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_UpdatingClauseContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_UpdatingClause(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_UpdatingClauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_UpdatingClause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_UpdatingClause }
}
antlr_rust::type_id! {OC_UpdatingClauseContextExt<'a>}

impl<'input> OC_UpdatingClauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_UpdatingClauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_UpdatingClauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_UpdatingClauseContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_UpdatingClauseContextExt<'input>>
{
    fn oC_Create(&self) -> Option<Rc<OC_CreateContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Merge(&self) -> Option<Rc<OC_MergeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Delete(&self) -> Option<Rc<OC_DeleteContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Set(&self) -> Option<Rc<OC_SetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Remove(&self) -> Option<Rc<OC_RemoveContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_UpdatingClauseContextAttrs<'input> for OC_UpdatingClauseContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_UpdatingClause(
        &mut self,
    ) -> Result<Rc<OC_UpdatingClauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_UpdatingClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 16, RULE_oC_UpdatingClause);
        let mut _localctx: Rc<OC_UpdatingClauseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(317);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                CREATE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_Create*/
                        recog.base.set_state(312);
                        recog.oC_Create()?;
                    }
                }

                MERGE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_Merge*/
                        recog.base.set_state(313);
                        recog.oC_Merge()?;
                    }
                }

                DETACH | DELETE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule oC_Delete*/
                        recog.base.set_state(314);
                        recog.oC_Delete()?;
                    }
                }

                SET => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule oC_Set*/
                        recog.base.set_state(315);
                        recog.oC_Set()?;
                    }
                }

                REMOVE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule oC_Remove*/
                        recog.base.set_state(316);
                        recog.oC_Remove()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ReadingClause ----------------
pub type OC_ReadingClauseContextAll<'input> = OC_ReadingClauseContext<'input>;

pub type OC_ReadingClauseContext<'input> =
    BaseParserRuleContext<'input, OC_ReadingClauseContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ReadingClauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ReadingClauseContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ReadingClauseContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ReadingClause(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ReadingClause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ReadingClauseContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ReadingClause(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ReadingClauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ReadingClause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ReadingClause }
}
antlr_rust::type_id! {OC_ReadingClauseContextExt<'a>}

impl<'input> OC_ReadingClauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ReadingClauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ReadingClauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ReadingClauseContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ReadingClauseContextExt<'input>>
{
    fn oC_Match(&self) -> Option<Rc<OC_MatchContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Unwind(&self) -> Option<Rc<OC_UnwindContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_InQueryCall(&self) -> Option<Rc<OC_InQueryCallContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ReadingClauseContextAttrs<'input> for OC_ReadingClauseContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ReadingClause(
        &mut self,
    ) -> Result<Rc<OC_ReadingClauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ReadingClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_oC_ReadingClause);
        let mut _localctx: Rc<OC_ReadingClauseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(322);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                OPTIONAL | MATCH => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_Match*/
                        recog.base.set_state(319);
                        recog.oC_Match()?;
                    }
                }

                UNWIND => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_Unwind*/
                        recog.base.set_state(320);
                        recog.oC_Unwind()?;
                    }
                }

                CALL => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule oC_InQueryCall*/
                        recog.base.set_state(321);
                        recog.oC_InQueryCall()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Match ----------------
pub type OC_MatchContextAll<'input> = OC_MatchContext<'input>;

pub type OC_MatchContext<'input> = BaseParserRuleContext<'input, OC_MatchContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MatchContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_MatchContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MatchContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Match(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Match(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_MatchContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Match(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_MatchContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Match
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Match }
}
antlr_rust::type_id! {OC_MatchContextExt<'a>}

impl<'input> OC_MatchContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_MatchContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_MatchContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_MatchContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_MatchContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token MATCH
    /// Returns `None` if there is no child corresponding to token MATCH
    fn MATCH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH, 0)
    }
    fn oC_Pattern(&self) -> Option<Rc<OC_PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPTIONAL
    /// Returns `None` if there is no child corresponding to token OPTIONAL
    fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPTIONAL, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_MatchContextAttrs<'input> for OC_MatchContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Match(&mut self) -> Result<Rc<OC_MatchContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_MatchContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_oC_Match);
        let mut _localctx: Rc<OC_MatchContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(326);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == OPTIONAL {
                    {
                        recog.base.set_state(324);
                        recog.base.match_token(OPTIONAL, &mut recog.err_handler)?;

                        recog.base.set_state(325);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(328);
                recog.base.match_token(MATCH, &mut recog.err_handler)?;

                recog.base.set_state(330);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(329);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Pattern*/
                recog.base.set_state(332);
                recog.oC_Pattern()?;

                recog.base.set_state(337);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(31, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(334);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(333);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Where*/
                            recog.base.set_state(336);
                            recog.oC_Where()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Unwind ----------------
pub type OC_UnwindContextAll<'input> = OC_UnwindContext<'input>;

pub type OC_UnwindContext<'input> = BaseParserRuleContext<'input, OC_UnwindContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UnwindContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_UnwindContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_UnwindContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Unwind(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Unwind(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_UnwindContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Unwind(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_UnwindContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Unwind
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Unwind }
}
antlr_rust::type_id! {OC_UnwindContextExt<'a>}

impl<'input> OC_UnwindContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_UnwindContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_UnwindContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_UnwindContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_UnwindContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UNWIND
    /// Returns `None` if there is no child corresponding to token UNWIND
    fn UNWIND(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNWIND, 0)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_UnwindContextAttrs<'input> for OC_UnwindContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Unwind(&mut self) -> Result<Rc<OC_UnwindContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_UnwindContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_oC_Unwind);
        let mut _localctx: Rc<OC_UnwindContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(339);
                recog.base.match_token(UNWIND, &mut recog.err_handler)?;

                recog.base.set_state(341);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(340);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Expression*/
                recog.base.set_state(343);
                recog.oC_Expression()?;

                recog.base.set_state(344);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                recog.base.set_state(345);
                recog.base.match_token(AS, &mut recog.err_handler)?;

                recog.base.set_state(346);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_Variable*/
                recog.base.set_state(347);
                recog.oC_Variable()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Merge ----------------
pub type OC_MergeContextAll<'input> = OC_MergeContext<'input>;

pub type OC_MergeContext<'input> = BaseParserRuleContext<'input, OC_MergeContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MergeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_MergeContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MergeContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Merge(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Merge(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_MergeContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Merge(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_MergeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Merge
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Merge }
}
antlr_rust::type_id! {OC_MergeContextExt<'a>}

impl<'input> OC_MergeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_MergeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_MergeContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_MergeContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_MergeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token MERGE
    /// Returns `None` if there is no child corresponding to token MERGE
    fn MERGE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MERGE, 0)
    }
    fn oC_PatternPart(&self) -> Option<Rc<OC_PatternPartContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_MergeAction_all(&self) -> Vec<Rc<OC_MergeActionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_MergeAction(&self, i: usize) -> Option<Rc<OC_MergeActionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_MergeContextAttrs<'input> for OC_MergeContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Merge(&mut self) -> Result<Rc<OC_MergeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_MergeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_oC_Merge);
        let mut _localctx: Rc<OC_MergeContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(349);
                recog.base.match_token(MERGE, &mut recog.err_handler)?;

                recog.base.set_state(351);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(350);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_PatternPart*/
                recog.base.set_state(353);
                recog.oC_PatternPart()?;

                recog.base.set_state(358);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(34, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(354);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                /*InvokeRule oC_MergeAction*/
                                recog.base.set_state(355);
                                recog.oC_MergeAction()?;
                            }
                        }
                    }
                    recog.base.set_state(360);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(34, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_MergeAction ----------------
pub type OC_MergeActionContextAll<'input> = OC_MergeActionContext<'input>;

pub type OC_MergeActionContext<'input> =
    BaseParserRuleContext<'input, OC_MergeActionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MergeActionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_MergeActionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MergeActionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_MergeAction(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_MergeAction(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_MergeActionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_MergeAction(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_MergeActionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_MergeAction
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_MergeAction }
}
antlr_rust::type_id! {OC_MergeActionContextExt<'a>}

impl<'input> OC_MergeActionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_MergeActionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_MergeActionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_MergeActionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_MergeActionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ON
    /// Returns `None` if there is no child corresponding to token ON
    fn ON(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ON, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token MATCH
    /// Returns `None` if there is no child corresponding to token MATCH
    fn MATCH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH, 0)
    }
    fn oC_Set(&self) -> Option<Rc<OC_SetContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token CREATE
    /// Returns `None` if there is no child corresponding to token CREATE
    fn CREATE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CREATE, 0)
    }
}

impl<'input> OC_MergeActionContextAttrs<'input> for OC_MergeActionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_MergeAction(&mut self) -> Result<Rc<OC_MergeActionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_MergeActionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_oC_MergeAction);
        let mut _localctx: Rc<OC_MergeActionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(371);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(35, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(361);
                            recog.base.match_token(ON, &mut recog.err_handler)?;

                            recog.base.set_state(362);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(363);
                            recog.base.match_token(MATCH, &mut recog.err_handler)?;

                            recog.base.set_state(364);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_Set*/
                            recog.base.set_state(365);
                            recog.oC_Set()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(366);
                            recog.base.match_token(ON, &mut recog.err_handler)?;

                            recog.base.set_state(367);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(368);
                            recog.base.match_token(CREATE, &mut recog.err_handler)?;

                            recog.base.set_state(369);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_Set*/
                            recog.base.set_state(370);
                            recog.oC_Set()?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Create ----------------
pub type OC_CreateContextAll<'input> = OC_CreateContext<'input>;

pub type OC_CreateContext<'input> = BaseParserRuleContext<'input, OC_CreateContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CreateContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_CreateContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CreateContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Create(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Create(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_CreateContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Create(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_CreateContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Create
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Create }
}
antlr_rust::type_id! {OC_CreateContextExt<'a>}

impl<'input> OC_CreateContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_CreateContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_CreateContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_CreateContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_CreateContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CREATE
    /// Returns `None` if there is no child corresponding to token CREATE
    fn CREATE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CREATE, 0)
    }
    fn oC_Pattern(&self) -> Option<Rc<OC_PatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_CreateContextAttrs<'input> for OC_CreateContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Create(&mut self) -> Result<Rc<OC_CreateContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_CreateContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_oC_Create);
        let mut _localctx: Rc<OC_CreateContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(373);
                recog.base.match_token(CREATE, &mut recog.err_handler)?;

                recog.base.set_state(375);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(374);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Pattern*/
                recog.base.set_state(377);
                recog.oC_Pattern()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Set ----------------
pub type OC_SetContextAll<'input> = OC_SetContext<'input>;

pub type OC_SetContext<'input> = BaseParserRuleContext<'input, OC_SetContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SetContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SetContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SetContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Set(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Set(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SetContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Set(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SetContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Set
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Set }
}
antlr_rust::type_id! {OC_SetContextExt<'a>}

impl<'input> OC_SetContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SetContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SetContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SetContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SetContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SET
    /// Returns `None` if there is no child corresponding to token SET
    fn SET(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SET, 0)
    }
    fn oC_SetItem_all(&self) -> Vec<Rc<OC_SetItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_SetItem(&self, i: usize) -> Option<Rc<OC_SetItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_SetContextAttrs<'input> for OC_SetContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Set(&mut self) -> Result<Rc<OC_SetContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_SetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_oC_Set);
        let mut _localctx: Rc<OC_SetContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(379);
                recog.base.match_token(SET, &mut recog.err_handler)?;

                recog.base.set_state(381);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(380);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_SetItem*/
                recog.base.set_state(383);
                recog.oC_SetItem()?;

                recog.base.set_state(388);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__1 {
                    {
                        {
                            recog.base.set_state(384);
                            recog.base.match_token(T__1, &mut recog.err_handler)?;

                            /*InvokeRule oC_SetItem*/
                            recog.base.set_state(385);
                            recog.oC_SetItem()?;
                        }
                    }
                    recog.base.set_state(390);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_SetItem ----------------
pub type OC_SetItemContextAll<'input> = OC_SetItemContext<'input>;

pub type OC_SetItemContext<'input> = BaseParserRuleContext<'input, OC_SetItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SetItemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SetItemContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SetItemContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_SetItem(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_SetItem(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SetItemContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_SetItem(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SetItemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_SetItem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_SetItem }
}
antlr_rust::type_id! {OC_SetItemContextExt<'a>}

impl<'input> OC_SetItemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SetItemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SetItemContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SetItemContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SetItemContextExt<'input>>
{
    fn oC_PropertyExpression(&self) -> Option<Rc<OC_PropertyExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_SetItemContextAttrs<'input> for OC_SetItemContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_SetItem(&mut self) -> Result<Rc<OC_SetItemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_SetItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 32, RULE_oC_SetItem);
        let mut _localctx: Rc<OC_SetItemContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(427);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(46, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_PropertyExpression*/
                            recog.base.set_state(391);
                            recog.oC_PropertyExpression()?;

                            recog.base.set_state(393);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(392);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(395);
                            recog.base.match_token(T__2, &mut recog.err_handler)?;

                            recog.base.set_state(397);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(396);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(399);
                            recog.oC_Expression()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            /*InvokeRule oC_Variable*/
                            recog.base.set_state(401);
                            recog.oC_Variable()?;

                            recog.base.set_state(403);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(402);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(405);
                            recog.base.match_token(T__2, &mut recog.err_handler)?;

                            recog.base.set_state(407);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(406);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(409);
                            recog.oC_Expression()?;
                        }
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        {
                            /*InvokeRule oC_Variable*/
                            recog.base.set_state(411);
                            recog.oC_Variable()?;

                            recog.base.set_state(413);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(412);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(415);
                            recog.base.match_token(T__3, &mut recog.err_handler)?;

                            recog.base.set_state(417);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(416);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(419);
                            recog.oC_Expression()?;
                        }
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        {
                            /*InvokeRule oC_Variable*/
                            recog.base.set_state(421);
                            recog.oC_Variable()?;

                            recog.base.set_state(423);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(422);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_NodeLabels*/
                            recog.base.set_state(425);
                            recog.oC_NodeLabels()?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Delete ----------------
pub type OC_DeleteContextAll<'input> = OC_DeleteContext<'input>;

pub type OC_DeleteContext<'input> = BaseParserRuleContext<'input, OC_DeleteContextExt<'input>>;

#[derive(Clone)]
pub struct OC_DeleteContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_DeleteContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_DeleteContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Delete(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Delete(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_DeleteContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Delete(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_DeleteContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Delete
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Delete }
}
antlr_rust::type_id! {OC_DeleteContextExt<'a>}

impl<'input> OC_DeleteContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_DeleteContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_DeleteContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_DeleteContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_DeleteContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token DELETE
    /// Returns `None` if there is no child corresponding to token DELETE
    fn DELETE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DELETE, 0)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token DETACH
    /// Returns `None` if there is no child corresponding to token DETACH
    fn DETACH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DETACH, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_DeleteContextAttrs<'input> for OC_DeleteContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Delete(&mut self) -> Result<Rc<OC_DeleteContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_DeleteContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_oC_Delete);
        let mut _localctx: Rc<OC_DeleteContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(431);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == DETACH {
                    {
                        recog.base.set_state(429);
                        recog.base.match_token(DETACH, &mut recog.err_handler)?;

                        recog.base.set_state(430);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(433);
                recog.base.match_token(DELETE, &mut recog.err_handler)?;

                recog.base.set_state(435);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(434);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Expression*/
                recog.base.set_state(437);
                recog.oC_Expression()?;

                recog.base.set_state(448);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(51, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(439);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(438);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                recog.base.set_state(441);
                                recog.base.match_token(T__1, &mut recog.err_handler)?;

                                recog.base.set_state(443);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(442);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_Expression*/
                                recog.base.set_state(445);
                                recog.oC_Expression()?;
                            }
                        }
                    }
                    recog.base.set_state(450);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(51, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Remove ----------------
pub type OC_RemoveContextAll<'input> = OC_RemoveContext<'input>;

pub type OC_RemoveContext<'input> = BaseParserRuleContext<'input, OC_RemoveContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RemoveContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RemoveContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RemoveContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Remove(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Remove(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RemoveContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Remove(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RemoveContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Remove
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Remove }
}
antlr_rust::type_id! {OC_RemoveContextExt<'a>}

impl<'input> OC_RemoveContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RemoveContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RemoveContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RemoveContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RemoveContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token REMOVE
    /// Returns `None` if there is no child corresponding to token REMOVE
    fn REMOVE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REMOVE, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_RemoveItem_all(&self) -> Vec<Rc<OC_RemoveItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_RemoveItem(&self, i: usize) -> Option<Rc<OC_RemoveItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_RemoveContextAttrs<'input> for OC_RemoveContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Remove(&mut self) -> Result<Rc<OC_RemoveContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_RemoveContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_oC_Remove);
        let mut _localctx: Rc<OC_RemoveContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(451);
                recog.base.match_token(REMOVE, &mut recog.err_handler)?;

                recog.base.set_state(452);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_RemoveItem*/
                recog.base.set_state(453);
                recog.oC_RemoveItem()?;

                recog.base.set_state(464);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(54, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(455);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(454);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                recog.base.set_state(457);
                                recog.base.match_token(T__1, &mut recog.err_handler)?;

                                recog.base.set_state(459);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(458);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_RemoveItem*/
                                recog.base.set_state(461);
                                recog.oC_RemoveItem()?;
                            }
                        }
                    }
                    recog.base.set_state(466);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(54, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RemoveItem ----------------
pub type OC_RemoveItemContextAll<'input> = OC_RemoveItemContext<'input>;

pub type OC_RemoveItemContext<'input> =
    BaseParserRuleContext<'input, OC_RemoveItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RemoveItemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RemoveItemContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RemoveItemContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RemoveItem(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RemoveItem(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RemoveItemContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RemoveItem(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RemoveItemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RemoveItem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RemoveItem }
}
antlr_rust::type_id! {OC_RemoveItemContextExt<'a>}

impl<'input> OC_RemoveItemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RemoveItemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RemoveItemContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RemoveItemContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RemoveItemContextExt<'input>>
{
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PropertyExpression(&self) -> Option<Rc<OC_PropertyExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_RemoveItemContextAttrs<'input> for OC_RemoveItemContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RemoveItem(&mut self) -> Result<Rc<OC_RemoveItemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RemoveItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 38, RULE_oC_RemoveItem);
        let mut _localctx: Rc<OC_RemoveItemContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(471);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(55, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_Variable*/
                            recog.base.set_state(467);
                            recog.oC_Variable()?;

                            /*InvokeRule oC_NodeLabels*/
                            recog.base.set_state(468);
                            recog.oC_NodeLabels()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_PropertyExpression*/
                        recog.base.set_state(470);
                        recog.oC_PropertyExpression()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_InQueryCall ----------------
pub type OC_InQueryCallContextAll<'input> = OC_InQueryCallContext<'input>;

pub type OC_InQueryCallContext<'input> =
    BaseParserRuleContext<'input, OC_InQueryCallContextExt<'input>>;

#[derive(Clone)]
pub struct OC_InQueryCallContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_InQueryCallContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_InQueryCallContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_InQueryCall(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_InQueryCall(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_InQueryCallContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_InQueryCall(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_InQueryCallContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_InQueryCall
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_InQueryCall }
}
antlr_rust::type_id! {OC_InQueryCallContextExt<'a>}

impl<'input> OC_InQueryCallContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_InQueryCallContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_InQueryCallContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_InQueryCallContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_InQueryCallContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CALL
    /// Returns `None` if there is no child corresponding to token CALL
    fn CALL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CALL, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_ExplicitProcedureInvocation(
        &self,
    ) -> Option<Rc<OC_ExplicitProcedureInvocationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token YIELD
    /// Returns `None` if there is no child corresponding to token YIELD
    fn YIELD(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(YIELD, 0)
    }
    fn oC_YieldItems(&self) -> Option<Rc<OC_YieldItemsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_InQueryCallContextAttrs<'input> for OC_InQueryCallContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_InQueryCall(&mut self) -> Result<Rc<OC_InQueryCallContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_InQueryCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_oC_InQueryCall);
        let mut _localctx: Rc<OC_InQueryCallContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(473);
                recog.base.match_token(CALL, &mut recog.err_handler)?;

                recog.base.set_state(474);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_ExplicitProcedureInvocation*/
                recog.base.set_state(475);
                recog.oC_ExplicitProcedureInvocation()?;

                recog.base.set_state(482);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(57, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(477);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(476);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(479);
                            recog.base.match_token(YIELD, &mut recog.err_handler)?;

                            recog.base.set_state(480);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_YieldItems*/
                            recog.base.set_state(481);
                            recog.oC_YieldItems()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_StandaloneCall ----------------
pub type OC_StandaloneCallContextAll<'input> = OC_StandaloneCallContext<'input>;

pub type OC_StandaloneCallContext<'input> =
    BaseParserRuleContext<'input, OC_StandaloneCallContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StandaloneCallContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_StandaloneCallContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_StandaloneCallContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_StandaloneCall(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_StandaloneCall(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_StandaloneCallContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_StandaloneCall(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_StandaloneCallContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_StandaloneCall
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_StandaloneCall }
}
antlr_rust::type_id! {OC_StandaloneCallContextExt<'a>}

impl<'input> OC_StandaloneCallContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_StandaloneCallContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_StandaloneCallContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_StandaloneCallContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_StandaloneCallContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token CALL
    /// Returns `None` if there is no child corresponding to token CALL
    fn CALL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CALL, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_ExplicitProcedureInvocation(
        &self,
    ) -> Option<Rc<OC_ExplicitProcedureInvocationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_ImplicitProcedureInvocation(
        &self,
    ) -> Option<Rc<OC_ImplicitProcedureInvocationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token YIELD
    /// Returns `None` if there is no child corresponding to token YIELD
    fn YIELD(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(YIELD, 0)
    }
    fn oC_YieldItems(&self) -> Option<Rc<OC_YieldItemsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_StandaloneCallContextAttrs<'input> for OC_StandaloneCallContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_StandaloneCall(
        &mut self,
    ) -> Result<Rc<OC_StandaloneCallContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_StandaloneCallContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_oC_StandaloneCall);
        let mut _localctx: Rc<OC_StandaloneCallContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(484);
                recog.base.match_token(CALL, &mut recog.err_handler)?;

                recog.base.set_state(485);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                recog.base.set_state(488);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(58, &mut recog.base)? {
                    1 => {
                        {
                            /*InvokeRule oC_ExplicitProcedureInvocation*/
                            recog.base.set_state(486);
                            recog.oC_ExplicitProcedureInvocation()?;
                        }
                    }
                    2 => {
                        {
                            /*InvokeRule oC_ImplicitProcedureInvocation*/
                            recog.base.set_state(487);
                            recog.oC_ImplicitProcedureInvocation()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(494);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(59, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(490);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(491);
                            recog.base.match_token(YIELD, &mut recog.err_handler)?;

                            recog.base.set_state(492);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_YieldItems*/
                            recog.base.set_state(493);
                            recog.oC_YieldItems()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_YieldItems ----------------
pub type OC_YieldItemsContextAll<'input> = OC_YieldItemsContext<'input>;

pub type OC_YieldItemsContext<'input> =
    BaseParserRuleContext<'input, OC_YieldItemsContextExt<'input>>;

#[derive(Clone)]
pub struct OC_YieldItemsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_YieldItemsContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_YieldItemsContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_YieldItems(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_YieldItems(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_YieldItemsContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_YieldItems(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_YieldItemsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_YieldItems
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_YieldItems }
}
antlr_rust::type_id! {OC_YieldItemsContextExt<'a>}

impl<'input> OC_YieldItemsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_YieldItemsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_YieldItemsContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_YieldItemsContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_YieldItemsContextExt<'input>>
{
    fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_YieldItem_all(&self) -> Vec<Rc<OC_YieldItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_YieldItem(&self, i: usize) -> Option<Rc<OC_YieldItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_YieldItemsContextAttrs<'input> for OC_YieldItemsContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_YieldItems(&mut self) -> Result<Rc<OC_YieldItemsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_YieldItemsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_oC_YieldItems);
        let mut _localctx: Rc<OC_YieldItemsContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(511);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    T__4 => {
                        recog.base.set_state(496);
                        recog.base.match_token(T__4, &mut recog.err_handler)?;
                    }

                    COUNT
                    | ANY
                    | NONE
                    | SINGLE
                    | HexLetter
                    | FILTER
                    | EXTRACT
                    | UnescapedSymbolicName
                    | EscapedSymbolicName => {
                        {
                            {
                                /*InvokeRule oC_YieldItem*/
                                recog.base.set_state(497);
                                recog.oC_YieldItem()?;

                                recog.base.set_state(508);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(62, &mut recog.base)?;
                                while { _alt != 2 && _alt != INVALID_ALT } {
                                    if _alt == 1 {
                                        {
                                            {
                                                recog.base.set_state(499);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == SP {
                                                    {
                                                        recog.base.set_state(498);
                                                        recog.base.match_token(
                                                            SP,
                                                            &mut recog.err_handler,
                                                        )?;
                                                    }
                                                }

                                                recog.base.set_state(501);
                                                recog
                                                    .base
                                                    .match_token(T__1, &mut recog.err_handler)?;

                                                recog.base.set_state(503);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == SP {
                                                    {
                                                        recog.base.set_state(502);
                                                        recog.base.match_token(
                                                            SP,
                                                            &mut recog.err_handler,
                                                        )?;
                                                    }
                                                }

                                                /*InvokeRule oC_YieldItem*/
                                                recog.base.set_state(505);
                                                recog.oC_YieldItem()?;
                                            }
                                        }
                                    }
                                    recog.base.set_state(510);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _alt =
                                        recog.interpreter.adaptive_predict(62, &mut recog.base)?;
                                }
                            }
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
                recog.base.set_state(517);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(65, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(514);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(513);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Where*/
                            recog.base.set_state(516);
                            recog.oC_Where()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_YieldItem ----------------
pub type OC_YieldItemContextAll<'input> = OC_YieldItemContext<'input>;

pub type OC_YieldItemContext<'input> =
    BaseParserRuleContext<'input, OC_YieldItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_YieldItemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_YieldItemContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_YieldItemContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_YieldItem(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_YieldItem(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_YieldItemContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_YieldItem(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_YieldItemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_YieldItem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_YieldItem }
}
antlr_rust::type_id! {OC_YieldItemContextExt<'a>}

impl<'input> OC_YieldItemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_YieldItemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_YieldItemContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_YieldItemContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_YieldItemContextExt<'input>>
{
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_ProcedureResultField(&self) -> Option<Rc<OC_ProcedureResultFieldContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
}

impl<'input> OC_YieldItemContextAttrs<'input> for OC_YieldItemContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_YieldItem(&mut self) -> Result<Rc<OC_YieldItemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_YieldItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 46, RULE_oC_YieldItem);
        let mut _localctx: Rc<OC_YieldItemContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(524);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(66, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule oC_ProcedureResultField*/
                            recog.base.set_state(519);
                            recog.oC_ProcedureResultField()?;

                            recog.base.set_state(520);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(521);
                            recog.base.match_token(AS, &mut recog.err_handler)?;

                            recog.base.set_state(522);
                            recog.base.match_token(SP, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
                /*InvokeRule oC_Variable*/
                recog.base.set_state(526);
                recog.oC_Variable()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_With ----------------
pub type OC_WithContextAll<'input> = OC_WithContext<'input>;

pub type OC_WithContext<'input> = BaseParserRuleContext<'input, OC_WithContextExt<'input>>;

#[derive(Clone)]
pub struct OC_WithContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_WithContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_WithContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_With(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_With(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_WithContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_With(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_WithContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_With
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_With }
}
antlr_rust::type_id! {OC_WithContextExt<'a>}

impl<'input> OC_WithContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_WithContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_WithContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_WithContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_WithContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token WITH
    /// Returns `None` if there is no child corresponding to token WITH
    fn WITH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WITH, 0)
    }
    fn oC_ProjectionBody(&self) -> Option<Rc<OC_ProjectionBodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_WithContextAttrs<'input> for OC_WithContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_With(&mut self) -> Result<Rc<OC_WithContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_WithContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_oC_With);
        let mut _localctx: Rc<OC_WithContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(528);
                recog.base.match_token(WITH, &mut recog.err_handler)?;

                /*InvokeRule oC_ProjectionBody*/
                recog.base.set_state(529);
                recog.oC_ProjectionBody()?;

                recog.base.set_state(534);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(68, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(531);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(530);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Where*/
                            recog.base.set_state(533);
                            recog.oC_Where()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Return ----------------
pub type OC_ReturnContextAll<'input> = OC_ReturnContext<'input>;

pub type OC_ReturnContext<'input> = BaseParserRuleContext<'input, OC_ReturnContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ReturnContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ReturnContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ReturnContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Return(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Return(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ReturnContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Return(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ReturnContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Return
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Return }
}
antlr_rust::type_id! {OC_ReturnContextExt<'a>}

impl<'input> OC_ReturnContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ReturnContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ReturnContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ReturnContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ReturnContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token RETURN
    /// Returns `None` if there is no child corresponding to token RETURN
    fn RETURN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RETURN, 0)
    }
    fn oC_ProjectionBody(&self) -> Option<Rc<OC_ProjectionBodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ReturnContextAttrs<'input> for OC_ReturnContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Return(&mut self) -> Result<Rc<OC_ReturnContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_ReturnContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_oC_Return);
        let mut _localctx: Rc<OC_ReturnContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(536);
                recog.base.match_token(RETURN, &mut recog.err_handler)?;

                /*InvokeRule oC_ProjectionBody*/
                recog.base.set_state(537);
                recog.oC_ProjectionBody()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ProjectionBody ----------------
pub type OC_ProjectionBodyContextAll<'input> = OC_ProjectionBodyContext<'input>;

pub type OC_ProjectionBodyContext<'input> =
    BaseParserRuleContext<'input, OC_ProjectionBodyContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProjectionBodyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ProjectionBodyContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProjectionBodyContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ProjectionBody(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ProjectionBody(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ProjectionBodyContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ProjectionBody(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ProjectionBodyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ProjectionBody
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProjectionBody }
}
antlr_rust::type_id! {OC_ProjectionBodyContextExt<'a>}

impl<'input> OC_ProjectionBodyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ProjectionBodyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ProjectionBodyContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ProjectionBodyContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ProjectionBodyContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_ProjectionItems(&self) -> Option<Rc<OC_ProjectionItemsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DISTINCT
    /// Returns `None` if there is no child corresponding to token DISTINCT
    fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DISTINCT, 0)
    }
    fn oC_Order(&self) -> Option<Rc<OC_OrderContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Skip(&self) -> Option<Rc<OC_SkipContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Limit(&self) -> Option<Rc<OC_LimitContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ProjectionBodyContextAttrs<'input> for OC_ProjectionBodyContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ProjectionBody(
        &mut self,
    ) -> Result<Rc<OC_ProjectionBodyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ProjectionBodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 52, RULE_oC_ProjectionBody);
        let mut _localctx: Rc<OC_ProjectionBodyContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(543);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(70, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(540);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(539);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(542);
                        recog.base.match_token(DISTINCT, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(545);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_ProjectionItems*/
                recog.base.set_state(546);
                recog.oC_ProjectionItems()?;

                recog.base.set_state(549);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(71, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(547);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_Order*/
                            recog.base.set_state(548);
                            recog.oC_Order()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(553);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(72, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(551);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_Skip*/
                            recog.base.set_state(552);
                            recog.oC_Skip()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(557);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(73, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(555);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_Limit*/
                            recog.base.set_state(556);
                            recog.oC_Limit()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ProjectionItems ----------------
pub type OC_ProjectionItemsContextAll<'input> = OC_ProjectionItemsContext<'input>;

pub type OC_ProjectionItemsContext<'input> =
    BaseParserRuleContext<'input, OC_ProjectionItemsContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProjectionItemsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ProjectionItemsContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProjectionItemsContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ProjectionItems(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ProjectionItems(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ProjectionItemsContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ProjectionItems(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ProjectionItemsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ProjectionItems
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProjectionItems }
}
antlr_rust::type_id! {OC_ProjectionItemsContextExt<'a>}

impl<'input> OC_ProjectionItemsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ProjectionItemsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ProjectionItemsContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ProjectionItemsContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ProjectionItemsContextExt<'input>>
{
    fn oC_ProjectionItem_all(&self) -> Vec<Rc<OC_ProjectionItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_ProjectionItem(&self, i: usize) -> Option<Rc<OC_ProjectionItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_ProjectionItemsContextAttrs<'input> for OC_ProjectionItemsContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ProjectionItems(
        &mut self,
    ) -> Result<Rc<OC_ProjectionItemsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ProjectionItemsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 54, RULE_oC_ProjectionItems);
        let mut _localctx: Rc<OC_ProjectionItemsContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            recog.base.set_state(587);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(559);
                            recog.base.match_token(T__4, &mut recog.err_handler)?;

                            recog.base.set_state(570);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(76, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(561);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(560);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(563);
                                            recog.base.match_token(T__1, &mut recog.err_handler)?;

                                            recog.base.set_state(565);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(564);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_ProjectionItem*/
                                            recog.base.set_state(567);
                                            recog.oC_ProjectionItem()?;
                                        }
                                    }
                                }
                                recog.base.set_state(572);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(76, &mut recog.base)?;
                            }
                        }
                    }
                }

                T__5
                | T__7
                | T__12
                | T__13
                | T__23
                | T__25
                | ALL
                | NOT
                | NULL
                | COUNT
                | ANY
                | NONE
                | SINGLE
                | TRUE
                | FALSE
                | EXISTS
                | CASE
                | StringLiteral
                | HexInteger
                | DecimalInteger
                | OctalInteger
                | HexLetter
                | ExponentDecimalReal
                | RegularDecimalReal
                | FILTER
                | EXTRACT
                | UnescapedSymbolicName
                | EscapedSymbolicName => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            /*InvokeRule oC_ProjectionItem*/
                            recog.base.set_state(573);
                            recog.oC_ProjectionItem()?;

                            recog.base.set_state(584);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(79, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(575);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(574);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(577);
                                            recog.base.match_token(T__1, &mut recog.err_handler)?;

                                            recog.base.set_state(579);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(578);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_ProjectionItem*/
                                            recog.base.set_state(581);
                                            recog.oC_ProjectionItem()?;
                                        }
                                    }
                                }
                                recog.base.set_state(586);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(79, &mut recog.base)?;
                            }
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ProjectionItem ----------------
pub type OC_ProjectionItemContextAll<'input> = OC_ProjectionItemContext<'input>;

pub type OC_ProjectionItemContext<'input> =
    BaseParserRuleContext<'input, OC_ProjectionItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProjectionItemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ProjectionItemContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProjectionItemContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ProjectionItem(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ProjectionItem(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ProjectionItemContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ProjectionItem(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ProjectionItemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ProjectionItem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProjectionItem }
}
antlr_rust::type_id! {OC_ProjectionItemContextExt<'a>}

impl<'input> OC_ProjectionItemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ProjectionItemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ProjectionItemContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ProjectionItemContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ProjectionItemContextExt<'input>>
{
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ProjectionItemContextAttrs<'input> for OC_ProjectionItemContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ProjectionItem(
        &mut self,
    ) -> Result<Rc<OC_ProjectionItemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ProjectionItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 56, RULE_oC_ProjectionItem);
        let mut _localctx: Rc<OC_ProjectionItemContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(596);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(81, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(589);
                            recog.oC_Expression()?;

                            recog.base.set_state(590);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(591);
                            recog.base.match_token(AS, &mut recog.err_handler)?;

                            recog.base.set_state(592);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            /*InvokeRule oC_Variable*/
                            recog.base.set_state(593);
                            recog.oC_Variable()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_Expression*/
                        recog.base.set_state(595);
                        recog.oC_Expression()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Order ----------------
pub type OC_OrderContextAll<'input> = OC_OrderContext<'input>;

pub type OC_OrderContext<'input> = BaseParserRuleContext<'input, OC_OrderContextExt<'input>>;

#[derive(Clone)]
pub struct OC_OrderContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_OrderContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_OrderContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Order(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Order(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_OrderContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Order(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_OrderContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Order
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Order }
}
antlr_rust::type_id! {OC_OrderContextExt<'a>}

impl<'input> OC_OrderContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_OrderContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_OrderContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_OrderContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_OrderContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ORDER
    /// Returns `None` if there is no child corresponding to token ORDER
    fn ORDER(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ORDER, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token BY
    /// Returns `None` if there is no child corresponding to token BY
    fn BY(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BY, 0)
    }
    fn oC_SortItem_all(&self) -> Vec<Rc<OC_SortItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_SortItem(&self, i: usize) -> Option<Rc<OC_SortItemContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_OrderContextAttrs<'input> for OC_OrderContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Order(&mut self) -> Result<Rc<OC_OrderContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_OrderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_oC_Order);
        let mut _localctx: Rc<OC_OrderContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(598);
                recog.base.match_token(ORDER, &mut recog.err_handler)?;

                recog.base.set_state(599);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                recog.base.set_state(600);
                recog.base.match_token(BY, &mut recog.err_handler)?;

                recog.base.set_state(601);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_SortItem*/
                recog.base.set_state(602);
                recog.oC_SortItem()?;

                recog.base.set_state(610);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__1 {
                    {
                        {
                            recog.base.set_state(603);
                            recog.base.match_token(T__1, &mut recog.err_handler)?;

                            recog.base.set_state(605);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(604);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_SortItem*/
                            recog.base.set_state(607);
                            recog.oC_SortItem()?;
                        }
                    }
                    recog.base.set_state(612);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Skip ----------------
pub type OC_SkipContextAll<'input> = OC_SkipContext<'input>;

pub type OC_SkipContext<'input> = BaseParserRuleContext<'input, OC_SkipContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SkipContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SkipContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SkipContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Skip(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Skip(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SkipContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Skip(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SkipContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Skip
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Skip }
}
antlr_rust::type_id! {OC_SkipContextExt<'a>}

impl<'input> OC_SkipContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SkipContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SkipContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SkipContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SkipContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token L_SKIP
    /// Returns `None` if there is no child corresponding to token L_SKIP
    fn L_SKIP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(L_SKIP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_SkipContextAttrs<'input> for OC_SkipContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Skip(&mut self) -> Result<Rc<OC_SkipContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_SkipContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_oC_Skip);
        let mut _localctx: Rc<OC_SkipContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(613);
                recog.base.match_token(L_SKIP, &mut recog.err_handler)?;

                recog.base.set_state(614);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_Expression*/
                recog.base.set_state(615);
                recog.oC_Expression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Limit ----------------
pub type OC_LimitContextAll<'input> = OC_LimitContext<'input>;

pub type OC_LimitContext<'input> = BaseParserRuleContext<'input, OC_LimitContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LimitContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_LimitContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LimitContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Limit(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Limit(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_LimitContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Limit(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_LimitContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Limit
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Limit }
}
antlr_rust::type_id! {OC_LimitContextExt<'a>}

impl<'input> OC_LimitContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_LimitContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_LimitContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_LimitContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_LimitContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token LIMIT
    /// Returns `None` if there is no child corresponding to token LIMIT
    fn LIMIT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LIMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_LimitContextAttrs<'input> for OC_LimitContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Limit(&mut self) -> Result<Rc<OC_LimitContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_LimitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_oC_Limit);
        let mut _localctx: Rc<OC_LimitContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(617);
                recog.base.match_token(LIMIT, &mut recog.err_handler)?;

                recog.base.set_state(618);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_Expression*/
                recog.base.set_state(619);
                recog.oC_Expression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_SortItem ----------------
pub type OC_SortItemContextAll<'input> = OC_SortItemContext<'input>;

pub type OC_SortItemContext<'input> = BaseParserRuleContext<'input, OC_SortItemContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SortItemContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SortItemContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SortItemContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_SortItem(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_SortItem(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SortItemContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_SortItem(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SortItemContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_SortItem
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_SortItem }
}
antlr_rust::type_id! {OC_SortItemContextExt<'a>}

impl<'input> OC_SortItemContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SortItemContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SortItemContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SortItemContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SortItemContextExt<'input>>
{
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASCENDING
    /// Returns `None` if there is no child corresponding to token ASCENDING
    fn ASCENDING(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASCENDING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASC
    /// Returns `None` if there is no child corresponding to token ASC
    fn ASC(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DESCENDING
    /// Returns `None` if there is no child corresponding to token DESCENDING
    fn DESCENDING(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DESCENDING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DESC
    /// Returns `None` if there is no child corresponding to token DESC
    fn DESC(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DESC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_SortItemContextAttrs<'input> for OC_SortItemContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_SortItem(&mut self) -> Result<Rc<OC_SortItemContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_SortItemContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 64, RULE_oC_SortItem);
        let mut _localctx: Rc<OC_SortItemContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_Expression*/
                recog.base.set_state(621);
                recog.oC_Expression()?;

                recog.base.set_state(626);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(85, &mut recog.base)? {
                    x if x == 1 => {
                        recog.base.set_state(623);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(622);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(625);
                        _la = recog.base.input.la(1);
                        if {
                            !(((_la - 68) & !0x3f) == 0
                                && ((1usize << (_la - 68))
                                    & ((1usize << (ASCENDING - 68))
                                        | (1usize << (ASC - 68))
                                        | (1usize << (DESCENDING - 68))
                                        | (1usize << (DESC - 68))))
                                    != 0)
                        } {
                            recog.err_handler.recover_inline(&mut recog.base)?;
                        } else {
                            if recog.base.input.la(1) == TOKEN_EOF {
                                recog.base.matched_eof = true
                            };
                            recog.err_handler.report_match(&mut recog.base);
                            recog.base.consume(&mut recog.err_handler);
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Where ----------------
pub type OC_WhereContextAll<'input> = OC_WhereContext<'input>;

pub type OC_WhereContext<'input> = BaseParserRuleContext<'input, OC_WhereContextExt<'input>>;

#[derive(Clone)]
pub struct OC_WhereContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_WhereContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_WhereContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Where(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Where(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_WhereContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Where(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_WhereContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Where
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Where }
}
antlr_rust::type_id! {OC_WhereContextExt<'a>}

impl<'input> OC_WhereContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_WhereContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_WhereContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_WhereContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_WhereContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token WHERE
    /// Returns `None` if there is no child corresponding to token WHERE
    fn WHERE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHERE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_WhereContextAttrs<'input> for OC_WhereContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Where(&mut self) -> Result<Rc<OC_WhereContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_WhereContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_oC_Where);
        let mut _localctx: Rc<OC_WhereContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(628);
                recog.base.match_token(WHERE, &mut recog.err_handler)?;

                recog.base.set_state(629);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_Expression*/
                recog.base.set_state(630);
                recog.oC_Expression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Pattern ----------------
pub type OC_PatternContextAll<'input> = OC_PatternContext<'input>;

pub type OC_PatternContext<'input> = BaseParserRuleContext<'input, OC_PatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PatternContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Pattern(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Pattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PatternContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Pattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Pattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Pattern }
}
antlr_rust::type_id! {OC_PatternContextExt<'a>}

impl<'input> OC_PatternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PatternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PatternContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PatternContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PatternContextExt<'input>>
{
    fn oC_PatternPart_all(&self) -> Vec<Rc<OC_PatternPartContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PatternPart(&self, i: usize) -> Option<Rc<OC_PatternPartContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_PatternContextAttrs<'input> for OC_PatternContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Pattern(&mut self) -> Result<Rc<OC_PatternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_PatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 68, RULE_oC_Pattern);
        let mut _localctx: Rc<OC_PatternContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_PatternPart*/
                recog.base.set_state(632);
                recog.oC_PatternPart()?;

                recog.base.set_state(643);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(88, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(634);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(633);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                recog.base.set_state(636);
                                recog.base.match_token(T__1, &mut recog.err_handler)?;

                                recog.base.set_state(638);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(637);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_PatternPart*/
                                recog.base.set_state(640);
                                recog.oC_PatternPart()?;
                            }
                        }
                    }
                    recog.base.set_state(645);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(88, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PatternPart ----------------
pub type OC_PatternPartContextAll<'input> = OC_PatternPartContext<'input>;

pub type OC_PatternPartContext<'input> =
    BaseParserRuleContext<'input, OC_PatternPartContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternPartContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PatternPartContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternPartContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PatternPart(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PatternPart(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PatternPartContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PatternPart(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PatternPartContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PatternPart
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternPart }
}
antlr_rust::type_id! {OC_PatternPartContextExt<'a>}

impl<'input> OC_PatternPartContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PatternPartContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PatternPartContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PatternPartContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PatternPartContextExt<'input>>
{
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_AnonymousPatternPart(&self) -> Option<Rc<OC_AnonymousPatternPartContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_PatternPartContextAttrs<'input> for OC_PatternPartContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PatternPart(&mut self) -> Result<Rc<OC_PatternPartContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PatternPartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 70, RULE_oC_PatternPart);
        let mut _localctx: Rc<OC_PatternPartContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(657);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                COUNT
                | ANY
                | NONE
                | SINGLE
                | HexLetter
                | FILTER
                | EXTRACT
                | UnescapedSymbolicName
                | EscapedSymbolicName => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_Variable*/
                            recog.base.set_state(646);
                            recog.oC_Variable()?;

                            recog.base.set_state(648);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(647);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(650);
                            recog.base.match_token(T__2, &mut recog.err_handler)?;

                            recog.base.set_state(652);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(651);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AnonymousPatternPart*/
                            recog.base.set_state(654);
                            recog.oC_AnonymousPatternPart()?;
                        }
                    }
                }

                T__5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_AnonymousPatternPart*/
                        recog.base.set_state(656);
                        recog.oC_AnonymousPatternPart()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_AnonymousPatternPart ----------------
pub type OC_AnonymousPatternPartContextAll<'input> = OC_AnonymousPatternPartContext<'input>;

pub type OC_AnonymousPatternPartContext<'input> =
    BaseParserRuleContext<'input, OC_AnonymousPatternPartContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AnonymousPatternPartContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_AnonymousPatternPartContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_AnonymousPatternPartContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_AnonymousPatternPart(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_AnonymousPatternPart(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_AnonymousPatternPartContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_AnonymousPatternPart(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_AnonymousPatternPartContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_AnonymousPatternPart
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_AnonymousPatternPart }
}
antlr_rust::type_id! {OC_AnonymousPatternPartContextExt<'a>}

impl<'input> OC_AnonymousPatternPartContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_AnonymousPatternPartContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_AnonymousPatternPartContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_AnonymousPatternPartContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_AnonymousPatternPartContextExt<'input>>
{
    fn oC_PatternElement(&self) -> Option<Rc<OC_PatternElementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_AnonymousPatternPartContextAttrs<'input>
    for OC_AnonymousPatternPartContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_AnonymousPatternPart(
        &mut self,
    ) -> Result<Rc<OC_AnonymousPatternPartContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_AnonymousPatternPartContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 72, RULE_oC_AnonymousPatternPart);
        let mut _localctx: Rc<OC_AnonymousPatternPartContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_PatternElement*/
                recog.base.set_state(659);
                recog.oC_PatternElement()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PatternElement ----------------
pub type OC_PatternElementContextAll<'input> = OC_PatternElementContext<'input>;

pub type OC_PatternElementContext<'input> =
    BaseParserRuleContext<'input, OC_PatternElementContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternElementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PatternElementContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PatternElementContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PatternElement(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PatternElement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PatternElementContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PatternElement(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PatternElementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PatternElement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternElement }
}
antlr_rust::type_id! {OC_PatternElementContextExt<'a>}

impl<'input> OC_PatternElementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PatternElementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PatternElementContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PatternElementContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PatternElementContextExt<'input>>
{
    fn oC_NodePattern(&self) -> Option<Rc<OC_NodePatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PatternElementChain_all(&self) -> Vec<Rc<OC_PatternElementChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PatternElementChain(
        &self,
        i: usize,
    ) -> Option<Rc<OC_PatternElementChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_PatternElement(&self) -> Option<Rc<OC_PatternElementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_PatternElementContextAttrs<'input> for OC_PatternElementContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PatternElement(
        &mut self,
    ) -> Result<Rc<OC_PatternElementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PatternElementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 74, RULE_oC_PatternElement);
        let mut _localctx: Rc<OC_PatternElementContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            recog.base.set_state(675);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(94, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_NodePattern*/
                            recog.base.set_state(661);
                            recog.oC_NodePattern()?;

                            recog.base.set_state(668);
                            recog.err_handler.sync(&mut recog.base)?;
                            _alt = recog.interpreter.adaptive_predict(93, &mut recog.base)?;
                            while { _alt != 2 && _alt != INVALID_ALT } {
                                if _alt == 1 {
                                    {
                                        {
                                            recog.base.set_state(663);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(662);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_PatternElementChain*/
                                            recog.base.set_state(665);
                                            recog.oC_PatternElementChain()?;
                                        }
                                    }
                                }
                                recog.base.set_state(670);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = recog.interpreter.adaptive_predict(93, &mut recog.base)?;
                            }
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(671);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            /*InvokeRule oC_PatternElement*/
                            recog.base.set_state(672);
                            recog.oC_PatternElement()?;

                            recog.base.set_state(673);
                            recog.base.match_token(T__6, &mut recog.err_handler)?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_NodePattern ----------------
pub type OC_NodePatternContextAll<'input> = OC_NodePatternContext<'input>;

pub type OC_NodePatternContext<'input> =
    BaseParserRuleContext<'input, OC_NodePatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NodePatternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NodePatternContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NodePatternContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_NodePattern(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_NodePattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_NodePatternContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_NodePattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NodePatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_NodePattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_NodePattern }
}
antlr_rust::type_id! {OC_NodePatternContextExt<'a>}

impl<'input> OC_NodePatternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NodePatternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NodePatternContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NodePatternContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NodePatternContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Properties(&self) -> Option<Rc<OC_PropertiesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_NodePatternContextAttrs<'input> for OC_NodePatternContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_NodePattern(&mut self) -> Result<Rc<OC_NodePatternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_NodePatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 76, RULE_oC_NodePattern);
        let mut _localctx: Rc<OC_NodePatternContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(677);
                recog.base.match_token(T__5, &mut recog.err_handler)?;

                recog.base.set_state(679);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(678);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(685);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 83) & !0x3f) == 0
                    && ((1usize << (_la - 83))
                        & ((1usize << (COUNT - 83))
                            | (1usize << (ANY - 83))
                            | (1usize << (NONE - 83))
                            | (1usize << (SINGLE - 83))
                            | (1usize << (HexLetter - 83))
                            | (1usize << (FILTER - 83))
                            | (1usize << (EXTRACT - 83))
                            | (1usize << (UnescapedSymbolicName - 83))
                            | (1usize << (EscapedSymbolicName - 83))))
                        != 0)
                {
                    {
                        /*InvokeRule oC_Variable*/
                        recog.base.set_state(681);
                        recog.oC_Variable()?;

                        recog.base.set_state(683);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(682);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(691);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__9 {
                    {
                        /*InvokeRule oC_NodeLabels*/
                        recog.base.set_state(687);
                        recog.oC_NodeLabels()?;

                        recog.base.set_state(689);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(688);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(697);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__23 || _la == T__25 {
                    {
                        /*InvokeRule oC_Properties*/
                        recog.base.set_state(693);
                        recog.oC_Properties()?;

                        recog.base.set_state(695);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(694);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(699);
                recog.base.match_token(T__6, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PatternElementChain ----------------
pub type OC_PatternElementChainContextAll<'input> = OC_PatternElementChainContext<'input>;

pub type OC_PatternElementChainContext<'input> =
    BaseParserRuleContext<'input, OC_PatternElementChainContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternElementChainContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PatternElementChainContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_PatternElementChainContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PatternElementChain(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PatternElementChain(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_PatternElementChainContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PatternElementChain(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PatternElementChainContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PatternElementChain
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternElementChain }
}
antlr_rust::type_id! {OC_PatternElementChainContextExt<'a>}

impl<'input> OC_PatternElementChainContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PatternElementChainContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PatternElementChainContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PatternElementChainContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PatternElementChainContextExt<'input>>
{
    fn oC_RelationshipPattern(&self) -> Option<Rc<OC_RelationshipPatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_NodePattern(&self) -> Option<Rc<OC_NodePatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_PatternElementChainContextAttrs<'input> for OC_PatternElementChainContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PatternElementChain(
        &mut self,
    ) -> Result<Rc<OC_PatternElementChainContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PatternElementChainContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 78, RULE_oC_PatternElementChain);
        let mut _localctx: Rc<OC_PatternElementChainContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_RelationshipPattern*/
                recog.base.set_state(701);
                recog.oC_RelationshipPattern()?;

                recog.base.set_state(703);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(702);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_NodePattern*/
                recog.base.set_state(705);
                recog.oC_NodePattern()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RelationshipPattern ----------------
pub type OC_RelationshipPatternContextAll<'input> = OC_RelationshipPatternContext<'input>;

pub type OC_RelationshipPatternContext<'input> =
    BaseParserRuleContext<'input, OC_RelationshipPatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipPatternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RelationshipPatternContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_RelationshipPatternContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RelationshipPattern(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RelationshipPattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_RelationshipPatternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RelationshipPattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipPatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RelationshipPattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipPattern }
}
antlr_rust::type_id! {OC_RelationshipPatternContextExt<'a>}

impl<'input> OC_RelationshipPatternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RelationshipPatternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RelationshipPatternContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RelationshipPatternContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RelationshipPatternContextExt<'input>>
{
    fn oC_LeftArrowHead(&self) -> Option<Rc<OC_LeftArrowHeadContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Dash_all(&self) -> Vec<Rc<OC_DashContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Dash(&self, i: usize) -> Option<Rc<OC_DashContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_RightArrowHead(&self) -> Option<Rc<OC_RightArrowHeadContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_RelationshipDetail(&self) -> Option<Rc<OC_RelationshipDetailContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_RelationshipPatternContextAttrs<'input> for OC_RelationshipPatternContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RelationshipPattern(
        &mut self,
    ) -> Result<Rc<OC_RelationshipPatternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RelationshipPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 80, RULE_oC_RelationshipPattern);
        let mut _localctx: Rc<OC_RelationshipPatternContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(771);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(119, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_LeftArrowHead*/
                            recog.base.set_state(707);
                            recog.oC_LeftArrowHead()?;

                            recog.base.set_state(709);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(708);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(711);
                            recog.oC_Dash()?;

                            recog.base.set_state(713);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(104, &mut recog.base)? {
                                x if x == 1 => {
                                    recog.base.set_state(712);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }

                                _ => {}
                            }
                            recog.base.set_state(716);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == T__7 {
                                {
                                    /*InvokeRule oC_RelationshipDetail*/
                                    recog.base.set_state(715);
                                    recog.oC_RelationshipDetail()?;
                                }
                            }

                            recog.base.set_state(719);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(718);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(721);
                            recog.oC_Dash()?;

                            recog.base.set_state(723);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(722);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_RightArrowHead*/
                            recog.base.set_state(725);
                            recog.oC_RightArrowHead()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            /*InvokeRule oC_LeftArrowHead*/
                            recog.base.set_state(727);
                            recog.oC_LeftArrowHead()?;

                            recog.base.set_state(729);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(728);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(731);
                            recog.oC_Dash()?;

                            recog.base.set_state(733);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(109, &mut recog.base)? {
                                x if x == 1 => {
                                    recog.base.set_state(732);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }

                                _ => {}
                            }
                            recog.base.set_state(736);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == T__7 {
                                {
                                    /*InvokeRule oC_RelationshipDetail*/
                                    recog.base.set_state(735);
                                    recog.oC_RelationshipDetail()?;
                                }
                            }

                            recog.base.set_state(739);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(738);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(741);
                            recog.oC_Dash()?;
                        }
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        {
                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(743);
                            recog.oC_Dash()?;

                            recog.base.set_state(745);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(112, &mut recog.base)? {
                                x if x == 1 => {
                                    recog.base.set_state(744);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }

                                _ => {}
                            }
                            recog.base.set_state(748);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == T__7 {
                                {
                                    /*InvokeRule oC_RelationshipDetail*/
                                    recog.base.set_state(747);
                                    recog.oC_RelationshipDetail()?;
                                }
                            }

                            recog.base.set_state(751);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(750);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(753);
                            recog.oC_Dash()?;

                            recog.base.set_state(755);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(754);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_RightArrowHead*/
                            recog.base.set_state(757);
                            recog.oC_RightArrowHead()?;
                        }
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        {
                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(759);
                            recog.oC_Dash()?;

                            recog.base.set_state(761);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(116, &mut recog.base)? {
                                x if x == 1 => {
                                    recog.base.set_state(760);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }

                                _ => {}
                            }
                            recog.base.set_state(764);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == T__7 {
                                {
                                    /*InvokeRule oC_RelationshipDetail*/
                                    recog.base.set_state(763);
                                    recog.oC_RelationshipDetail()?;
                                }
                            }

                            recog.base.set_state(767);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(766);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Dash*/
                            recog.base.set_state(769);
                            recog.oC_Dash()?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RelationshipDetail ----------------
pub type OC_RelationshipDetailContextAll<'input> = OC_RelationshipDetailContext<'input>;

pub type OC_RelationshipDetailContext<'input> =
    BaseParserRuleContext<'input, OC_RelationshipDetailContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipDetailContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RelationshipDetailContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_RelationshipDetailContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RelationshipDetail(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RelationshipDetail(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_RelationshipDetailContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RelationshipDetail(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipDetailContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RelationshipDetail
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipDetail }
}
antlr_rust::type_id! {OC_RelationshipDetailContextExt<'a>}

impl<'input> OC_RelationshipDetailContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RelationshipDetailContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RelationshipDetailContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RelationshipDetailContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RelationshipDetailContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_RelationshipTypes(&self) -> Option<Rc<OC_RelationshipTypesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_RangeLiteral(&self) -> Option<Rc<OC_RangeLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Properties(&self) -> Option<Rc<OC_PropertiesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_RelationshipDetailContextAttrs<'input> for OC_RelationshipDetailContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RelationshipDetail(
        &mut self,
    ) -> Result<Rc<OC_RelationshipDetailContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RelationshipDetailContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 82, RULE_oC_RelationshipDetail);
        let mut _localctx: Rc<OC_RelationshipDetailContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(773);
                recog.base.match_token(T__7, &mut recog.err_handler)?;

                recog.base.set_state(775);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(774);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(781);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 83) & !0x3f) == 0
                    && ((1usize << (_la - 83))
                        & ((1usize << (COUNT - 83))
                            | (1usize << (ANY - 83))
                            | (1usize << (NONE - 83))
                            | (1usize << (SINGLE - 83))
                            | (1usize << (HexLetter - 83))
                            | (1usize << (FILTER - 83))
                            | (1usize << (EXTRACT - 83))
                            | (1usize << (UnescapedSymbolicName - 83))
                            | (1usize << (EscapedSymbolicName - 83))))
                        != 0)
                {
                    {
                        /*InvokeRule oC_Variable*/
                        recog.base.set_state(777);
                        recog.oC_Variable()?;

                        recog.base.set_state(779);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(778);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(787);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__9 {
                    {
                        /*InvokeRule oC_RelationshipTypes*/
                        recog.base.set_state(783);
                        recog.oC_RelationshipTypes()?;

                        recog.base.set_state(785);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(784);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(790);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__4 {
                    {
                        /*InvokeRule oC_RangeLiteral*/
                        recog.base.set_state(789);
                        recog.oC_RangeLiteral()?;
                    }
                }

                recog.base.set_state(796);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__23 || _la == T__25 {
                    {
                        /*InvokeRule oC_Properties*/
                        recog.base.set_state(792);
                        recog.oC_Properties()?;

                        recog.base.set_state(794);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(793);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(798);
                recog.base.match_token(T__8, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Properties ----------------
pub type OC_PropertiesContextAll<'input> = OC_PropertiesContext<'input>;

pub type OC_PropertiesContext<'input> =
    BaseParserRuleContext<'input, OC_PropertiesContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertiesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PropertiesContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertiesContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Properties(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Properties(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PropertiesContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Properties(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PropertiesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Properties
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Properties }
}
antlr_rust::type_id! {OC_PropertiesContextExt<'a>}

impl<'input> OC_PropertiesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PropertiesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PropertiesContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PropertiesContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PropertiesContextExt<'input>>
{
    fn oC_MapLiteral(&self) -> Option<Rc<OC_MapLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Parameter(&self) -> Option<Rc<OC_ParameterContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_PropertiesContextAttrs<'input> for OC_PropertiesContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Properties(&mut self) -> Result<Rc<OC_PropertiesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PropertiesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 84, RULE_oC_Properties);
        let mut _localctx: Rc<OC_PropertiesContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(802);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__23 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_MapLiteral*/
                        recog.base.set_state(800);
                        recog.oC_MapLiteral()?;
                    }
                }

                T__25 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_Parameter*/
                        recog.base.set_state(801);
                        recog.oC_Parameter()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RelationshipTypes ----------------
pub type OC_RelationshipTypesContextAll<'input> = OC_RelationshipTypesContext<'input>;

pub type OC_RelationshipTypesContext<'input> =
    BaseParserRuleContext<'input, OC_RelationshipTypesContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipTypesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RelationshipTypesContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_RelationshipTypesContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RelationshipTypes(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RelationshipTypes(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RelationshipTypesContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RelationshipTypes(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipTypesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RelationshipTypes
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipTypes }
}
antlr_rust::type_id! {OC_RelationshipTypesContextExt<'a>}

impl<'input> OC_RelationshipTypesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RelationshipTypesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RelationshipTypesContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RelationshipTypesContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RelationshipTypesContextExt<'input>>
{
    fn oC_RelTypeName_all(&self) -> Vec<Rc<OC_RelTypeNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_RelTypeName(&self, i: usize) -> Option<Rc<OC_RelTypeNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_RelationshipTypesContextAttrs<'input> for OC_RelationshipTypesContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RelationshipTypes(
        &mut self,
    ) -> Result<Rc<OC_RelationshipTypesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RelationshipTypesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 86, RULE_oC_RelationshipTypes);
        let mut _localctx: Rc<OC_RelationshipTypesContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(804);
                recog.base.match_token(T__9, &mut recog.err_handler)?;

                recog.base.set_state(806);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(805);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_RelTypeName*/
                recog.base.set_state(808);
                recog.oC_RelTypeName()?;

                recog.base.set_state(822);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(133, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(810);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(809);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                recog.base.set_state(812);
                                recog.base.match_token(T__10, &mut recog.err_handler)?;

                                recog.base.set_state(814);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == T__9 {
                                    {
                                        recog.base.set_state(813);
                                        recog.base.match_token(T__9, &mut recog.err_handler)?;
                                    }
                                }

                                recog.base.set_state(817);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(816);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_RelTypeName*/
                                recog.base.set_state(819);
                                recog.oC_RelTypeName()?;
                            }
                        }
                    }
                    recog.base.set_state(824);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(133, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_NodeLabels ----------------
pub type OC_NodeLabelsContextAll<'input> = OC_NodeLabelsContext<'input>;

pub type OC_NodeLabelsContext<'input> =
    BaseParserRuleContext<'input, OC_NodeLabelsContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NodeLabelsContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NodeLabelsContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NodeLabelsContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_NodeLabels(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_NodeLabels(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_NodeLabelsContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_NodeLabels(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NodeLabelsContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_NodeLabels
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_NodeLabels }
}
antlr_rust::type_id! {OC_NodeLabelsContextExt<'a>}

impl<'input> OC_NodeLabelsContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NodeLabelsContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NodeLabelsContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NodeLabelsContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NodeLabelsContextExt<'input>>
{
    fn oC_NodeLabel_all(&self) -> Vec<Rc<OC_NodeLabelContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_NodeLabel(&self, i: usize) -> Option<Rc<OC_NodeLabelContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_NodeLabelsContextAttrs<'input> for OC_NodeLabelsContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_NodeLabels(&mut self) -> Result<Rc<OC_NodeLabelsContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_NodeLabelsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 88, RULE_oC_NodeLabels);
        let mut _localctx: Rc<OC_NodeLabelsContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_NodeLabel*/
                recog.base.set_state(825);
                recog.oC_NodeLabel()?;

                recog.base.set_state(832);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(135, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(827);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(826);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_NodeLabel*/
                                recog.base.set_state(829);
                                recog.oC_NodeLabel()?;
                            }
                        }
                    }
                    recog.base.set_state(834);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(135, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_NodeLabel ----------------
pub type OC_NodeLabelContextAll<'input> = OC_NodeLabelContext<'input>;

pub type OC_NodeLabelContext<'input> =
    BaseParserRuleContext<'input, OC_NodeLabelContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NodeLabelContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NodeLabelContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NodeLabelContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_NodeLabel(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_NodeLabel(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_NodeLabelContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_NodeLabel(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NodeLabelContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_NodeLabel
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_NodeLabel }
}
antlr_rust::type_id! {OC_NodeLabelContextExt<'a>}

impl<'input> OC_NodeLabelContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NodeLabelContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NodeLabelContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NodeLabelContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NodeLabelContextExt<'input>>
{
    fn oC_LabelName(&self) -> Option<Rc<OC_LabelNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_NodeLabelContextAttrs<'input> for OC_NodeLabelContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_NodeLabel(&mut self) -> Result<Rc<OC_NodeLabelContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_NodeLabelContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 90, RULE_oC_NodeLabel);
        let mut _localctx: Rc<OC_NodeLabelContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(835);
                recog.base.match_token(T__9, &mut recog.err_handler)?;

                recog.base.set_state(837);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(836);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_LabelName*/
                recog.base.set_state(839);
                recog.oC_LabelName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RangeLiteral ----------------
pub type OC_RangeLiteralContextAll<'input> = OC_RangeLiteralContext<'input>;

pub type OC_RangeLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_RangeLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RangeLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RangeLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RangeLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RangeLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RangeLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RangeLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RangeLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RangeLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RangeLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RangeLiteral }
}
antlr_rust::type_id! {OC_RangeLiteralContextExt<'a>}

impl<'input> OC_RangeLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RangeLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RangeLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RangeLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RangeLiteralContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_IntegerLiteral_all(&self) -> Vec<Rc<OC_IntegerLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_IntegerLiteral(&self, i: usize) -> Option<Rc<OC_IntegerLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_RangeLiteralContextAttrs<'input> for OC_RangeLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RangeLiteral(&mut self) -> Result<Rc<OC_RangeLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RangeLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 92, RULE_oC_RangeLiteral);
        let mut _localctx: Rc<OC_RangeLiteralContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(841);
                recog.base.match_token(T__4, &mut recog.err_handler)?;

                recog.base.set_state(843);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(842);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(849);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 97) & !0x3f) == 0
                    && ((1usize << (_la - 97))
                        & ((1usize << (HexInteger - 97))
                            | (1usize << (DecimalInteger - 97))
                            | (1usize << (OctalInteger - 97))))
                        != 0)
                {
                    {
                        /*InvokeRule oC_IntegerLiteral*/
                        recog.base.set_state(845);
                        recog.oC_IntegerLiteral()?;

                        recog.base.set_state(847);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(846);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(861);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == T__11 {
                    {
                        recog.base.set_state(851);
                        recog.base.match_token(T__11, &mut recog.err_handler)?;

                        recog.base.set_state(853);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(852);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(859);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if (((_la - 97) & !0x3f) == 0
                            && ((1usize << (_la - 97))
                                & ((1usize << (HexInteger - 97))
                                    | (1usize << (DecimalInteger - 97))
                                    | (1usize << (OctalInteger - 97))))
                                != 0)
                        {
                            {
                                /*InvokeRule oC_IntegerLiteral*/
                                recog.base.set_state(855);
                                recog.oC_IntegerLiteral()?;

                                recog.base.set_state(857);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(856);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_LabelName ----------------
pub type OC_LabelNameContextAll<'input> = OC_LabelNameContext<'input>;

pub type OC_LabelNameContext<'input> =
    BaseParserRuleContext<'input, OC_LabelNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LabelNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_LabelNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LabelNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_LabelName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_LabelName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_LabelNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_LabelName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_LabelNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_LabelName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_LabelName }
}
antlr_rust::type_id! {OC_LabelNameContextExt<'a>}

impl<'input> OC_LabelNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_LabelNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_LabelNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_LabelNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_LabelNameContextExt<'input>>
{
    fn oC_SchemaName(&self) -> Option<Rc<OC_SchemaNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_LabelNameContextAttrs<'input> for OC_LabelNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_LabelName(&mut self) -> Result<Rc<OC_LabelNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_LabelNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 94, RULE_oC_LabelName);
        let mut _localctx: Rc<OC_LabelNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_SchemaName*/
                recog.base.set_state(863);
                recog.oC_SchemaName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RelTypeName ----------------
pub type OC_RelTypeNameContextAll<'input> = OC_RelTypeNameContext<'input>;

pub type OC_RelTypeNameContext<'input> =
    BaseParserRuleContext<'input, OC_RelTypeNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelTypeNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RelTypeNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RelTypeNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RelTypeName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RelTypeName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RelTypeNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RelTypeName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RelTypeNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RelTypeName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelTypeName }
}
antlr_rust::type_id! {OC_RelTypeNameContextExt<'a>}

impl<'input> OC_RelTypeNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RelTypeNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RelTypeNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RelTypeNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RelTypeNameContextExt<'input>>
{
    fn oC_SchemaName(&self) -> Option<Rc<OC_SchemaNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_RelTypeNameContextAttrs<'input> for OC_RelTypeNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RelTypeName(&mut self) -> Result<Rc<OC_RelTypeNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RelTypeNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 96, RULE_oC_RelTypeName);
        let mut _localctx: Rc<OC_RelTypeNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_SchemaName*/
                recog.base.set_state(865);
                recog.oC_SchemaName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Expression ----------------
pub type OC_ExpressionContextAll<'input> = OC_ExpressionContext<'input>;

pub type OC_ExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Expression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Expression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Expression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Expression }
}
antlr_rust::type_id! {OC_ExpressionContextExt<'a>}

impl<'input> OC_ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ExpressionContextExt<'input>>
{
    fn oC_OrExpression(&self) -> Option<Rc<OC_OrExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ExpressionContextAttrs<'input> for OC_ExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Expression(&mut self) -> Result<Rc<OC_ExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 98, RULE_oC_Expression);
        let mut _localctx: Rc<OC_ExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_OrExpression*/
                recog.base.set_state(867);
                recog.oC_OrExpression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_OrExpression ----------------
pub type OC_OrExpressionContextAll<'input> = OC_OrExpressionContext<'input>;

pub type OC_OrExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_OrExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_OrExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_OrExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_OrExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_OrExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_OrExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_OrExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_OrExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_OrExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_OrExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_OrExpression }
}
antlr_rust::type_id! {OC_OrExpressionContextExt<'a>}

impl<'input> OC_OrExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_OrExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_OrExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_OrExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_OrExpressionContextExt<'input>>
{
    fn oC_XorExpression_all(&self) -> Vec<Rc<OC_XorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_XorExpression(&self, i: usize) -> Option<Rc<OC_XorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
    fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR, i)
    }
}

impl<'input> OC_OrExpressionContextAttrs<'input> for OC_OrExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_OrExpression(&mut self) -> Result<Rc<OC_OrExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_OrExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 100, RULE_oC_OrExpression);
        let mut _localctx: Rc<OC_OrExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_XorExpression*/
                recog.base.set_state(869);
                recog.oC_XorExpression()?;

                recog.base.set_state(876);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(144, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(870);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                recog.base.set_state(871);
                                recog.base.match_token(OR, &mut recog.err_handler)?;

                                recog.base.set_state(872);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                /*InvokeRule oC_XorExpression*/
                                recog.base.set_state(873);
                                recog.oC_XorExpression()?;
                            }
                        }
                    }
                    recog.base.set_state(878);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(144, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_XorExpression ----------------
pub type OC_XorExpressionContextAll<'input> = OC_XorExpressionContext<'input>;

pub type OC_XorExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_XorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_XorExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_XorExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_XorExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_XorExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_XorExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_XorExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_XorExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_XorExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_XorExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_XorExpression }
}
antlr_rust::type_id! {OC_XorExpressionContextExt<'a>}

impl<'input> OC_XorExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_XorExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_XorExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_XorExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_XorExpressionContextExt<'input>>
{
    fn oC_AndExpression_all(&self) -> Vec<Rc<OC_AndExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_AndExpression(&self, i: usize) -> Option<Rc<OC_AndExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token XOR in current rule
    fn XOR_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token XOR, starting from 0.
    /// Returns `None` if number of children corresponding to token XOR is less or equal than `i`.
    fn XOR(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(XOR, i)
    }
}

impl<'input> OC_XorExpressionContextAttrs<'input> for OC_XorExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_XorExpression(
        &mut self,
    ) -> Result<Rc<OC_XorExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_XorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 102, RULE_oC_XorExpression);
        let mut _localctx: Rc<OC_XorExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_AndExpression*/
                recog.base.set_state(879);
                recog.oC_AndExpression()?;

                recog.base.set_state(886);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(145, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(880);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                recog.base.set_state(881);
                                recog.base.match_token(XOR, &mut recog.err_handler)?;

                                recog.base.set_state(882);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                /*InvokeRule oC_AndExpression*/
                                recog.base.set_state(883);
                                recog.oC_AndExpression()?;
                            }
                        }
                    }
                    recog.base.set_state(888);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(145, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_AndExpression ----------------
pub type OC_AndExpressionContextAll<'input> = OC_AndExpressionContext<'input>;

pub type OC_AndExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_AndExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AndExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_AndExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_AndExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_AndExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_AndExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_AndExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_AndExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_AndExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_AndExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_AndExpression }
}
antlr_rust::type_id! {OC_AndExpressionContextExt<'a>}

impl<'input> OC_AndExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_AndExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_AndExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_AndExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_AndExpressionContextExt<'input>>
{
    fn oC_NotExpression_all(&self) -> Vec<Rc<OC_NotExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_NotExpression(&self, i: usize) -> Option<Rc<OC_NotExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token AND in current rule
    fn AND_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
    /// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
    fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AND, i)
    }
}

impl<'input> OC_AndExpressionContextAttrs<'input> for OC_AndExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_AndExpression(
        &mut self,
    ) -> Result<Rc<OC_AndExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_AndExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 104, RULE_oC_AndExpression);
        let mut _localctx: Rc<OC_AndExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_NotExpression*/
                recog.base.set_state(889);
                recog.oC_NotExpression()?;

                recog.base.set_state(896);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(146, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(890);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                recog.base.set_state(891);
                                recog.base.match_token(AND, &mut recog.err_handler)?;

                                recog.base.set_state(892);
                                recog.base.match_token(SP, &mut recog.err_handler)?;

                                /*InvokeRule oC_NotExpression*/
                                recog.base.set_state(893);
                                recog.oC_NotExpression()?;
                            }
                        }
                    }
                    recog.base.set_state(898);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(146, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_NotExpression ----------------
pub type OC_NotExpressionContextAll<'input> = OC_NotExpressionContext<'input>;

pub type OC_NotExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_NotExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NotExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NotExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NotExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_NotExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_NotExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_NotExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_NotExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NotExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_NotExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_NotExpression }
}
antlr_rust::type_id! {OC_NotExpressionContextExt<'a>}

impl<'input> OC_NotExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NotExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NotExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NotExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NotExpressionContextExt<'input>>
{
    fn oC_ComparisonExpression(&self) -> Option<Rc<OC_ComparisonExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token NOT in current rule
    fn NOT_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token NOT, starting from 0.
    /// Returns `None` if number of children corresponding to token NOT is less or equal than `i`.
    fn NOT(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_NotExpressionContextAttrs<'input> for OC_NotExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_NotExpression(
        &mut self,
    ) -> Result<Rc<OC_NotExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_NotExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 106, RULE_oC_NotExpression);
        let mut _localctx: Rc<OC_NotExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(905);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == NOT {
                    {
                        {
                            recog.base.set_state(899);
                            recog.base.match_token(NOT, &mut recog.err_handler)?;

                            recog.base.set_state(901);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(900);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }
                    recog.base.set_state(907);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                /*InvokeRule oC_ComparisonExpression*/
                recog.base.set_state(908);
                recog.oC_ComparisonExpression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ComparisonExpression ----------------
pub type OC_ComparisonExpressionContextAll<'input> = OC_ComparisonExpressionContext<'input>;

pub type OC_ComparisonExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_ComparisonExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ComparisonExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ComparisonExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ComparisonExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ComparisonExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ComparisonExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_ComparisonExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ComparisonExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ComparisonExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ComparisonExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ComparisonExpression }
}
antlr_rust::type_id! {OC_ComparisonExpressionContextExt<'a>}

impl<'input> OC_ComparisonExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ComparisonExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ComparisonExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ComparisonExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ComparisonExpressionContextExt<'input>>
{
    fn oC_AddOrSubtractExpression(&self) -> Option<Rc<OC_AddOrSubtractExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PartialComparisonExpression_all(
        &self,
    ) -> Vec<Rc<OC_PartialComparisonExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PartialComparisonExpression(
        &self,
        i: usize,
    ) -> Option<Rc<OC_PartialComparisonExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_ComparisonExpressionContextAttrs<'input>
    for OC_ComparisonExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ComparisonExpression(
        &mut self,
    ) -> Result<Rc<OC_ComparisonExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ComparisonExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 108, RULE_oC_ComparisonExpression);
        let mut _localctx: Rc<OC_ComparisonExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_AddOrSubtractExpression*/
                recog.base.set_state(910);
                recog.oC_AddOrSubtractExpression()?;

                recog.base.set_state(917);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(150, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(912);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(911);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_PartialComparisonExpression*/
                                recog.base.set_state(914);
                                recog.oC_PartialComparisonExpression()?;
                            }
                        }
                    }
                    recog.base.set_state(919);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(150, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_AddOrSubtractExpression ----------------
pub type OC_AddOrSubtractExpressionContextAll<'input> = OC_AddOrSubtractExpressionContext<'input>;

pub type OC_AddOrSubtractExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_AddOrSubtractExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AddOrSubtractExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_AddOrSubtractExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_AddOrSubtractExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_AddOrSubtractExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_AddOrSubtractExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_AddOrSubtractExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_AddOrSubtractExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_AddOrSubtractExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_AddOrSubtractExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_AddOrSubtractExpression }
}
antlr_rust::type_id! {OC_AddOrSubtractExpressionContextExt<'a>}

impl<'input> OC_AddOrSubtractExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_AddOrSubtractExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_AddOrSubtractExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_AddOrSubtractExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_AddOrSubtractExpressionContextExt<'input>>
{
    fn oC_MultiplyDivideModuloExpression_all(
        &self,
    ) -> Vec<Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_MultiplyDivideModuloExpression(
        &self,
        i: usize,
    ) -> Option<Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_AddOrSubtractExpressionContextAttrs<'input>
    for OC_AddOrSubtractExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_AddOrSubtractExpression(
        &mut self,
    ) -> Result<Rc<OC_AddOrSubtractExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_AddOrSubtractExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 110, RULE_oC_AddOrSubtractExpression);
        let mut _localctx: Rc<OC_AddOrSubtractExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_MultiplyDivideModuloExpression*/
                recog.base.set_state(920);
                recog.oC_MultiplyDivideModuloExpression()?;

                recog.base.set_state(939);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(156, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            recog.base.set_state(937);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(155, &mut recog.base)? {
                                1 => {
                                    {
                                        {
                                            recog.base.set_state(922);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(921);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(924);
                                            recog
                                                .base
                                                .match_token(T__12, &mut recog.err_handler)?;

                                            recog.base.set_state(926);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(925);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_MultiplyDivideModuloExpression*/
                                            recog.base.set_state(928);
                                            recog.oC_MultiplyDivideModuloExpression()?;
                                        }
                                    }
                                }
                                2 => {
                                    {
                                        {
                                            recog.base.set_state(930);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(929);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(932);
                                            recog
                                                .base
                                                .match_token(T__13, &mut recog.err_handler)?;

                                            recog.base.set_state(934);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(933);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_MultiplyDivideModuloExpression*/
                                            recog.base.set_state(936);
                                            recog.oC_MultiplyDivideModuloExpression()?;
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(941);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(156, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_MultiplyDivideModuloExpression ----------------
pub type OC_MultiplyDivideModuloExpressionContextAll<'input> =
    OC_MultiplyDivideModuloExpressionContext<'input>;

pub type OC_MultiplyDivideModuloExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_MultiplyDivideModuloExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MultiplyDivideModuloExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_MultiplyDivideModuloExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_MultiplyDivideModuloExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_MultiplyDivideModuloExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_MultiplyDivideModuloExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_MultiplyDivideModuloExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_MultiplyDivideModuloExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_MultiplyDivideModuloExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_MultiplyDivideModuloExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_MultiplyDivideModuloExpression }
}
antlr_rust::type_id! {OC_MultiplyDivideModuloExpressionContextExt<'a>}

impl<'input> OC_MultiplyDivideModuloExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_MultiplyDivideModuloExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_MultiplyDivideModuloExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_MultiplyDivideModuloExpressionContextExt<'input>>
{
    fn oC_PowerOfExpression_all(&self) -> Vec<Rc<OC_PowerOfExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PowerOfExpression(&self, i: usize) -> Option<Rc<OC_PowerOfExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_MultiplyDivideModuloExpressionContextAttrs<'input>
    for OC_MultiplyDivideModuloExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_MultiplyDivideModuloExpression(
        &mut self,
    ) -> Result<Rc<OC_MultiplyDivideModuloExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_MultiplyDivideModuloExpressionContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog.base.enter_rule(
            _localctx.clone(),
            112,
            RULE_oC_MultiplyDivideModuloExpression,
        );
        let mut _localctx: Rc<OC_MultiplyDivideModuloExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_PowerOfExpression*/
                recog.base.set_state(942);
                recog.oC_PowerOfExpression()?;

                recog.base.set_state(969);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(164, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            recog.base.set_state(967);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(163, &mut recog.base)? {
                                1 => {
                                    {
                                        {
                                            recog.base.set_state(944);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(943);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(946);
                                            recog.base.match_token(T__4, &mut recog.err_handler)?;

                                            recog.base.set_state(948);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(947);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_PowerOfExpression*/
                                            recog.base.set_state(950);
                                            recog.oC_PowerOfExpression()?;
                                        }
                                    }
                                }
                                2 => {
                                    {
                                        {
                                            recog.base.set_state(952);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(951);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(954);
                                            recog
                                                .base
                                                .match_token(T__14, &mut recog.err_handler)?;

                                            recog.base.set_state(956);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(955);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_PowerOfExpression*/
                                            recog.base.set_state(958);
                                            recog.oC_PowerOfExpression()?;
                                        }
                                    }
                                }
                                3 => {
                                    {
                                        {
                                            recog.base.set_state(960);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(959);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            recog.base.set_state(962);
                                            recog
                                                .base
                                                .match_token(T__15, &mut recog.err_handler)?;

                                            recog.base.set_state(964);
                                            recog.err_handler.sync(&mut recog.base)?;
                                            _la = recog.base.input.la(1);
                                            if _la == SP {
                                                {
                                                    recog.base.set_state(963);
                                                    recog
                                                        .base
                                                        .match_token(SP, &mut recog.err_handler)?;
                                                }
                                            }

                                            /*InvokeRule oC_PowerOfExpression*/
                                            recog.base.set_state(966);
                                            recog.oC_PowerOfExpression()?;
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(971);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(164, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PowerOfExpression ----------------
pub type OC_PowerOfExpressionContextAll<'input> = OC_PowerOfExpressionContext<'input>;

pub type OC_PowerOfExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_PowerOfExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PowerOfExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PowerOfExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_PowerOfExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PowerOfExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PowerOfExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PowerOfExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PowerOfExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PowerOfExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PowerOfExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PowerOfExpression }
}
antlr_rust::type_id! {OC_PowerOfExpressionContextExt<'a>}

impl<'input> OC_PowerOfExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PowerOfExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PowerOfExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PowerOfExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PowerOfExpressionContextExt<'input>>
{
    fn oC_UnaryAddOrSubtractExpression_all(
        &self,
    ) -> Vec<Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_UnaryAddOrSubtractExpression(
        &self,
        i: usize,
    ) -> Option<Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_PowerOfExpressionContextAttrs<'input> for OC_PowerOfExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PowerOfExpression(
        &mut self,
    ) -> Result<Rc<OC_PowerOfExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PowerOfExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 114, RULE_oC_PowerOfExpression);
        let mut _localctx: Rc<OC_PowerOfExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_UnaryAddOrSubtractExpression*/
                recog.base.set_state(972);
                recog.oC_UnaryAddOrSubtractExpression()?;

                recog.base.set_state(983);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(167, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(974);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(973);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                recog.base.set_state(976);
                                recog.base.match_token(T__16, &mut recog.err_handler)?;

                                recog.base.set_state(978);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(977);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_UnaryAddOrSubtractExpression*/
                                recog.base.set_state(980);
                                recog.oC_UnaryAddOrSubtractExpression()?;
                            }
                        }
                    }
                    recog.base.set_state(985);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(167, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_UnaryAddOrSubtractExpression ----------------
pub type OC_UnaryAddOrSubtractExpressionContextAll<'input> =
    OC_UnaryAddOrSubtractExpressionContext<'input>;

pub type OC_UnaryAddOrSubtractExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_UnaryAddOrSubtractExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_UnaryAddOrSubtractExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_UnaryAddOrSubtractExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_UnaryAddOrSubtractExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_UnaryAddOrSubtractExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_UnaryAddOrSubtractExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_UnaryAddOrSubtractExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_UnaryAddOrSubtractExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_UnaryAddOrSubtractExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_UnaryAddOrSubtractExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_UnaryAddOrSubtractExpression }
}
antlr_rust::type_id! {OC_UnaryAddOrSubtractExpressionContextExt<'a>}

impl<'input> OC_UnaryAddOrSubtractExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_UnaryAddOrSubtractExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_UnaryAddOrSubtractExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_UnaryAddOrSubtractExpressionContextExt<'input>>
{
    fn oC_StringListNullOperatorExpression(
        &self,
    ) -> Option<Rc<OC_StringListNullOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_UnaryAddOrSubtractExpressionContextAttrs<'input>
    for OC_UnaryAddOrSubtractExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_UnaryAddOrSubtractExpression(
        &mut self,
    ) -> Result<Rc<OC_UnaryAddOrSubtractExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_UnaryAddOrSubtractExpressionContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog
            .base
            .enter_rule(_localctx.clone(), 116, RULE_oC_UnaryAddOrSubtractExpression);
        let mut _localctx: Rc<OC_UnaryAddOrSubtractExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(992);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == T__12 || _la == T__13 {
                    {
                        {
                            recog.base.set_state(986);
                            _la = recog.base.input.la(1);
                            if { !(_la == T__12 || _la == T__13) } {
                                recog.err_handler.recover_inline(&mut recog.base)?;
                            } else {
                                if recog.base.input.la(1) == TOKEN_EOF {
                                    recog.base.matched_eof = true
                                };
                                recog.err_handler.report_match(&mut recog.base);
                                recog.base.consume(&mut recog.err_handler);
                            }
                            recog.base.set_state(988);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(987);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }
                        }
                    }
                    recog.base.set_state(994);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                /*InvokeRule oC_StringListNullOperatorExpression*/
                recog.base.set_state(995);
                recog.oC_StringListNullOperatorExpression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_StringListNullOperatorExpression ----------------
pub type OC_StringListNullOperatorExpressionContextAll<'input> =
    OC_StringListNullOperatorExpressionContext<'input>;

pub type OC_StringListNullOperatorExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_StringListNullOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StringListNullOperatorExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_StringListNullOperatorExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_StringListNullOperatorExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_StringListNullOperatorExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_StringListNullOperatorExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_StringListNullOperatorExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_StringListNullOperatorExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_StringListNullOperatorExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_StringListNullOperatorExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_StringListNullOperatorExpression }
}
antlr_rust::type_id! {OC_StringListNullOperatorExpressionContextExt<'a>}

impl<'input> OC_StringListNullOperatorExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_StringListNullOperatorExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_StringListNullOperatorExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_StringListNullOperatorExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_StringListNullOperatorExpressionContextExt<'input>>
{
    fn oC_PropertyOrLabelsExpression(
        &self,
    ) -> Option<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_StringOperatorExpression_all(
        &self,
    ) -> Vec<Rc<OC_StringOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_StringOperatorExpression(
        &self,
        i: usize,
    ) -> Option<Rc<OC_StringOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_ListOperatorExpression_all(&self) -> Vec<Rc<OC_ListOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_ListOperatorExpression(
        &self,
        i: usize,
    ) -> Option<Rc<OC_ListOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_NullOperatorExpression_all(&self) -> Vec<Rc<OC_NullOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_NullOperatorExpression(
        &self,
        i: usize,
    ) -> Option<Rc<OC_NullOperatorExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_StringListNullOperatorExpressionContextAttrs<'input>
    for OC_StringListNullOperatorExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_StringListNullOperatorExpression(
        &mut self,
    ) -> Result<Rc<OC_StringListNullOperatorExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_StringListNullOperatorExpressionContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog.base.enter_rule(
            _localctx.clone(),
            118,
            RULE_oC_StringListNullOperatorExpression,
        );
        let mut _localctx: Rc<OC_StringListNullOperatorExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_PropertyOrLabelsExpression*/
                recog.base.set_state(997);
                recog.oC_PropertyOrLabelsExpression()?;

                recog.base.set_state(1003);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(171, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            recog.base.set_state(1001);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(170, &mut recog.base)? {
                                1 => {
                                    {
                                        /*InvokeRule oC_StringOperatorExpression*/
                                        recog.base.set_state(998);
                                        recog.oC_StringOperatorExpression()?;
                                    }
                                }
                                2 => {
                                    {
                                        /*InvokeRule oC_ListOperatorExpression*/
                                        recog.base.set_state(999);
                                        recog.oC_ListOperatorExpression()?;
                                    }
                                }
                                3 => {
                                    {
                                        /*InvokeRule oC_NullOperatorExpression*/
                                        recog.base.set_state(1000);
                                        recog.oC_NullOperatorExpression()?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(1005);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(171, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ListOperatorExpression ----------------
pub type OC_ListOperatorExpressionContextAll<'input> = OC_ListOperatorExpressionContext<'input>;

pub type OC_ListOperatorExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_ListOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ListOperatorExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ListOperatorExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ListOperatorExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ListOperatorExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ListOperatorExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_ListOperatorExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ListOperatorExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ListOperatorExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ListOperatorExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ListOperatorExpression }
}
antlr_rust::type_id! {OC_ListOperatorExpressionContextExt<'a>}

impl<'input> OC_ListOperatorExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ListOperatorExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ListOperatorExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ListOperatorExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ListOperatorExpressionContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    fn oC_PropertyOrLabelsExpression(
        &self,
    ) -> Option<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_ListOperatorExpressionContextAttrs<'input>
    for OC_ListOperatorExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ListOperatorExpression(
        &mut self,
    ) -> Result<Rc<OC_ListOperatorExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ListOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 120, RULE_oC_ListOperatorExpression);
        let mut _localctx: Rc<OC_ListOperatorExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1031);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(177, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(1006);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(1007);
                            recog.base.match_token(IN, &mut recog.err_handler)?;

                            recog.base.set_state(1009);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1008);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_PropertyOrLabelsExpression*/
                            recog.base.set_state(1011);
                            recog.oC_PropertyOrLabelsExpression()?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(1013);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1012);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1015);
                            recog.base.match_token(T__7, &mut recog.err_handler)?;

                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(1016);
                            recog.oC_Expression()?;

                            recog.base.set_state(1017);
                            recog.base.match_token(T__8, &mut recog.err_handler)?;
                        }
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        {
                            recog.base.set_state(1020);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1019);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1022);
                            recog.base.match_token(T__7, &mut recog.err_handler)?;

                            recog.base.set_state(1024);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if (((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << T__5)
                                        | (1usize << T__7)
                                        | (1usize << T__12)
                                        | (1usize << T__13)
                                        | (1usize << T__23)
                                        | (1usize << T__25)
                                        | (1usize << ALL)))
                                    != 0)
                                || (((_la - 76) & !0x3f) == 0
                                    && ((1usize << (_la - 76))
                                        & ((1usize << (NOT - 76))
                                            | (1usize << (NULL - 76))
                                            | (1usize << (COUNT - 76))
                                            | (1usize << (ANY - 76))
                                            | (1usize << (NONE - 76))
                                            | (1usize << (SINGLE - 76))
                                            | (1usize << (TRUE - 76))
                                            | (1usize << (FALSE - 76))
                                            | (1usize << (EXISTS - 76))
                                            | (1usize << (CASE - 76))
                                            | (1usize << (StringLiteral - 76))
                                            | (1usize << (HexInteger - 76))
                                            | (1usize << (DecimalInteger - 76))
                                            | (1usize << (OctalInteger - 76))
                                            | (1usize << (HexLetter - 76))
                                            | (1usize << (ExponentDecimalReal - 76))
                                            | (1usize << (RegularDecimalReal - 76))
                                            | (1usize << (FILTER - 76))
                                            | (1usize << (EXTRACT - 76))
                                            | (1usize << (UnescapedSymbolicName - 76))
                                            | (1usize << (EscapedSymbolicName - 76))))
                                        != 0)
                            {
                                {
                                    /*InvokeRule oC_Expression*/
                                    recog.base.set_state(1023);
                                    recog.oC_Expression()?;
                                }
                            }

                            recog.base.set_state(1026);
                            recog.base.match_token(T__11, &mut recog.err_handler)?;

                            recog.base.set_state(1028);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if (((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << T__5)
                                        | (1usize << T__7)
                                        | (1usize << T__12)
                                        | (1usize << T__13)
                                        | (1usize << T__23)
                                        | (1usize << T__25)
                                        | (1usize << ALL)))
                                    != 0)
                                || (((_la - 76) & !0x3f) == 0
                                    && ((1usize << (_la - 76))
                                        & ((1usize << (NOT - 76))
                                            | (1usize << (NULL - 76))
                                            | (1usize << (COUNT - 76))
                                            | (1usize << (ANY - 76))
                                            | (1usize << (NONE - 76))
                                            | (1usize << (SINGLE - 76))
                                            | (1usize << (TRUE - 76))
                                            | (1usize << (FALSE - 76))
                                            | (1usize << (EXISTS - 76))
                                            | (1usize << (CASE - 76))
                                            | (1usize << (StringLiteral - 76))
                                            | (1usize << (HexInteger - 76))
                                            | (1usize << (DecimalInteger - 76))
                                            | (1usize << (OctalInteger - 76))
                                            | (1usize << (HexLetter - 76))
                                            | (1usize << (ExponentDecimalReal - 76))
                                            | (1usize << (RegularDecimalReal - 76))
                                            | (1usize << (FILTER - 76))
                                            | (1usize << (EXTRACT - 76))
                                            | (1usize << (UnescapedSymbolicName - 76))
                                            | (1usize << (EscapedSymbolicName - 76))))
                                        != 0)
                            {
                                {
                                    /*InvokeRule oC_Expression*/
                                    recog.base.set_state(1027);
                                    recog.oC_Expression()?;
                                }
                            }

                            recog.base.set_state(1030);
                            recog.base.match_token(T__8, &mut recog.err_handler)?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_StringOperatorExpression ----------------
pub type OC_StringOperatorExpressionContextAll<'input> = OC_StringOperatorExpressionContext<'input>;

pub type OC_StringOperatorExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_StringOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_StringOperatorExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_StringOperatorExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_StringOperatorExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_StringOperatorExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_StringOperatorExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_StringOperatorExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_StringOperatorExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_StringOperatorExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_StringOperatorExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_StringOperatorExpression }
}
antlr_rust::type_id! {OC_StringOperatorExpressionContextExt<'a>}

impl<'input> OC_StringOperatorExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_StringOperatorExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_StringOperatorExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_StringOperatorExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_StringOperatorExpressionContextExt<'input>>
{
    fn oC_PropertyOrLabelsExpression(
        &self,
    ) -> Option<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token STARTS
    /// Returns `None` if there is no child corresponding to token STARTS
    fn STARTS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STARTS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WITH
    /// Returns `None` if there is no child corresponding to token WITH
    fn WITH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WITH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ENDS
    /// Returns `None` if there is no child corresponding to token ENDS
    fn ENDS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ENDS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CONTAINS
    /// Returns `None` if there is no child corresponding to token CONTAINS
    fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONTAINS, 0)
    }
}

impl<'input> OC_StringOperatorExpressionContextAttrs<'input>
    for OC_StringOperatorExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_StringOperatorExpression(
        &mut self,
    ) -> Result<Rc<OC_StringOperatorExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_StringOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 122, RULE_oC_StringOperatorExpression);
        let mut _localctx: Rc<OC_StringOperatorExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1043);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(178, &mut recog.base)? {
                    1 => {
                        recog.base.set_state(1033);
                        recog.base.match_token(SP, &mut recog.err_handler)?;

                        recog.base.set_state(1034);
                        recog.base.match_token(STARTS, &mut recog.err_handler)?;

                        recog.base.set_state(1035);
                        recog.base.match_token(SP, &mut recog.err_handler)?;

                        recog.base.set_state(1036);
                        recog.base.match_token(WITH, &mut recog.err_handler)?;
                    }
                    2 => {
                        recog.base.set_state(1037);
                        recog.base.match_token(SP, &mut recog.err_handler)?;

                        recog.base.set_state(1038);
                        recog.base.match_token(ENDS, &mut recog.err_handler)?;

                        recog.base.set_state(1039);
                        recog.base.match_token(SP, &mut recog.err_handler)?;

                        recog.base.set_state(1040);
                        recog.base.match_token(WITH, &mut recog.err_handler)?;
                    }
                    3 => {
                        recog.base.set_state(1041);
                        recog.base.match_token(SP, &mut recog.err_handler)?;

                        recog.base.set_state(1042);
                        recog.base.match_token(CONTAINS, &mut recog.err_handler)?;
                    }

                    _ => {}
                }
                recog.base.set_state(1046);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1045);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_PropertyOrLabelsExpression*/
                recog.base.set_state(1048);
                recog.oC_PropertyOrLabelsExpression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_NullOperatorExpression ----------------
pub type OC_NullOperatorExpressionContextAll<'input> = OC_NullOperatorExpressionContext<'input>;

pub type OC_NullOperatorExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_NullOperatorExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NullOperatorExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NullOperatorExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_NullOperatorExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_NullOperatorExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_NullOperatorExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_NullOperatorExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_NullOperatorExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NullOperatorExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_NullOperatorExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_NullOperatorExpression }
}
antlr_rust::type_id! {OC_NullOperatorExpressionContextExt<'a>}

impl<'input> OC_NullOperatorExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NullOperatorExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NullOperatorExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NullOperatorExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NullOperatorExpressionContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token IS
    /// Returns `None` if there is no child corresponding to token IS
    fn IS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NULL
    /// Returns `None` if there is no child corresponding to token NULL
    fn NULL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT, 0)
    }
}

impl<'input> OC_NullOperatorExpressionContextAttrs<'input>
    for OC_NullOperatorExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_NullOperatorExpression(
        &mut self,
    ) -> Result<Rc<OC_NullOperatorExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_NullOperatorExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 124, RULE_oC_NullOperatorExpression);
        let mut _localctx: Rc<OC_NullOperatorExpressionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1060);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(180, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(1050);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(1051);
                            recog.base.match_token(IS, &mut recog.err_handler)?;

                            recog.base.set_state(1052);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(1053);
                            recog.base.match_token(NULL, &mut recog.err_handler)?;
                        }
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(1054);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(1055);
                            recog.base.match_token(IS, &mut recog.err_handler)?;

                            recog.base.set_state(1056);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(1057);
                            recog.base.match_token(NOT, &mut recog.err_handler)?;

                            recog.base.set_state(1058);
                            recog.base.match_token(SP, &mut recog.err_handler)?;

                            recog.base.set_state(1059);
                            recog.base.match_token(NULL, &mut recog.err_handler)?;
                        }
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PropertyOrLabelsExpression ----------------
pub type OC_PropertyOrLabelsExpressionContextAll<'input> =
    OC_PropertyOrLabelsExpressionContext<'input>;

pub type OC_PropertyOrLabelsExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_PropertyOrLabelsExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyOrLabelsExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PropertyOrLabelsExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_PropertyOrLabelsExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PropertyOrLabelsExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PropertyOrLabelsExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_PropertyOrLabelsExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PropertyOrLabelsExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PropertyOrLabelsExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PropertyOrLabelsExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyOrLabelsExpression }
}
antlr_rust::type_id! {OC_PropertyOrLabelsExpressionContextExt<'a>}

impl<'input> OC_PropertyOrLabelsExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PropertyOrLabelsExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PropertyOrLabelsExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PropertyOrLabelsExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PropertyOrLabelsExpressionContextExt<'input>>
{
    fn oC_Atom(&self) -> Option<Rc<OC_AtomContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PropertyLookup_all(&self) -> Vec<Rc<OC_PropertyLookupContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PropertyLookup(&self, i: usize) -> Option<Rc<OC_PropertyLookupContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_NodeLabels(&self) -> Option<Rc<OC_NodeLabelsContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_PropertyOrLabelsExpressionContextAttrs<'input>
    for OC_PropertyOrLabelsExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PropertyOrLabelsExpression(
        &mut self,
    ) -> Result<Rc<OC_PropertyOrLabelsExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_PropertyOrLabelsExpressionContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog
            .base
            .enter_rule(_localctx.clone(), 126, RULE_oC_PropertyOrLabelsExpression);
        let mut _localctx: Rc<OC_PropertyOrLabelsExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_Atom*/
                recog.base.set_state(1062);
                recog.oC_Atom()?;

                recog.base.set_state(1069);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(182, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                recog.base.set_state(1064);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(1063);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_PropertyLookup*/
                                recog.base.set_state(1066);
                                recog.oC_PropertyLookup()?;
                            }
                        }
                    }
                    recog.base.set_state(1071);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(182, &mut recog.base)?;
                }
                recog.base.set_state(1076);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(184, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(1073);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1072);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_NodeLabels*/
                            recog.base.set_state(1075);
                            recog.oC_NodeLabels()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Atom ----------------
pub type OC_AtomContextAll<'input> = OC_AtomContext<'input>;

pub type OC_AtomContext<'input> = BaseParserRuleContext<'input, OC_AtomContextExt<'input>>;

#[derive(Clone)]
pub struct OC_AtomContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_AtomContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_AtomContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Atom(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Atom(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_AtomContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Atom(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_AtomContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Atom
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Atom }
}
antlr_rust::type_id! {OC_AtomContextExt<'a>}

impl<'input> OC_AtomContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_AtomContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_AtomContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_AtomContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_AtomContextExt<'input>>
{
    fn oC_Literal(&self) -> Option<Rc<OC_LiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Parameter(&self) -> Option<Rc<OC_ParameterContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_CaseExpression(&self) -> Option<Rc<OC_CaseExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COUNT
    /// Returns `None` if there is no child corresponding to token COUNT
    fn COUNT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COUNT, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_ListComprehension(&self) -> Option<Rc<OC_ListComprehensionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PatternComprehension(&self) -> Option<Rc<OC_PatternComprehensionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ALL
    /// Returns `None` if there is no child corresponding to token ALL
    fn ALL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ALL, 0)
    }
    fn oC_FilterExpression(&self) -> Option<Rc<OC_FilterExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ANY
    /// Returns `None` if there is no child corresponding to token ANY
    fn ANY(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ANY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NONE
    /// Returns `None` if there is no child corresponding to token NONE
    fn NONE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SINGLE
    /// Returns `None` if there is no child corresponding to token SINGLE
    fn SINGLE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SINGLE, 0)
    }
    fn oC_RelationshipsPattern(&self) -> Option<Rc<OC_RelationshipsPatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_ParenthesizedExpression(&self) -> Option<Rc<OC_ParenthesizedExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_FunctionInvocation(&self) -> Option<Rc<OC_FunctionInvocationContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_AtomContextAttrs<'input> for OC_AtomContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Atom(&mut self) -> Result<Rc<OC_AtomContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 128, RULE_oC_Atom);
        let mut _localctx: Rc<OC_AtomContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1156);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(200, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_Literal*/
                        recog.base.set_state(1078);
                        recog.oC_Literal()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_Parameter*/
                        recog.base.set_state(1079);
                        recog.oC_Parameter()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule oC_CaseExpression*/
                        recog.base.set_state(1080);
                        recog.oC_CaseExpression()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        {
                            recog.base.set_state(1081);
                            recog.base.match_token(COUNT, &mut recog.err_handler)?;

                            recog.base.set_state(1083);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1082);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1085);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            recog.base.set_state(1087);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1086);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1089);
                            recog.base.match_token(T__4, &mut recog.err_handler)?;

                            recog.base.set_state(1091);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1090);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1093);
                            recog.base.match_token(T__6, &mut recog.err_handler)?;
                        }
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule oC_ListComprehension*/
                        recog.base.set_state(1094);
                        recog.oC_ListComprehension()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule oC_PatternComprehension*/
                        recog.base.set_state(1095);
                        recog.oC_PatternComprehension()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        {
                            recog.base.set_state(1096);
                            recog.base.match_token(ALL, &mut recog.err_handler)?;

                            recog.base.set_state(1098);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1097);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1100);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            recog.base.set_state(1102);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1101);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_FilterExpression*/
                            recog.base.set_state(1104);
                            recog.oC_FilterExpression()?;

                            recog.base.set_state(1106);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1105);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1108);
                            recog.base.match_token(T__6, &mut recog.err_handler)?;
                        }
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        {
                            recog.base.set_state(1110);
                            recog.base.match_token(ANY, &mut recog.err_handler)?;

                            recog.base.set_state(1112);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1111);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1114);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            recog.base.set_state(1116);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1115);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_FilterExpression*/
                            recog.base.set_state(1118);
                            recog.oC_FilterExpression()?;

                            recog.base.set_state(1120);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1119);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1122);
                            recog.base.match_token(T__6, &mut recog.err_handler)?;
                        }
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        {
                            recog.base.set_state(1124);
                            recog.base.match_token(NONE, &mut recog.err_handler)?;

                            recog.base.set_state(1126);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1125);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1128);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            recog.base.set_state(1130);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1129);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_FilterExpression*/
                            recog.base.set_state(1132);
                            recog.oC_FilterExpression()?;

                            recog.base.set_state(1134);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1133);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1136);
                            recog.base.match_token(T__6, &mut recog.err_handler)?;
                        }
                    }
                }
                10 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 10);
                    recog.base.enter_outer_alt(None, 10);
                    {
                        {
                            recog.base.set_state(1138);
                            recog.base.match_token(SINGLE, &mut recog.err_handler)?;

                            recog.base.set_state(1140);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1139);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1142);
                            recog.base.match_token(T__5, &mut recog.err_handler)?;

                            recog.base.set_state(1144);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1143);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_FilterExpression*/
                            recog.base.set_state(1146);
                            recog.oC_FilterExpression()?;

                            recog.base.set_state(1148);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1147);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1150);
                            recog.base.match_token(T__6, &mut recog.err_handler)?;
                        }
                    }
                }
                11 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 11);
                    recog.base.enter_outer_alt(None, 11);
                    {
                        /*InvokeRule oC_RelationshipsPattern*/
                        recog.base.set_state(1152);
                        recog.oC_RelationshipsPattern()?;
                    }
                }
                12 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 12);
                    recog.base.enter_outer_alt(None, 12);
                    {
                        /*InvokeRule oC_ParenthesizedExpression*/
                        recog.base.set_state(1153);
                        recog.oC_ParenthesizedExpression()?;
                    }
                }
                13 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 13);
                    recog.base.enter_outer_alt(None, 13);
                    {
                        /*InvokeRule oC_FunctionInvocation*/
                        recog.base.set_state(1154);
                        recog.oC_FunctionInvocation()?;
                    }
                }
                14 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 14);
                    recog.base.enter_outer_alt(None, 14);
                    {
                        /*InvokeRule oC_Variable*/
                        recog.base.set_state(1155);
                        recog.oC_Variable()?;
                    }
                }

                _ => {}
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Literal ----------------
pub type OC_LiteralContextAll<'input> = OC_LiteralContext<'input>;

pub type OC_LiteralContext<'input> = BaseParserRuleContext<'input, OC_LiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_LiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Literal(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Literal(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_LiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Literal(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_LiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Literal
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Literal }
}
antlr_rust::type_id! {OC_LiteralContextExt<'a>}

impl<'input> OC_LiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_LiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_LiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_LiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_LiteralContextExt<'input>>
{
    fn oC_NumberLiteral(&self) -> Option<Rc<OC_NumberLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token StringLiteral
    /// Returns `None` if there is no child corresponding to token StringLiteral
    fn StringLiteral(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(StringLiteral, 0)
    }
    fn oC_BooleanLiteral(&self) -> Option<Rc<OC_BooleanLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NULL
    /// Returns `None` if there is no child corresponding to token NULL
    fn NULL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NULL, 0)
    }
    fn oC_MapLiteral(&self) -> Option<Rc<OC_MapLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_ListLiteral(&self) -> Option<Rc<OC_ListLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_LiteralContextAttrs<'input> for OC_LiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Literal(&mut self) -> Result<Rc<OC_LiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_LiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 130, RULE_oC_Literal);
        let mut _localctx: Rc<OC_LiteralContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1164);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                HexInteger | DecimalInteger | OctalInteger | ExponentDecimalReal
                | RegularDecimalReal => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_NumberLiteral*/
                        recog.base.set_state(1158);
                        recog.oC_NumberLiteral()?;
                    }
                }

                StringLiteral => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1159);
                        recog
                            .base
                            .match_token(StringLiteral, &mut recog.err_handler)?;
                    }
                }

                TRUE | FALSE => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule oC_BooleanLiteral*/
                        recog.base.set_state(1160);
                        recog.oC_BooleanLiteral()?;
                    }
                }

                NULL => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        recog.base.set_state(1161);
                        recog.base.match_token(NULL, &mut recog.err_handler)?;
                    }
                }

                T__23 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule oC_MapLiteral*/
                        recog.base.set_state(1162);
                        recog.oC_MapLiteral()?;
                    }
                }

                T__7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule oC_ListLiteral*/
                        recog.base.set_state(1163);
                        recog.oC_ListLiteral()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_BooleanLiteral ----------------
pub type OC_BooleanLiteralContextAll<'input> = OC_BooleanLiteralContext<'input>;

pub type OC_BooleanLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_BooleanLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_BooleanLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_BooleanLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_BooleanLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_BooleanLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_BooleanLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_BooleanLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_BooleanLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_BooleanLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_BooleanLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_BooleanLiteral }
}
antlr_rust::type_id! {OC_BooleanLiteralContextExt<'a>}

impl<'input> OC_BooleanLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_BooleanLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_BooleanLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_BooleanLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_BooleanLiteralContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
}

impl<'input> OC_BooleanLiteralContextAttrs<'input> for OC_BooleanLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_BooleanLiteral(
        &mut self,
    ) -> Result<Rc<OC_BooleanLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_BooleanLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 132, RULE_oC_BooleanLiteral);
        let mut _localctx: Rc<OC_BooleanLiteralContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1166);
                _la = recog.base.input.la(1);
                if { !(_la == TRUE || _la == FALSE) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ListLiteral ----------------
pub type OC_ListLiteralContextAll<'input> = OC_ListLiteralContext<'input>;

pub type OC_ListLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_ListLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ListLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ListLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ListLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ListLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ListLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ListLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ListLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ListLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ListLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ListLiteral }
}
antlr_rust::type_id! {OC_ListLiteralContextExt<'a>}

impl<'input> OC_ListLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ListLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ListLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ListLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ListLiteralContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_ListLiteralContextAttrs<'input> for OC_ListLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ListLiteral(&mut self) -> Result<Rc<OC_ListLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ListLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 134, RULE_oC_ListLiteral);
        let mut _localctx: Rc<OC_ListLiteralContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1168);
                recog.base.match_token(T__7, &mut recog.err_handler)?;

                recog.base.set_state(1170);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1169);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1189);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__5)
                            | (1usize << T__7)
                            | (1usize << T__12)
                            | (1usize << T__13)
                            | (1usize << T__23)
                            | (1usize << T__25)
                            | (1usize << ALL)))
                        != 0)
                    || (((_la - 76) & !0x3f) == 0
                        && ((1usize << (_la - 76))
                            & ((1usize << (NOT - 76))
                                | (1usize << (NULL - 76))
                                | (1usize << (COUNT - 76))
                                | (1usize << (ANY - 76))
                                | (1usize << (NONE - 76))
                                | (1usize << (SINGLE - 76))
                                | (1usize << (TRUE - 76))
                                | (1usize << (FALSE - 76))
                                | (1usize << (EXISTS - 76))
                                | (1usize << (CASE - 76))
                                | (1usize << (StringLiteral - 76))
                                | (1usize << (HexInteger - 76))
                                | (1usize << (DecimalInteger - 76))
                                | (1usize << (OctalInteger - 76))
                                | (1usize << (HexLetter - 76))
                                | (1usize << (ExponentDecimalReal - 76))
                                | (1usize << (RegularDecimalReal - 76))
                                | (1usize << (FILTER - 76))
                                | (1usize << (EXTRACT - 76))
                                | (1usize << (UnescapedSymbolicName - 76))
                                | (1usize << (EscapedSymbolicName - 76))))
                            != 0)
                {
                    {
                        /*InvokeRule oC_Expression*/
                        recog.base.set_state(1172);
                        recog.oC_Expression()?;

                        recog.base.set_state(1174);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1173);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1186);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == T__1 {
                            {
                                {
                                    recog.base.set_state(1176);
                                    recog.base.match_token(T__1, &mut recog.err_handler)?;

                                    recog.base.set_state(1178);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1177);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }

                                    /*InvokeRule oC_Expression*/
                                    recog.base.set_state(1180);
                                    recog.oC_Expression()?;

                                    recog.base.set_state(1182);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1181);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }
                                }
                            }
                            recog.base.set_state(1188);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(1191);
                recog.base.match_token(T__8, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PartialComparisonExpression ----------------
pub type OC_PartialComparisonExpressionContextAll<'input> =
    OC_PartialComparisonExpressionContext<'input>;

pub type OC_PartialComparisonExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_PartialComparisonExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PartialComparisonExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PartialComparisonExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_PartialComparisonExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PartialComparisonExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PartialComparisonExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_PartialComparisonExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PartialComparisonExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PartialComparisonExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PartialComparisonExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PartialComparisonExpression }
}
antlr_rust::type_id! {OC_PartialComparisonExpressionContextExt<'a>}

impl<'input> OC_PartialComparisonExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PartialComparisonExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PartialComparisonExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PartialComparisonExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PartialComparisonExpressionContextExt<'input>>
{
    fn oC_AddOrSubtractExpression(&self) -> Option<Rc<OC_AddOrSubtractExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_PartialComparisonExpressionContextAttrs<'input>
    for OC_PartialComparisonExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PartialComparisonExpression(
        &mut self,
    ) -> Result<Rc<OC_PartialComparisonExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_PartialComparisonExpressionContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog
            .base
            .enter_rule(_localctx.clone(), 136, RULE_oC_PartialComparisonExpression);
        let mut _localctx: Rc<OC_PartialComparisonExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1223);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                T__2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            recog.base.set_state(1193);
                            recog.base.match_token(T__2, &mut recog.err_handler)?;

                            recog.base.set_state(1195);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1194);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AddOrSubtractExpression*/
                            recog.base.set_state(1197);
                            recog.oC_AddOrSubtractExpression()?;
                        }
                    }
                }

                T__17 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        {
                            recog.base.set_state(1198);
                            recog.base.match_token(T__17, &mut recog.err_handler)?;

                            recog.base.set_state(1200);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1199);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AddOrSubtractExpression*/
                            recog.base.set_state(1202);
                            recog.oC_AddOrSubtractExpression()?;
                        }
                    }
                }

                T__18 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        {
                            recog.base.set_state(1203);
                            recog.base.match_token(T__18, &mut recog.err_handler)?;

                            recog.base.set_state(1205);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1204);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AddOrSubtractExpression*/
                            recog.base.set_state(1207);
                            recog.oC_AddOrSubtractExpression()?;
                        }
                    }
                }

                T__19 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        {
                            recog.base.set_state(1208);
                            recog.base.match_token(T__19, &mut recog.err_handler)?;

                            recog.base.set_state(1210);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1209);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AddOrSubtractExpression*/
                            recog.base.set_state(1212);
                            recog.oC_AddOrSubtractExpression()?;
                        }
                    }
                }

                T__20 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        {
                            recog.base.set_state(1213);
                            recog.base.match_token(T__20, &mut recog.err_handler)?;

                            recog.base.set_state(1215);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1214);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AddOrSubtractExpression*/
                            recog.base.set_state(1217);
                            recog.oC_AddOrSubtractExpression()?;
                        }
                    }
                }

                T__21 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        {
                            recog.base.set_state(1218);
                            recog.base.match_token(T__21, &mut recog.err_handler)?;

                            recog.base.set_state(1220);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1219);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_AddOrSubtractExpression*/
                            recog.base.set_state(1222);
                            recog.oC_AddOrSubtractExpression()?;
                        }
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ParenthesizedExpression ----------------
pub type OC_ParenthesizedExpressionContextAll<'input> = OC_ParenthesizedExpressionContext<'input>;

pub type OC_ParenthesizedExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_ParenthesizedExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ParenthesizedExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ParenthesizedExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ParenthesizedExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ParenthesizedExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ParenthesizedExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_ParenthesizedExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ParenthesizedExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ParenthesizedExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ParenthesizedExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ParenthesizedExpression }
}
antlr_rust::type_id! {OC_ParenthesizedExpressionContextExt<'a>}

impl<'input> OC_ParenthesizedExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ParenthesizedExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ParenthesizedExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ParenthesizedExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ParenthesizedExpressionContextExt<'input>>
{
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_ParenthesizedExpressionContextAttrs<'input>
    for OC_ParenthesizedExpressionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ParenthesizedExpression(
        &mut self,
    ) -> Result<Rc<OC_ParenthesizedExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ParenthesizedExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 138, RULE_oC_ParenthesizedExpression);
        let mut _localctx: Rc<OC_ParenthesizedExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1225);
                recog.base.match_token(T__5, &mut recog.err_handler)?;

                recog.base.set_state(1227);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1226);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Expression*/
                recog.base.set_state(1229);
                recog.oC_Expression()?;

                recog.base.set_state(1231);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1230);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1233);
                recog.base.match_token(T__6, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RelationshipsPattern ----------------
pub type OC_RelationshipsPatternContextAll<'input> = OC_RelationshipsPatternContext<'input>;

pub type OC_RelationshipsPatternContext<'input> =
    BaseParserRuleContext<'input, OC_RelationshipsPatternContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RelationshipsPatternContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RelationshipsPatternContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_RelationshipsPatternContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RelationshipsPattern(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RelationshipsPattern(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_RelationshipsPatternContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RelationshipsPattern(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RelationshipsPatternContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RelationshipsPattern
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RelationshipsPattern }
}
antlr_rust::type_id! {OC_RelationshipsPatternContextExt<'a>}

impl<'input> OC_RelationshipsPatternContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RelationshipsPatternContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RelationshipsPatternContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RelationshipsPatternContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RelationshipsPatternContextExt<'input>>
{
    fn oC_NodePattern(&self) -> Option<Rc<OC_NodePatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PatternElementChain_all(&self) -> Vec<Rc<OC_PatternElementChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PatternElementChain(
        &self,
        i: usize,
    ) -> Option<Rc<OC_PatternElementChainContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_RelationshipsPatternContextAttrs<'input>
    for OC_RelationshipsPatternContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RelationshipsPattern(
        &mut self,
    ) -> Result<Rc<OC_RelationshipsPatternContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RelationshipsPatternContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 140, RULE_oC_RelationshipsPattern);
        let mut _localctx: Rc<OC_RelationshipsPatternContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_NodePattern*/
                recog.base.set_state(1235);
                recog.oC_NodePattern()?;

                recog.base.set_state(1240);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1;
                loop {
                    match _alt {
                        x if x == 1 => {
                            {
                                recog.base.set_state(1237);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(1236);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_PatternElementChain*/
                                recog.base.set_state(1239);
                                recog.oC_PatternElementChain()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(1242);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(218, &mut recog.base)?;
                    if _alt == 2 || _alt == INVALID_ALT {
                        break;
                    }
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_FilterExpression ----------------
pub type OC_FilterExpressionContextAll<'input> = OC_FilterExpressionContext<'input>;

pub type OC_FilterExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_FilterExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_FilterExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_FilterExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_FilterExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_FilterExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_FilterExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_FilterExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_FilterExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_FilterExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_FilterExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_FilterExpression }
}
antlr_rust::type_id! {OC_FilterExpressionContextExt<'a>}

impl<'input> OC_FilterExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_FilterExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_FilterExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_FilterExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_FilterExpressionContextExt<'input>>
{
    fn oC_IdInColl(&self) -> Option<Rc<OC_IdInCollContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Where(&self) -> Option<Rc<OC_WhereContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_FilterExpressionContextAttrs<'input> for OC_FilterExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_FilterExpression(
        &mut self,
    ) -> Result<Rc<OC_FilterExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_FilterExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 142, RULE_oC_FilterExpression);
        let mut _localctx: Rc<OC_FilterExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_IdInColl*/
                recog.base.set_state(1244);
                recog.oC_IdInColl()?;

                recog.base.set_state(1249);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(220, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(1246);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1245);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Where*/
                            recog.base.set_state(1248);
                            recog.oC_Where()?;
                        }
                    }

                    _ => {}
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_IdInColl ----------------
pub type OC_IdInCollContextAll<'input> = OC_IdInCollContext<'input>;

pub type OC_IdInCollContext<'input> = BaseParserRuleContext<'input, OC_IdInCollContextExt<'input>>;

#[derive(Clone)]
pub struct OC_IdInCollContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_IdInCollContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_IdInCollContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_IdInColl(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_IdInColl(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_IdInCollContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_IdInColl(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_IdInCollContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_IdInColl
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_IdInColl }
}
antlr_rust::type_id! {OC_IdInCollContextExt<'a>}

impl<'input> OC_IdInCollContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_IdInCollContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_IdInCollContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_IdInCollContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_IdInCollContextExt<'input>>
{
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_IdInCollContextAttrs<'input> for OC_IdInCollContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_IdInColl(&mut self) -> Result<Rc<OC_IdInCollContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_IdInCollContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 144, RULE_oC_IdInColl);
        let mut _localctx: Rc<OC_IdInCollContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_Variable*/
                recog.base.set_state(1251);
                recog.oC_Variable()?;

                recog.base.set_state(1252);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                recog.base.set_state(1253);
                recog.base.match_token(IN, &mut recog.err_handler)?;

                recog.base.set_state(1254);
                recog.base.match_token(SP, &mut recog.err_handler)?;

                /*InvokeRule oC_Expression*/
                recog.base.set_state(1255);
                recog.oC_Expression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_FunctionInvocation ----------------
pub type OC_FunctionInvocationContextAll<'input> = OC_FunctionInvocationContext<'input>;

pub type OC_FunctionInvocationContext<'input> =
    BaseParserRuleContext<'input, OC_FunctionInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct OC_FunctionInvocationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_FunctionInvocationContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_FunctionInvocationContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_FunctionInvocation(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_FunctionInvocation(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_FunctionInvocationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_FunctionInvocation(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_FunctionInvocationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_FunctionInvocation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_FunctionInvocation }
}
antlr_rust::type_id! {OC_FunctionInvocationContextExt<'a>}

impl<'input> OC_FunctionInvocationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_FunctionInvocationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_FunctionInvocationContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_FunctionInvocationContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_FunctionInvocationContextExt<'input>>
{
    fn oC_FunctionName(&self) -> Option<Rc<OC_FunctionNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token DISTINCT
    /// Returns `None` if there is no child corresponding to token DISTINCT
    fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DISTINCT, 0)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_FunctionInvocationContextAttrs<'input> for OC_FunctionInvocationContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_FunctionInvocation(
        &mut self,
    ) -> Result<Rc<OC_FunctionInvocationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_FunctionInvocationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 146, RULE_oC_FunctionInvocation);
        let mut _localctx: Rc<OC_FunctionInvocationContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_FunctionName*/
                recog.base.set_state(1257);
                recog.oC_FunctionName()?;

                recog.base.set_state(1259);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1258);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1261);
                recog.base.match_token(T__5, &mut recog.err_handler)?;

                recog.base.set_state(1263);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1262);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1269);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == DISTINCT {
                    {
                        recog.base.set_state(1265);
                        recog.base.match_token(DISTINCT, &mut recog.err_handler)?;

                        recog.base.set_state(1267);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1266);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(1288);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__5)
                            | (1usize << T__7)
                            | (1usize << T__12)
                            | (1usize << T__13)
                            | (1usize << T__23)
                            | (1usize << T__25)
                            | (1usize << ALL)))
                        != 0)
                    || (((_la - 76) & !0x3f) == 0
                        && ((1usize << (_la - 76))
                            & ((1usize << (NOT - 76))
                                | (1usize << (NULL - 76))
                                | (1usize << (COUNT - 76))
                                | (1usize << (ANY - 76))
                                | (1usize << (NONE - 76))
                                | (1usize << (SINGLE - 76))
                                | (1usize << (TRUE - 76))
                                | (1usize << (FALSE - 76))
                                | (1usize << (EXISTS - 76))
                                | (1usize << (CASE - 76))
                                | (1usize << (StringLiteral - 76))
                                | (1usize << (HexInteger - 76))
                                | (1usize << (DecimalInteger - 76))
                                | (1usize << (OctalInteger - 76))
                                | (1usize << (HexLetter - 76))
                                | (1usize << (ExponentDecimalReal - 76))
                                | (1usize << (RegularDecimalReal - 76))
                                | (1usize << (FILTER - 76))
                                | (1usize << (EXTRACT - 76))
                                | (1usize << (UnescapedSymbolicName - 76))
                                | (1usize << (EscapedSymbolicName - 76))))
                            != 0)
                {
                    {
                        /*InvokeRule oC_Expression*/
                        recog.base.set_state(1271);
                        recog.oC_Expression()?;

                        recog.base.set_state(1273);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1272);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1285);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == T__1 {
                            {
                                {
                                    recog.base.set_state(1275);
                                    recog.base.match_token(T__1, &mut recog.err_handler)?;

                                    recog.base.set_state(1277);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1276);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }

                                    /*InvokeRule oC_Expression*/
                                    recog.base.set_state(1279);
                                    recog.oC_Expression()?;

                                    recog.base.set_state(1281);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1280);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }
                                }
                            }
                            recog.base.set_state(1287);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(1290);
                recog.base.match_token(T__6, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_FunctionName ----------------
pub type OC_FunctionNameContextAll<'input> = OC_FunctionNameContext<'input>;

pub type OC_FunctionNameContext<'input> =
    BaseParserRuleContext<'input, OC_FunctionNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_FunctionNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_FunctionNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_FunctionNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_FunctionName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_FunctionName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_FunctionNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_FunctionName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_FunctionNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_FunctionName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_FunctionName }
}
antlr_rust::type_id! {OC_FunctionNameContextExt<'a>}

impl<'input> OC_FunctionNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_FunctionNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_FunctionNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_FunctionNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_FunctionNameContextExt<'input>>
{
    fn oC_Namespace(&self) -> Option<Rc<OC_NamespaceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EXISTS
    /// Returns `None` if there is no child corresponding to token EXISTS
    fn EXISTS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXISTS, 0)
    }
}

impl<'input> OC_FunctionNameContextAttrs<'input> for OC_FunctionNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_FunctionName(&mut self) -> Result<Rc<OC_FunctionNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_FunctionNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 148, RULE_oC_FunctionName);
        let mut _localctx: Rc<OC_FunctionNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1296);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                COUNT
                | ANY
                | NONE
                | SINGLE
                | HexLetter
                | FILTER
                | EXTRACT
                | UnescapedSymbolicName
                | EscapedSymbolicName => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        {
                            /*InvokeRule oC_Namespace*/
                            recog.base.set_state(1292);
                            recog.oC_Namespace()?;

                            /*InvokeRule oC_SymbolicName*/
                            recog.base.set_state(1293);
                            recog.oC_SymbolicName()?;
                        }
                    }
                }

                EXISTS => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        recog.base.set_state(1295);
                        recog.base.match_token(EXISTS, &mut recog.err_handler)?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ExplicitProcedureInvocation ----------------
pub type OC_ExplicitProcedureInvocationContextAll<'input> =
    OC_ExplicitProcedureInvocationContext<'input>;

pub type OC_ExplicitProcedureInvocationContext<'input> =
    BaseParserRuleContext<'input, OC_ExplicitProcedureInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ExplicitProcedureInvocationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ExplicitProcedureInvocationContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ExplicitProcedureInvocationContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ExplicitProcedureInvocation(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ExplicitProcedureInvocation(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_ExplicitProcedureInvocationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ExplicitProcedureInvocation(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ExplicitProcedureInvocationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ExplicitProcedureInvocation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ExplicitProcedureInvocation }
}
antlr_rust::type_id! {OC_ExplicitProcedureInvocationContextExt<'a>}

impl<'input> OC_ExplicitProcedureInvocationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ExplicitProcedureInvocationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ExplicitProcedureInvocationContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ExplicitProcedureInvocationContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ExplicitProcedureInvocationContextExt<'input>>
{
    fn oC_ProcedureName(&self) -> Option<Rc<OC_ProcedureNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_ExplicitProcedureInvocationContextAttrs<'input>
    for OC_ExplicitProcedureInvocationContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ExplicitProcedureInvocation(
        &mut self,
    ) -> Result<Rc<OC_ExplicitProcedureInvocationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_ExplicitProcedureInvocationContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog
            .base
            .enter_rule(_localctx.clone(), 150, RULE_oC_ExplicitProcedureInvocation);
        let mut _localctx: Rc<OC_ExplicitProcedureInvocationContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_ProcedureName*/
                recog.base.set_state(1298);
                recog.oC_ProcedureName()?;

                recog.base.set_state(1300);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1299);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1302);
                recog.base.match_token(T__5, &mut recog.err_handler)?;

                recog.base.set_state(1304);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1303);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1323);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << T__5)
                            | (1usize << T__7)
                            | (1usize << T__12)
                            | (1usize << T__13)
                            | (1usize << T__23)
                            | (1usize << T__25)
                            | (1usize << ALL)))
                        != 0)
                    || (((_la - 76) & !0x3f) == 0
                        && ((1usize << (_la - 76))
                            & ((1usize << (NOT - 76))
                                | (1usize << (NULL - 76))
                                | (1usize << (COUNT - 76))
                                | (1usize << (ANY - 76))
                                | (1usize << (NONE - 76))
                                | (1usize << (SINGLE - 76))
                                | (1usize << (TRUE - 76))
                                | (1usize << (FALSE - 76))
                                | (1usize << (EXISTS - 76))
                                | (1usize << (CASE - 76))
                                | (1usize << (StringLiteral - 76))
                                | (1usize << (HexInteger - 76))
                                | (1usize << (DecimalInteger - 76))
                                | (1usize << (OctalInteger - 76))
                                | (1usize << (HexLetter - 76))
                                | (1usize << (ExponentDecimalReal - 76))
                                | (1usize << (RegularDecimalReal - 76))
                                | (1usize << (FILTER - 76))
                                | (1usize << (EXTRACT - 76))
                                | (1usize << (UnescapedSymbolicName - 76))
                                | (1usize << (EscapedSymbolicName - 76))))
                            != 0)
                {
                    {
                        /*InvokeRule oC_Expression*/
                        recog.base.set_state(1306);
                        recog.oC_Expression()?;

                        recog.base.set_state(1308);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1307);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1320);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == T__1 {
                            {
                                {
                                    recog.base.set_state(1310);
                                    recog.base.match_token(T__1, &mut recog.err_handler)?;

                                    recog.base.set_state(1312);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1311);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }

                                    /*InvokeRule oC_Expression*/
                                    recog.base.set_state(1314);
                                    recog.oC_Expression()?;

                                    recog.base.set_state(1316);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1315);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }
                                }
                            }
                            recog.base.set_state(1322);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(1325);
                recog.base.match_token(T__6, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ImplicitProcedureInvocation ----------------
pub type OC_ImplicitProcedureInvocationContextAll<'input> =
    OC_ImplicitProcedureInvocationContext<'input>;

pub type OC_ImplicitProcedureInvocationContext<'input> =
    BaseParserRuleContext<'input, OC_ImplicitProcedureInvocationContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ImplicitProcedureInvocationContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ImplicitProcedureInvocationContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ImplicitProcedureInvocationContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ImplicitProcedureInvocation(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ImplicitProcedureInvocation(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_ImplicitProcedureInvocationContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ImplicitProcedureInvocation(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ImplicitProcedureInvocationContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ImplicitProcedureInvocation
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ImplicitProcedureInvocation }
}
antlr_rust::type_id! {OC_ImplicitProcedureInvocationContextExt<'a>}

impl<'input> OC_ImplicitProcedureInvocationContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ImplicitProcedureInvocationContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ImplicitProcedureInvocationContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ImplicitProcedureInvocationContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ImplicitProcedureInvocationContextExt<'input>>
{
    fn oC_ProcedureName(&self) -> Option<Rc<OC_ProcedureNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ImplicitProcedureInvocationContextAttrs<'input>
    for OC_ImplicitProcedureInvocationContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ImplicitProcedureInvocation(
        &mut self,
    ) -> Result<Rc<OC_ImplicitProcedureInvocationContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_ImplicitProcedureInvocationContextExt::new(
            _parentctx.clone(),
            recog.base.get_state(),
        );
        recog
            .base
            .enter_rule(_localctx.clone(), 152, RULE_oC_ImplicitProcedureInvocation);
        let mut _localctx: Rc<OC_ImplicitProcedureInvocationContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_ProcedureName*/
                recog.base.set_state(1327);
                recog.oC_ProcedureName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ProcedureResultField ----------------
pub type OC_ProcedureResultFieldContextAll<'input> = OC_ProcedureResultFieldContext<'input>;

pub type OC_ProcedureResultFieldContext<'input> =
    BaseParserRuleContext<'input, OC_ProcedureResultFieldContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProcedureResultFieldContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ProcedureResultFieldContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ProcedureResultFieldContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ProcedureResultField(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ProcedureResultField(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_ProcedureResultFieldContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ProcedureResultField(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ProcedureResultFieldContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ProcedureResultField
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProcedureResultField }
}
antlr_rust::type_id! {OC_ProcedureResultFieldContextExt<'a>}

impl<'input> OC_ProcedureResultFieldContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ProcedureResultFieldContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ProcedureResultFieldContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ProcedureResultFieldContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ProcedureResultFieldContextExt<'input>>
{
    fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ProcedureResultFieldContextAttrs<'input>
    for OC_ProcedureResultFieldContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ProcedureResultField(
        &mut self,
    ) -> Result<Rc<OC_ProcedureResultFieldContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ProcedureResultFieldContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 154, RULE_oC_ProcedureResultField);
        let mut _localctx: Rc<OC_ProcedureResultFieldContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_SymbolicName*/
                recog.base.set_state(1329);
                recog.oC_SymbolicName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ProcedureName ----------------
pub type OC_ProcedureNameContextAll<'input> = OC_ProcedureNameContext<'input>;

pub type OC_ProcedureNameContext<'input> =
    BaseParserRuleContext<'input, OC_ProcedureNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ProcedureNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ProcedureNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ProcedureNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ProcedureName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ProcedureName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ProcedureNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ProcedureName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ProcedureNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ProcedureName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ProcedureName }
}
antlr_rust::type_id! {OC_ProcedureNameContextExt<'a>}

impl<'input> OC_ProcedureNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ProcedureNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ProcedureNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ProcedureNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ProcedureNameContextExt<'input>>
{
    fn oC_Namespace(&self) -> Option<Rc<OC_NamespaceContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ProcedureNameContextAttrs<'input> for OC_ProcedureNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ProcedureName(
        &mut self,
    ) -> Result<Rc<OC_ProcedureNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ProcedureNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 156, RULE_oC_ProcedureName);
        let mut _localctx: Rc<OC_ProcedureNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_Namespace*/
                recog.base.set_state(1331);
                recog.oC_Namespace()?;

                /*InvokeRule oC_SymbolicName*/
                recog.base.set_state(1332);
                recog.oC_SymbolicName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Namespace ----------------
pub type OC_NamespaceContextAll<'input> = OC_NamespaceContext<'input>;

pub type OC_NamespaceContext<'input> =
    BaseParserRuleContext<'input, OC_NamespaceContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NamespaceContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NamespaceContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NamespaceContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Namespace(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Namespace(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_NamespaceContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Namespace(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NamespaceContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Namespace
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Namespace }
}
antlr_rust::type_id! {OC_NamespaceContextExt<'a>}

impl<'input> OC_NamespaceContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NamespaceContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NamespaceContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NamespaceContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NamespaceContextExt<'input>>
{
    fn oC_SymbolicName_all(&self) -> Vec<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_SymbolicName(&self, i: usize) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_NamespaceContextAttrs<'input> for OC_NamespaceContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Namespace(&mut self) -> Result<Rc<OC_NamespaceContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_NamespaceContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 158, RULE_oC_Namespace);
        let mut _localctx: Rc<OC_NamespaceContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1339);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(238, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule oC_SymbolicName*/
                                recog.base.set_state(1334);
                                recog.oC_SymbolicName()?;

                                recog.base.set_state(1335);
                                recog.base.match_token(T__22, &mut recog.err_handler)?;
                            }
                        }
                    }
                    recog.base.set_state(1341);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(238, &mut recog.base)?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ListComprehension ----------------
pub type OC_ListComprehensionContextAll<'input> = OC_ListComprehensionContext<'input>;

pub type OC_ListComprehensionContext<'input> =
    BaseParserRuleContext<'input, OC_ListComprehensionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ListComprehensionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ListComprehensionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_ListComprehensionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ListComprehension(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ListComprehension(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ListComprehensionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ListComprehension(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ListComprehensionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ListComprehension
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ListComprehension }
}
antlr_rust::type_id! {OC_ListComprehensionContextExt<'a>}

impl<'input> OC_ListComprehensionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ListComprehensionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ListComprehensionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ListComprehensionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ListComprehensionContextExt<'input>>
{
    fn oC_FilterExpression(&self) -> Option<Rc<OC_FilterExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Expression(&self) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_ListComprehensionContextAttrs<'input> for OC_ListComprehensionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ListComprehension(
        &mut self,
    ) -> Result<Rc<OC_ListComprehensionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ListComprehensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 160, RULE_oC_ListComprehension);
        let mut _localctx: Rc<OC_ListComprehensionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1342);
                recog.base.match_token(T__7, &mut recog.err_handler)?;

                recog.base.set_state(1344);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1343);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_FilterExpression*/
                recog.base.set_state(1346);
                recog.oC_FilterExpression()?;

                recog.base.set_state(1355);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(242, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(1348);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1347);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1350);
                            recog.base.match_token(T__10, &mut recog.err_handler)?;

                            recog.base.set_state(1352);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1351);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(1354);
                            recog.oC_Expression()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(1358);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1357);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1360);
                recog.base.match_token(T__8, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PatternComprehension ----------------
pub type OC_PatternComprehensionContextAll<'input> = OC_PatternComprehensionContext<'input>;

pub type OC_PatternComprehensionContext<'input> =
    BaseParserRuleContext<'input, OC_PatternComprehensionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PatternComprehensionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PatternComprehensionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_PatternComprehensionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PatternComprehension(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PatternComprehension(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_PatternComprehensionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PatternComprehension(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PatternComprehensionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PatternComprehension
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PatternComprehension }
}
antlr_rust::type_id! {OC_PatternComprehensionContextExt<'a>}

impl<'input> OC_PatternComprehensionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PatternComprehensionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PatternComprehensionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PatternComprehensionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PatternComprehensionContextExt<'input>>
{
    fn oC_RelationshipsPattern(&self) -> Option<Rc<OC_RelationshipsPatternContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_Variable(&self) -> Option<Rc<OC_VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token WHERE
    /// Returns `None` if there is no child corresponding to token WHERE
    fn WHERE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHERE, 0)
    }
}

impl<'input> OC_PatternComprehensionContextAttrs<'input>
    for OC_PatternComprehensionContext<'input>
{
}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PatternComprehension(
        &mut self,
    ) -> Result<Rc<OC_PatternComprehensionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PatternComprehensionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 162, RULE_oC_PatternComprehension);
        let mut _localctx: Rc<OC_PatternComprehensionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1362);
                recog.base.match_token(T__7, &mut recog.err_handler)?;

                recog.base.set_state(1364);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1363);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1374);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la - 83) & !0x3f) == 0
                    && ((1usize << (_la - 83))
                        & ((1usize << (COUNT - 83))
                            | (1usize << (ANY - 83))
                            | (1usize << (NONE - 83))
                            | (1usize << (SINGLE - 83))
                            | (1usize << (HexLetter - 83))
                            | (1usize << (FILTER - 83))
                            | (1usize << (EXTRACT - 83))
                            | (1usize << (UnescapedSymbolicName - 83))
                            | (1usize << (EscapedSymbolicName - 83))))
                        != 0)
                {
                    {
                        /*InvokeRule oC_Variable*/
                        recog.base.set_state(1366);
                        recog.oC_Variable()?;

                        recog.base.set_state(1368);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1367);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1370);
                        recog.base.match_token(T__2, &mut recog.err_handler)?;

                        recog.base.set_state(1372);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1371);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                /*InvokeRule oC_RelationshipsPattern*/
                recog.base.set_state(1376);
                recog.oC_RelationshipsPattern()?;

                recog.base.set_state(1378);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1377);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1388);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == WHERE {
                    {
                        recog.base.set_state(1380);
                        recog.base.match_token(WHERE, &mut recog.err_handler)?;

                        recog.base.set_state(1382);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1381);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        /*InvokeRule oC_Expression*/
                        recog.base.set_state(1384);
                        recog.oC_Expression()?;

                        recog.base.set_state(1386);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1385);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }
                    }
                }

                recog.base.set_state(1390);
                recog.base.match_token(T__10, &mut recog.err_handler)?;

                recog.base.set_state(1392);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1391);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Expression*/
                recog.base.set_state(1394);
                recog.oC_Expression()?;

                recog.base.set_state(1396);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1395);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1398);
                recog.base.match_token(T__8, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PropertyLookup ----------------
pub type OC_PropertyLookupContextAll<'input> = OC_PropertyLookupContext<'input>;

pub type OC_PropertyLookupContext<'input> =
    BaseParserRuleContext<'input, OC_PropertyLookupContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyLookupContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PropertyLookupContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertyLookupContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PropertyLookup(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PropertyLookup(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PropertyLookupContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PropertyLookup(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PropertyLookupContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PropertyLookup
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyLookup }
}
antlr_rust::type_id! {OC_PropertyLookupContextExt<'a>}

impl<'input> OC_PropertyLookupContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PropertyLookupContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PropertyLookupContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PropertyLookupContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PropertyLookupContextExt<'input>>
{
    fn oC_PropertyKeyName(&self) -> Option<Rc<OC_PropertyKeyNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SP
    /// Returns `None` if there is no child corresponding to token SP
    fn SP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, 0)
    }
}

impl<'input> OC_PropertyLookupContextAttrs<'input> for OC_PropertyLookupContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PropertyLookup(
        &mut self,
    ) -> Result<Rc<OC_PropertyLookupContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PropertyLookupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 164, RULE_oC_PropertyLookup);
        let mut _localctx: Rc<OC_PropertyLookupContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1400);
                recog.base.match_token(T__22, &mut recog.err_handler)?;

                recog.base.set_state(1402);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1401);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                {
                    /*InvokeRule oC_PropertyKeyName*/
                    recog.base.set_state(1404);
                    recog.oC_PropertyKeyName()?;
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_CaseExpression ----------------
pub type OC_CaseExpressionContextAll<'input> = OC_CaseExpressionContext<'input>;

pub type OC_CaseExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_CaseExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CaseExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_CaseExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_CaseExpressionContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_CaseExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_CaseExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_CaseExpressionContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_CaseExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_CaseExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_CaseExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_CaseExpression }
}
antlr_rust::type_id! {OC_CaseExpressionContextExt<'a>}

impl<'input> OC_CaseExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_CaseExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_CaseExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_CaseExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_CaseExpressionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token END
    /// Returns `None` if there is no child corresponding to token END
    fn END(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CASE, 0)
    }
    fn oC_CaseAlternatives_all(&self) -> Vec<Rc<OC_CaseAlternativesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_CaseAlternatives(&self, i: usize) -> Option<Rc<OC_CaseAlternativesContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_CaseExpressionContextAttrs<'input> for OC_CaseExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_CaseExpression(
        &mut self,
    ) -> Result<Rc<OC_CaseExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_CaseExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 166, RULE_oC_CaseExpression);
        let mut _localctx: Rc<OC_CaseExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1428);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(260, &mut recog.base)? {
                    1 => {
                        {
                            {
                                recog.base.set_state(1406);
                                recog.base.match_token(CASE, &mut recog.err_handler)?;

                                recog.base.set_state(1411);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = 1;
                                loop {
                                    match _alt {
                                        x if x == 1 => {
                                            {
                                                recog.base.set_state(1408);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == SP {
                                                    {
                                                        recog.base.set_state(1407);
                                                        recog.base.match_token(
                                                            SP,
                                                            &mut recog.err_handler,
                                                        )?;
                                                    }
                                                }

                                                /*InvokeRule oC_CaseAlternatives*/
                                                recog.base.set_state(1410);
                                                recog.oC_CaseAlternatives()?;
                                            }
                                        }

                                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                            &mut recog.base,
                                        )))?,
                                    }
                                    recog.base.set_state(1413);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _alt =
                                        recog.interpreter.adaptive_predict(256, &mut recog.base)?;
                                    if _alt == 2 || _alt == INVALID_ALT {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    2 => {
                        {
                            {
                                recog.base.set_state(1415);
                                recog.base.match_token(CASE, &mut recog.err_handler)?;

                                recog.base.set_state(1417);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(1416);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_Expression*/
                                recog.base.set_state(1419);
                                recog.oC_Expression()?;

                                recog.base.set_state(1424);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = 1;
                                loop {
                                    match _alt {
                                        x if x == 1 => {
                                            {
                                                recog.base.set_state(1421);
                                                recog.err_handler.sync(&mut recog.base)?;
                                                _la = recog.base.input.la(1);
                                                if _la == SP {
                                                    {
                                                        recog.base.set_state(1420);
                                                        recog.base.match_token(
                                                            SP,
                                                            &mut recog.err_handler,
                                                        )?;
                                                    }
                                                }

                                                /*InvokeRule oC_CaseAlternatives*/
                                                recog.base.set_state(1423);
                                                recog.oC_CaseAlternatives()?;
                                            }
                                        }

                                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                            &mut recog.base,
                                        )))?,
                                    }
                                    recog.base.set_state(1426);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _alt =
                                        recog.interpreter.adaptive_predict(259, &mut recog.base)?;
                                    if _alt == 2 || _alt == INVALID_ALT {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(1438);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(263, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(1431);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1430);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            recog.base.set_state(1433);
                            recog.base.match_token(ELSE, &mut recog.err_handler)?;

                            recog.base.set_state(1435);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            if _la == SP {
                                {
                                    recog.base.set_state(1434);
                                    recog.base.match_token(SP, &mut recog.err_handler)?;
                                }
                            }

                            /*InvokeRule oC_Expression*/
                            recog.base.set_state(1437);
                            recog.oC_Expression()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(1441);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1440);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1443);
                recog.base.match_token(END, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_CaseAlternatives ----------------
pub type OC_CaseAlternativesContextAll<'input> = OC_CaseAlternativesContext<'input>;

pub type OC_CaseAlternativesContext<'input> =
    BaseParserRuleContext<'input, OC_CaseAlternativesContextExt<'input>>;

#[derive(Clone)]
pub struct OC_CaseAlternativesContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_CaseAlternativesContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_CaseAlternativesContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_CaseAlternatives(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_CaseAlternatives(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_CaseAlternativesContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_CaseAlternatives(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_CaseAlternativesContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_CaseAlternatives
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_CaseAlternatives }
}
antlr_rust::type_id! {OC_CaseAlternativesContextExt<'a>}

impl<'input> OC_CaseAlternativesContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_CaseAlternativesContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_CaseAlternativesContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_CaseAlternativesContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_CaseAlternativesContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token WHEN
    /// Returns `None` if there is no child corresponding to token WHEN
    fn WHEN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHEN, 0)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token THEN
    /// Returns `None` if there is no child corresponding to token THEN
    fn THEN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(THEN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_CaseAlternativesContextAttrs<'input> for OC_CaseAlternativesContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_CaseAlternatives(
        &mut self,
    ) -> Result<Rc<OC_CaseAlternativesContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_CaseAlternativesContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 168, RULE_oC_CaseAlternatives);
        let mut _localctx: Rc<OC_CaseAlternativesContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1445);
                recog.base.match_token(WHEN, &mut recog.err_handler)?;

                recog.base.set_state(1447);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1446);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Expression*/
                recog.base.set_state(1449);
                recog.oC_Expression()?;

                recog.base.set_state(1451);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1450);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1453);
                recog.base.match_token(THEN, &mut recog.err_handler)?;

                recog.base.set_state(1455);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1454);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                /*InvokeRule oC_Expression*/
                recog.base.set_state(1457);
                recog.oC_Expression()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Variable ----------------
pub type OC_VariableContextAll<'input> = OC_VariableContext<'input>;

pub type OC_VariableContext<'input> = BaseParserRuleContext<'input, OC_VariableContextExt<'input>>;

#[derive(Clone)]
pub struct OC_VariableContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_VariableContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_VariableContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Variable(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Variable(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_VariableContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Variable(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_VariableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Variable
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Variable }
}
antlr_rust::type_id! {OC_VariableContextExt<'a>}

impl<'input> OC_VariableContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_VariableContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_VariableContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_VariableContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_VariableContextExt<'input>>
{
    fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_VariableContextAttrs<'input> for OC_VariableContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Variable(&mut self) -> Result<Rc<OC_VariableContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 170, RULE_oC_Variable);
        let mut _localctx: Rc<OC_VariableContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_SymbolicName*/
                recog.base.set_state(1459);
                recog.oC_SymbolicName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_NumberLiteral ----------------
pub type OC_NumberLiteralContextAll<'input> = OC_NumberLiteralContext<'input>;

pub type OC_NumberLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_NumberLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_NumberLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_NumberLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_NumberLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_NumberLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_NumberLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_NumberLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_NumberLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_NumberLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_NumberLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_NumberLiteral }
}
antlr_rust::type_id! {OC_NumberLiteralContextExt<'a>}

impl<'input> OC_NumberLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_NumberLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_NumberLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_NumberLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_NumberLiteralContextExt<'input>>
{
    fn oC_DoubleLiteral(&self) -> Option<Rc<OC_DoubleLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_IntegerLiteral(&self) -> Option<Rc<OC_IntegerLiteralContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_NumberLiteralContextAttrs<'input> for OC_NumberLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_NumberLiteral(
        &mut self,
    ) -> Result<Rc<OC_NumberLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_NumberLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 172, RULE_oC_NumberLiteral);
        let mut _localctx: Rc<OC_NumberLiteralContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1463);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                ExponentDecimalReal | RegularDecimalReal => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_DoubleLiteral*/
                        recog.base.set_state(1461);
                        recog.oC_DoubleLiteral()?;
                    }
                }

                HexInteger | DecimalInteger | OctalInteger => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_IntegerLiteral*/
                        recog.base.set_state(1462);
                        recog.oC_IntegerLiteral()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_MapLiteral ----------------
pub type OC_MapLiteralContextAll<'input> = OC_MapLiteralContext<'input>;

pub type OC_MapLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_MapLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_MapLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_MapLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_MapLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_MapLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_MapLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_MapLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_MapLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_MapLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_MapLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_MapLiteral }
}
antlr_rust::type_id! {OC_MapLiteralContextExt<'a>}

impl<'input> OC_MapLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_MapLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_MapLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_MapLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_MapLiteralContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
    fn oC_PropertyKeyName_all(&self) -> Vec<Rc<OC_PropertyKeyNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PropertyKeyName(&self, i: usize) -> Option<Rc<OC_PropertyKeyNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn oC_Expression_all(&self) -> Vec<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_Expression(&self, i: usize) -> Option<Rc<OC_ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> OC_MapLiteralContextAttrs<'input> for OC_MapLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_MapLiteral(&mut self) -> Result<Rc<OC_MapLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_MapLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 174, RULE_oC_MapLiteral);
        let mut _localctx: Rc<OC_MapLiteralContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1465);
                recog.base.match_token(T__23, &mut recog.err_handler)?;

                recog.base.set_state(1467);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == SP {
                    {
                        recog.base.set_state(1466);
                        recog.base.match_token(SP, &mut recog.err_handler)?;
                    }
                }

                recog.base.set_state(1502);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << UNION)
                            | (1usize << ALL)
                            | (1usize << OPTIONAL)
                            | (1usize << MATCH)
                            | (1usize << UNWIND)
                            | (1usize << AS)
                            | (1usize << MERGE)
                            | (1usize << ON)
                            | (1usize << CREATE)
                            | (1usize << SET)
                            | (1usize << DETACH)
                            | (1usize << DELETE)
                            | (1usize << REMOVE)
                            | (1usize << WITH)
                            | (1usize << RETURN)
                            | (1usize << DISTINCT)))
                        != 0)
                    || (((_la - 64) & !0x3f) == 0
                        && ((1usize << (_la - 64))
                            & ((1usize << (ORDER - 64))
                                | (1usize << (BY - 64))
                                | (1usize << (L_SKIP - 64))
                                | (1usize << (LIMIT - 64))
                                | (1usize << (ASCENDING - 64))
                                | (1usize << (ASC - 64))
                                | (1usize << (DESCENDING - 64))
                                | (1usize << (DESC - 64))
                                | (1usize << (WHERE - 64))
                                | (1usize << (OR - 64))
                                | (1usize << (XOR - 64))
                                | (1usize << (AND - 64))
                                | (1usize << (NOT - 64))
                                | (1usize << (IN - 64))
                                | (1usize << (STARTS - 64))
                                | (1usize << (ENDS - 64))
                                | (1usize << (CONTAINS - 64))
                                | (1usize << (IS - 64))
                                | (1usize << (NULL - 64))
                                | (1usize << (COUNT - 64))
                                | (1usize << (ANY - 64))
                                | (1usize << (NONE - 64))
                                | (1usize << (SINGLE - 64))
                                | (1usize << (TRUE - 64))
                                | (1usize << (FALSE - 64))
                                | (1usize << (EXISTS - 64))
                                | (1usize << (CASE - 64))
                                | (1usize << (ELSE - 64))
                                | (1usize << (END - 64))
                                | (1usize << (WHEN - 64))
                                | (1usize << (THEN - 64))
                                | (1usize << (HexLetter - 64))
                                | (1usize << (CONSTRAINT - 64))
                                | (1usize << (DO - 64))
                                | (1usize << (FOR - 64))
                                | (1usize << (REQUIRE - 64))
                                | (1usize << (UNIQUE - 64))
                                | (1usize << (MANDATORY - 64))
                                | (1usize << (SCALAR - 64))
                                | (1usize << (OF - 64))
                                | (1usize << (ADD - 64))
                                | (1usize << (DROP - 64))
                                | (1usize << (FILTER - 64))
                                | (1usize << (EXTRACT - 64))
                                | (1usize << (UnescapedSymbolicName - 64))
                                | (1usize << (EscapedSymbolicName - 64))))
                            != 0)
                {
                    {
                        /*InvokeRule oC_PropertyKeyName*/
                        recog.base.set_state(1469);
                        recog.oC_PropertyKeyName()?;

                        recog.base.set_state(1471);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1470);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1473);
                        recog.base.match_token(T__9, &mut recog.err_handler)?;

                        recog.base.set_state(1475);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1474);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        /*InvokeRule oC_Expression*/
                        recog.base.set_state(1477);
                        recog.oC_Expression()?;

                        recog.base.set_state(1479);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        if _la == SP {
                            {
                                recog.base.set_state(1478);
                                recog.base.match_token(SP, &mut recog.err_handler)?;
                            }
                        }

                        recog.base.set_state(1499);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while _la == T__1 {
                            {
                                {
                                    recog.base.set_state(1481);
                                    recog.base.match_token(T__1, &mut recog.err_handler)?;

                                    recog.base.set_state(1483);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1482);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }

                                    /*InvokeRule oC_PropertyKeyName*/
                                    recog.base.set_state(1485);
                                    recog.oC_PropertyKeyName()?;

                                    recog.base.set_state(1487);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1486);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }

                                    recog.base.set_state(1489);
                                    recog.base.match_token(T__9, &mut recog.err_handler)?;

                                    recog.base.set_state(1491);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1490);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }

                                    /*InvokeRule oC_Expression*/
                                    recog.base.set_state(1493);
                                    recog.oC_Expression()?;

                                    recog.base.set_state(1495);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _la = recog.base.input.la(1);
                                    if _la == SP {
                                        {
                                            recog.base.set_state(1494);
                                            recog.base.match_token(SP, &mut recog.err_handler)?;
                                        }
                                    }
                                }
                            }
                            recog.base.set_state(1501);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                    }
                }

                recog.base.set_state(1504);
                recog.base.match_token(T__24, &mut recog.err_handler)?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Parameter ----------------
pub type OC_ParameterContextAll<'input> = OC_ParameterContext<'input>;

pub type OC_ParameterContext<'input> =
    BaseParserRuleContext<'input, OC_ParameterContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ParameterContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ParameterContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ParameterContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Parameter(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Parameter(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ParameterContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Parameter(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ParameterContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Parameter
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Parameter }
}
antlr_rust::type_id! {OC_ParameterContextExt<'a>}

impl<'input> OC_ParameterContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ParameterContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ParameterContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ParameterContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ParameterContextExt<'input>>
{
    fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token DecimalInteger
    /// Returns `None` if there is no child corresponding to token DecimalInteger
    fn DecimalInteger(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DecimalInteger, 0)
    }
}

impl<'input> OC_ParameterContextAttrs<'input> for OC_ParameterContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Parameter(&mut self) -> Result<Rc<OC_ParameterContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_ParameterContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 176, RULE_oC_Parameter);
        let mut _localctx: Rc<OC_ParameterContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1506);
                recog.base.match_token(T__25, &mut recog.err_handler)?;

                recog.base.set_state(1509);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    COUNT
                    | ANY
                    | NONE
                    | SINGLE
                    | HexLetter
                    | FILTER
                    | EXTRACT
                    | UnescapedSymbolicName
                    | EscapedSymbolicName => {
                        {
                            /*InvokeRule oC_SymbolicName*/
                            recog.base.set_state(1507);
                            recog.oC_SymbolicName()?;
                        }
                    }

                    DecimalInteger => {
                        recog.base.set_state(1508);
                        recog
                            .base
                            .match_token(DecimalInteger, &mut recog.err_handler)?;
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PropertyExpression ----------------
pub type OC_PropertyExpressionContextAll<'input> = OC_PropertyExpressionContext<'input>;

pub type OC_PropertyExpressionContext<'input> =
    BaseParserRuleContext<'input, OC_PropertyExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PropertyExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a>
    for OC_PropertyExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PropertyExpression(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PropertyExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a>
    for OC_PropertyExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PropertyExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PropertyExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PropertyExpression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyExpression }
}
antlr_rust::type_id! {OC_PropertyExpressionContextExt<'a>}

impl<'input> OC_PropertyExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PropertyExpressionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PropertyExpressionContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PropertyExpressionContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PropertyExpressionContextExt<'input>>
{
    fn oC_Atom(&self) -> Option<Rc<OC_AtomContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_PropertyLookup_all(&self) -> Vec<Rc<OC_PropertyLookupContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn oC_PropertyLookup(&self, i: usize) -> Option<Rc<OC_PropertyLookupContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SP in current rule
    fn SP_all(&self) -> Vec<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token SP, starting from 0.
    /// Returns `None` if number of children corresponding to token SP is less or equal than `i`.
    fn SP(&self, i: usize) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SP, i)
    }
}

impl<'input> OC_PropertyExpressionContextAttrs<'input> for OC_PropertyExpressionContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PropertyExpression(
        &mut self,
    ) -> Result<Rc<OC_PropertyExpressionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PropertyExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 178, RULE_oC_PropertyExpression);
        let mut _localctx: Rc<OC_PropertyExpressionContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_Atom*/
                recog.base.set_state(1511);
                recog.oC_Atom()?;

                recog.base.set_state(1516);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = 1;
                loop {
                    match _alt {
                        x if x == 1 => {
                            {
                                recog.base.set_state(1513);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                                if _la == SP {
                                    {
                                        recog.base.set_state(1512);
                                        recog.base.match_token(SP, &mut recog.err_handler)?;
                                    }
                                }

                                /*InvokeRule oC_PropertyLookup*/
                                recog.base.set_state(1515);
                                recog.oC_PropertyLookup()?;
                            }
                        }

                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                            &mut recog.base,
                        )))?,
                    }
                    recog.base.set_state(1518);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(281, &mut recog.base)?;
                    if _alt == 2 || _alt == INVALID_ALT {
                        break;
                    }
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_PropertyKeyName ----------------
pub type OC_PropertyKeyNameContextAll<'input> = OC_PropertyKeyNameContext<'input>;

pub type OC_PropertyKeyNameContext<'input> =
    BaseParserRuleContext<'input, OC_PropertyKeyNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_PropertyKeyNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_PropertyKeyNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_PropertyKeyNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_PropertyKeyName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_PropertyKeyName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_PropertyKeyNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_PropertyKeyName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_PropertyKeyNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_PropertyKeyName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_PropertyKeyName }
}
antlr_rust::type_id! {OC_PropertyKeyNameContextExt<'a>}

impl<'input> OC_PropertyKeyNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_PropertyKeyNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_PropertyKeyNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_PropertyKeyNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_PropertyKeyNameContextExt<'input>>
{
    fn oC_SchemaName(&self) -> Option<Rc<OC_SchemaNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_PropertyKeyNameContextAttrs<'input> for OC_PropertyKeyNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_PropertyKeyName(
        &mut self,
    ) -> Result<Rc<OC_PropertyKeyNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_PropertyKeyNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 180, RULE_oC_PropertyKeyName);
        let mut _localctx: Rc<OC_PropertyKeyNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule oC_SchemaName*/
                recog.base.set_state(1520);
                recog.oC_SchemaName()?;
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_IntegerLiteral ----------------
pub type OC_IntegerLiteralContextAll<'input> = OC_IntegerLiteralContext<'input>;

pub type OC_IntegerLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_IntegerLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_IntegerLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_IntegerLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_IntegerLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_IntegerLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_IntegerLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_IntegerLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_IntegerLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_IntegerLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_IntegerLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_IntegerLiteral }
}
antlr_rust::type_id! {OC_IntegerLiteralContextExt<'a>}

impl<'input> OC_IntegerLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_IntegerLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_IntegerLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_IntegerLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_IntegerLiteralContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token HexInteger
    /// Returns `None` if there is no child corresponding to token HexInteger
    fn HexInteger(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HexInteger, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OctalInteger
    /// Returns `None` if there is no child corresponding to token OctalInteger
    fn OctalInteger(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OctalInteger, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DecimalInteger
    /// Returns `None` if there is no child corresponding to token DecimalInteger
    fn DecimalInteger(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DecimalInteger, 0)
    }
}

impl<'input> OC_IntegerLiteralContextAttrs<'input> for OC_IntegerLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_IntegerLiteral(
        &mut self,
    ) -> Result<Rc<OC_IntegerLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_IntegerLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 182, RULE_oC_IntegerLiteral);
        let mut _localctx: Rc<OC_IntegerLiteralContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1522);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 97) & !0x3f) == 0
                        && ((1usize << (_la - 97))
                            & ((1usize << (HexInteger - 97))
                                | (1usize << (DecimalInteger - 97))
                                | (1usize << (OctalInteger - 97))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_DoubleLiteral ----------------
pub type OC_DoubleLiteralContextAll<'input> = OC_DoubleLiteralContext<'input>;

pub type OC_DoubleLiteralContext<'input> =
    BaseParserRuleContext<'input, OC_DoubleLiteralContextExt<'input>>;

#[derive(Clone)]
pub struct OC_DoubleLiteralContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_DoubleLiteralContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_DoubleLiteralContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_DoubleLiteral(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_DoubleLiteral(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_DoubleLiteralContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_DoubleLiteral(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_DoubleLiteralContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_DoubleLiteral
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_DoubleLiteral }
}
antlr_rust::type_id! {OC_DoubleLiteralContextExt<'a>}

impl<'input> OC_DoubleLiteralContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_DoubleLiteralContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_DoubleLiteralContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_DoubleLiteralContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_DoubleLiteralContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ExponentDecimalReal
    /// Returns `None` if there is no child corresponding to token ExponentDecimalReal
    fn ExponentDecimalReal(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ExponentDecimalReal, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RegularDecimalReal
    /// Returns `None` if there is no child corresponding to token RegularDecimalReal
    fn RegularDecimalReal(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RegularDecimalReal, 0)
    }
}

impl<'input> OC_DoubleLiteralContextAttrs<'input> for OC_DoubleLiteralContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_DoubleLiteral(
        &mut self,
    ) -> Result<Rc<OC_DoubleLiteralContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_DoubleLiteralContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 184, RULE_oC_DoubleLiteral);
        let mut _localctx: Rc<OC_DoubleLiteralContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1524);
                _la = recog.base.input.la(1);
                if { !(_la == ExponentDecimalReal || _la == RegularDecimalReal) } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_SchemaName ----------------
pub type OC_SchemaNameContextAll<'input> = OC_SchemaNameContext<'input>;

pub type OC_SchemaNameContext<'input> =
    BaseParserRuleContext<'input, OC_SchemaNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SchemaNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SchemaNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SchemaNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_SchemaName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_SchemaName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SchemaNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_SchemaName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SchemaNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_SchemaName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_SchemaName }
}
antlr_rust::type_id! {OC_SchemaNameContextExt<'a>}

impl<'input> OC_SchemaNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SchemaNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SchemaNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SchemaNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SchemaNameContextExt<'input>>
{
    fn oC_SymbolicName(&self) -> Option<Rc<OC_SymbolicNameContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn oC_ReservedWord(&self) -> Option<Rc<OC_ReservedWordContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> OC_SchemaNameContextAttrs<'input> for OC_SchemaNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_SchemaName(&mut self) -> Result<Rc<OC_SchemaNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_SchemaNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 186, RULE_oC_SchemaName);
        let mut _localctx: Rc<OC_SchemaNameContextAll> = _localctx;
        let result: Result<(), ANTLRError> = try {
            recog.base.set_state(1528);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                COUNT
                | ANY
                | NONE
                | SINGLE
                | HexLetter
                | FILTER
                | EXTRACT
                | UnescapedSymbolicName
                | EscapedSymbolicName => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule oC_SymbolicName*/
                        recog.base.set_state(1526);
                        recog.oC_SymbolicName()?;
                    }
                }

                UNION | ALL | OPTIONAL | MATCH | UNWIND | AS | MERGE | ON | CREATE | SET
                | DETACH | DELETE | REMOVE | WITH | RETURN | DISTINCT | ORDER | BY | L_SKIP
                | LIMIT | ASCENDING | ASC | DESCENDING | DESC | WHERE | OR | XOR | AND | NOT
                | IN | STARTS | ENDS | CONTAINS | IS | NULL | TRUE | FALSE | EXISTS | CASE
                | ELSE | END | WHEN | THEN | CONSTRAINT | DO | FOR | REQUIRE | UNIQUE
                | MANDATORY | SCALAR | OF | ADD | DROP => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule oC_ReservedWord*/
                        recog.base.set_state(1527);
                        recog.oC_ReservedWord()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_ReservedWord ----------------
pub type OC_ReservedWordContextAll<'input> = OC_ReservedWordContext<'input>;

pub type OC_ReservedWordContext<'input> =
    BaseParserRuleContext<'input, OC_ReservedWordContextExt<'input>>;

#[derive(Clone)]
pub struct OC_ReservedWordContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_ReservedWordContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_ReservedWordContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_ReservedWord(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_ReservedWord(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_ReservedWordContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_ReservedWord(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_ReservedWordContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_ReservedWord
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_ReservedWord }
}
antlr_rust::type_id! {OC_ReservedWordContextExt<'a>}

impl<'input> OC_ReservedWordContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_ReservedWordContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_ReservedWordContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_ReservedWordContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_ReservedWordContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token ALL
    /// Returns `None` if there is no child corresponding to token ALL
    fn ALL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ALL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASC
    /// Returns `None` if there is no child corresponding to token ASC
    fn ASC(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ASCENDING
    /// Returns `None` if there is no child corresponding to token ASCENDING
    fn ASCENDING(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ASCENDING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token BY
    /// Returns `None` if there is no child corresponding to token BY
    fn BY(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CREATE
    /// Returns `None` if there is no child corresponding to token CREATE
    fn CREATE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CREATE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DELETE
    /// Returns `None` if there is no child corresponding to token DELETE
    fn DELETE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DELETE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DESC
    /// Returns `None` if there is no child corresponding to token DESC
    fn DESC(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DESC, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DESCENDING
    /// Returns `None` if there is no child corresponding to token DESCENDING
    fn DESCENDING(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DESCENDING, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DETACH
    /// Returns `None` if there is no child corresponding to token DETACH
    fn DETACH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DETACH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EXISTS
    /// Returns `None` if there is no child corresponding to token EXISTS
    fn EXISTS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXISTS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LIMIT
    /// Returns `None` if there is no child corresponding to token LIMIT
    fn LIMIT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LIMIT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MATCH
    /// Returns `None` if there is no child corresponding to token MATCH
    fn MATCH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MATCH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MERGE
    /// Returns `None` if there is no child corresponding to token MERGE
    fn MERGE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MERGE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ON
    /// Returns `None` if there is no child corresponding to token ON
    fn ON(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ON, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPTIONAL
    /// Returns `None` if there is no child corresponding to token OPTIONAL
    fn OPTIONAL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPTIONAL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ORDER
    /// Returns `None` if there is no child corresponding to token ORDER
    fn ORDER(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ORDER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REMOVE
    /// Returns `None` if there is no child corresponding to token REMOVE
    fn REMOVE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REMOVE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RETURN
    /// Returns `None` if there is no child corresponding to token RETURN
    fn RETURN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RETURN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SET
    /// Returns `None` if there is no child corresponding to token SET
    fn SET(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SET, 0)
    }
    /// Retrieves first TerminalNode corresponding to token L_SKIP
    /// Returns `None` if there is no child corresponding to token L_SKIP
    fn L_SKIP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(L_SKIP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WHERE
    /// Returns `None` if there is no child corresponding to token WHERE
    fn WHERE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHERE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WITH
    /// Returns `None` if there is no child corresponding to token WITH
    fn WITH(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WITH, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UNION
    /// Returns `None` if there is no child corresponding to token UNION
    fn UNION(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UNWIND
    /// Returns `None` if there is no child corresponding to token UNWIND
    fn UNWIND(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNWIND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AND
    /// Returns `None` if there is no child corresponding to token AND
    fn AND(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(AS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CONTAINS
    /// Returns `None` if there is no child corresponding to token CONTAINS
    fn CONTAINS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONTAINS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DISTINCT
    /// Returns `None` if there is no child corresponding to token DISTINCT
    fn DISTINCT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DISTINCT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ENDS
    /// Returns `None` if there is no child corresponding to token ENDS
    fn ENDS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ENDS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token IS
    /// Returns `None` if there is no child corresponding to token IS
    fn IS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(IS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NOT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OR
    /// Returns `None` if there is no child corresponding to token OR
    fn OR(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token STARTS
    /// Returns `None` if there is no child corresponding to token STARTS
    fn STARTS(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STARTS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token XOR
    /// Returns `None` if there is no child corresponding to token XOR
    fn XOR(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(XOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FALSE
    /// Returns `None` if there is no child corresponding to token FALSE
    fn FALSE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FALSE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token TRUE
    /// Returns `None` if there is no child corresponding to token TRUE
    fn TRUE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TRUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NULL
    /// Returns `None` if there is no child corresponding to token NULL
    fn NULL(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NULL, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CONSTRAINT
    /// Returns `None` if there is no child corresponding to token CONSTRAINT
    fn CONSTRAINT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CONSTRAINT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DO
    /// Returns `None` if there is no child corresponding to token DO
    fn DO(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DO, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FOR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REQUIRE
    /// Returns `None` if there is no child corresponding to token REQUIRE
    fn REQUIRE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REQUIRE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token UNIQUE
    /// Returns `None` if there is no child corresponding to token UNIQUE
    fn UNIQUE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UNIQUE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(CASE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token WHEN
    /// Returns `None` if there is no child corresponding to token WHEN
    fn WHEN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(WHEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token THEN
    /// Returns `None` if there is no child corresponding to token THEN
    fn THEN(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(THEN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ELSE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token END
    /// Returns `None` if there is no child corresponding to token END
    fn END(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token MANDATORY
    /// Returns `None` if there is no child corresponding to token MANDATORY
    fn MANDATORY(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(MANDATORY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SCALAR
    /// Returns `None` if there is no child corresponding to token SCALAR
    fn SCALAR(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SCALAR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OF
    /// Returns `None` if there is no child corresponding to token OF
    fn OF(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ADD
    /// Returns `None` if there is no child corresponding to token ADD
    fn ADD(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ADD, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DROP
    /// Returns `None` if there is no child corresponding to token DROP
    fn DROP(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DROP, 0)
    }
}

impl<'input> OC_ReservedWordContextAttrs<'input> for OC_ReservedWordContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_ReservedWord(&mut self) -> Result<Rc<OC_ReservedWordContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_ReservedWordContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 188, RULE_oC_ReservedWord);
        let mut _localctx: Rc<OC_ReservedWordContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1530);
                _la = recog.base.input.la(1);
                if {
                    !((((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << UNION)
                                | (1usize << ALL)
                                | (1usize << OPTIONAL)
                                | (1usize << MATCH)
                                | (1usize << UNWIND)
                                | (1usize << AS)
                                | (1usize << MERGE)
                                | (1usize << ON)
                                | (1usize << CREATE)
                                | (1usize << SET)
                                | (1usize << DETACH)
                                | (1usize << DELETE)
                                | (1usize << REMOVE)
                                | (1usize << WITH)
                                | (1usize << RETURN)
                                | (1usize << DISTINCT)))
                            != 0)
                        || (((_la - 64) & !0x3f) == 0
                            && ((1usize << (_la - 64))
                                & ((1usize << (ORDER - 64))
                                    | (1usize << (BY - 64))
                                    | (1usize << (L_SKIP - 64))
                                    | (1usize << (LIMIT - 64))
                                    | (1usize << (ASCENDING - 64))
                                    | (1usize << (ASC - 64))
                                    | (1usize << (DESCENDING - 64))
                                    | (1usize << (DESC - 64))
                                    | (1usize << (WHERE - 64))
                                    | (1usize << (OR - 64))
                                    | (1usize << (XOR - 64))
                                    | (1usize << (AND - 64))
                                    | (1usize << (NOT - 64))
                                    | (1usize << (IN - 64))
                                    | (1usize << (STARTS - 64))
                                    | (1usize << (ENDS - 64))
                                    | (1usize << (CONTAINS - 64))
                                    | (1usize << (IS - 64))
                                    | (1usize << (NULL - 64))
                                    | (1usize << (TRUE - 64))
                                    | (1usize << (FALSE - 64))
                                    | (1usize << (EXISTS - 64))
                                    | (1usize << (CASE - 64))
                                    | (1usize << (ELSE - 64))
                                    | (1usize << (END - 64))
                                    | (1usize << (WHEN - 64))
                                    | (1usize << (THEN - 64))
                                    | (1usize << (CONSTRAINT - 64))
                                    | (1usize << (DO - 64))
                                    | (1usize << (FOR - 64))
                                    | (1usize << (REQUIRE - 64))
                                    | (1usize << (UNIQUE - 64))
                                    | (1usize << (MANDATORY - 64))
                                    | (1usize << (SCALAR - 64))
                                    | (1usize << (OF - 64))
                                    | (1usize << (ADD - 64))
                                    | (1usize << (DROP - 64))))
                                != 0))
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_SymbolicName ----------------
pub type OC_SymbolicNameContextAll<'input> = OC_SymbolicNameContext<'input>;

pub type OC_SymbolicNameContext<'input> =
    BaseParserRuleContext<'input, OC_SymbolicNameContextExt<'input>>;

#[derive(Clone)]
pub struct OC_SymbolicNameContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_SymbolicNameContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_SymbolicNameContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_SymbolicName(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_SymbolicName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_SymbolicNameContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_SymbolicName(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_SymbolicNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_SymbolicName
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_SymbolicName }
}
antlr_rust::type_id! {OC_SymbolicNameContextExt<'a>}

impl<'input> OC_SymbolicNameContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_SymbolicNameContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_SymbolicNameContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_SymbolicNameContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_SymbolicNameContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token UnescapedSymbolicName
    /// Returns `None` if there is no child corresponding to token UnescapedSymbolicName
    fn UnescapedSymbolicName(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(UnescapedSymbolicName, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EscapedSymbolicName
    /// Returns `None` if there is no child corresponding to token EscapedSymbolicName
    fn EscapedSymbolicName(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EscapedSymbolicName, 0)
    }
    /// Retrieves first TerminalNode corresponding to token HexLetter
    /// Returns `None` if there is no child corresponding to token HexLetter
    fn HexLetter(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HexLetter, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COUNT
    /// Returns `None` if there is no child corresponding to token COUNT
    fn COUNT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COUNT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FILTER
    /// Returns `None` if there is no child corresponding to token FILTER
    fn FILTER(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FILTER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EXTRACT
    /// Returns `None` if there is no child corresponding to token EXTRACT
    fn EXTRACT(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXTRACT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ANY
    /// Returns `None` if there is no child corresponding to token ANY
    fn ANY(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ANY, 0)
    }
    /// Retrieves first TerminalNode corresponding to token NONE
    /// Returns `None` if there is no child corresponding to token NONE
    fn NONE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NONE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SINGLE
    /// Returns `None` if there is no child corresponding to token SINGLE
    fn SINGLE(&self) -> Option<Rc<TerminalNode<'input, CypherParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SINGLE, 0)
    }
}

impl<'input> OC_SymbolicNameContextAttrs<'input> for OC_SymbolicNameContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_SymbolicName(&mut self) -> Result<Rc<OC_SymbolicNameContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_SymbolicNameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 190, RULE_oC_SymbolicName);
        let mut _localctx: Rc<OC_SymbolicNameContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1532);
                _la = recog.base.input.la(1);
                if {
                    !(((_la - 83) & !0x3f) == 0
                        && ((1usize << (_la - 83))
                            & ((1usize << (COUNT - 83))
                                | (1usize << (ANY - 83))
                                | (1usize << (NONE - 83))
                                | (1usize << (SINGLE - 83))
                                | (1usize << (HexLetter - 83))
                                | (1usize << (FILTER - 83))
                                | (1usize << (EXTRACT - 83))
                                | (1usize << (UnescapedSymbolicName - 83))
                                | (1usize << (EscapedSymbolicName - 83))))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_LeftArrowHead ----------------
pub type OC_LeftArrowHeadContextAll<'input> = OC_LeftArrowHeadContext<'input>;

pub type OC_LeftArrowHeadContext<'input> =
    BaseParserRuleContext<'input, OC_LeftArrowHeadContextExt<'input>>;

#[derive(Clone)]
pub struct OC_LeftArrowHeadContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_LeftArrowHeadContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_LeftArrowHeadContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_LeftArrowHead(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_LeftArrowHead(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_LeftArrowHeadContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_LeftArrowHead(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_LeftArrowHeadContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_LeftArrowHead
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_LeftArrowHead }
}
antlr_rust::type_id! {OC_LeftArrowHeadContextExt<'a>}

impl<'input> OC_LeftArrowHeadContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_LeftArrowHeadContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_LeftArrowHeadContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_LeftArrowHeadContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_LeftArrowHeadContextExt<'input>>
{
}

impl<'input> OC_LeftArrowHeadContextAttrs<'input> for OC_LeftArrowHeadContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_LeftArrowHead(
        &mut self,
    ) -> Result<Rc<OC_LeftArrowHeadContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_LeftArrowHeadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 192, RULE_oC_LeftArrowHead);
        let mut _localctx: Rc<OC_LeftArrowHeadContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1534);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << T__18)
                                | (1usize << T__26)
                                | (1usize << T__27)
                                | (1usize << T__28)
                                | (1usize << T__29)))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_RightArrowHead ----------------
pub type OC_RightArrowHeadContextAll<'input> = OC_RightArrowHeadContext<'input>;

pub type OC_RightArrowHeadContext<'input> =
    BaseParserRuleContext<'input, OC_RightArrowHeadContextExt<'input>>;

#[derive(Clone)]
pub struct OC_RightArrowHeadContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_RightArrowHeadContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_RightArrowHeadContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_RightArrowHead(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_RightArrowHead(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_RightArrowHeadContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_RightArrowHead(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_RightArrowHeadContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_RightArrowHead
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_RightArrowHead }
}
antlr_rust::type_id! {OC_RightArrowHeadContextExt<'a>}

impl<'input> OC_RightArrowHeadContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_RightArrowHeadContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_RightArrowHeadContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_RightArrowHeadContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_RightArrowHeadContextExt<'input>>
{
}

impl<'input> OC_RightArrowHeadContextAttrs<'input> for OC_RightArrowHeadContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_RightArrowHead(
        &mut self,
    ) -> Result<Rc<OC_RightArrowHeadContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            OC_RightArrowHeadContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 194, RULE_oC_RightArrowHead);
        let mut _localctx: Rc<OC_RightArrowHeadContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1536);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << T__19)
                                | (1usize << T__30)
                                | (1usize << T__31)
                                | (1usize << T__32)
                                | (1usize << T__33)))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- oC_Dash ----------------
pub type OC_DashContextAll<'input> = OC_DashContext<'input>;

pub type OC_DashContext<'input> = BaseParserRuleContext<'input, OC_DashContextExt<'input>>;

#[derive(Clone)]
pub struct OC_DashContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> CypherParserContext<'input> for OC_DashContext<'input> {}

impl<'input, 'a> Listenable<dyn CypherListener<'input> + 'a> for OC_DashContext<'input> {
    fn enter(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_oC_Dash(self);
    }
    fn exit(&self, listener: &mut (dyn CypherListener<'input> + 'a)) {
        listener.exit_oC_Dash(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn CypherVisitor<'input> + 'a> for OC_DashContext<'input> {
    fn accept(&self, visitor: &mut (dyn CypherVisitor<'input> + 'a)) {
        visitor.visit_oC_Dash(self);
    }
}

impl<'input> CustomRuleContext<'input> for OC_DashContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = CypherParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_oC_Dash
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_oC_Dash }
}
antlr_rust::type_id! {OC_DashContextExt<'a>}

impl<'input> OC_DashContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn CypherParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<OC_DashContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            OC_DashContextExt { ph: PhantomData },
        ))
    }
}

pub trait OC_DashContextAttrs<'input>:
    CypherParserContext<'input> + BorrowMut<OC_DashContextExt<'input>>
{
}

impl<'input> OC_DashContextAttrs<'input> for OC_DashContext<'input> {}

impl<'input, I, H> CypherParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn oC_Dash(&mut self) -> Result<Rc<OC_DashContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = OC_DashContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 196, RULE_oC_Dash);
        let mut _localctx: Rc<OC_DashContextAll> = _localctx;
        let mut _la: isize;
        let result: Result<(), ANTLRError> = try {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(1538);
                _la = recog.base.input.la(1);
                if {
                    !(((_la) & !0x3f) == 0
                        && ((1usize << _la)
                            & ((1usize << T__13)
                                | (1usize << T__34)
                                | (1usize << T__35)
                                | (1usize << T__36)
                                | (1usize << T__37)
                                | (1usize << T__38)
                                | (1usize << T__39)
                                | (1usize << T__40)
                                | (1usize << T__41)
                                | (1usize << T__42)
                                | (1usize << T__43)
                                | (1usize << T__44)))
                            != 0)
                } {
                    recog.err_handler.recover_inline(&mut recog.base)?;
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
            }
        };
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\u{81}\u{607}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x04\x31\x09\x31\x04\x32\x09\
	\x32\x04\x33\x09\x33\x04\x34\x09\x34\x04\x35\x09\x35\x04\x36\x09\x36\x04\
	\x37\x09\x37\x04\x38\x09\x38\x04\x39\x09\x39\x04\x3a\x09\x3a\x04\x3b\x09\
	\x3b\x04\x3c\x09\x3c\x04\x3d\x09\x3d\x04\x3e\x09\x3e\x04\x3f\x09\x3f\x04\
	\x40\x09\x40\x04\x41\x09\x41\x04\x42\x09\x42\x04\x43\x09\x43\x04\x44\x09\
	\x44\x04\x45\x09\x45\x04\x46\x09\x46\x04\x47\x09\x47\x04\x48\x09\x48\x04\
	\x49\x09\x49\x04\x4a\x09\x4a\x04\x4b\x09\x4b\x04\x4c\x09\x4c\x04\x4d\x09\
	\x4d\x04\x4e\x09\x4e\x04\x4f\x09\x4f\x04\x50\x09\x50\x04\x51\x09\x51\x04\
	\x52\x09\x52\x04\x53\x09\x53\x04\x54\x09\x54\x04\x55\x09\x55\x04\x56\x09\
	\x56\x04\x57\x09\x57\x04\x58\x09\x58\x04\x59\x09\x59\x04\x5a\x09\x5a\x04\
	\x5b\x09\x5b\x04\x5c\x09\x5c\x04\x5d\x09\x5d\x04\x5e\x09\x5e\x04\x5f\x09\
	\x5f\x04\x60\x09\x60\x04\x61\x09\x61\x04\x62\x09\x62\x04\x63\x09\x63\x04\
	\x64\x09\x64\x03\x02\x05\x02\u{ca}\x0a\x02\x03\x02\x03\x02\x05\x02\u{ce}\
	\x0a\x02\x03\x02\x05\x02\u{d1}\x0a\x02\x03\x02\x05\x02\u{d4}\x0a\x02\x03\
	\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\x05\x04\u{dc}\x0a\x04\x03\x05\
	\x03\x05\x05\x05\u{e0}\x0a\x05\x03\x05\x07\x05\u{e3}\x0a\x05\x0c\x05\x0e\
	\x05\u{e6}\x0b\x05\x03\x06\x03\x06\x03\x06\x03\x06\x05\x06\u{ec}\x0a\x06\
	\x03\x06\x03\x06\x03\x06\x05\x06\u{f1}\x0a\x06\x03\x06\x05\x06\u{f4}\x0a\
	\x06\x03\x07\x03\x07\x05\x07\u{f8}\x0a\x07\x03\x08\x03\x08\x05\x08\u{fc}\
	\x0a\x08\x07\x08\u{fe}\x0a\x08\x0c\x08\x0e\x08\u{101}\x0b\x08\x03\x08\x03\
	\x08\x03\x08\x05\x08\u{106}\x0a\x08\x07\x08\u{108}\x0a\x08\x0c\x08\x0e\x08\
	\u{10b}\x0b\x08\x03\x08\x03\x08\x05\x08\u{10f}\x0a\x08\x03\x08\x07\x08\u{112}\
	\x0a\x08\x0c\x08\x0e\x08\u{115}\x0b\x08\x03\x08\x05\x08\u{118}\x0a\x08\x03\
	\x08\x05\x08\u{11b}\x0a\x08\x05\x08\u{11d}\x0a\x08\x03\x09\x03\x09\x05\x09\
	\u{121}\x0a\x09\x07\x09\u{123}\x0a\x09\x0c\x09\x0e\x09\u{126}\x0b\x09\x03\
	\x09\x03\x09\x05\x09\u{12a}\x0a\x09\x07\x09\u{12c}\x0a\x09\x0c\x09\x0e\x09\
	\u{12f}\x0b\x09\x03\x09\x03\x09\x05\x09\u{133}\x0a\x09\x06\x09\u{135}\x0a\
	\x09\x0d\x09\x0e\x09\u{136}\x03\x09\x03\x09\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x05\x0a\u{140}\x0a\x0a\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{145}\
	\x0a\x0b\x03\x0c\x03\x0c\x05\x0c\u{149}\x0a\x0c\x03\x0c\x03\x0c\x05\x0c\
	\u{14d}\x0a\x0c\x03\x0c\x03\x0c\x05\x0c\u{151}\x0a\x0c\x03\x0c\x05\x0c\u{154}\
	\x0a\x0c\x03\x0d\x03\x0d\x05\x0d\u{158}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x05\x0e\u{162}\x0a\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x07\x0e\u{167}\x0a\x0e\x0c\x0e\x0e\x0e\u{16a}\x0b\x0e\x03\
	\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\
	\x0f\x05\x0f\u{176}\x0a\x0f\x03\x10\x03\x10\x05\x10\u{17a}\x0a\x10\x03\x10\
	\x03\x10\x03\x11\x03\x11\x05\x11\u{180}\x0a\x11\x03\x11\x03\x11\x03\x11\
	\x07\x11\u{185}\x0a\x11\x0c\x11\x0e\x11\u{188}\x0b\x11\x03\x12\x03\x12\x05\
	\x12\u{18c}\x0a\x12\x03\x12\x03\x12\x05\x12\u{190}\x0a\x12\x03\x12\x03\x12\
	\x03\x12\x03\x12\x05\x12\u{196}\x0a\x12\x03\x12\x03\x12\x05\x12\u{19a}\x0a\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{1a0}\x0a\x12\x03\x12\x03\
	\x12\x05\x12\u{1a4}\x0a\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{1aa}\
	\x0a\x12\x03\x12\x03\x12\x05\x12\u{1ae}\x0a\x12\x03\x13\x03\x13\x05\x13\
	\u{1b2}\x0a\x13\x03\x13\x03\x13\x05\x13\u{1b6}\x0a\x13\x03\x13\x03\x13\x05\
	\x13\u{1ba}\x0a\x13\x03\x13\x03\x13\x05\x13\u{1be}\x0a\x13\x03\x13\x07\x13\
	\u{1c1}\x0a\x13\x0c\x13\x0e\x13\u{1c4}\x0b\x13\x03\x14\x03\x14\x03\x14\x03\
	\x14\x05\x14\u{1ca}\x0a\x14\x03\x14\x03\x14\x05\x14\u{1ce}\x0a\x14\x03\x14\
	\x07\x14\u{1d1}\x0a\x14\x0c\x14\x0e\x14\u{1d4}\x0b\x14\x03\x15\x03\x15\x03\
	\x15\x03\x15\x05\x15\u{1da}\x0a\x15\x03\x16\x03\x16\x03\x16\x03\x16\x05\
	\x16\u{1e0}\x0a\x16\x03\x16\x03\x16\x03\x16\x05\x16\u{1e5}\x0a\x16\x03\x17\
	\x03\x17\x03\x17\x03\x17\x05\x17\u{1eb}\x0a\x17\x03\x17\x03\x17\x03\x17\
	\x03\x17\x05\x17\u{1f1}\x0a\x17\x03\x18\x03\x18\x03\x18\x05\x18\u{1f6}\x0a\
	\x18\x03\x18\x03\x18\x05\x18\u{1fa}\x0a\x18\x03\x18\x07\x18\u{1fd}\x0a\x18\
	\x0c\x18\x0e\x18\u{200}\x0b\x18\x05\x18\u{202}\x0a\x18\x03\x18\x05\x18\u{205}\
	\x0a\x18\x03\x18\x05\x18\u{208}\x0a\x18\x03\x19\x03\x19\x03\x19\x03\x19\
	\x03\x19\x05\x19\u{20f}\x0a\x19\x03\x19\x03\x19\x03\x1a\x03\x1a\x03\x1a\
	\x05\x1a\u{216}\x0a\x1a\x03\x1a\x05\x1a\u{219}\x0a\x1a\x03\x1b\x03\x1b\x03\
	\x1b\x03\x1c\x05\x1c\u{21f}\x0a\x1c\x03\x1c\x05\x1c\u{222}\x0a\x1c\x03\x1c\
	\x03\x1c\x03\x1c\x03\x1c\x05\x1c\u{228}\x0a\x1c\x03\x1c\x03\x1c\x05\x1c\
	\u{22c}\x0a\x1c\x03\x1c\x03\x1c\x05\x1c\u{230}\x0a\x1c\x03\x1d\x03\x1d\x05\
	\x1d\u{234}\x0a\x1d\x03\x1d\x03\x1d\x05\x1d\u{238}\x0a\x1d\x03\x1d\x07\x1d\
	\u{23b}\x0a\x1d\x0c\x1d\x0e\x1d\u{23e}\x0b\x1d\x03\x1d\x03\x1d\x05\x1d\u{242}\
	\x0a\x1d\x03\x1d\x03\x1d\x05\x1d\u{246}\x0a\x1d\x03\x1d\x07\x1d\u{249}\x0a\
	\x1d\x0c\x1d\x0e\x1d\u{24c}\x0b\x1d\x05\x1d\u{24e}\x0a\x1d\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x03\x1e\x05\x1e\u{257}\x0a\x1e\x03\x1f\
	\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x05\x1f\u{260}\x0a\x1f\
	\x03\x1f\x07\x1f\u{263}\x0a\x1f\x0c\x1f\x0e\x1f\u{266}\x0b\x1f\x03\x20\x03\
	\x20\x03\x20\x03\x20\x03\x21\x03\x21\x03\x21\x03\x21\x03\x22\x03\x22\x05\
	\x22\u{272}\x0a\x22\x03\x22\x05\x22\u{275}\x0a\x22\x03\x23\x03\x23\x03\x23\
	\x03\x23\x03\x24\x03\x24\x05\x24\u{27d}\x0a\x24\x03\x24\x03\x24\x05\x24\
	\u{281}\x0a\x24\x03\x24\x07\x24\u{284}\x0a\x24\x0c\x24\x0e\x24\u{287}\x0b\
	\x24\x03\x25\x03\x25\x05\x25\u{28b}\x0a\x25\x03\x25\x03\x25\x05\x25\u{28f}\
	\x0a\x25\x03\x25\x03\x25\x03\x25\x05\x25\u{294}\x0a\x25\x03\x26\x03\x26\
	\x03\x27\x03\x27\x05\x27\u{29a}\x0a\x27\x03\x27\x07\x27\u{29d}\x0a\x27\x0c\
	\x27\x0e\x27\u{2a0}\x0b\x27\x03\x27\x03\x27\x03\x27\x03\x27\x05\x27\u{2a6}\
	\x0a\x27\x03\x28\x03\x28\x05\x28\u{2aa}\x0a\x28\x03\x28\x03\x28\x05\x28\
	\u{2ae}\x0a\x28\x05\x28\u{2b0}\x0a\x28\x03\x28\x03\x28\x05\x28\u{2b4}\x0a\
	\x28\x05\x28\u{2b6}\x0a\x28\x03\x28\x03\x28\x05\x28\u{2ba}\x0a\x28\x05\x28\
	\u{2bc}\x0a\x28\x03\x28\x03\x28\x03\x29\x03\x29\x05\x29\u{2c2}\x0a\x29\x03\
	\x29\x03\x29\x03\x2a\x03\x2a\x05\x2a\u{2c8}\x0a\x2a\x03\x2a\x03\x2a\x05\
	\x2a\u{2cc}\x0a\x2a\x03\x2a\x05\x2a\u{2cf}\x0a\x2a\x03\x2a\x05\x2a\u{2d2}\
	\x0a\x2a\x03\x2a\x03\x2a\x05\x2a\u{2d6}\x0a\x2a\x03\x2a\x03\x2a\x03\x2a\
	\x03\x2a\x05\x2a\u{2dc}\x0a\x2a\x03\x2a\x03\x2a\x05\x2a\u{2e0}\x0a\x2a\x03\
	\x2a\x05\x2a\u{2e3}\x0a\x2a\x03\x2a\x05\x2a\u{2e6}\x0a\x2a\x03\x2a\x03\x2a\
	\x03\x2a\x03\x2a\x05\x2a\u{2ec}\x0a\x2a\x03\x2a\x05\x2a\u{2ef}\x0a\x2a\x03\
	\x2a\x05\x2a\u{2f2}\x0a\x2a\x03\x2a\x03\x2a\x05\x2a\u{2f6}\x0a\x2a\x03\x2a\
	\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{2fc}\x0a\x2a\x03\x2a\x05\x2a\u{2ff}\x0a\
	\x2a\x03\x2a\x05\x2a\u{302}\x0a\x2a\x03\x2a\x03\x2a\x05\x2a\u{306}\x0a\x2a\
	\x03\x2b\x03\x2b\x05\x2b\u{30a}\x0a\x2b\x03\x2b\x03\x2b\x05\x2b\u{30e}\x0a\
	\x2b\x05\x2b\u{310}\x0a\x2b\x03\x2b\x03\x2b\x05\x2b\u{314}\x0a\x2b\x05\x2b\
	\u{316}\x0a\x2b\x03\x2b\x05\x2b\u{319}\x0a\x2b\x03\x2b\x03\x2b\x05\x2b\u{31d}\
	\x0a\x2b\x05\x2b\u{31f}\x0a\x2b\x03\x2b\x03\x2b\x03\x2c\x03\x2c\x05\x2c\
	\u{325}\x0a\x2c\x03\x2d\x03\x2d\x05\x2d\u{329}\x0a\x2d\x03\x2d\x03\x2d\x05\
	\x2d\u{32d}\x0a\x2d\x03\x2d\x03\x2d\x05\x2d\u{331}\x0a\x2d\x03\x2d\x05\x2d\
	\u{334}\x0a\x2d\x03\x2d\x07\x2d\u{337}\x0a\x2d\x0c\x2d\x0e\x2d\u{33a}\x0b\
	\x2d\x03\x2e\x03\x2e\x05\x2e\u{33e}\x0a\x2e\x03\x2e\x07\x2e\u{341}\x0a\x2e\
	\x0c\x2e\x0e\x2e\u{344}\x0b\x2e\x03\x2f\x03\x2f\x05\x2f\u{348}\x0a\x2f\x03\
	\x2f\x03\x2f\x03\x30\x03\x30\x05\x30\u{34e}\x0a\x30\x03\x30\x03\x30\x05\
	\x30\u{352}\x0a\x30\x05\x30\u{354}\x0a\x30\x03\x30\x03\x30\x05\x30\u{358}\
	\x0a\x30\x03\x30\x03\x30\x05\x30\u{35c}\x0a\x30\x05\x30\u{35e}\x0a\x30\x05\
	\x30\u{360}\x0a\x30\x03\x31\x03\x31\x03\x32\x03\x32\x03\x33\x03\x33\x03\
	\x34\x03\x34\x03\x34\x03\x34\x03\x34\x07\x34\u{36d}\x0a\x34\x0c\x34\x0e\
	\x34\u{370}\x0b\x34\x03\x35\x03\x35\x03\x35\x03\x35\x03\x35\x07\x35\u{377}\
	\x0a\x35\x0c\x35\x0e\x35\u{37a}\x0b\x35\x03\x36\x03\x36\x03\x36\x03\x36\
	\x03\x36\x07\x36\u{381}\x0a\x36\x0c\x36\x0e\x36\u{384}\x0b\x36\x03\x37\x03\
	\x37\x05\x37\u{388}\x0a\x37\x07\x37\u{38a}\x0a\x37\x0c\x37\x0e\x37\u{38d}\
	\x0b\x37\x03\x37\x03\x37\x03\x38\x03\x38\x05\x38\u{393}\x0a\x38\x03\x38\
	\x07\x38\u{396}\x0a\x38\x0c\x38\x0e\x38\u{399}\x0b\x38\x03\x39\x03\x39\x05\
	\x39\u{39d}\x0a\x39\x03\x39\x03\x39\x05\x39\u{3a1}\x0a\x39\x03\x39\x03\x39\
	\x05\x39\u{3a5}\x0a\x39\x03\x39\x03\x39\x05\x39\u{3a9}\x0a\x39\x03\x39\x07\
	\x39\u{3ac}\x0a\x39\x0c\x39\x0e\x39\u{3af}\x0b\x39\x03\x3a\x03\x3a\x05\x3a\
	\u{3b3}\x0a\x3a\x03\x3a\x03\x3a\x05\x3a\u{3b7}\x0a\x3a\x03\x3a\x03\x3a\x05\
	\x3a\u{3bb}\x0a\x3a\x03\x3a\x03\x3a\x05\x3a\u{3bf}\x0a\x3a\x03\x3a\x03\x3a\
	\x05\x3a\u{3c3}\x0a\x3a\x03\x3a\x03\x3a\x05\x3a\u{3c7}\x0a\x3a\x03\x3a\x07\
	\x3a\u{3ca}\x0a\x3a\x0c\x3a\x0e\x3a\u{3cd}\x0b\x3a\x03\x3b\x03\x3b\x05\x3b\
	\u{3d1}\x0a\x3b\x03\x3b\x03\x3b\x05\x3b\u{3d5}\x0a\x3b\x03\x3b\x07\x3b\u{3d8}\
	\x0a\x3b\x0c\x3b\x0e\x3b\u{3db}\x0b\x3b\x03\x3c\x03\x3c\x05\x3c\u{3df}\x0a\
	\x3c\x07\x3c\u{3e1}\x0a\x3c\x0c\x3c\x0e\x3c\u{3e4}\x0b\x3c\x03\x3c\x03\x3c\
	\x03\x3d\x03\x3d\x03\x3d\x03\x3d\x07\x3d\u{3ec}\x0a\x3d\x0c\x3d\x0e\x3d\
	\u{3ef}\x0b\x3d\x03\x3e\x03\x3e\x03\x3e\x05\x3e\u{3f4}\x0a\x3e\x03\x3e\x03\
	\x3e\x05\x3e\u{3f8}\x0a\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x03\x3e\x05\
	\x3e\u{3ff}\x0a\x3e\x03\x3e\x03\x3e\x05\x3e\u{403}\x0a\x3e\x03\x3e\x03\x3e\
	\x05\x3e\u{407}\x0a\x3e\x03\x3e\x05\x3e\u{40a}\x0a\x3e\x03\x3f\x03\x3f\x03\
	\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x03\x3f\x05\x3f\u{416}\
	\x0a\x3f\x03\x3f\x05\x3f\u{419}\x0a\x3f\x03\x3f\x03\x3f\x03\x40\x03\x40\
	\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x03\x40\x05\x40\
	\u{427}\x0a\x40\x03\x41\x03\x41\x05\x41\u{42b}\x0a\x41\x03\x41\x07\x41\u{42e}\
	\x0a\x41\x0c\x41\x0e\x41\u{431}\x0b\x41\x03\x41\x05\x41\u{434}\x0a\x41\x03\
	\x41\x05\x41\u{437}\x0a\x41\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\
	\x42\u{43e}\x0a\x42\x03\x42\x03\x42\x05\x42\u{442}\x0a\x42\x03\x42\x03\x42\
	\x05\x42\u{446}\x0a\x42\x03\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\x42\
	\u{44d}\x0a\x42\x03\x42\x03\x42\x05\x42\u{451}\x0a\x42\x03\x42\x03\x42\x05\
	\x42\u{455}\x0a\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\x42\u{45b}\x0a\x42\
	\x03\x42\x03\x42\x05\x42\u{45f}\x0a\x42\x03\x42\x03\x42\x05\x42\u{463}\x0a\
	\x42\x03\x42\x03\x42\x03\x42\x03\x42\x05\x42\u{469}\x0a\x42\x03\x42\x03\
	\x42\x05\x42\u{46d}\x0a\x42\x03\x42\x03\x42\x05\x42\u{471}\x0a\x42\x03\x42\
	\x03\x42\x03\x42\x03\x42\x05\x42\u{477}\x0a\x42\x03\x42\x03\x42\x05\x42\
	\u{47b}\x0a\x42\x03\x42\x03\x42\x05\x42\u{47f}\x0a\x42\x03\x42\x03\x42\x03\
	\x42\x03\x42\x03\x42\x03\x42\x05\x42\u{487}\x0a\x42\x03\x43\x03\x43\x03\
	\x43\x03\x43\x03\x43\x03\x43\x05\x43\u{48f}\x0a\x43\x03\x44\x03\x44\x03\
	\x45\x03\x45\x05\x45\u{495}\x0a\x45\x03\x45\x03\x45\x05\x45\u{499}\x0a\x45\
	\x03\x45\x03\x45\x05\x45\u{49d}\x0a\x45\x03\x45\x03\x45\x05\x45\u{4a1}\x0a\
	\x45\x07\x45\u{4a3}\x0a\x45\x0c\x45\x0e\x45\u{4a6}\x0b\x45\x05\x45\u{4a8}\
	\x0a\x45\x03\x45\x03\x45\x03\x46\x03\x46\x05\x46\u{4ae}\x0a\x46\x03\x46\
	\x03\x46\x03\x46\x05\x46\u{4b3}\x0a\x46\x03\x46\x03\x46\x03\x46\x05\x46\
	\u{4b8}\x0a\x46\x03\x46\x03\x46\x03\x46\x05\x46\u{4bd}\x0a\x46\x03\x46\x03\
	\x46\x03\x46\x05\x46\u{4c2}\x0a\x46\x03\x46\x03\x46\x03\x46\x05\x46\u{4c7}\
	\x0a\x46\x03\x46\x05\x46\u{4ca}\x0a\x46\x03\x47\x03\x47\x05\x47\u{4ce}\x0a\
	\x47\x03\x47\x03\x47\x05\x47\u{4d2}\x0a\x47\x03\x47\x03\x47\x03\x48\x03\
	\x48\x05\x48\u{4d8}\x0a\x48\x03\x48\x06\x48\u{4db}\x0a\x48\x0d\x48\x0e\x48\
	\u{4dc}\x03\x49\x03\x49\x05\x49\u{4e1}\x0a\x49\x03\x49\x05\x49\u{4e4}\x0a\
	\x49\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4a\x03\x4b\x03\x4b\x05\
	\x4b\u{4ee}\x0a\x4b\x03\x4b\x03\x4b\x05\x4b\u{4f2}\x0a\x4b\x03\x4b\x03\x4b\
	\x05\x4b\u{4f6}\x0a\x4b\x05\x4b\u{4f8}\x0a\x4b\x03\x4b\x03\x4b\x05\x4b\u{4fc}\
	\x0a\x4b\x03\x4b\x03\x4b\x05\x4b\u{500}\x0a\x4b\x03\x4b\x03\x4b\x05\x4b\
	\u{504}\x0a\x4b\x07\x4b\u{506}\x0a\x4b\x0c\x4b\x0e\x4b\u{509}\x0b\x4b\x05\
	\x4b\u{50b}\x0a\x4b\x03\x4b\x03\x4b\x03\x4c\x03\x4c\x03\x4c\x03\x4c\x05\
	\x4c\u{513}\x0a\x4c\x03\x4d\x03\x4d\x05\x4d\u{517}\x0a\x4d\x03\x4d\x03\x4d\
	\x05\x4d\u{51b}\x0a\x4d\x03\x4d\x03\x4d\x05\x4d\u{51f}\x0a\x4d\x03\x4d\x03\
	\x4d\x05\x4d\u{523}\x0a\x4d\x03\x4d\x03\x4d\x05\x4d\u{527}\x0a\x4d\x07\x4d\
	\u{529}\x0a\x4d\x0c\x4d\x0e\x4d\u{52c}\x0b\x4d\x05\x4d\u{52e}\x0a\x4d\x03\
	\x4d\x03\x4d\x03\x4e\x03\x4e\x03\x4f\x03\x4f\x03\x50\x03\x50\x03\x50\x03\
	\x51\x03\x51\x03\x51\x07\x51\u{53c}\x0a\x51\x0c\x51\x0e\x51\u{53f}\x0b\x51\
	\x03\x52\x03\x52\x05\x52\u{543}\x0a\x52\x03\x52\x03\x52\x05\x52\u{547}\x0a\
	\x52\x03\x52\x03\x52\x05\x52\u{54b}\x0a\x52\x03\x52\x05\x52\u{54e}\x0a\x52\
	\x03\x52\x05\x52\u{551}\x0a\x52\x03\x52\x03\x52\x03\x53\x03\x53\x05\x53\
	\u{557}\x0a\x53\x03\x53\x03\x53\x05\x53\u{55b}\x0a\x53\x03\x53\x03\x53\x05\
	\x53\u{55f}\x0a\x53\x05\x53\u{561}\x0a\x53\x03\x53\x03\x53\x05\x53\u{565}\
	\x0a\x53\x03\x53\x03\x53\x05\x53\u{569}\x0a\x53\x03\x53\x03\x53\x05\x53\
	\u{56d}\x0a\x53\x05\x53\u{56f}\x0a\x53\x03\x53\x03\x53\x05\x53\u{573}\x0a\
	\x53\x03\x53\x03\x53\x05\x53\u{577}\x0a\x53\x03\x53\x03\x53\x03\x54\x03\
	\x54\x05\x54\u{57d}\x0a\x54\x03\x54\x03\x54\x03\x55\x03\x55\x05\x55\u{583}\
	\x0a\x55\x03\x55\x06\x55\u{586}\x0a\x55\x0d\x55\x0e\x55\u{587}\x03\x55\x03\
	\x55\x05\x55\u{58c}\x0a\x55\x03\x55\x03\x55\x05\x55\u{590}\x0a\x55\x03\x55\
	\x06\x55\u{593}\x0a\x55\x0d\x55\x0e\x55\u{594}\x05\x55\u{597}\x0a\x55\x03\
	\x55\x05\x55\u{59a}\x0a\x55\x03\x55\x03\x55\x05\x55\u{59e}\x0a\x55\x03\x55\
	\x05\x55\u{5a1}\x0a\x55\x03\x55\x05\x55\u{5a4}\x0a\x55\x03\x55\x03\x55\x03\
	\x56\x03\x56\x05\x56\u{5aa}\x0a\x56\x03\x56\x03\x56\x05\x56\u{5ae}\x0a\x56\
	\x03\x56\x03\x56\x05\x56\u{5b2}\x0a\x56\x03\x56\x03\x56\x03\x57\x03\x57\
	\x03\x58\x03\x58\x05\x58\u{5ba}\x0a\x58\x03\x59\x03\x59\x05\x59\u{5be}\x0a\
	\x59\x03\x59\x03\x59\x05\x59\u{5c2}\x0a\x59\x03\x59\x03\x59\x05\x59\u{5c6}\
	\x0a\x59\x03\x59\x03\x59\x05\x59\u{5ca}\x0a\x59\x03\x59\x03\x59\x05\x59\
	\u{5ce}\x0a\x59\x03\x59\x03\x59\x05\x59\u{5d2}\x0a\x59\x03\x59\x03\x59\x05\
	\x59\u{5d6}\x0a\x59\x03\x59\x03\x59\x05\x59\u{5da}\x0a\x59\x07\x59\u{5dc}\
	\x0a\x59\x0c\x59\x0e\x59\u{5df}\x0b\x59\x05\x59\u{5e1}\x0a\x59\x03\x59\x03\
	\x59\x03\x5a\x03\x5a\x03\x5a\x05\x5a\u{5e8}\x0a\x5a\x03\x5b\x03\x5b\x05\
	\x5b\u{5ec}\x0a\x5b\x03\x5b\x06\x5b\u{5ef}\x0a\x5b\x0d\x5b\x0e\x5b\u{5f0}\
	\x03\x5c\x03\x5c\x03\x5d\x03\x5d\x03\x5e\x03\x5e\x03\x5f\x03\x5f\x05\x5f\
	\u{5fb}\x0a\x5f\x03\x60\x03\x60\x03\x61\x03\x61\x03\x62\x03\x62\x03\x63\
	\x03\x63\x03\x64\x03\x64\x03\x64\x02\x02\x65\x02\x04\x06\x08\x0a\x0c\x0e\
	\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\
	\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\
	\x58\x5a\x5c\x5e\x60\x62\x64\x66\x68\x6a\x6c\x6e\x70\x72\x74\x76\x78\x7a\
	\x7c\x7e\u{80}\u{82}\u{84}\u{86}\u{88}\u{8a}\u{8c}\u{8e}\u{90}\u{92}\u{94}\
	\u{96}\u{98}\u{9a}\u{9c}\u{9e}\u{a0}\u{a2}\u{a4}\u{a6}\u{a8}\u{aa}\u{ac}\
	\u{ae}\u{b0}\u{b2}\u{b4}\u{b6}\u{b8}\u{ba}\u{bc}\u{be}\u{c0}\u{c2}\u{c4}\
	\u{c6}\x02\x0c\x03\x02\x46\x49\x03\x02\x0f\x10\x03\x02\x59\x5a\x03\x02\x63\
	\x65\x03\x02\x6d\x6e\x06\x02\x30\x3c\x3f\x54\x59\x60\x6f\x78\x06\x02\x55\
	\x58\x66\x66\x79\x7b\x7e\x7e\x04\x02\x15\x15\x1d\x20\x04\x02\x16\x16\x21\
	\x24\x04\x02\x10\x10\x25\x2f\x02\u{6de}\x02\u{c9}\x03\x02\x02\x02\x04\u{d7}\
	\x03\x02\x02\x02\x06\u{db}\x03\x02\x02\x02\x08\u{dd}\x03\x02\x02\x02\x0a\
	\u{f3}\x03\x02\x02\x02\x0c\u{f7}\x03\x02\x02\x02\x0e\u{11c}\x03\x02\x02\
	\x02\x10\u{134}\x03\x02\x02\x02\x12\u{13f}\x03\x02\x02\x02\x14\u{144}\x03\
	\x02\x02\x02\x16\u{148}\x03\x02\x02\x02\x18\u{155}\x03\x02\x02\x02\x1a\u{15f}\
	\x03\x02\x02\x02\x1c\u{175}\x03\x02\x02\x02\x1e\u{177}\x03\x02\x02\x02\x20\
	\u{17d}\x03\x02\x02\x02\x22\u{1ad}\x03\x02\x02\x02\x24\u{1b1}\x03\x02\x02\
	\x02\x26\u{1c5}\x03\x02\x02\x02\x28\u{1d9}\x03\x02\x02\x02\x2a\u{1db}\x03\
	\x02\x02\x02\x2c\u{1e6}\x03\x02\x02\x02\x2e\u{201}\x03\x02\x02\x02\x30\u{20e}\
	\x03\x02\x02\x02\x32\u{212}\x03\x02\x02\x02\x34\u{21a}\x03\x02\x02\x02\x36\
	\u{221}\x03\x02\x02\x02\x38\u{24d}\x03\x02\x02\x02\x3a\u{256}\x03\x02\x02\
	\x02\x3c\u{258}\x03\x02\x02\x02\x3e\u{267}\x03\x02\x02\x02\x40\u{26b}\x03\
	\x02\x02\x02\x42\u{26f}\x03\x02\x02\x02\x44\u{276}\x03\x02\x02\x02\x46\u{27a}\
	\x03\x02\x02\x02\x48\u{293}\x03\x02\x02\x02\x4a\u{295}\x03\x02\x02\x02\x4c\
	\u{2a5}\x03\x02\x02\x02\x4e\u{2a7}\x03\x02\x02\x02\x50\u{2bf}\x03\x02\x02\
	\x02\x52\u{305}\x03\x02\x02\x02\x54\u{307}\x03\x02\x02\x02\x56\u{324}\x03\
	\x02\x02\x02\x58\u{326}\x03\x02\x02\x02\x5a\u{33b}\x03\x02\x02\x02\x5c\u{345}\
	\x03\x02\x02\x02\x5e\u{34b}\x03\x02\x02\x02\x60\u{361}\x03\x02\x02\x02\x62\
	\u{363}\x03\x02\x02\x02\x64\u{365}\x03\x02\x02\x02\x66\u{367}\x03\x02\x02\
	\x02\x68\u{371}\x03\x02\x02\x02\x6a\u{37b}\x03\x02\x02\x02\x6c\u{38b}\x03\
	\x02\x02\x02\x6e\u{390}\x03\x02\x02\x02\x70\u{39a}\x03\x02\x02\x02\x72\u{3b0}\
	\x03\x02\x02\x02\x74\u{3ce}\x03\x02\x02\x02\x76\u{3e2}\x03\x02\x02\x02\x78\
	\u{3e7}\x03\x02\x02\x02\x7a\u{409}\x03\x02\x02\x02\x7c\u{415}\x03\x02\x02\
	\x02\x7e\u{426}\x03\x02\x02\x02\u{80}\u{428}\x03\x02\x02\x02\u{82}\u{486}\
	\x03\x02\x02\x02\u{84}\u{48e}\x03\x02\x02\x02\u{86}\u{490}\x03\x02\x02\x02\
	\u{88}\u{492}\x03\x02\x02\x02\u{8a}\u{4c9}\x03\x02\x02\x02\u{8c}\u{4cb}\
	\x03\x02\x02\x02\u{8e}\u{4d5}\x03\x02\x02\x02\u{90}\u{4de}\x03\x02\x02\x02\
	\u{92}\u{4e5}\x03\x02\x02\x02\u{94}\u{4eb}\x03\x02\x02\x02\u{96}\u{512}\
	\x03\x02\x02\x02\u{98}\u{514}\x03\x02\x02\x02\u{9a}\u{531}\x03\x02\x02\x02\
	\u{9c}\u{533}\x03\x02\x02\x02\u{9e}\u{535}\x03\x02\x02\x02\u{a0}\u{53d}\
	\x03\x02\x02\x02\u{a2}\u{540}\x03\x02\x02\x02\u{a4}\u{554}\x03\x02\x02\x02\
	\u{a6}\u{57a}\x03\x02\x02\x02\u{a8}\u{596}\x03\x02\x02\x02\u{aa}\u{5a7}\
	\x03\x02\x02\x02\u{ac}\u{5b5}\x03\x02\x02\x02\u{ae}\u{5b9}\x03\x02\x02\x02\
	\u{b0}\u{5bb}\x03\x02\x02\x02\u{b2}\u{5e4}\x03\x02\x02\x02\u{b4}\u{5e9}\
	\x03\x02\x02\x02\u{b6}\u{5f2}\x03\x02\x02\x02\u{b8}\u{5f4}\x03\x02\x02\x02\
	\u{ba}\u{5f6}\x03\x02\x02\x02\u{bc}\u{5fa}\x03\x02\x02\x02\u{be}\u{5fc}\
	\x03\x02\x02\x02\u{c0}\u{5fe}\x03\x02\x02\x02\u{c2}\u{600}\x03\x02\x02\x02\
	\u{c4}\u{602}\x03\x02\x02\x02\u{c6}\u{604}\x03\x02\x02\x02\u{c8}\u{ca}\x07\
	\x7f\x02\x02\u{c9}\u{c8}\x03\x02\x02\x02\u{c9}\u{ca}\x03\x02\x02\x02\u{ca}\
	\u{cb}\x03\x02\x02\x02\u{cb}\u{d0}\x05\x04\x03\x02\u{cc}\u{ce}\x07\x7f\x02\
	\x02\u{cd}\u{cc}\x03\x02\x02\x02\u{cd}\u{ce}\x03\x02\x02\x02\u{ce}\u{cf}\
	\x03\x02\x02\x02\u{cf}\u{d1}\x07\x03\x02\x02\u{d0}\u{cd}\x03\x02\x02\x02\
	\u{d0}\u{d1}\x03\x02\x02\x02\u{d1}\u{d3}\x03\x02\x02\x02\u{d2}\u{d4}\x07\
	\x7f\x02\x02\u{d3}\u{d2}\x03\x02\x02\x02\u{d3}\u{d4}\x03\x02\x02\x02\u{d4}\
	\u{d5}\x03\x02\x02\x02\u{d5}\u{d6}\x07\x02\x02\x03\u{d6}\x03\x03\x02\x02\
	\x02\u{d7}\u{d8}\x05\x06\x04\x02\u{d8}\x05\x03\x02\x02\x02\u{d9}\u{dc}\x05\
	\x08\x05\x02\u{da}\u{dc}\x05\x2c\x17\x02\u{db}\u{d9}\x03\x02\x02\x02\u{db}\
	\u{da}\x03\x02\x02\x02\u{dc}\x07\x03\x02\x02\x02\u{dd}\u{e4}\x05\x0c\x07\
	\x02\u{de}\u{e0}\x07\x7f\x02\x02\u{df}\u{de}\x03\x02\x02\x02\u{df}\u{e0}\
	\x03\x02\x02\x02\u{e0}\u{e1}\x03\x02\x02\x02\u{e1}\u{e3}\x05\x0a\x06\x02\
	\u{e2}\u{df}\x03\x02\x02\x02\u{e3}\u{e6}\x03\x02\x02\x02\u{e4}\u{e2}\x03\
	\x02\x02\x02\u{e4}\u{e5}\x03\x02\x02\x02\u{e5}\x09\x03\x02\x02\x02\u{e6}\
	\u{e4}\x03\x02\x02\x02\u{e7}\u{e8}\x07\x30\x02\x02\u{e8}\u{e9}\x07\x7f\x02\
	\x02\u{e9}\u{eb}\x07\x31\x02\x02\u{ea}\u{ec}\x07\x7f\x02\x02\u{eb}\u{ea}\
	\x03\x02\x02\x02\u{eb}\u{ec}\x03\x02\x02\x02\u{ec}\u{ed}\x03\x02\x02\x02\
	\u{ed}\u{f4}\x05\x0c\x07\x02\u{ee}\u{f0}\x07\x30\x02\x02\u{ef}\u{f1}\x07\
	\x7f\x02\x02\u{f0}\u{ef}\x03\x02\x02\x02\u{f0}\u{f1}\x03\x02\x02\x02\u{f1}\
	\u{f2}\x03\x02\x02\x02\u{f2}\u{f4}\x05\x0c\x07\x02\u{f3}\u{e7}\x03\x02\x02\
	\x02\u{f3}\u{ee}\x03\x02\x02\x02\u{f4}\x0b\x03\x02\x02\x02\u{f5}\u{f8}\x05\
	\x0e\x08\x02\u{f6}\u{f8}\x05\x10\x09\x02\u{f7}\u{f5}\x03\x02\x02\x02\u{f7}\
	\u{f6}\x03\x02\x02\x02\u{f8}\x0d\x03\x02\x02\x02\u{f9}\u{fb}\x05\x14\x0b\
	\x02\u{fa}\u{fc}\x07\x7f\x02\x02\u{fb}\u{fa}\x03\x02\x02\x02\u{fb}\u{fc}\
	\x03\x02\x02\x02\u{fc}\u{fe}\x03\x02\x02\x02\u{fd}\u{f9}\x03\x02\x02\x02\
	\u{fe}\u{101}\x03\x02\x02\x02\u{ff}\u{fd}\x03\x02\x02\x02\u{ff}\u{100}\x03\
	\x02\x02\x02\u{100}\u{102}\x03\x02\x02\x02\u{101}\u{ff}\x03\x02\x02\x02\
	\u{102}\u{11d}\x05\x34\x1b\x02\u{103}\u{105}\x05\x14\x0b\x02\u{104}\u{106}\
	\x07\x7f\x02\x02\u{105}\u{104}\x03\x02\x02\x02\u{105}\u{106}\x03\x02\x02\
	\x02\u{106}\u{108}\x03\x02\x02\x02\u{107}\u{103}\x03\x02\x02\x02\u{108}\
	\u{10b}\x03\x02\x02\x02\u{109}\u{107}\x03\x02\x02\x02\u{109}\u{10a}\x03\
	\x02\x02\x02\u{10a}\u{10c}\x03\x02\x02\x02\u{10b}\u{109}\x03\x02\x02\x02\
	\u{10c}\u{113}\x05\x12\x0a\x02\u{10d}\u{10f}\x07\x7f\x02\x02\u{10e}\u{10d}\
	\x03\x02\x02\x02\u{10e}\u{10f}\x03\x02\x02\x02\u{10f}\u{110}\x03\x02\x02\
	\x02\u{110}\u{112}\x05\x12\x0a\x02\u{111}\u{10e}\x03\x02\x02\x02\u{112}\
	\u{115}\x03\x02\x02\x02\u{113}\u{111}\x03\x02\x02\x02\u{113}\u{114}\x03\
	\x02\x02\x02\u{114}\u{11a}\x03\x02\x02\x02\u{115}\u{113}\x03\x02\x02\x02\
	\u{116}\u{118}\x07\x7f\x02\x02\u{117}\u{116}\x03\x02\x02\x02\u{117}\u{118}\
	\x03\x02\x02\x02\u{118}\u{119}\x03\x02\x02\x02\u{119}\u{11b}\x05\x34\x1b\
	\x02\u{11a}\u{117}\x03\x02\x02\x02\u{11a}\u{11b}\x03\x02\x02\x02\u{11b}\
	\u{11d}\x03\x02\x02\x02\u{11c}\u{ff}\x03\x02\x02\x02\u{11c}\u{109}\x03\x02\
	\x02\x02\u{11d}\x0f\x03\x02\x02\x02\u{11e}\u{120}\x05\x14\x0b\x02\u{11f}\
	\u{121}\x07\x7f\x02\x02\u{120}\u{11f}\x03\x02\x02\x02\u{120}\u{121}\x03\
	\x02\x02\x02\u{121}\u{123}\x03\x02\x02\x02\u{122}\u{11e}\x03\x02\x02\x02\
	\u{123}\u{126}\x03\x02\x02\x02\u{124}\u{122}\x03\x02\x02\x02\u{124}\u{125}\
	\x03\x02\x02\x02\u{125}\u{12d}\x03\x02\x02\x02\u{126}\u{124}\x03\x02\x02\
	\x02\u{127}\u{129}\x05\x12\x0a\x02\u{128}\u{12a}\x07\x7f\x02\x02\u{129}\
	\u{128}\x03\x02\x02\x02\u{129}\u{12a}\x03\x02\x02\x02\u{12a}\u{12c}\x03\
	\x02\x02\x02\u{12b}\u{127}\x03\x02\x02\x02\u{12c}\u{12f}\x03\x02\x02\x02\
	\u{12d}\u{12b}\x03\x02\x02\x02\u{12d}\u{12e}\x03\x02\x02\x02\u{12e}\u{130}\
	\x03\x02\x02\x02\u{12f}\u{12d}\x03\x02\x02\x02\u{130}\u{132}\x05\x32\x1a\
	\x02\u{131}\u{133}\x07\x7f\x02\x02\u{132}\u{131}\x03\x02\x02\x02\u{132}\
	\u{133}\x03\x02\x02\x02\u{133}\u{135}\x03\x02\x02\x02\u{134}\u{124}\x03\
	\x02\x02\x02\u{135}\u{136}\x03\x02\x02\x02\u{136}\u{134}\x03\x02\x02\x02\
	\u{136}\u{137}\x03\x02\x02\x02\u{137}\u{138}\x03\x02\x02\x02\u{138}\u{139}\
	\x05\x0e\x08\x02\u{139}\x11\x03\x02\x02\x02\u{13a}\u{140}\x05\x1e\x10\x02\
	\u{13b}\u{140}\x05\x1a\x0e\x02\u{13c}\u{140}\x05\x24\x13\x02\u{13d}\u{140}\
	\x05\x20\x11\x02\u{13e}\u{140}\x05\x26\x14\x02\u{13f}\u{13a}\x03\x02\x02\
	\x02\u{13f}\u{13b}\x03\x02\x02\x02\u{13f}\u{13c}\x03\x02\x02\x02\u{13f}\
	\u{13d}\x03\x02\x02\x02\u{13f}\u{13e}\x03\x02\x02\x02\u{140}\x13\x03\x02\
	\x02\x02\u{141}\u{145}\x05\x16\x0c\x02\u{142}\u{145}\x05\x18\x0d\x02\u{143}\
	\u{145}\x05\x2a\x16\x02\u{144}\u{141}\x03\x02\x02\x02\u{144}\u{142}\x03\
	\x02\x02\x02\u{144}\u{143}\x03\x02\x02\x02\u{145}\x15\x03\x02\x02\x02\u{146}\
	\u{147}\x07\x32\x02\x02\u{147}\u{149}\x07\x7f\x02\x02\u{148}\u{146}\x03\
	\x02\x02\x02\u{148}\u{149}\x03\x02\x02\x02\u{149}\u{14a}\x03\x02\x02\x02\
	\u{14a}\u{14c}\x07\x33\x02\x02\u{14b}\u{14d}\x07\x7f\x02\x02\u{14c}\u{14b}\
	\x03\x02\x02\x02\u{14c}\u{14d}\x03\x02\x02\x02\u{14d}\u{14e}\x03\x02\x02\
	\x02\u{14e}\u{153}\x05\x46\x24\x02\u{14f}\u{151}\x07\x7f\x02\x02\u{150}\
	\u{14f}\x03\x02\x02\x02\u{150}\u{151}\x03\x02\x02\x02\u{151}\u{152}\x03\
	\x02\x02\x02\u{152}\u{154}\x05\x44\x23\x02\u{153}\u{150}\x03\x02\x02\x02\
	\u{153}\u{154}\x03\x02\x02\x02\u{154}\x17\x03\x02\x02\x02\u{155}\u{157}\
	\x07\x34\x02\x02\u{156}\u{158}\x07\x7f\x02\x02\u{157}\u{156}\x03\x02\x02\
	\x02\u{157}\u{158}\x03\x02\x02\x02\u{158}\u{159}\x03\x02\x02\x02\u{159}\
	\u{15a}\x05\x64\x33\x02\u{15a}\u{15b}\x07\x7f\x02\x02\u{15b}\u{15c}\x07\
	\x35\x02\x02\u{15c}\u{15d}\x07\x7f\x02\x02\u{15d}\u{15e}\x05\u{ac}\x57\x02\
	\u{15e}\x19\x03\x02\x02\x02\u{15f}\u{161}\x07\x36\x02\x02\u{160}\u{162}\
	\x07\x7f\x02\x02\u{161}\u{160}\x03\x02\x02\x02\u{161}\u{162}\x03\x02\x02\
	\x02\u{162}\u{163}\x03\x02\x02\x02\u{163}\u{168}\x05\x48\x25\x02\u{164}\
	\u{165}\x07\x7f\x02\x02\u{165}\u{167}\x05\x1c\x0f\x02\u{166}\u{164}\x03\
	\x02\x02\x02\u{167}\u{16a}\x03\x02\x02\x02\u{168}\u{166}\x03\x02\x02\x02\
	\u{168}\u{169}\x03\x02\x02\x02\u{169}\x1b\x03\x02\x02\x02\u{16a}\u{168}\
	\x03\x02\x02\x02\u{16b}\u{16c}\x07\x37\x02\x02\u{16c}\u{16d}\x07\x7f\x02\
	\x02\u{16d}\u{16e}\x07\x33\x02\x02\u{16e}\u{16f}\x07\x7f\x02\x02\u{16f}\
	\u{176}\x05\x20\x11\x02\u{170}\u{171}\x07\x37\x02\x02\u{171}\u{172}\x07\
	\x7f\x02\x02\u{172}\u{173}\x07\x38\x02\x02\u{173}\u{174}\x07\x7f\x02\x02\
	\u{174}\u{176}\x05\x20\x11\x02\u{175}\u{16b}\x03\x02\x02\x02\u{175}\u{170}\
	\x03\x02\x02\x02\u{176}\x1d\x03\x02\x02\x02\u{177}\u{179}\x07\x38\x02\x02\
	\u{178}\u{17a}\x07\x7f\x02\x02\u{179}\u{178}\x03\x02\x02\x02\u{179}\u{17a}\
	\x03\x02\x02\x02\u{17a}\u{17b}\x03\x02\x02\x02\u{17b}\u{17c}\x05\x46\x24\
	\x02\u{17c}\x1f\x03\x02\x02\x02\u{17d}\u{17f}\x07\x39\x02\x02\u{17e}\u{180}\
	\x07\x7f\x02\x02\u{17f}\u{17e}\x03\x02\x02\x02\u{17f}\u{180}\x03\x02\x02\
	\x02\u{180}\u{181}\x03\x02\x02\x02\u{181}\u{186}\x05\x22\x12\x02\u{182}\
	\u{183}\x07\x04\x02\x02\u{183}\u{185}\x05\x22\x12\x02\u{184}\u{182}\x03\
	\x02\x02\x02\u{185}\u{188}\x03\x02\x02\x02\u{186}\u{184}\x03\x02\x02\x02\
	\u{186}\u{187}\x03\x02\x02\x02\u{187}\x21\x03\x02\x02\x02\u{188}\u{186}\
	\x03\x02\x02\x02\u{189}\u{18b}\x05\u{b4}\x5b\x02\u{18a}\u{18c}\x07\x7f\x02\
	\x02\u{18b}\u{18a}\x03\x02\x02\x02\u{18b}\u{18c}\x03\x02\x02\x02\u{18c}\
	\u{18d}\x03\x02\x02\x02\u{18d}\u{18f}\x07\x05\x02\x02\u{18e}\u{190}\x07\
	\x7f\x02\x02\u{18f}\u{18e}\x03\x02\x02\x02\u{18f}\u{190}\x03\x02\x02\x02\
	\u{190}\u{191}\x03\x02\x02\x02\u{191}\u{192}\x05\x64\x33\x02\u{192}\u{1ae}\
	\x03\x02\x02\x02\u{193}\u{195}\x05\u{ac}\x57\x02\u{194}\u{196}\x07\x7f\x02\
	\x02\u{195}\u{194}\x03\x02\x02\x02\u{195}\u{196}\x03\x02\x02\x02\u{196}\
	\u{197}\x03\x02\x02\x02\u{197}\u{199}\x07\x05\x02\x02\u{198}\u{19a}\x07\
	\x7f\x02\x02\u{199}\u{198}\x03\x02\x02\x02\u{199}\u{19a}\x03\x02\x02\x02\
	\u{19a}\u{19b}\x03\x02\x02\x02\u{19b}\u{19c}\x05\x64\x33\x02\u{19c}\u{1ae}\
	\x03\x02\x02\x02\u{19d}\u{19f}\x05\u{ac}\x57\x02\u{19e}\u{1a0}\x07\x7f\x02\
	\x02\u{19f}\u{19e}\x03\x02\x02\x02\u{19f}\u{1a0}\x03\x02\x02\x02\u{1a0}\
	\u{1a1}\x03\x02\x02\x02\u{1a1}\u{1a3}\x07\x06\x02\x02\u{1a2}\u{1a4}\x07\
	\x7f\x02\x02\u{1a3}\u{1a2}\x03\x02\x02\x02\u{1a3}\u{1a4}\x03\x02\x02\x02\
	\u{1a4}\u{1a5}\x03\x02\x02\x02\u{1a5}\u{1a6}\x05\x64\x33\x02\u{1a6}\u{1ae}\
	\x03\x02\x02\x02\u{1a7}\u{1a9}\x05\u{ac}\x57\x02\u{1a8}\u{1aa}\x07\x7f\x02\
	\x02\u{1a9}\u{1a8}\x03\x02\x02\x02\u{1a9}\u{1aa}\x03\x02\x02\x02\u{1aa}\
	\u{1ab}\x03\x02\x02\x02\u{1ab}\u{1ac}\x05\x5a\x2e\x02\u{1ac}\u{1ae}\x03\
	\x02\x02\x02\u{1ad}\u{189}\x03\x02\x02\x02\u{1ad}\u{193}\x03\x02\x02\x02\
	\u{1ad}\u{19d}\x03\x02\x02\x02\u{1ad}\u{1a7}\x03\x02\x02\x02\u{1ae}\x23\
	\x03\x02\x02\x02\u{1af}\u{1b0}\x07\x3a\x02\x02\u{1b0}\u{1b2}\x07\x7f\x02\
	\x02\u{1b1}\u{1af}\x03\x02\x02\x02\u{1b1}\u{1b2}\x03\x02\x02\x02\u{1b2}\
	\u{1b3}\x03\x02\x02\x02\u{1b3}\u{1b5}\x07\x3b\x02\x02\u{1b4}\u{1b6}\x07\
	\x7f\x02\x02\u{1b5}\u{1b4}\x03\x02\x02\x02\u{1b5}\u{1b6}\x03\x02\x02\x02\
	\u{1b6}\u{1b7}\x03\x02\x02\x02\u{1b7}\u{1c2}\x05\x64\x33\x02\u{1b8}\u{1ba}\
	\x07\x7f\x02\x02\u{1b9}\u{1b8}\x03\x02\x02\x02\u{1b9}\u{1ba}\x03\x02\x02\
	\x02\u{1ba}\u{1bb}\x03\x02\x02\x02\u{1bb}\u{1bd}\x07\x04\x02\x02\u{1bc}\
	\u{1be}\x07\x7f\x02\x02\u{1bd}\u{1bc}\x03\x02\x02\x02\u{1bd}\u{1be}\x03\
	\x02\x02\x02\u{1be}\u{1bf}\x03\x02\x02\x02\u{1bf}\u{1c1}\x05\x64\x33\x02\
	\u{1c0}\u{1b9}\x03\x02\x02\x02\u{1c1}\u{1c4}\x03\x02\x02\x02\u{1c2}\u{1c0}\
	\x03\x02\x02\x02\u{1c2}\u{1c3}\x03\x02\x02\x02\u{1c3}\x25\x03\x02\x02\x02\
	\u{1c4}\u{1c2}\x03\x02\x02\x02\u{1c5}\u{1c6}\x07\x3c\x02\x02\u{1c6}\u{1c7}\
	\x07\x7f\x02\x02\u{1c7}\u{1d2}\x05\x28\x15\x02\u{1c8}\u{1ca}\x07\x7f\x02\
	\x02\u{1c9}\u{1c8}\x03\x02\x02\x02\u{1c9}\u{1ca}\x03\x02\x02\x02\u{1ca}\
	\u{1cb}\x03\x02\x02\x02\u{1cb}\u{1cd}\x07\x04\x02\x02\u{1cc}\u{1ce}\x07\
	\x7f\x02\x02\u{1cd}\u{1cc}\x03\x02\x02\x02\u{1cd}\u{1ce}\x03\x02\x02\x02\
	\u{1ce}\u{1cf}\x03\x02\x02\x02\u{1cf}\u{1d1}\x05\x28\x15\x02\u{1d0}\u{1c9}\
	\x03\x02\x02\x02\u{1d1}\u{1d4}\x03\x02\x02\x02\u{1d2}\u{1d0}\x03\x02\x02\
	\x02\u{1d2}\u{1d3}\x03\x02\x02\x02\u{1d3}\x27\x03\x02\x02\x02\u{1d4}\u{1d2}\
	\x03\x02\x02\x02\u{1d5}\u{1d6}\x05\u{ac}\x57\x02\u{1d6}\u{1d7}\x05\x5a\x2e\
	\x02\u{1d7}\u{1da}\x03\x02\x02\x02\u{1d8}\u{1da}\x05\u{b4}\x5b\x02\u{1d9}\
	\u{1d5}\x03\x02\x02\x02\u{1d9}\u{1d8}\x03\x02\x02\x02\u{1da}\x29\x03\x02\
	\x02\x02\u{1db}\u{1dc}\x07\x3d\x02\x02\u{1dc}\u{1dd}\x07\x7f\x02\x02\u{1dd}\
	\u{1e4}\x05\u{98}\x4d\x02\u{1de}\u{1e0}\x07\x7f\x02\x02\u{1df}\u{1de}\x03\
	\x02\x02\x02\u{1df}\u{1e0}\x03\x02\x02\x02\u{1e0}\u{1e1}\x03\x02\x02\x02\
	\u{1e1}\u{1e2}\x07\x3e\x02\x02\u{1e2}\u{1e3}\x07\x7f\x02\x02\u{1e3}\u{1e5}\
	\x05\x2e\x18\x02\u{1e4}\u{1df}\x03\x02\x02\x02\u{1e4}\u{1e5}\x03\x02\x02\
	\x02\u{1e5}\x2b\x03\x02\x02\x02\u{1e6}\u{1e7}\x07\x3d\x02\x02\u{1e7}\u{1ea}\
	\x07\x7f\x02\x02\u{1e8}\u{1eb}\x05\u{98}\x4d\x02\u{1e9}\u{1eb}\x05\u{9a}\
	\x4e\x02\u{1ea}\u{1e8}\x03\x02\x02\x02\u{1ea}\u{1e9}\x03\x02\x02\x02\u{1eb}\
	\u{1f0}\x03\x02\x02\x02\u{1ec}\u{1ed}\x07\x7f\x02\x02\u{1ed}\u{1ee}\x07\
	\x3e\x02\x02\u{1ee}\u{1ef}\x07\x7f\x02\x02\u{1ef}\u{1f1}\x05\x2e\x18\x02\
	\u{1f0}\u{1ec}\x03\x02\x02\x02\u{1f0}\u{1f1}\x03\x02\x02\x02\u{1f1}\x2d\
	\x03\x02\x02\x02\u{1f2}\u{202}\x07\x07\x02\x02\u{1f3}\u{1fe}\x05\x30\x19\
	\x02\u{1f4}\u{1f6}\x07\x7f\x02\x02\u{1f5}\u{1f4}\x03\x02\x02\x02\u{1f5}\
	\u{1f6}\x03\x02\x02\x02\u{1f6}\u{1f7}\x03\x02\x02\x02\u{1f7}\u{1f9}\x07\
	\x04\x02\x02\u{1f8}\u{1fa}\x07\x7f\x02\x02\u{1f9}\u{1f8}\x03\x02\x02\x02\
	\u{1f9}\u{1fa}\x03\x02\x02\x02\u{1fa}\u{1fb}\x03\x02\x02\x02\u{1fb}\u{1fd}\
	\x05\x30\x19\x02\u{1fc}\u{1f5}\x03\x02\x02\x02\u{1fd}\u{200}\x03\x02\x02\
	\x02\u{1fe}\u{1fc}\x03\x02\x02\x02\u{1fe}\u{1ff}\x03\x02\x02\x02\u{1ff}\
	\u{202}\x03\x02\x02\x02\u{200}\u{1fe}\x03\x02\x02\x02\u{201}\u{1f2}\x03\
	\x02\x02\x02\u{201}\u{1f3}\x03\x02\x02\x02\u{202}\u{207}\x03\x02\x02\x02\
	\u{203}\u{205}\x07\x7f\x02\x02\u{204}\u{203}\x03\x02\x02\x02\u{204}\u{205}\
	\x03\x02\x02\x02\u{205}\u{206}\x03\x02\x02\x02\u{206}\u{208}\x05\x44\x23\
	\x02\u{207}\u{204}\x03\x02\x02\x02\u{207}\u{208}\x03\x02\x02\x02\u{208}\
	\x2f\x03\x02\x02\x02\u{209}\u{20a}\x05\u{9c}\x4f\x02\u{20a}\u{20b}\x07\x7f\
	\x02\x02\u{20b}\u{20c}\x07\x35\x02\x02\u{20c}\u{20d}\x07\x7f\x02\x02\u{20d}\
	\u{20f}\x03\x02\x02\x02\u{20e}\u{209}\x03\x02\x02\x02\u{20e}\u{20f}\x03\
	\x02\x02\x02\u{20f}\u{210}\x03\x02\x02\x02\u{210}\u{211}\x05\u{ac}\x57\x02\
	\u{211}\x31\x03\x02\x02\x02\u{212}\u{213}\x07\x3f\x02\x02\u{213}\u{218}\
	\x05\x36\x1c\x02\u{214}\u{216}\x07\x7f\x02\x02\u{215}\u{214}\x03\x02\x02\
	\x02\u{215}\u{216}\x03\x02\x02\x02\u{216}\u{217}\x03\x02\x02\x02\u{217}\
	\u{219}\x05\x44\x23\x02\u{218}\u{215}\x03\x02\x02\x02\u{218}\u{219}\x03\
	\x02\x02\x02\u{219}\x33\x03\x02\x02\x02\u{21a}\u{21b}\x07\x40\x02\x02\u{21b}\
	\u{21c}\x05\x36\x1c\x02\u{21c}\x35\x03\x02\x02\x02\u{21d}\u{21f}\x07\x7f\
	\x02\x02\u{21e}\u{21d}\x03\x02\x02\x02\u{21e}\u{21f}\x03\x02\x02\x02\u{21f}\
	\u{220}\x03\x02\x02\x02\u{220}\u{222}\x07\x41\x02\x02\u{221}\u{21e}\x03\
	\x02\x02\x02\u{221}\u{222}\x03\x02\x02\x02\u{222}\u{223}\x03\x02\x02\x02\
	\u{223}\u{224}\x07\x7f\x02\x02\u{224}\u{227}\x05\x38\x1d\x02\u{225}\u{226}\
	\x07\x7f\x02\x02\u{226}\u{228}\x05\x3c\x1f\x02\u{227}\u{225}\x03\x02\x02\
	\x02\u{227}\u{228}\x03\x02\x02\x02\u{228}\u{22b}\x03\x02\x02\x02\u{229}\
	\u{22a}\x07\x7f\x02\x02\u{22a}\u{22c}\x05\x3e\x20\x02\u{22b}\u{229}\x03\
	\x02\x02\x02\u{22b}\u{22c}\x03\x02\x02\x02\u{22c}\u{22f}\x03\x02\x02\x02\
	\u{22d}\u{22e}\x07\x7f\x02\x02\u{22e}\u{230}\x05\x40\x21\x02\u{22f}\u{22d}\
	\x03\x02\x02\x02\u{22f}\u{230}\x03\x02\x02\x02\u{230}\x37\x03\x02\x02\x02\
	\u{231}\u{23c}\x07\x07\x02\x02\u{232}\u{234}\x07\x7f\x02\x02\u{233}\u{232}\
	\x03\x02\x02\x02\u{233}\u{234}\x03\x02\x02\x02\u{234}\u{235}\x03\x02\x02\
	\x02\u{235}\u{237}\x07\x04\x02\x02\u{236}\u{238}\x07\x7f\x02\x02\u{237}\
	\u{236}\x03\x02\x02\x02\u{237}\u{238}\x03\x02\x02\x02\u{238}\u{239}\x03\
	\x02\x02\x02\u{239}\u{23b}\x05\x3a\x1e\x02\u{23a}\u{233}\x03\x02\x02\x02\
	\u{23b}\u{23e}\x03\x02\x02\x02\u{23c}\u{23a}\x03\x02\x02\x02\u{23c}\u{23d}\
	\x03\x02\x02\x02\u{23d}\u{24e}\x03\x02\x02\x02\u{23e}\u{23c}\x03\x02\x02\
	\x02\u{23f}\u{24a}\x05\x3a\x1e\x02\u{240}\u{242}\x07\x7f\x02\x02\u{241}\
	\u{240}\x03\x02\x02\x02\u{241}\u{242}\x03\x02\x02\x02\u{242}\u{243}\x03\
	\x02\x02\x02\u{243}\u{245}\x07\x04\x02\x02\u{244}\u{246}\x07\x7f\x02\x02\
	\u{245}\u{244}\x03\x02\x02\x02\u{245}\u{246}\x03\x02\x02\x02\u{246}\u{247}\
	\x03\x02\x02\x02\u{247}\u{249}\x05\x3a\x1e\x02\u{248}\u{241}\x03\x02\x02\
	\x02\u{249}\u{24c}\x03\x02\x02\x02\u{24a}\u{248}\x03\x02\x02\x02\u{24a}\
	\u{24b}\x03\x02\x02\x02\u{24b}\u{24e}\x03\x02\x02\x02\u{24c}\u{24a}\x03\
	\x02\x02\x02\u{24d}\u{231}\x03\x02\x02\x02\u{24d}\u{23f}\x03\x02\x02\x02\
	\u{24e}\x39\x03\x02\x02\x02\u{24f}\u{250}\x05\x64\x33\x02\u{250}\u{251}\
	\x07\x7f\x02\x02\u{251}\u{252}\x07\x35\x02\x02\u{252}\u{253}\x07\x7f\x02\
	\x02\u{253}\u{254}\x05\u{ac}\x57\x02\u{254}\u{257}\x03\x02\x02\x02\u{255}\
	\u{257}\x05\x64\x33\x02\u{256}\u{24f}\x03\x02\x02\x02\u{256}\u{255}\x03\
	\x02\x02\x02\u{257}\x3b\x03\x02\x02\x02\u{258}\u{259}\x07\x42\x02\x02\u{259}\
	\u{25a}\x07\x7f\x02\x02\u{25a}\u{25b}\x07\x43\x02\x02\u{25b}\u{25c}\x07\
	\x7f\x02\x02\u{25c}\u{264}\x05\x42\x22\x02\u{25d}\u{25f}\x07\x04\x02\x02\
	\u{25e}\u{260}\x07\x7f\x02\x02\u{25f}\u{25e}\x03\x02\x02\x02\u{25f}\u{260}\
	\x03\x02\x02\x02\u{260}\u{261}\x03\x02\x02\x02\u{261}\u{263}\x05\x42\x22\
	\x02\u{262}\u{25d}\x03\x02\x02\x02\u{263}\u{266}\x03\x02\x02\x02\u{264}\
	\u{262}\x03\x02\x02\x02\u{264}\u{265}\x03\x02\x02\x02\u{265}\x3d\x03\x02\
	\x02\x02\u{266}\u{264}\x03\x02\x02\x02\u{267}\u{268}\x07\x44\x02\x02\u{268}\
	\u{269}\x07\x7f\x02\x02\u{269}\u{26a}\x05\x64\x33\x02\u{26a}\x3f\x03\x02\
	\x02\x02\u{26b}\u{26c}\x07\x45\x02\x02\u{26c}\u{26d}\x07\x7f\x02\x02\u{26d}\
	\u{26e}\x05\x64\x33\x02\u{26e}\x41\x03\x02\x02\x02\u{26f}\u{274}\x05\x64\
	\x33\x02\u{270}\u{272}\x07\x7f\x02\x02\u{271}\u{270}\x03\x02\x02\x02\u{271}\
	\u{272}\x03\x02\x02\x02\u{272}\u{273}\x03\x02\x02\x02\u{273}\u{275}\x09\
	\x02\x02\x02\u{274}\u{271}\x03\x02\x02\x02\u{274}\u{275}\x03\x02\x02\x02\
	\u{275}\x43\x03\x02\x02\x02\u{276}\u{277}\x07\x4a\x02\x02\u{277}\u{278}\
	\x07\x7f\x02\x02\u{278}\u{279}\x05\x64\x33\x02\u{279}\x45\x03\x02\x02\x02\
	\u{27a}\u{285}\x05\x48\x25\x02\u{27b}\u{27d}\x07\x7f\x02\x02\u{27c}\u{27b}\
	\x03\x02\x02\x02\u{27c}\u{27d}\x03\x02\x02\x02\u{27d}\u{27e}\x03\x02\x02\
	\x02\u{27e}\u{280}\x07\x04\x02\x02\u{27f}\u{281}\x07\x7f\x02\x02\u{280}\
	\u{27f}\x03\x02\x02\x02\u{280}\u{281}\x03\x02\x02\x02\u{281}\u{282}\x03\
	\x02\x02\x02\u{282}\u{284}\x05\x48\x25\x02\u{283}\u{27c}\x03\x02\x02\x02\
	\u{284}\u{287}\x03\x02\x02\x02\u{285}\u{283}\x03\x02\x02\x02\u{285}\u{286}\
	\x03\x02\x02\x02\u{286}\x47\x03\x02\x02\x02\u{287}\u{285}\x03\x02\x02\x02\
	\u{288}\u{28a}\x05\u{ac}\x57\x02\u{289}\u{28b}\x07\x7f\x02\x02\u{28a}\u{289}\
	\x03\x02\x02\x02\u{28a}\u{28b}\x03\x02\x02\x02\u{28b}\u{28c}\x03\x02\x02\
	\x02\u{28c}\u{28e}\x07\x05\x02\x02\u{28d}\u{28f}\x07\x7f\x02\x02\u{28e}\
	\u{28d}\x03\x02\x02\x02\u{28e}\u{28f}\x03\x02\x02\x02\u{28f}\u{290}\x03\
	\x02\x02\x02\u{290}\u{291}\x05\x4a\x26\x02\u{291}\u{294}\x03\x02\x02\x02\
	\u{292}\u{294}\x05\x4a\x26\x02\u{293}\u{288}\x03\x02\x02\x02\u{293}\u{292}\
	\x03\x02\x02\x02\u{294}\x49\x03\x02\x02\x02\u{295}\u{296}\x05\x4c\x27\x02\
	\u{296}\x4b\x03\x02\x02\x02\u{297}\u{29e}\x05\x4e\x28\x02\u{298}\u{29a}\
	\x07\x7f\x02\x02\u{299}\u{298}\x03\x02\x02\x02\u{299}\u{29a}\x03\x02\x02\
	\x02\u{29a}\u{29b}\x03\x02\x02\x02\u{29b}\u{29d}\x05\x50\x29\x02\u{29c}\
	\u{299}\x03\x02\x02\x02\u{29d}\u{2a0}\x03\x02\x02\x02\u{29e}\u{29c}\x03\
	\x02\x02\x02\u{29e}\u{29f}\x03\x02\x02\x02\u{29f}\u{2a6}\x03\x02\x02\x02\
	\u{2a0}\u{29e}\x03\x02\x02\x02\u{2a1}\u{2a2}\x07\x08\x02\x02\u{2a2}\u{2a3}\
	\x05\x4c\x27\x02\u{2a3}\u{2a4}\x07\x09\x02\x02\u{2a4}\u{2a6}\x03\x02\x02\
	\x02\u{2a5}\u{297}\x03\x02\x02\x02\u{2a5}\u{2a1}\x03\x02\x02\x02\u{2a6}\
	\x4d\x03\x02\x02\x02\u{2a7}\u{2a9}\x07\x08\x02\x02\u{2a8}\u{2aa}\x07\x7f\
	\x02\x02\u{2a9}\u{2a8}\x03\x02\x02\x02\u{2a9}\u{2aa}\x03\x02\x02\x02\u{2aa}\
	\u{2af}\x03\x02\x02\x02\u{2ab}\u{2ad}\x05\u{ac}\x57\x02\u{2ac}\u{2ae}\x07\
	\x7f\x02\x02\u{2ad}\u{2ac}\x03\x02\x02\x02\u{2ad}\u{2ae}\x03\x02\x02\x02\
	\u{2ae}\u{2b0}\x03\x02\x02\x02\u{2af}\u{2ab}\x03\x02\x02\x02\u{2af}\u{2b0}\
	\x03\x02\x02\x02\u{2b0}\u{2b5}\x03\x02\x02\x02\u{2b1}\u{2b3}\x05\x5a\x2e\
	\x02\u{2b2}\u{2b4}\x07\x7f\x02\x02\u{2b3}\u{2b2}\x03\x02\x02\x02\u{2b3}\
	\u{2b4}\x03\x02\x02\x02\u{2b4}\u{2b6}\x03\x02\x02\x02\u{2b5}\u{2b1}\x03\
	\x02\x02\x02\u{2b5}\u{2b6}\x03\x02\x02\x02\u{2b6}\u{2bb}\x03\x02\x02\x02\
	\u{2b7}\u{2b9}\x05\x56\x2c\x02\u{2b8}\u{2ba}\x07\x7f\x02\x02\u{2b9}\u{2b8}\
	\x03\x02\x02\x02\u{2b9}\u{2ba}\x03\x02\x02\x02\u{2ba}\u{2bc}\x03\x02\x02\
	\x02\u{2bb}\u{2b7}\x03\x02\x02\x02\u{2bb}\u{2bc}\x03\x02\x02\x02\u{2bc}\
	\u{2bd}\x03\x02\x02\x02\u{2bd}\u{2be}\x07\x09\x02\x02\u{2be}\x4f\x03\x02\
	\x02\x02\u{2bf}\u{2c1}\x05\x52\x2a\x02\u{2c0}\u{2c2}\x07\x7f\x02\x02\u{2c1}\
	\u{2c0}\x03\x02\x02\x02\u{2c1}\u{2c2}\x03\x02\x02\x02\u{2c2}\u{2c3}\x03\
	\x02\x02\x02\u{2c3}\u{2c4}\x05\x4e\x28\x02\u{2c4}\x51\x03\x02\x02\x02\u{2c5}\
	\u{2c7}\x05\u{c2}\x62\x02\u{2c6}\u{2c8}\x07\x7f\x02\x02\u{2c7}\u{2c6}\x03\
	\x02\x02\x02\u{2c7}\u{2c8}\x03\x02\x02\x02\u{2c8}\u{2c9}\x03\x02\x02\x02\
	\u{2c9}\u{2cb}\x05\u{c6}\x64\x02\u{2ca}\u{2cc}\x07\x7f\x02\x02\u{2cb}\u{2ca}\
	\x03\x02\x02\x02\u{2cb}\u{2cc}\x03\x02\x02\x02\u{2cc}\u{2ce}\x03\x02\x02\
	\x02\u{2cd}\u{2cf}\x05\x54\x2b\x02\u{2ce}\u{2cd}\x03\x02\x02\x02\u{2ce}\
	\u{2cf}\x03\x02\x02\x02\u{2cf}\u{2d1}\x03\x02\x02\x02\u{2d0}\u{2d2}\x07\
	\x7f\x02\x02\u{2d1}\u{2d0}\x03\x02\x02\x02\u{2d1}\u{2d2}\x03\x02\x02\x02\
	\u{2d2}\u{2d3}\x03\x02\x02\x02\u{2d3}\u{2d5}\x05\u{c6}\x64\x02\u{2d4}\u{2d6}\
	\x07\x7f\x02\x02\u{2d5}\u{2d4}\x03\x02\x02\x02\u{2d5}\u{2d6}\x03\x02\x02\
	\x02\u{2d6}\u{2d7}\x03\x02\x02\x02\u{2d7}\u{2d8}\x05\u{c4}\x63\x02\u{2d8}\
	\u{306}\x03\x02\x02\x02\u{2d9}\u{2db}\x05\u{c2}\x62\x02\u{2da}\u{2dc}\x07\
	\x7f\x02\x02\u{2db}\u{2da}\x03\x02\x02\x02\u{2db}\u{2dc}\x03\x02\x02\x02\
	\u{2dc}\u{2dd}\x03\x02\x02\x02\u{2dd}\u{2df}\x05\u{c6}\x64\x02\u{2de}\u{2e0}\
	\x07\x7f\x02\x02\u{2df}\u{2de}\x03\x02\x02\x02\u{2df}\u{2e0}\x03\x02\x02\
	\x02\u{2e0}\u{2e2}\x03\x02\x02\x02\u{2e1}\u{2e3}\x05\x54\x2b\x02\u{2e2}\
	\u{2e1}\x03\x02\x02\x02\u{2e2}\u{2e3}\x03\x02\x02\x02\u{2e3}\u{2e5}\x03\
	\x02\x02\x02\u{2e4}\u{2e6}\x07\x7f\x02\x02\u{2e5}\u{2e4}\x03\x02\x02\x02\
	\u{2e5}\u{2e6}\x03\x02\x02\x02\u{2e6}\u{2e7}\x03\x02\x02\x02\u{2e7}\u{2e8}\
	\x05\u{c6}\x64\x02\u{2e8}\u{306}\x03\x02\x02\x02\u{2e9}\u{2eb}\x05\u{c6}\
	\x64\x02\u{2ea}\u{2ec}\x07\x7f\x02\x02\u{2eb}\u{2ea}\x03\x02\x02\x02\u{2eb}\
	\u{2ec}\x03\x02\x02\x02\u{2ec}\u{2ee}\x03\x02\x02\x02\u{2ed}\u{2ef}\x05\
	\x54\x2b\x02\u{2ee}\u{2ed}\x03\x02\x02\x02\u{2ee}\u{2ef}\x03\x02\x02\x02\
	\u{2ef}\u{2f1}\x03\x02\x02\x02\u{2f0}\u{2f2}\x07\x7f\x02\x02\u{2f1}\u{2f0}\
	\x03\x02\x02\x02\u{2f1}\u{2f2}\x03\x02\x02\x02\u{2f2}\u{2f3}\x03\x02\x02\
	\x02\u{2f3}\u{2f5}\x05\u{c6}\x64\x02\u{2f4}\u{2f6}\x07\x7f\x02\x02\u{2f5}\
	\u{2f4}\x03\x02\x02\x02\u{2f5}\u{2f6}\x03\x02\x02\x02\u{2f6}\u{2f7}\x03\
	\x02\x02\x02\u{2f7}\u{2f8}\x05\u{c4}\x63\x02\u{2f8}\u{306}\x03\x02\x02\x02\
	\u{2f9}\u{2fb}\x05\u{c6}\x64\x02\u{2fa}\u{2fc}\x07\x7f\x02\x02\u{2fb}\u{2fa}\
	\x03\x02\x02\x02\u{2fb}\u{2fc}\x03\x02\x02\x02\u{2fc}\u{2fe}\x03\x02\x02\
	\x02\u{2fd}\u{2ff}\x05\x54\x2b\x02\u{2fe}\u{2fd}\x03\x02\x02\x02\u{2fe}\
	\u{2ff}\x03\x02\x02\x02\u{2ff}\u{301}\x03\x02\x02\x02\u{300}\u{302}\x07\
	\x7f\x02\x02\u{301}\u{300}\x03\x02\x02\x02\u{301}\u{302}\x03\x02\x02\x02\
	\u{302}\u{303}\x03\x02\x02\x02\u{303}\u{304}\x05\u{c6}\x64\x02\u{304}\u{306}\
	\x03\x02\x02\x02\u{305}\u{2c5}\x03\x02\x02\x02\u{305}\u{2d9}\x03\x02\x02\
	\x02\u{305}\u{2e9}\x03\x02\x02\x02\u{305}\u{2f9}\x03\x02\x02\x02\u{306}\
	\x53\x03\x02\x02\x02\u{307}\u{309}\x07\x0a\x02\x02\u{308}\u{30a}\x07\x7f\
	\x02\x02\u{309}\u{308}\x03\x02\x02\x02\u{309}\u{30a}\x03\x02\x02\x02\u{30a}\
	\u{30f}\x03\x02\x02\x02\u{30b}\u{30d}\x05\u{ac}\x57\x02\u{30c}\u{30e}\x07\
	\x7f\x02\x02\u{30d}\u{30c}\x03\x02\x02\x02\u{30d}\u{30e}\x03\x02\x02\x02\
	\u{30e}\u{310}\x03\x02\x02\x02\u{30f}\u{30b}\x03\x02\x02\x02\u{30f}\u{310}\
	\x03\x02\x02\x02\u{310}\u{315}\x03\x02\x02\x02\u{311}\u{313}\x05\x58\x2d\
	\x02\u{312}\u{314}\x07\x7f\x02\x02\u{313}\u{312}\x03\x02\x02\x02\u{313}\
	\u{314}\x03\x02\x02\x02\u{314}\u{316}\x03\x02\x02\x02\u{315}\u{311}\x03\
	\x02\x02\x02\u{315}\u{316}\x03\x02\x02\x02\u{316}\u{318}\x03\x02\x02\x02\
	\u{317}\u{319}\x05\x5e\x30\x02\u{318}\u{317}\x03\x02\x02\x02\u{318}\u{319}\
	\x03\x02\x02\x02\u{319}\u{31e}\x03\x02\x02\x02\u{31a}\u{31c}\x05\x56\x2c\
	\x02\u{31b}\u{31d}\x07\x7f\x02\x02\u{31c}\u{31b}\x03\x02\x02\x02\u{31c}\
	\u{31d}\x03\x02\x02\x02\u{31d}\u{31f}\x03\x02\x02\x02\u{31e}\u{31a}\x03\
	\x02\x02\x02\u{31e}\u{31f}\x03\x02\x02\x02\u{31f}\u{320}\x03\x02\x02\x02\
	\u{320}\u{321}\x07\x0b\x02\x02\u{321}\x55\x03\x02\x02\x02\u{322}\u{325}\
	\x05\u{b0}\x59\x02\u{323}\u{325}\x05\u{b2}\x5a\x02\u{324}\u{322}\x03\x02\
	\x02\x02\u{324}\u{323}\x03\x02\x02\x02\u{325}\x57\x03\x02\x02\x02\u{326}\
	\u{328}\x07\x0c\x02\x02\u{327}\u{329}\x07\x7f\x02\x02\u{328}\u{327}\x03\
	\x02\x02\x02\u{328}\u{329}\x03\x02\x02\x02\u{329}\u{32a}\x03\x02\x02\x02\
	\u{32a}\u{338}\x05\x62\x32\x02\u{32b}\u{32d}\x07\x7f\x02\x02\u{32c}\u{32b}\
	\x03\x02\x02\x02\u{32c}\u{32d}\x03\x02\x02\x02\u{32d}\u{32e}\x03\x02\x02\
	\x02\u{32e}\u{330}\x07\x0d\x02\x02\u{32f}\u{331}\x07\x0c\x02\x02\u{330}\
	\u{32f}\x03\x02\x02\x02\u{330}\u{331}\x03\x02\x02\x02\u{331}\u{333}\x03\
	\x02\x02\x02\u{332}\u{334}\x07\x7f\x02\x02\u{333}\u{332}\x03\x02\x02\x02\
	\u{333}\u{334}\x03\x02\x02\x02\u{334}\u{335}\x03\x02\x02\x02\u{335}\u{337}\
	\x05\x62\x32\x02\u{336}\u{32c}\x03\x02\x02\x02\u{337}\u{33a}\x03\x02\x02\
	\x02\u{338}\u{336}\x03\x02\x02\x02\u{338}\u{339}\x03\x02\x02\x02\u{339}\
	\x59\x03\x02\x02\x02\u{33a}\u{338}\x03\x02\x02\x02\u{33b}\u{342}\x05\x5c\
	\x2f\x02\u{33c}\u{33e}\x07\x7f\x02\x02\u{33d}\u{33c}\x03\x02\x02\x02\u{33d}\
	\u{33e}\x03\x02\x02\x02\u{33e}\u{33f}\x03\x02\x02\x02\u{33f}\u{341}\x05\
	\x5c\x2f\x02\u{340}\u{33d}\x03\x02\x02\x02\u{341}\u{344}\x03\x02\x02\x02\
	\u{342}\u{340}\x03\x02\x02\x02\u{342}\u{343}\x03\x02\x02\x02\u{343}\x5b\
	\x03\x02\x02\x02\u{344}\u{342}\x03\x02\x02\x02\u{345}\u{347}\x07\x0c\x02\
	\x02\u{346}\u{348}\x07\x7f\x02\x02\u{347}\u{346}\x03\x02\x02\x02\u{347}\
	\u{348}\x03\x02\x02\x02\u{348}\u{349}\x03\x02\x02\x02\u{349}\u{34a}\x05\
	\x60\x31\x02\u{34a}\x5d\x03\x02\x02\x02\u{34b}\u{34d}\x07\x07\x02\x02\u{34c}\
	\u{34e}\x07\x7f\x02\x02\u{34d}\u{34c}\x03\x02\x02\x02\u{34d}\u{34e}\x03\
	\x02\x02\x02\u{34e}\u{353}\x03\x02\x02\x02\u{34f}\u{351}\x05\u{b8}\x5d\x02\
	\u{350}\u{352}\x07\x7f\x02\x02\u{351}\u{350}\x03\x02\x02\x02\u{351}\u{352}\
	\x03\x02\x02\x02\u{352}\u{354}\x03\x02\x02\x02\u{353}\u{34f}\x03\x02\x02\
	\x02\u{353}\u{354}\x03\x02\x02\x02\u{354}\u{35f}\x03\x02\x02\x02\u{355}\
	\u{357}\x07\x0e\x02\x02\u{356}\u{358}\x07\x7f\x02\x02\u{357}\u{356}\x03\
	\x02\x02\x02\u{357}\u{358}\x03\x02\x02\x02\u{358}\u{35d}\x03\x02\x02\x02\
	\u{359}\u{35b}\x05\u{b8}\x5d\x02\u{35a}\u{35c}\x07\x7f\x02\x02\u{35b}\u{35a}\
	\x03\x02\x02\x02\u{35b}\u{35c}\x03\x02\x02\x02\u{35c}\u{35e}\x03\x02\x02\
	\x02\u{35d}\u{359}\x03\x02\x02\x02\u{35d}\u{35e}\x03\x02\x02\x02\u{35e}\
	\u{360}\x03\x02\x02\x02\u{35f}\u{355}\x03\x02\x02\x02\u{35f}\u{360}\x03\
	\x02\x02\x02\u{360}\x5f\x03\x02\x02\x02\u{361}\u{362}\x05\u{bc}\x5f\x02\
	\u{362}\x61\x03\x02\x02\x02\u{363}\u{364}\x05\u{bc}\x5f\x02\u{364}\x63\x03\
	\x02\x02\x02\u{365}\u{366}\x05\x66\x34\x02\u{366}\x65\x03\x02\x02\x02\u{367}\
	\u{36e}\x05\x68\x35\x02\u{368}\u{369}\x07\x7f\x02\x02\u{369}\u{36a}\x07\
	\x4b\x02\x02\u{36a}\u{36b}\x07\x7f\x02\x02\u{36b}\u{36d}\x05\x68\x35\x02\
	\u{36c}\u{368}\x03\x02\x02\x02\u{36d}\u{370}\x03\x02\x02\x02\u{36e}\u{36c}\
	\x03\x02\x02\x02\u{36e}\u{36f}\x03\x02\x02\x02\u{36f}\x67\x03\x02\x02\x02\
	\u{370}\u{36e}\x03\x02\x02\x02\u{371}\u{378}\x05\x6a\x36\x02\u{372}\u{373}\
	\x07\x7f\x02\x02\u{373}\u{374}\x07\x4c\x02\x02\u{374}\u{375}\x07\x7f\x02\
	\x02\u{375}\u{377}\x05\x6a\x36\x02\u{376}\u{372}\x03\x02\x02\x02\u{377}\
	\u{37a}\x03\x02\x02\x02\u{378}\u{376}\x03\x02\x02\x02\u{378}\u{379}\x03\
	\x02\x02\x02\u{379}\x69\x03\x02\x02\x02\u{37a}\u{378}\x03\x02\x02\x02\u{37b}\
	\u{382}\x05\x6c\x37\x02\u{37c}\u{37d}\x07\x7f\x02\x02\u{37d}\u{37e}\x07\
	\x4d\x02\x02\u{37e}\u{37f}\x07\x7f\x02\x02\u{37f}\u{381}\x05\x6c\x37\x02\
	\u{380}\u{37c}\x03\x02\x02\x02\u{381}\u{384}\x03\x02\x02\x02\u{382}\u{380}\
	\x03\x02\x02\x02\u{382}\u{383}\x03\x02\x02\x02\u{383}\x6b\x03\x02\x02\x02\
	\u{384}\u{382}\x03\x02\x02\x02\u{385}\u{387}\x07\x4e\x02\x02\u{386}\u{388}\
	\x07\x7f\x02\x02\u{387}\u{386}\x03\x02\x02\x02\u{387}\u{388}\x03\x02\x02\
	\x02\u{388}\u{38a}\x03\x02\x02\x02\u{389}\u{385}\x03\x02\x02\x02\u{38a}\
	\u{38d}\x03\x02\x02\x02\u{38b}\u{389}\x03\x02\x02\x02\u{38b}\u{38c}\x03\
	\x02\x02\x02\u{38c}\u{38e}\x03\x02\x02\x02\u{38d}\u{38b}\x03\x02\x02\x02\
	\u{38e}\u{38f}\x05\x6e\x38\x02\u{38f}\x6d\x03\x02\x02\x02\u{390}\u{397}\
	\x05\x70\x39\x02\u{391}\u{393}\x07\x7f\x02\x02\u{392}\u{391}\x03\x02\x02\
	\x02\u{392}\u{393}\x03\x02\x02\x02\u{393}\u{394}\x03\x02\x02\x02\u{394}\
	\u{396}\x05\u{8a}\x46\x02\u{395}\u{392}\x03\x02\x02\x02\u{396}\u{399}\x03\
	\x02\x02\x02\u{397}\u{395}\x03\x02\x02\x02\u{397}\u{398}\x03\x02\x02\x02\
	\u{398}\x6f\x03\x02\x02\x02\u{399}\u{397}\x03\x02\x02\x02\u{39a}\u{3ad}\
	\x05\x72\x3a\x02\u{39b}\u{39d}\x07\x7f\x02\x02\u{39c}\u{39b}\x03\x02\x02\
	\x02\u{39c}\u{39d}\x03\x02\x02\x02\u{39d}\u{39e}\x03\x02\x02\x02\u{39e}\
	\u{3a0}\x07\x0f\x02\x02\u{39f}\u{3a1}\x07\x7f\x02\x02\u{3a0}\u{39f}\x03\
	\x02\x02\x02\u{3a0}\u{3a1}\x03\x02\x02\x02\u{3a1}\u{3a2}\x03\x02\x02\x02\
	\u{3a2}\u{3ac}\x05\x72\x3a\x02\u{3a3}\u{3a5}\x07\x7f\x02\x02\u{3a4}\u{3a3}\
	\x03\x02\x02\x02\u{3a4}\u{3a5}\x03\x02\x02\x02\u{3a5}\u{3a6}\x03\x02\x02\
	\x02\u{3a6}\u{3a8}\x07\x10\x02\x02\u{3a7}\u{3a9}\x07\x7f\x02\x02\u{3a8}\
	\u{3a7}\x03\x02\x02\x02\u{3a8}\u{3a9}\x03\x02\x02\x02\u{3a9}\u{3aa}\x03\
	\x02\x02\x02\u{3aa}\u{3ac}\x05\x72\x3a\x02\u{3ab}\u{39c}\x03\x02\x02\x02\
	\u{3ab}\u{3a4}\x03\x02\x02\x02\u{3ac}\u{3af}\x03\x02\x02\x02\u{3ad}\u{3ab}\
	\x03\x02\x02\x02\u{3ad}\u{3ae}\x03\x02\x02\x02\u{3ae}\x71\x03\x02\x02\x02\
	\u{3af}\u{3ad}\x03\x02\x02\x02\u{3b0}\u{3cb}\x05\x74\x3b\x02\u{3b1}\u{3b3}\
	\x07\x7f\x02\x02\u{3b2}\u{3b1}\x03\x02\x02\x02\u{3b2}\u{3b3}\x03\x02\x02\
	\x02\u{3b3}\u{3b4}\x03\x02\x02\x02\u{3b4}\u{3b6}\x07\x07\x02\x02\u{3b5}\
	\u{3b7}\x07\x7f\x02\x02\u{3b6}\u{3b5}\x03\x02\x02\x02\u{3b6}\u{3b7}\x03\
	\x02\x02\x02\u{3b7}\u{3b8}\x03\x02\x02\x02\u{3b8}\u{3ca}\x05\x74\x3b\x02\
	\u{3b9}\u{3bb}\x07\x7f\x02\x02\u{3ba}\u{3b9}\x03\x02\x02\x02\u{3ba}\u{3bb}\
	\x03\x02\x02\x02\u{3bb}\u{3bc}\x03\x02\x02\x02\u{3bc}\u{3be}\x07\x11\x02\
	\x02\u{3bd}\u{3bf}\x07\x7f\x02\x02\u{3be}\u{3bd}\x03\x02\x02\x02\u{3be}\
	\u{3bf}\x03\x02\x02\x02\u{3bf}\u{3c0}\x03\x02\x02\x02\u{3c0}\u{3ca}\x05\
	\x74\x3b\x02\u{3c1}\u{3c3}\x07\x7f\x02\x02\u{3c2}\u{3c1}\x03\x02\x02\x02\
	\u{3c2}\u{3c3}\x03\x02\x02\x02\u{3c3}\u{3c4}\x03\x02\x02\x02\u{3c4}\u{3c6}\
	\x07\x12\x02\x02\u{3c5}\u{3c7}\x07\x7f\x02\x02\u{3c6}\u{3c5}\x03\x02\x02\
	\x02\u{3c6}\u{3c7}\x03\x02\x02\x02\u{3c7}\u{3c8}\x03\x02\x02\x02\u{3c8}\
	\u{3ca}\x05\x74\x3b\x02\u{3c9}\u{3b2}\x03\x02\x02\x02\u{3c9}\u{3ba}\x03\
	\x02\x02\x02\u{3c9}\u{3c2}\x03\x02\x02\x02\u{3ca}\u{3cd}\x03\x02\x02\x02\
	\u{3cb}\u{3c9}\x03\x02\x02\x02\u{3cb}\u{3cc}\x03\x02\x02\x02\u{3cc}\x73\
	\x03\x02\x02\x02\u{3cd}\u{3cb}\x03\x02\x02\x02\u{3ce}\u{3d9}\x05\x76\x3c\
	\x02\u{3cf}\u{3d1}\x07\x7f\x02\x02\u{3d0}\u{3cf}\x03\x02\x02\x02\u{3d0}\
	\u{3d1}\x03\x02\x02\x02\u{3d1}\u{3d2}\x03\x02\x02\x02\u{3d2}\u{3d4}\x07\
	\x13\x02\x02\u{3d3}\u{3d5}\x07\x7f\x02\x02\u{3d4}\u{3d3}\x03\x02\x02\x02\
	\u{3d4}\u{3d5}\x03\x02\x02\x02\u{3d5}\u{3d6}\x03\x02\x02\x02\u{3d6}\u{3d8}\
	\x05\x76\x3c\x02\u{3d7}\u{3d0}\x03\x02\x02\x02\u{3d8}\u{3db}\x03\x02\x02\
	\x02\u{3d9}\u{3d7}\x03\x02\x02\x02\u{3d9}\u{3da}\x03\x02\x02\x02\u{3da}\
	\x75\x03\x02\x02\x02\u{3db}\u{3d9}\x03\x02\x02\x02\u{3dc}\u{3de}\x09\x03\
	\x02\x02\u{3dd}\u{3df}\x07\x7f\x02\x02\u{3de}\u{3dd}\x03\x02\x02\x02\u{3de}\
	\u{3df}\x03\x02\x02\x02\u{3df}\u{3e1}\x03\x02\x02\x02\u{3e0}\u{3dc}\x03\
	\x02\x02\x02\u{3e1}\u{3e4}\x03\x02\x02\x02\u{3e2}\u{3e0}\x03\x02\x02\x02\
	\u{3e2}\u{3e3}\x03\x02\x02\x02\u{3e3}\u{3e5}\x03\x02\x02\x02\u{3e4}\u{3e2}\
	\x03\x02\x02\x02\u{3e5}\u{3e6}\x05\x78\x3d\x02\u{3e6}\x77\x03\x02\x02\x02\
	\u{3e7}\u{3ed}\x05\u{80}\x41\x02\u{3e8}\u{3ec}\x05\x7c\x3f\x02\u{3e9}\u{3ec}\
	\x05\x7a\x3e\x02\u{3ea}\u{3ec}\x05\x7e\x40\x02\u{3eb}\u{3e8}\x03\x02\x02\
	\x02\u{3eb}\u{3e9}\x03\x02\x02\x02\u{3eb}\u{3ea}\x03\x02\x02\x02\u{3ec}\
	\u{3ef}\x03\x02\x02\x02\u{3ed}\u{3eb}\x03\x02\x02\x02\u{3ed}\u{3ee}\x03\
	\x02\x02\x02\u{3ee}\x79\x03\x02\x02\x02\u{3ef}\u{3ed}\x03\x02\x02\x02\u{3f0}\
	\u{3f1}\x07\x7f\x02\x02\u{3f1}\u{3f3}\x07\x4f\x02\x02\u{3f2}\u{3f4}\x07\
	\x7f\x02\x02\u{3f3}\u{3f2}\x03\x02\x02\x02\u{3f3}\u{3f4}\x03\x02\x02\x02\
	\u{3f4}\u{3f5}\x03\x02\x02\x02\u{3f5}\u{40a}\x05\u{80}\x41\x02\u{3f6}\u{3f8}\
	\x07\x7f\x02\x02\u{3f7}\u{3f6}\x03\x02\x02\x02\u{3f7}\u{3f8}\x03\x02\x02\
	\x02\u{3f8}\u{3f9}\x03\x02\x02\x02\u{3f9}\u{3fa}\x07\x0a\x02\x02\u{3fa}\
	\u{3fb}\x05\x64\x33\x02\u{3fb}\u{3fc}\x07\x0b\x02\x02\u{3fc}\u{40a}\x03\
	\x02\x02\x02\u{3fd}\u{3ff}\x07\x7f\x02\x02\u{3fe}\u{3fd}\x03\x02\x02\x02\
	\u{3fe}\u{3ff}\x03\x02\x02\x02\u{3ff}\u{400}\x03\x02\x02\x02\u{400}\u{402}\
	\x07\x0a\x02\x02\u{401}\u{403}\x05\x64\x33\x02\u{402}\u{401}\x03\x02\x02\
	\x02\u{402}\u{403}\x03\x02\x02\x02\u{403}\u{404}\x03\x02\x02\x02\u{404}\
	\u{406}\x07\x0e\x02\x02\u{405}\u{407}\x05\x64\x33\x02\u{406}\u{405}\x03\
	\x02\x02\x02\u{406}\u{407}\x03\x02\x02\x02\u{407}\u{408}\x03\x02\x02\x02\
	\u{408}\u{40a}\x07\x0b\x02\x02\u{409}\u{3f0}\x03\x02\x02\x02\u{409}\u{3f7}\
	\x03\x02\x02\x02\u{409}\u{3fe}\x03\x02\x02\x02\u{40a}\x7b\x03\x02\x02\x02\
	\u{40b}\u{40c}\x07\x7f\x02\x02\u{40c}\u{40d}\x07\x50\x02\x02\u{40d}\u{40e}\
	\x07\x7f\x02\x02\u{40e}\u{416}\x07\x3f\x02\x02\u{40f}\u{410}\x07\x7f\x02\
	\x02\u{410}\u{411}\x07\x51\x02\x02\u{411}\u{412}\x07\x7f\x02\x02\u{412}\
	\u{416}\x07\x3f\x02\x02\u{413}\u{414}\x07\x7f\x02\x02\u{414}\u{416}\x07\
	\x52\x02\x02\u{415}\u{40b}\x03\x02\x02\x02\u{415}\u{40f}\x03\x02\x02\x02\
	\u{415}\u{413}\x03\x02\x02\x02\u{416}\u{418}\x03\x02\x02\x02\u{417}\u{419}\
	\x07\x7f\x02\x02\u{418}\u{417}\x03\x02\x02\x02\u{418}\u{419}\x03\x02\x02\
	\x02\u{419}\u{41a}\x03\x02\x02\x02\u{41a}\u{41b}\x05\u{80}\x41\x02\u{41b}\
	\x7d\x03\x02\x02\x02\u{41c}\u{41d}\x07\x7f\x02\x02\u{41d}\u{41e}\x07\x53\
	\x02\x02\u{41e}\u{41f}\x07\x7f\x02\x02\u{41f}\u{427}\x07\x54\x02\x02\u{420}\
	\u{421}\x07\x7f\x02\x02\u{421}\u{422}\x07\x53\x02\x02\u{422}\u{423}\x07\
	\x7f\x02\x02\u{423}\u{424}\x07\x4e\x02\x02\u{424}\u{425}\x07\x7f\x02\x02\
	\u{425}\u{427}\x07\x54\x02\x02\u{426}\u{41c}\x03\x02\x02\x02\u{426}\u{420}\
	\x03\x02\x02\x02\u{427}\x7f\x03\x02\x02\x02\u{428}\u{42f}\x05\u{82}\x42\
	\x02\u{429}\u{42b}\x07\x7f\x02\x02\u{42a}\u{429}\x03\x02\x02\x02\u{42a}\
	\u{42b}\x03\x02\x02\x02\u{42b}\u{42c}\x03\x02\x02\x02\u{42c}\u{42e}\x05\
	\u{a6}\x54\x02\u{42d}\u{42a}\x03\x02\x02\x02\u{42e}\u{431}\x03\x02\x02\x02\
	\u{42f}\u{42d}\x03\x02\x02\x02\u{42f}\u{430}\x03\x02\x02\x02\u{430}\u{436}\
	\x03\x02\x02\x02\u{431}\u{42f}\x03\x02\x02\x02\u{432}\u{434}\x07\x7f\x02\
	\x02\u{433}\u{432}\x03\x02\x02\x02\u{433}\u{434}\x03\x02\x02\x02\u{434}\
	\u{435}\x03\x02\x02\x02\u{435}\u{437}\x05\x5a\x2e\x02\u{436}\u{433}\x03\
	\x02\x02\x02\u{436}\u{437}\x03\x02\x02\x02\u{437}\u{81}\x03\x02\x02\x02\
	\u{438}\u{487}\x05\u{84}\x43\x02\u{439}\u{487}\x05\u{b2}\x5a\x02\u{43a}\
	\u{487}\x05\u{a8}\x55\x02\u{43b}\u{43d}\x07\x55\x02\x02\u{43c}\u{43e}\x07\
	\x7f\x02\x02\u{43d}\u{43c}\x03\x02\x02\x02\u{43d}\u{43e}\x03\x02\x02\x02\
	\u{43e}\u{43f}\x03\x02\x02\x02\u{43f}\u{441}\x07\x08\x02\x02\u{440}\u{442}\
	\x07\x7f\x02\x02\u{441}\u{440}\x03\x02\x02\x02\u{441}\u{442}\x03\x02\x02\
	\x02\u{442}\u{443}\x03\x02\x02\x02\u{443}\u{445}\x07\x07\x02\x02\u{444}\
	\u{446}\x07\x7f\x02\x02\u{445}\u{444}\x03\x02\x02\x02\u{445}\u{446}\x03\
	\x02\x02\x02\u{446}\u{447}\x03\x02\x02\x02\u{447}\u{487}\x07\x09\x02\x02\
	\u{448}\u{487}\x05\u{a2}\x52\x02\u{449}\u{487}\x05\u{a4}\x53\x02\u{44a}\
	\u{44c}\x07\x31\x02\x02\u{44b}\u{44d}\x07\x7f\x02\x02\u{44c}\u{44b}\x03\
	\x02\x02\x02\u{44c}\u{44d}\x03\x02\x02\x02\u{44d}\u{44e}\x03\x02\x02\x02\
	\u{44e}\u{450}\x07\x08\x02\x02\u{44f}\u{451}\x07\x7f\x02\x02\u{450}\u{44f}\
	\x03\x02\x02\x02\u{450}\u{451}\x03\x02\x02\x02\u{451}\u{452}\x03\x02\x02\
	\x02\u{452}\u{454}\x05\u{90}\x49\x02\u{453}\u{455}\x07\x7f\x02\x02\u{454}\
	\u{453}\x03\x02\x02\x02\u{454}\u{455}\x03\x02\x02\x02\u{455}\u{456}\x03\
	\x02\x02\x02\u{456}\u{457}\x07\x09\x02\x02\u{457}\u{487}\x03\x02\x02\x02\
	\u{458}\u{45a}\x07\x56\x02\x02\u{459}\u{45b}\x07\x7f\x02\x02\u{45a}\u{459}\
	\x03\x02\x02\x02\u{45a}\u{45b}\x03\x02\x02\x02\u{45b}\u{45c}\x03\x02\x02\
	\x02\u{45c}\u{45e}\x07\x08\x02\x02\u{45d}\u{45f}\x07\x7f\x02\x02\u{45e}\
	\u{45d}\x03\x02\x02\x02\u{45e}\u{45f}\x03\x02\x02\x02\u{45f}\u{460}\x03\
	\x02\x02\x02\u{460}\u{462}\x05\u{90}\x49\x02\u{461}\u{463}\x07\x7f\x02\x02\
	\u{462}\u{461}\x03\x02\x02\x02\u{462}\u{463}\x03\x02\x02\x02\u{463}\u{464}\
	\x03\x02\x02\x02\u{464}\u{465}\x07\x09\x02\x02\u{465}\u{487}\x03\x02\x02\
	\x02\u{466}\u{468}\x07\x57\x02\x02\u{467}\u{469}\x07\x7f\x02\x02\u{468}\
	\u{467}\x03\x02\x02\x02\u{468}\u{469}\x03\x02\x02\x02\u{469}\u{46a}\x03\
	\x02\x02\x02\u{46a}\u{46c}\x07\x08\x02\x02\u{46b}\u{46d}\x07\x7f\x02\x02\
	\u{46c}\u{46b}\x03\x02\x02\x02\u{46c}\u{46d}\x03\x02\x02\x02\u{46d}\u{46e}\
	\x03\x02\x02\x02\u{46e}\u{470}\x05\u{90}\x49\x02\u{46f}\u{471}\x07\x7f\x02\
	\x02\u{470}\u{46f}\x03\x02\x02\x02\u{470}\u{471}\x03\x02\x02\x02\u{471}\
	\u{472}\x03\x02\x02\x02\u{472}\u{473}\x07\x09\x02\x02\u{473}\u{487}\x03\
	\x02\x02\x02\u{474}\u{476}\x07\x58\x02\x02\u{475}\u{477}\x07\x7f\x02\x02\
	\u{476}\u{475}\x03\x02\x02\x02\u{476}\u{477}\x03\x02\x02\x02\u{477}\u{478}\
	\x03\x02\x02\x02\u{478}\u{47a}\x07\x08\x02\x02\u{479}\u{47b}\x07\x7f\x02\
	\x02\u{47a}\u{479}\x03\x02\x02\x02\u{47a}\u{47b}\x03\x02\x02\x02\u{47b}\
	\u{47c}\x03\x02\x02\x02\u{47c}\u{47e}\x05\u{90}\x49\x02\u{47d}\u{47f}\x07\
	\x7f\x02\x02\u{47e}\u{47d}\x03\x02\x02\x02\u{47e}\u{47f}\x03\x02\x02\x02\
	\u{47f}\u{480}\x03\x02\x02\x02\u{480}\u{481}\x07\x09\x02\x02\u{481}\u{487}\
	\x03\x02\x02\x02\u{482}\u{487}\x05\u{8e}\x48\x02\u{483}\u{487}\x05\u{8c}\
	\x47\x02\u{484}\u{487}\x05\u{94}\x4b\x02\u{485}\u{487}\x05\u{ac}\x57\x02\
	\u{486}\u{438}\x03\x02\x02\x02\u{486}\u{439}\x03\x02\x02\x02\u{486}\u{43a}\
	\x03\x02\x02\x02\u{486}\u{43b}\x03\x02\x02\x02\u{486}\u{448}\x03\x02\x02\
	\x02\u{486}\u{449}\x03\x02\x02\x02\u{486}\u{44a}\x03\x02\x02\x02\u{486}\
	\u{458}\x03\x02\x02\x02\u{486}\u{466}\x03\x02\x02\x02\u{486}\u{474}\x03\
	\x02\x02\x02\u{486}\u{482}\x03\x02\x02\x02\u{486}\u{483}\x03\x02\x02\x02\
	\u{486}\u{484}\x03\x02\x02\x02\u{486}\u{485}\x03\x02\x02\x02\u{487}\u{83}\
	\x03\x02\x02\x02\u{488}\u{48f}\x05\u{ae}\x58\x02\u{489}\u{48f}\x07\x61\x02\
	\x02\u{48a}\u{48f}\x05\u{86}\x44\x02\u{48b}\u{48f}\x07\x54\x02\x02\u{48c}\
	\u{48f}\x05\u{b0}\x59\x02\u{48d}\u{48f}\x05\u{88}\x45\x02\u{48e}\u{488}\
	\x03\x02\x02\x02\u{48e}\u{489}\x03\x02\x02\x02\u{48e}\u{48a}\x03\x02\x02\
	\x02\u{48e}\u{48b}\x03\x02\x02\x02\u{48e}\u{48c}\x03\x02\x02\x02\u{48e}\
	\u{48d}\x03\x02\x02\x02\u{48f}\u{85}\x03\x02\x02\x02\u{490}\u{491}\x09\x04\
	\x02\x02\u{491}\u{87}\x03\x02\x02\x02\u{492}\u{494}\x07\x0a\x02\x02\u{493}\
	\u{495}\x07\x7f\x02\x02\u{494}\u{493}\x03\x02\x02\x02\u{494}\u{495}\x03\
	\x02\x02\x02\u{495}\u{4a7}\x03\x02\x02\x02\u{496}\u{498}\x05\x64\x33\x02\
	\u{497}\u{499}\x07\x7f\x02\x02\u{498}\u{497}\x03\x02\x02\x02\u{498}\u{499}\
	\x03\x02\x02\x02\u{499}\u{4a4}\x03\x02\x02\x02\u{49a}\u{49c}\x07\x04\x02\
	\x02\u{49b}\u{49d}\x07\x7f\x02\x02\u{49c}\u{49b}\x03\x02\x02\x02\u{49c}\
	\u{49d}\x03\x02\x02\x02\u{49d}\u{49e}\x03\x02\x02\x02\u{49e}\u{4a0}\x05\
	\x64\x33\x02\u{49f}\u{4a1}\x07\x7f\x02\x02\u{4a0}\u{49f}\x03\x02\x02\x02\
	\u{4a0}\u{4a1}\x03\x02\x02\x02\u{4a1}\u{4a3}\x03\x02\x02\x02\u{4a2}\u{49a}\
	\x03\x02\x02\x02\u{4a3}\u{4a6}\x03\x02\x02\x02\u{4a4}\u{4a2}\x03\x02\x02\
	\x02\u{4a4}\u{4a5}\x03\x02\x02\x02\u{4a5}\u{4a8}\x03\x02\x02\x02\u{4a6}\
	\u{4a4}\x03\x02\x02\x02\u{4a7}\u{496}\x03\x02\x02\x02\u{4a7}\u{4a8}\x03\
	\x02\x02\x02\u{4a8}\u{4a9}\x03\x02\x02\x02\u{4a9}\u{4aa}\x07\x0b\x02\x02\
	\u{4aa}\u{89}\x03\x02\x02\x02\u{4ab}\u{4ad}\x07\x05\x02\x02\u{4ac}\u{4ae}\
	\x07\x7f\x02\x02\u{4ad}\u{4ac}\x03\x02\x02\x02\u{4ad}\u{4ae}\x03\x02\x02\
	\x02\u{4ae}\u{4af}\x03\x02\x02\x02\u{4af}\u{4ca}\x05\x70\x39\x02\u{4b0}\
	\u{4b2}\x07\x14\x02\x02\u{4b1}\u{4b3}\x07\x7f\x02\x02\u{4b2}\u{4b1}\x03\
	\x02\x02\x02\u{4b2}\u{4b3}\x03\x02\x02\x02\u{4b3}\u{4b4}\x03\x02\x02\x02\
	\u{4b4}\u{4ca}\x05\x70\x39\x02\u{4b5}\u{4b7}\x07\x15\x02\x02\u{4b6}\u{4b8}\
	\x07\x7f\x02\x02\u{4b7}\u{4b6}\x03\x02\x02\x02\u{4b7}\u{4b8}\x03\x02\x02\
	\x02\u{4b8}\u{4b9}\x03\x02\x02\x02\u{4b9}\u{4ca}\x05\x70\x39\x02\u{4ba}\
	\u{4bc}\x07\x16\x02\x02\u{4bb}\u{4bd}\x07\x7f\x02\x02\u{4bc}\u{4bb}\x03\
	\x02\x02\x02\u{4bc}\u{4bd}\x03\x02\x02\x02\u{4bd}\u{4be}\x03\x02\x02\x02\
	\u{4be}\u{4ca}\x05\x70\x39\x02\u{4bf}\u{4c1}\x07\x17\x02\x02\u{4c0}\u{4c2}\
	\x07\x7f\x02\x02\u{4c1}\u{4c0}\x03\x02\x02\x02\u{4c1}\u{4c2}\x03\x02\x02\
	\x02\u{4c2}\u{4c3}\x03\x02\x02\x02\u{4c3}\u{4ca}\x05\x70\x39\x02\u{4c4}\
	\u{4c6}\x07\x18\x02\x02\u{4c5}\u{4c7}\x07\x7f\x02\x02\u{4c6}\u{4c5}\x03\
	\x02\x02\x02\u{4c6}\u{4c7}\x03\x02\x02\x02\u{4c7}\u{4c8}\x03\x02\x02\x02\
	\u{4c8}\u{4ca}\x05\x70\x39\x02\u{4c9}\u{4ab}\x03\x02\x02\x02\u{4c9}\u{4b0}\
	\x03\x02\x02\x02\u{4c9}\u{4b5}\x03\x02\x02\x02\u{4c9}\u{4ba}\x03\x02\x02\
	\x02\u{4c9}\u{4bf}\x03\x02\x02\x02\u{4c9}\u{4c4}\x03\x02\x02\x02\u{4ca}\
	\u{8b}\x03\x02\x02\x02\u{4cb}\u{4cd}\x07\x08\x02\x02\u{4cc}\u{4ce}\x07\x7f\
	\x02\x02\u{4cd}\u{4cc}\x03\x02\x02\x02\u{4cd}\u{4ce}\x03\x02\x02\x02\u{4ce}\
	\u{4cf}\x03\x02\x02\x02\u{4cf}\u{4d1}\x05\x64\x33\x02\u{4d0}\u{4d2}\x07\
	\x7f\x02\x02\u{4d1}\u{4d0}\x03\x02\x02\x02\u{4d1}\u{4d2}\x03\x02\x02\x02\
	\u{4d2}\u{4d3}\x03\x02\x02\x02\u{4d3}\u{4d4}\x07\x09\x02\x02\u{4d4}\u{8d}\
	\x03\x02\x02\x02\u{4d5}\u{4da}\x05\x4e\x28\x02\u{4d6}\u{4d8}\x07\x7f\x02\
	\x02\u{4d7}\u{4d6}\x03\x02\x02\x02\u{4d7}\u{4d8}\x03\x02\x02\x02\u{4d8}\
	\u{4d9}\x03\x02\x02\x02\u{4d9}\u{4db}\x05\x50\x29\x02\u{4da}\u{4d7}\x03\
	\x02\x02\x02\u{4db}\u{4dc}\x03\x02\x02\x02\u{4dc}\u{4da}\x03\x02\x02\x02\
	\u{4dc}\u{4dd}\x03\x02\x02\x02\u{4dd}\u{8f}\x03\x02\x02\x02\u{4de}\u{4e3}\
	\x05\u{92}\x4a\x02\u{4df}\u{4e1}\x07\x7f\x02\x02\u{4e0}\u{4df}\x03\x02\x02\
	\x02\u{4e0}\u{4e1}\x03\x02\x02\x02\u{4e1}\u{4e2}\x03\x02\x02\x02\u{4e2}\
	\u{4e4}\x05\x44\x23\x02\u{4e3}\u{4e0}\x03\x02\x02\x02\u{4e3}\u{4e4}\x03\
	\x02\x02\x02\u{4e4}\u{91}\x03\x02\x02\x02\u{4e5}\u{4e6}\x05\u{ac}\x57\x02\
	\u{4e6}\u{4e7}\x07\x7f\x02\x02\u{4e7}\u{4e8}\x07\x4f\x02\x02\u{4e8}\u{4e9}\
	\x07\x7f\x02\x02\u{4e9}\u{4ea}\x05\x64\x33\x02\u{4ea}\u{93}\x03\x02\x02\
	\x02\u{4eb}\u{4ed}\x05\u{96}\x4c\x02\u{4ec}\u{4ee}\x07\x7f\x02\x02\u{4ed}\
	\u{4ec}\x03\x02\x02\x02\u{4ed}\u{4ee}\x03\x02\x02\x02\u{4ee}\u{4ef}\x03\
	\x02\x02\x02\u{4ef}\u{4f1}\x07\x08\x02\x02\u{4f0}\u{4f2}\x07\x7f\x02\x02\
	\u{4f1}\u{4f0}\x03\x02\x02\x02\u{4f1}\u{4f2}\x03\x02\x02\x02\u{4f2}\u{4f7}\
	\x03\x02\x02\x02\u{4f3}\u{4f5}\x07\x41\x02\x02\u{4f4}\u{4f6}\x07\x7f\x02\
	\x02\u{4f5}\u{4f4}\x03\x02\x02\x02\u{4f5}\u{4f6}\x03\x02\x02\x02\u{4f6}\
	\u{4f8}\x03\x02\x02\x02\u{4f7}\u{4f3}\x03\x02\x02\x02\u{4f7}\u{4f8}\x03\
	\x02\x02\x02\u{4f8}\u{50a}\x03\x02\x02\x02\u{4f9}\u{4fb}\x05\x64\x33\x02\
	\u{4fa}\u{4fc}\x07\x7f\x02\x02\u{4fb}\u{4fa}\x03\x02\x02\x02\u{4fb}\u{4fc}\
	\x03\x02\x02\x02\u{4fc}\u{507}\x03\x02\x02\x02\u{4fd}\u{4ff}\x07\x04\x02\
	\x02\u{4fe}\u{500}\x07\x7f\x02\x02\u{4ff}\u{4fe}\x03\x02\x02\x02\u{4ff}\
	\u{500}\x03\x02\x02\x02\u{500}\u{501}\x03\x02\x02\x02\u{501}\u{503}\x05\
	\x64\x33\x02\u{502}\u{504}\x07\x7f\x02\x02\u{503}\u{502}\x03\x02\x02\x02\
	\u{503}\u{504}\x03\x02\x02\x02\u{504}\u{506}\x03\x02\x02\x02\u{505}\u{4fd}\
	\x03\x02\x02\x02\u{506}\u{509}\x03\x02\x02\x02\u{507}\u{505}\x03\x02\x02\
	\x02\u{507}\u{508}\x03\x02\x02\x02\u{508}\u{50b}\x03\x02\x02\x02\u{509}\
	\u{507}\x03\x02\x02\x02\u{50a}\u{4f9}\x03\x02\x02\x02\u{50a}\u{50b}\x03\
	\x02\x02\x02\u{50b}\u{50c}\x03\x02\x02\x02\u{50c}\u{50d}\x07\x09\x02\x02\
	\u{50d}\u{95}\x03\x02\x02\x02\u{50e}\u{50f}\x05\u{a0}\x51\x02\u{50f}\u{510}\
	\x05\u{c0}\x61\x02\u{510}\u{513}\x03\x02\x02\x02\u{511}\u{513}\x07\x5b\x02\
	\x02\u{512}\u{50e}\x03\x02\x02\x02\u{512}\u{511}\x03\x02\x02\x02\u{513}\
	\u{97}\x03\x02\x02\x02\u{514}\u{516}\x05\u{9e}\x50\x02\u{515}\u{517}\x07\
	\x7f\x02\x02\u{516}\u{515}\x03\x02\x02\x02\u{516}\u{517}\x03\x02\x02\x02\
	\u{517}\u{518}\x03\x02\x02\x02\u{518}\u{51a}\x07\x08\x02\x02\u{519}\u{51b}\
	\x07\x7f\x02\x02\u{51a}\u{519}\x03\x02\x02\x02\u{51a}\u{51b}\x03\x02\x02\
	\x02\u{51b}\u{52d}\x03\x02\x02\x02\u{51c}\u{51e}\x05\x64\x33\x02\u{51d}\
	\u{51f}\x07\x7f\x02\x02\u{51e}\u{51d}\x03\x02\x02\x02\u{51e}\u{51f}\x03\
	\x02\x02\x02\u{51f}\u{52a}\x03\x02\x02\x02\u{520}\u{522}\x07\x04\x02\x02\
	\u{521}\u{523}\x07\x7f\x02\x02\u{522}\u{521}\x03\x02\x02\x02\u{522}\u{523}\
	\x03\x02\x02\x02\u{523}\u{524}\x03\x02\x02\x02\u{524}\u{526}\x05\x64\x33\
	\x02\u{525}\u{527}\x07\x7f\x02\x02\u{526}\u{525}\x03\x02\x02\x02\u{526}\
	\u{527}\x03\x02\x02\x02\u{527}\u{529}\x03\x02\x02\x02\u{528}\u{520}\x03\
	\x02\x02\x02\u{529}\u{52c}\x03\x02\x02\x02\u{52a}\u{528}\x03\x02\x02\x02\
	\u{52a}\u{52b}\x03\x02\x02\x02\u{52b}\u{52e}\x03\x02\x02\x02\u{52c}\u{52a}\
	\x03\x02\x02\x02\u{52d}\u{51c}\x03\x02\x02\x02\u{52d}\u{52e}\x03\x02\x02\
	\x02\u{52e}\u{52f}\x03\x02\x02\x02\u{52f}\u{530}\x07\x09\x02\x02\u{530}\
	\u{99}\x03\x02\x02\x02\u{531}\u{532}\x05\u{9e}\x50\x02\u{532}\u{9b}\x03\
	\x02\x02\x02\u{533}\u{534}\x05\u{c0}\x61\x02\u{534}\u{9d}\x03\x02\x02\x02\
	\u{535}\u{536}\x05\u{a0}\x51\x02\u{536}\u{537}\x05\u{c0}\x61\x02\u{537}\
	\u{9f}\x03\x02\x02\x02\u{538}\u{539}\x05\u{c0}\x61\x02\u{539}\u{53a}\x07\
	\x19\x02\x02\u{53a}\u{53c}\x03\x02\x02\x02\u{53b}\u{538}\x03\x02\x02\x02\
	\u{53c}\u{53f}\x03\x02\x02\x02\u{53d}\u{53b}\x03\x02\x02\x02\u{53d}\u{53e}\
	\x03\x02\x02\x02\u{53e}\u{a1}\x03\x02\x02\x02\u{53f}\u{53d}\x03\x02\x02\
	\x02\u{540}\u{542}\x07\x0a\x02\x02\u{541}\u{543}\x07\x7f\x02\x02\u{542}\
	\u{541}\x03\x02\x02\x02\u{542}\u{543}\x03\x02\x02\x02\u{543}\u{544}\x03\
	\x02\x02\x02\u{544}\u{54d}\x05\u{90}\x49\x02\u{545}\u{547}\x07\x7f\x02\x02\
	\u{546}\u{545}\x03\x02\x02\x02\u{546}\u{547}\x03\x02\x02\x02\u{547}\u{548}\
	\x03\x02\x02\x02\u{548}\u{54a}\x07\x0d\x02\x02\u{549}\u{54b}\x07\x7f\x02\
	\x02\u{54a}\u{549}\x03\x02\x02\x02\u{54a}\u{54b}\x03\x02\x02\x02\u{54b}\
	\u{54c}\x03\x02\x02\x02\u{54c}\u{54e}\x05\x64\x33\x02\u{54d}\u{546}\x03\
	\x02\x02\x02\u{54d}\u{54e}\x03\x02\x02\x02\u{54e}\u{550}\x03\x02\x02\x02\
	\u{54f}\u{551}\x07\x7f\x02\x02\u{550}\u{54f}\x03\x02\x02\x02\u{550}\u{551}\
	\x03\x02\x02\x02\u{551}\u{552}\x03\x02\x02\x02\u{552}\u{553}\x07\x0b\x02\
	\x02\u{553}\u{a3}\x03\x02\x02\x02\u{554}\u{556}\x07\x0a\x02\x02\u{555}\u{557}\
	\x07\x7f\x02\x02\u{556}\u{555}\x03\x02\x02\x02\u{556}\u{557}\x03\x02\x02\
	\x02\u{557}\u{560}\x03\x02\x02\x02\u{558}\u{55a}\x05\u{ac}\x57\x02\u{559}\
	\u{55b}\x07\x7f\x02\x02\u{55a}\u{559}\x03\x02\x02\x02\u{55a}\u{55b}\x03\
	\x02\x02\x02\u{55b}\u{55c}\x03\x02\x02\x02\u{55c}\u{55e}\x07\x05\x02\x02\
	\u{55d}\u{55f}\x07\x7f\x02\x02\u{55e}\u{55d}\x03\x02\x02\x02\u{55e}\u{55f}\
	\x03\x02\x02\x02\u{55f}\u{561}\x03\x02\x02\x02\u{560}\u{558}\x03\x02\x02\
	\x02\u{560}\u{561}\x03\x02\x02\x02\u{561}\u{562}\x03\x02\x02\x02\u{562}\
	\u{564}\x05\u{8e}\x48\x02\u{563}\u{565}\x07\x7f\x02\x02\u{564}\u{563}\x03\
	\x02\x02\x02\u{564}\u{565}\x03\x02\x02\x02\u{565}\u{56e}\x03\x02\x02\x02\
	\u{566}\u{568}\x07\x4a\x02\x02\u{567}\u{569}\x07\x7f\x02\x02\u{568}\u{567}\
	\x03\x02\x02\x02\u{568}\u{569}\x03\x02\x02\x02\u{569}\u{56a}\x03\x02\x02\
	\x02\u{56a}\u{56c}\x05\x64\x33\x02\u{56b}\u{56d}\x07\x7f\x02\x02\u{56c}\
	\u{56b}\x03\x02\x02\x02\u{56c}\u{56d}\x03\x02\x02\x02\u{56d}\u{56f}\x03\
	\x02\x02\x02\u{56e}\u{566}\x03\x02\x02\x02\u{56e}\u{56f}\x03\x02\x02\x02\
	\u{56f}\u{570}\x03\x02\x02\x02\u{570}\u{572}\x07\x0d\x02\x02\u{571}\u{573}\
	\x07\x7f\x02\x02\u{572}\u{571}\x03\x02\x02\x02\u{572}\u{573}\x03\x02\x02\
	\x02\u{573}\u{574}\x03\x02\x02\x02\u{574}\u{576}\x05\x64\x33\x02\u{575}\
	\u{577}\x07\x7f\x02\x02\u{576}\u{575}\x03\x02\x02\x02\u{576}\u{577}\x03\
	\x02\x02\x02\u{577}\u{578}\x03\x02\x02\x02\u{578}\u{579}\x07\x0b\x02\x02\
	\u{579}\u{a5}\x03\x02\x02\x02\u{57a}\u{57c}\x07\x19\x02\x02\u{57b}\u{57d}\
	\x07\x7f\x02\x02\u{57c}\u{57b}\x03\x02\x02\x02\u{57c}\u{57d}\x03\x02\x02\
	\x02\u{57d}\u{57e}\x03\x02\x02\x02\u{57e}\u{57f}\x05\u{b6}\x5c\x02\u{57f}\
	\u{a7}\x03\x02\x02\x02\u{580}\u{585}\x07\x5c\x02\x02\u{581}\u{583}\x07\x7f\
	\x02\x02\u{582}\u{581}\x03\x02\x02\x02\u{582}\u{583}\x03\x02\x02\x02\u{583}\
	\u{584}\x03\x02\x02\x02\u{584}\u{586}\x05\u{aa}\x56\x02\u{585}\u{582}\x03\
	\x02\x02\x02\u{586}\u{587}\x03\x02\x02\x02\u{587}\u{585}\x03\x02\x02\x02\
	\u{587}\u{588}\x03\x02\x02\x02\u{588}\u{597}\x03\x02\x02\x02\u{589}\u{58b}\
	\x07\x5c\x02\x02\u{58a}\u{58c}\x07\x7f\x02\x02\u{58b}\u{58a}\x03\x02\x02\
	\x02\u{58b}\u{58c}\x03\x02\x02\x02\u{58c}\u{58d}\x03\x02\x02\x02\u{58d}\
	\u{592}\x05\x64\x33\x02\u{58e}\u{590}\x07\x7f\x02\x02\u{58f}\u{58e}\x03\
	\x02\x02\x02\u{58f}\u{590}\x03\x02\x02\x02\u{590}\u{591}\x03\x02\x02\x02\
	\u{591}\u{593}\x05\u{aa}\x56\x02\u{592}\u{58f}\x03\x02\x02\x02\u{593}\u{594}\
	\x03\x02\x02\x02\u{594}\u{592}\x03\x02\x02\x02\u{594}\u{595}\x03\x02\x02\
	\x02\u{595}\u{597}\x03\x02\x02\x02\u{596}\u{580}\x03\x02\x02\x02\u{596}\
	\u{589}\x03\x02\x02\x02\u{597}\u{5a0}\x03\x02\x02\x02\u{598}\u{59a}\x07\
	\x7f\x02\x02\u{599}\u{598}\x03\x02\x02\x02\u{599}\u{59a}\x03\x02\x02\x02\
	\u{59a}\u{59b}\x03\x02\x02\x02\u{59b}\u{59d}\x07\x5d\x02\x02\u{59c}\u{59e}\
	\x07\x7f\x02\x02\u{59d}\u{59c}\x03\x02\x02\x02\u{59d}\u{59e}\x03\x02\x02\
	\x02\u{59e}\u{59f}\x03\x02\x02\x02\u{59f}\u{5a1}\x05\x64\x33\x02\u{5a0}\
	\u{599}\x03\x02\x02\x02\u{5a0}\u{5a1}\x03\x02\x02\x02\u{5a1}\u{5a3}\x03\
	\x02\x02\x02\u{5a2}\u{5a4}\x07\x7f\x02\x02\u{5a3}\u{5a2}\x03\x02\x02\x02\
	\u{5a3}\u{5a4}\x03\x02\x02\x02\u{5a4}\u{5a5}\x03\x02\x02\x02\u{5a5}\u{5a6}\
	\x07\x5e\x02\x02\u{5a6}\u{a9}\x03\x02\x02\x02\u{5a7}\u{5a9}\x07\x5f\x02\
	\x02\u{5a8}\u{5aa}\x07\x7f\x02\x02\u{5a9}\u{5a8}\x03\x02\x02\x02\u{5a9}\
	\u{5aa}\x03\x02\x02\x02\u{5aa}\u{5ab}\x03\x02\x02\x02\u{5ab}\u{5ad}\x05\
	\x64\x33\x02\u{5ac}\u{5ae}\x07\x7f\x02\x02\u{5ad}\u{5ac}\x03\x02\x02\x02\
	\u{5ad}\u{5ae}\x03\x02\x02\x02\u{5ae}\u{5af}\x03\x02\x02\x02\u{5af}\u{5b1}\
	\x07\x60\x02\x02\u{5b0}\u{5b2}\x07\x7f\x02\x02\u{5b1}\u{5b0}\x03\x02\x02\
	\x02\u{5b1}\u{5b2}\x03\x02\x02\x02\u{5b2}\u{5b3}\x03\x02\x02\x02\u{5b3}\
	\u{5b4}\x05\x64\x33\x02\u{5b4}\u{ab}\x03\x02\x02\x02\u{5b5}\u{5b6}\x05\u{c0}\
	\x61\x02\u{5b6}\u{ad}\x03\x02\x02\x02\u{5b7}\u{5ba}\x05\u{ba}\x5e\x02\u{5b8}\
	\u{5ba}\x05\u{b8}\x5d\x02\u{5b9}\u{5b7}\x03\x02\x02\x02\u{5b9}\u{5b8}\x03\
	\x02\x02\x02\u{5ba}\u{af}\x03\x02\x02\x02\u{5bb}\u{5bd}\x07\x1a\x02\x02\
	\u{5bc}\u{5be}\x07\x7f\x02\x02\u{5bd}\u{5bc}\x03\x02\x02\x02\u{5bd}\u{5be}\
	\x03\x02\x02\x02\u{5be}\u{5e0}\x03\x02\x02\x02\u{5bf}\u{5c1}\x05\u{b6}\x5c\
	\x02\u{5c0}\u{5c2}\x07\x7f\x02\x02\u{5c1}\u{5c0}\x03\x02\x02\x02\u{5c1}\
	\u{5c2}\x03\x02\x02\x02\u{5c2}\u{5c3}\x03\x02\x02\x02\u{5c3}\u{5c5}\x07\
	\x0c\x02\x02\u{5c4}\u{5c6}\x07\x7f\x02\x02\u{5c5}\u{5c4}\x03\x02\x02\x02\
	\u{5c5}\u{5c6}\x03\x02\x02\x02\u{5c6}\u{5c7}\x03\x02\x02\x02\u{5c7}\u{5c9}\
	\x05\x64\x33\x02\u{5c8}\u{5ca}\x07\x7f\x02\x02\u{5c9}\u{5c8}\x03\x02\x02\
	\x02\u{5c9}\u{5ca}\x03\x02\x02\x02\u{5ca}\u{5dd}\x03\x02\x02\x02\u{5cb}\
	\u{5cd}\x07\x04\x02\x02\u{5cc}\u{5ce}\x07\x7f\x02\x02\u{5cd}\u{5cc}\x03\
	\x02\x02\x02\u{5cd}\u{5ce}\x03\x02\x02\x02\u{5ce}\u{5cf}\x03\x02\x02\x02\
	\u{5cf}\u{5d1}\x05\u{b6}\x5c\x02\u{5d0}\u{5d2}\x07\x7f\x02\x02\u{5d1}\u{5d0}\
	\x03\x02\x02\x02\u{5d1}\u{5d2}\x03\x02\x02\x02\u{5d2}\u{5d3}\x03\x02\x02\
	\x02\u{5d3}\u{5d5}\x07\x0c\x02\x02\u{5d4}\u{5d6}\x07\x7f\x02\x02\u{5d5}\
	\u{5d4}\x03\x02\x02\x02\u{5d5}\u{5d6}\x03\x02\x02\x02\u{5d6}\u{5d7}\x03\
	\x02\x02\x02\u{5d7}\u{5d9}\x05\x64\x33\x02\u{5d8}\u{5da}\x07\x7f\x02\x02\
	\u{5d9}\u{5d8}\x03\x02\x02\x02\u{5d9}\u{5da}\x03\x02\x02\x02\u{5da}\u{5dc}\
	\x03\x02\x02\x02\u{5db}\u{5cb}\x03\x02\x02\x02\u{5dc}\u{5df}\x03\x02\x02\
	\x02\u{5dd}\u{5db}\x03\x02\x02\x02\u{5dd}\u{5de}\x03\x02\x02\x02\u{5de}\
	\u{5e1}\x03\x02\x02\x02\u{5df}\u{5dd}\x03\x02\x02\x02\u{5e0}\u{5bf}\x03\
	\x02\x02\x02\u{5e0}\u{5e1}\x03\x02\x02\x02\u{5e1}\u{5e2}\x03\x02\x02\x02\
	\u{5e2}\u{5e3}\x07\x1b\x02\x02\u{5e3}\u{b1}\x03\x02\x02\x02\u{5e4}\u{5e7}\
	\x07\x1c\x02\x02\u{5e5}\u{5e8}\x05\u{c0}\x61\x02\u{5e6}\u{5e8}\x07\x64\x02\
	\x02\u{5e7}\u{5e5}\x03\x02\x02\x02\u{5e7}\u{5e6}\x03\x02\x02\x02\u{5e8}\
	\u{b3}\x03\x02\x02\x02\u{5e9}\u{5ee}\x05\u{82}\x42\x02\u{5ea}\u{5ec}\x07\
	\x7f\x02\x02\u{5eb}\u{5ea}\x03\x02\x02\x02\u{5eb}\u{5ec}\x03\x02\x02\x02\
	\u{5ec}\u{5ed}\x03\x02\x02\x02\u{5ed}\u{5ef}\x05\u{a6}\x54\x02\u{5ee}\u{5eb}\
	\x03\x02\x02\x02\u{5ef}\u{5f0}\x03\x02\x02\x02\u{5f0}\u{5ee}\x03\x02\x02\
	\x02\u{5f0}\u{5f1}\x03\x02\x02\x02\u{5f1}\u{b5}\x03\x02\x02\x02\u{5f2}\u{5f3}\
	\x05\u{bc}\x5f\x02\u{5f3}\u{b7}\x03\x02\x02\x02\u{5f4}\u{5f5}\x09\x05\x02\
	\x02\u{5f5}\u{b9}\x03\x02\x02\x02\u{5f6}\u{5f7}\x09\x06\x02\x02\u{5f7}\u{bb}\
	\x03\x02\x02\x02\u{5f8}\u{5fb}\x05\u{c0}\x61\x02\u{5f9}\u{5fb}\x05\u{be}\
	\x60\x02\u{5fa}\u{5f8}\x03\x02\x02\x02\u{5fa}\u{5f9}\x03\x02\x02\x02\u{5fb}\
	\u{bd}\x03\x02\x02\x02\u{5fc}\u{5fd}\x09\x07\x02\x02\u{5fd}\u{bf}\x03\x02\
	\x02\x02\u{5fe}\u{5ff}\x09\x08\x02\x02\u{5ff}\u{c1}\x03\x02\x02\x02\u{600}\
	\u{601}\x09\x09\x02\x02\u{601}\u{c3}\x03\x02\x02\x02\u{602}\u{603}\x09\x0a\
	\x02\x02\u{603}\u{c5}\x03\x02\x02\x02\u{604}\u{605}\x09\x0b\x02\x02\u{605}\
	\u{c7}\x03\x02\x02\x02\u{11d}\u{c9}\u{cd}\u{d0}\u{d3}\u{db}\u{df}\u{e4}\
	\u{eb}\u{f0}\u{f3}\u{f7}\u{fb}\u{ff}\u{105}\u{109}\u{10e}\u{113}\u{117}\
	\u{11a}\u{11c}\u{120}\u{124}\u{129}\u{12d}\u{132}\u{136}\u{13f}\u{144}\u{148}\
	\u{14c}\u{150}\u{153}\u{157}\u{161}\u{168}\u{175}\u{179}\u{17f}\u{186}\u{18b}\
	\u{18f}\u{195}\u{199}\u{19f}\u{1a3}\u{1a9}\u{1ad}\u{1b1}\u{1b5}\u{1b9}\u{1bd}\
	\u{1c2}\u{1c9}\u{1cd}\u{1d2}\u{1d9}\u{1df}\u{1e4}\u{1ea}\u{1f0}\u{1f5}\u{1f9}\
	\u{1fe}\u{201}\u{204}\u{207}\u{20e}\u{215}\u{218}\u{21e}\u{221}\u{227}\u{22b}\
	\u{22f}\u{233}\u{237}\u{23c}\u{241}\u{245}\u{24a}\u{24d}\u{256}\u{25f}\u{264}\
	\u{271}\u{274}\u{27c}\u{280}\u{285}\u{28a}\u{28e}\u{293}\u{299}\u{29e}\u{2a5}\
	\u{2a9}\u{2ad}\u{2af}\u{2b3}\u{2b5}\u{2b9}\u{2bb}\u{2c1}\u{2c7}\u{2cb}\u{2ce}\
	\u{2d1}\u{2d5}\u{2db}\u{2df}\u{2e2}\u{2e5}\u{2eb}\u{2ee}\u{2f1}\u{2f5}\u{2fb}\
	\u{2fe}\u{301}\u{305}\u{309}\u{30d}\u{30f}\u{313}\u{315}\u{318}\u{31c}\u{31e}\
	\u{324}\u{328}\u{32c}\u{330}\u{333}\u{338}\u{33d}\u{342}\u{347}\u{34d}\u{351}\
	\u{353}\u{357}\u{35b}\u{35d}\u{35f}\u{36e}\u{378}\u{382}\u{387}\u{38b}\u{392}\
	\u{397}\u{39c}\u{3a0}\u{3a4}\u{3a8}\u{3ab}\u{3ad}\u{3b2}\u{3b6}\u{3ba}\u{3be}\
	\u{3c2}\u{3c6}\u{3c9}\u{3cb}\u{3d0}\u{3d4}\u{3d9}\u{3de}\u{3e2}\u{3eb}\u{3ed}\
	\u{3f3}\u{3f7}\u{3fe}\u{402}\u{406}\u{409}\u{415}\u{418}\u{426}\u{42a}\u{42f}\
	\u{433}\u{436}\u{43d}\u{441}\u{445}\u{44c}\u{450}\u{454}\u{45a}\u{45e}\u{462}\
	\u{468}\u{46c}\u{470}\u{476}\u{47a}\u{47e}\u{486}\u{48e}\u{494}\u{498}\u{49c}\
	\u{4a0}\u{4a4}\u{4a7}\u{4ad}\u{4b2}\u{4b7}\u{4bc}\u{4c1}\u{4c6}\u{4c9}\u{4cd}\
	\u{4d1}\u{4d7}\u{4dc}\u{4e0}\u{4e3}\u{4ed}\u{4f1}\u{4f5}\u{4f7}\u{4fb}\u{4ff}\
	\u{503}\u{507}\u{50a}\u{512}\u{516}\u{51a}\u{51e}\u{522}\u{526}\u{52a}\u{52d}\
	\u{53d}\u{542}\u{546}\u{54a}\u{54d}\u{550}\u{556}\u{55a}\u{55e}\u{560}\u{564}\
	\u{568}\u{56c}\u{56e}\u{572}\u{576}\u{57c}\u{582}\u{587}\u{58b}\u{58f}\u{594}\
	\u{596}\u{599}\u{59d}\u{5a0}\u{5a3}\u{5a9}\u{5ad}\u{5b1}\u{5b9}\u{5bd}\u{5c1}\
	\u{5c5}\u{5c9}\u{5cd}\u{5d1}\u{5d5}\u{5d9}\u{5dd}\u{5e0}\u{5e7}\u{5eb}\u{5f0}\
	\u{5fa}";
