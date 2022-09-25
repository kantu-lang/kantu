use crate::data::{node_registry::NodeId, FileId, TextPosition};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub file_id: FileId,
    pub id: NodeId<Self>,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub id: NodeId<Self>,
    pub name: Identifier,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub id: NodeId<Self>,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: WrappedExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub id: NodeId<Self>,
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: WrappedExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub id: NodeId<Self>,
    pub name: Identifier,
    pub value: WrappedExpression,
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
    pub left: WrappedExpression,
    pub right: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub id: NodeId<Self>,
    pub callee: WrappedExpression,
    pub args: Vec<WrappedExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub id: NodeId<Self>,
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: WrappedExpression,
    pub body: WrappedExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub id: NodeId<Self>,
    pub matchee: WrappedExpression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub id: NodeId<Self>,
    pub variant_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: WrappedExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub id: NodeId<Self>,
    pub params: Vec<Param>,
    pub output: WrappedExpression,
}
