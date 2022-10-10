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
    pub type_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub return_type_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub value_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedExpression {
    pub id: NodeId<Self>,
    // TODO: This should be `expression_id`.
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Dot(Box<Dot>),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub id: NodeId<Self>,
    /// This is `None` if the identifier is either
    /// 1. a built-in identifier (e.g., `Type`)
    /// 2. an identifier that appears in compiler-generated expressions
    pub start: Option<TextPosition>,
    pub name: IdentifierName,
}

pub use crate::data::unregistered_ast::IdentifierName;

pub use crate::data::unregistered_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dot {
    pub id: NodeId<Self>,
    pub left_id: NodeId<WrappedExpression>,
    pub right_id: NodeId<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub id: NodeId<Self>,
    pub callee_id: NodeId<WrappedExpression>,
    pub arg_list_id: ListId<NodeId<WrappedExpression>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub return_type_id: NodeId<WrappedExpression>,
    pub body_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub id: NodeId<Self>,
    pub matchee_id: NodeId<WrappedExpression>,
    pub case_list_id: ListId<NodeId<MatchCase>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub variant_name_id: NodeId<Identifier>,
    pub param_list_id: ListId<NodeId<Identifier>>,
    pub output_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub id: NodeId<Self>,
    pub param_list_id: ListId<NodeId<Param>>,
    pub output_id: NodeId<WrappedExpression>,
}
