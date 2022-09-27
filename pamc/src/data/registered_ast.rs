use crate::data::{node_registry::NodeId, FileId, TextPosition};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub file_id: FileId,
    pub id: NodeId<Self>,
    pub item_ids: Vec<NodeId<WrappedFileItem>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedFileItem {
    pub id: NodeId<Self>,
    pub file_item: FileItem,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub param_ids: Vec<NodeId<Param>>,
    pub variant_ids: Vec<NodeId<Variant>>,
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
    pub param_ids: Vec<NodeId<Param>>,
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
    pub arg_ids: Vec<NodeId<WrappedExpression>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
    pub name_id: NodeId<Identifier>,
    pub param_ids: Vec<NodeId<Param>>,
    pub return_type_id: NodeId<WrappedExpression>,
    pub body_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub id: NodeId<Self>,
    pub matchee_id: NodeId<WrappedExpression>,
    pub case_ids: Vec<NodeId<MatchCase>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub variant_name_id: NodeId<Identifier>,
    pub param_ids: Vec<NodeId<Identifier>>,
    pub output_id: NodeId<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub id: NodeId<Self>,
    pub param_ids: Vec<NodeId<Param>>,
    pub output_id: NodeId<WrappedExpression>,
}
