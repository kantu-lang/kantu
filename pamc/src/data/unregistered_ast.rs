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
    pub name: StandardIdentifier,
    pub params: Vec<Param>,
    pub constructors: Vec<Constructor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StandardIdentifier {
    pub start: TextPosition,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: StandardIdentifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constructor {
    pub name: StandardIdentifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: StandardIdentifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    ReservedIdentifier(ReservedIdentifier),
    StandardIdentifier(StandardIdentifier),
    Dot(Box<Dot>),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReservedIdentifier {
    pub start: TextPosition,
    pub name: ReservedIdentifierName,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReservedIdentifierName {
    TypeTitleCase,
    Underscore,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dot {
    pub left: Expression,
    pub right: StandardIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub name: StandardIdentifier,
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
    pub constructor_name: StandardIdentifier,
    pub params: Vec<StandardIdentifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}
