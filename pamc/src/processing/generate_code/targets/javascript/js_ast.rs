use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Const(ConstStatement),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstStatement {
    pub name: String,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Call(Box<Call>),
    Function(Box<Function>),
    BinaryOp(Box<BinaryOp>),
    Dot(Box<Dot>),
    Ternary(Box<Ternary>),
    Array(Box<Array>),
    Object(Box<Object>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Literal {
    Boolean(bool),
    Number(i32),
    String { unescaped: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub return_value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BinaryOp {
    pub op: BinaryOpKind,
    pub left: Expression,
    pub right: Expression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinaryOpKind {
    TripleEqual,
    Index,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dot {
    pub left: Expression,
    pub right: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ternary {
    pub condition: Expression,
    pub true_body: Expression,
    pub false_body: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Array {
    pub items: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Object {
    pub entries: Vec<ObjectEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObjectEntry {
    pub key: String,
    pub value: Expression,
}
