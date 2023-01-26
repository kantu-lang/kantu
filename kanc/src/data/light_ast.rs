use crate::data::{
    bind_error::BindError,
    fun_recursion_validation_result::IllegalFunRecursionError,
    non_empty_vec::{NonEmptySlice, NonEmptyVec},
    simplified_ast as unbound,
    text_span::*,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement<'a> {
    pub span: Option<TextSpan>,
    pub visibility: Visibility,
    pub name_id: &'a Identifier,
    pub param_list_id: Option<&'a NonEmptyParamVec<'a>>,
    pub variant_list_id: &'a [Variant<'a>],
}

pub use crate::data::bound_ast::ModScope;

pub use crate::data::bound_ast::Visibility;

pub use crate::data::bound_ast::Transparency;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NonEmptyParamVec<'a> {
    Unlabeled(NonEmptyVec<UnlabeledParam<'a>>),
    UniquelyLabeled(NonEmptyVec<LabeledParam<'a>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnlabeledParam<'a> {
    pub span: Option<TextSpan>,
    pub is_dashed: bool,
    pub name_id: &'a Identifier,
    pub type_id: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledParam<'a> {
    pub span: Option<TextSpan>,
    pub label_clause: &'a ParamLabelClause<'a>,
    pub is_dashed: bool,
    pub name_id: &'a Identifier,
    pub type_id: ExpressionRef<'a>,
}

impl LabeledParam<'_> {
    pub fn label(&self) -> &Identifier {
        match self.label_clause {
            ParamLabelClause::Implicit => self.name_id,
            ParamLabelClause::Explicit(label) => label,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParamLabelClause<'a> {
    Implicit,
    Explicit(&'a Identifier),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<'a> {
    pub span: Option<TextSpan>,
    pub name_id: &'a Identifier,
    pub param_list_id: Option<&'a NonEmptyParamVec<'a>>,
    pub return_type_id: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement<'a> {
    pub span: Option<TextSpan>,
    pub visibility: Visibility,
    pub transparency: Transparency,
    pub name_id: &'a Identifier,
    pub value_id: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionRef<'a> {
    Name(&'a NameExpression<'a>),
    Todo(&'a TodoExpression),
    Call(&'a Call<'a>),
    Fun(&'a Fun),
    Match(&'a Match),
    Forall(&'a Forall),
    Check(&'a Check),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression<'a> {
    pub span: Option<TextSpan>,
    pub component_list_id: NonEmptySlice<'a, Identifier>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub span: Option<TextSpan>,
    pub name: IdentifierName,
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::UnreservedIdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoExpression {
    pub span: Option<TextSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call<'a> {
    pub span: Option<TextSpan>,
    pub callee_id: ExpressionRef<'a>,
    pub arg_list_id: &'a NonEmptyCallArgVec<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyCallArgVec<'a> {
    Unlabeled(NonEmptyVec<ExpressionRef<'a>>),
    UniquelyLabeled(NonEmptyVec<LabeledCallArg>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: &'a Identifier,
    pub param_list_id: NonEmptyParamListId,
    pub return_type_id: ExpressionRef<'a>,
    pub body_id: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub matchee_id: ExpressionRef<'a>,
    pub case_list_id: Option<NonEmptyListId<NodeId<MatchCase>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub variant_name_id: &'a Identifier,
    pub param_list_id: Option<NonEmptyMatchCaseParamListId>,
    pub output_id: MatchCaseOutputId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabeledMatchCaseParam {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub label_id: ParamLabelId,
    pub name_id: &'a Identifier,
}

impl LabeledMatchCaseParam {
    pub fn label_identifier_id(&self) -> &'a Identifier {
        match self.label_id {
            ParamLabelId::Implicit => self.name_id,
            ParamLabelId::Explicit(label_id) => label_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub param_list_id: NonEmptyParamListId,
    pub output_id: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub assertion_list_id: NonEmptyListId<NodeId<CheckAssertion>>,
    pub output_id: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckAssertion {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub kind: CheckAssertionKind,
    pub left_id: GoalKwOrPossiblyInvalidExpressionId,
    pub right_id: QuestionMarkOrPossiblyInvalidExpressionId,
}

pub use crate::data::bound_ast::CheckAssertionKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SymbolicallyInvalidExpression {
    pub id: NodeId<Self>,
    pub expression: unbound::Expression,
    pub error: BindError,
    pub span_invalidated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IllegalFunRecursionExpression {
    pub id: NodeId<Self>,
    pub expression_id: ExpressionRef<'a>,
    pub error: IllegalFunRecursionError,
    pub span_invalidated: bool,
}
