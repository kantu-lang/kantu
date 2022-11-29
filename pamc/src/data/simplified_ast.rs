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
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
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
    Name(NameExpression),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
    Check(Box<Check>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression {
    pub components: Vec<Identifier>,
}

pub use crate::data::unsimplified_ast::Identifier;

pub use crate::data::unsimplified_ast::IdentifierName;

pub use crate::data::unsimplified_ast::ReservedIdentifierName;

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
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub variant_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check {
    pub checkee_annotation: CheckeeAnnotation,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CheckeeAnnotation {
    Goal(GoalCheckeeAnnotation),
    Expression(ExpressionCheckeeAnnotation),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GoalCheckeeAnnotation {
    pub goal_kw_position: TextPosition,
    pub checkee_type: QuestionMarkOrExpression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionCheckeeAnnotation {
    pub checkee: Expression,
    pub checkee_type: QuestionMarkOrExpression,
    pub checkee_value: Option<QuestionMarkOrExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuestionMarkOrExpression {
    QuestionMark { start: TextPosition },
    Expression(Expression),
}
