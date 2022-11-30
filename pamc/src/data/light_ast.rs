use crate::data::{
    node_registry::{ListId, NodeId},
    FileId, TextPosition,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub file_id: FileId,
    pub id: NodeId<Self>,
    pub item_list_id: ListId<FileItemNodeId>,
}

pub type FileItemNodeId = crate::data::node_registry::FileItemNodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub variant_list_id: ListId<NodeId<Variant>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub id: NodeId<Self>,
    pub is_dashed: bool,
    pub name_id: NodeId<Identifier>,
    pub type_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub return_type_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub value_id: ExpressionId,
}

pub type ExpressionId = crate::data::node_registry::ExpressionId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression {
    pub id: NodeId<Self>,
    pub component_list_id: ListId<NodeId<Identifier>>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub id: NodeId<Self>,
    pub start: Option<TextPosition>,
    pub name: IdentifierName,
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub id: NodeId<Self>,
    pub callee_id: ExpressionId,
    pub arg_list_id: ListId<ExpressionId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
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
    pub matchee_id: ExpressionId,
    pub case_list_id: ListId<NodeId<MatchCase>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub variant_name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Identifier>>,
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub id: NodeId<Self>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug)]
pub struct Check {
    pub checkee_annotation_id: CheckeeAnnotationId,
    pub output_id: ExpressionId,
}

#[derive(Clone, Debug)]
pub enum CheckeeAnnotationId {
    Goal(NodeId<GoalCheckeeAnnotation>),
    Expression(NodeId<ExpressionCheckeeAnnotation>),
}

#[derive(Clone, Debug)]
pub struct GoalCheckeeAnnotation {
    pub goal_kw_position: TextPosition,
    pub checkee_type: QuestionMarkOrPossiblyInvalidExpressionId,
}

#[derive(Clone, Debug)]
pub struct ExpressionCheckeeAnnotation {
    pub checkee: ExpressionId,
    pub checkee_type: QuestionMarkOrPossiblyInvalidExpressionId,
    pub checkee_value: Option<QuestionMarkOrPossiblyInvalidExpressionId>,
}

#[derive(Clone, Debug)]
pub enum QuestionMarkOrPossiblyInvalidExpressionId {
    QuestionMark { start: TextPosition },
    Expression(PossiblyInvalidExpressionId),
}

#[derive(Clone, Debug)]
pub enum PossiblyInvalidExpressionId {
    Valid(ExpressionId),
    Invalid(NodeId<InvalidExpression>),
}

pub use crate::data::bound_ast::InvalidExpression;
