use crate::data::{
    bind_error::BindError,
    fun_recursion_validation_result::IllegalFunRecursionError,
    node_registry::{NodeId, NonEmptyListId},
    simplified_ast as unbound, TextSpan,
};

pub use crate::data::node_registry::FileItemNodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub visibility: Visibility,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: Option<NonEmptyParamListId>,
    pub variant_list_id: Option<NonEmptyListId<NodeId<Variant>>>,
}

pub use crate::data::node_registry::NonEmptyParamListId;

pub use crate::data::bound_ast::Visibility;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnlabeledParam {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub is_dashed: bool,
    pub name_id: NodeId<Identifier>,
    pub type_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledParam {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub label_id: ParamLabelId,
    pub is_dashed: bool,
    pub name_id: NodeId<Identifier>,
    pub type_id: ExpressionId,
}

pub use crate::data::node_registry::ParamLabelId;

impl LabeledParam {
    pub fn label_identifier_id(&self) -> NodeId<Identifier> {
        match self.label_id {
            ParamLabelId::Implicit => self.name_id,
            ParamLabelId::Explicit(label_id) => label_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: Option<NonEmptyParamListId>,
    pub return_type_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub visibility: Visibility,
    pub transparency: Transparency,
    pub name_id: NodeId<Identifier>,
    pub value_id: ExpressionId,
}

pub use crate::data::bound_ast::Transparency;

pub use crate::data::node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub component_list_id: NonEmptyListId<NodeId<Identifier>>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name: IdentifierName,
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::UnreservedIdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoExpression {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub callee_id: ExpressionId,
    pub arg_list_id: NonEmptyCallArgListId,
}

pub use crate::data::node_registry::NonEmptyCallArgListId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: NonEmptyParamListId,
    pub return_type_id: ExpressionId,
    pub body_id: ExpressionId,
    /// This is used by the type checker to
    /// determine whether it can skip type-checking
    /// the function's body.
    /// This is necessary to prevent infinite recursion
    /// when checking recursive functions.
    pub skip_type_checking_body: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub matchee_id: ExpressionId,
    pub case_list_id: Option<NonEmptyListId<NodeId<MatchCase>>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub variant_name_id: NodeId<Identifier>,
    pub param_list_id: Option<NonEmptyMatchCaseParamListId>,
    pub output_id: MatchCaseOutputId,
}

pub use crate::data::node_registry::NonEmptyMatchCaseParamListId;

pub use crate::data::node_registry::MatchCaseOutputId;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabeledMatchCaseParam {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub label_id: ParamLabelId,
    pub name_id: NodeId<Identifier>,
}

impl LabeledMatchCaseParam {
    pub fn label_identifier_id(&self) -> NodeId<Identifier> {
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
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug)]
pub struct Check {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub assertion_list_id: NonEmptyListId<NodeId<CheckAssertion>>,
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug)]
pub struct CheckAssertion {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub kind: CheckAssertionKind,
    pub left_id: GoalKwOrPossiblyInvalidExpressionId,
    pub right_id: QuestionMarkOrPossiblyInvalidExpressionId,
}

pub use crate::data::bound_ast::CheckAssertionKind;

pub use crate::data::node_registry::GoalKwOrPossiblyInvalidExpressionId;

pub use crate::data::node_registry::{
    InvalidExpressionId, PossiblyInvalidExpressionId, QuestionMarkOrPossiblyInvalidExpressionId,
};

#[derive(Clone, Debug)]
pub struct SymbolicallyInvalidExpression {
    pub id: NodeId<Self>,
    pub expression: unbound::Expression,
    pub error: BindError,
    pub span_invalidated: bool,
}

#[derive(Clone, Debug)]
pub struct IllegalFunRecursionExpression {
    pub id: NodeId<Self>,
    pub expression_id: ExpressionId,
    pub error: IllegalFunRecursionError,
    pub span_invalidated: bool,
}
