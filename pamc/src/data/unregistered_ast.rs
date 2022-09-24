use crate::data::{FileId, TextPosition};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub constructors: Vec<Constructor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constructor {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
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
    pub start: TextPosition,
    pub name: IdentifierName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IdentifierName {
    Standard(String),
    Reserved(ReservedIdentifierName),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReservedIdentifierName {
    TypeTitleCase,
    Underscore,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dot {
    pub left: Expression,
    pub right: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
    pub return_value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub constructor_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}
