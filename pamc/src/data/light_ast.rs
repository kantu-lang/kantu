use crate::data::{
    bind_error::BindError,
    fun_recursion_validation_result::IllegalFunRecursionError,
    node_registry::{ListId, NodeId},
    simplified_ast as unbound, FileId, TextSpan,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub span: Option<TextSpan>,
    pub file_id: FileId,
    pub id: NodeId<Self>,
    pub item_list_id: ListId<FileItemNodeId>,
}

pub use crate::data::node_registry::FileItemNodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub variant_list_id: ListId<NodeId<Variant>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub is_dashed: bool,
    pub name_id: NodeId<Identifier>,
    pub type_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub return_type_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: NodeId<Identifier>,
    pub value_id: ExpressionId,
}

pub type ExpressionId = crate::data::node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub component_list_id: ListId<NodeId<Identifier>>,
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

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub callee_id: ExpressionId,
    pub arg_list_id: ListId<ExpressionId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
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
    pub case_list_id: ListId<NodeId<MatchCase>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub variant_name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Identifier>>,
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug)]
pub struct Check {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub assertion_list_id: ListId<CheckAssertionId>,
    pub output_id: ExpressionId,
}

pub use crate::data::node_registry::CheckAssertionId;

#[derive(Clone, Debug)]
pub struct TypeAssertion {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub left_id: ExpressionId,
    pub right_id: QuestionMarkOrPossiblyInvalidExpressionId,
}

#[derive(Clone, Debug)]
pub struct NormalFormAssertion {
    pub id: NodeId<Self>,
    pub span: Option<TextSpan>,
    pub left_id: GoalKwOrExpressionId,
    pub right_id: QuestionMarkOrPossiblyInvalidExpressionId,
}

pub use crate::data::node_registry::GoalKwOrExpressionId;

pub use crate::data::node_registry::{
    InvalidExpressionId, PossiblyInvalidExpressionId, QuestionMarkOrPossiblyInvalidExpressionId,
};

// TODO: Fix span of these (there's no way to remove spans)

#[derive(Clone, Debug)]
pub struct SymbolicallyInvalidExpression {
    pub id: NodeId<Self>,
    pub expression: unbound::Expression,
    pub error: BindError,
}

#[derive(Clone, Debug)]
pub struct IllegalFunRecursionExpression {
    pub id: NodeId<Self>,
    pub expression_id: ExpressionId,
    pub error: IllegalFunRecursionError,
}
