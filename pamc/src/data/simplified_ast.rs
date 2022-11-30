use crate::data::{FileId, TextPosition};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Name(NameExpression),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
    Check(Box<Check>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub components: Vec<Identifier>,
}

pub use crate::data::unsimplified_ast::Identifier;

pub use crate::data::unsimplified_ast::IdentifierName;

pub use crate::data::unsimplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub variant_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub checkee_annotation: CheckeeAnnotation,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CheckeeAnnotation {
    Goal(GoalCheckeeAnnotation),
    Expression(ExpressionCheckeeAnnotation),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GoalCheckeeAnnotation {
    pub goal_kw_position: TextPosition,
    pub checkee_type: QuestionMarkOrExpression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionCheckeeAnnotation {
    pub checkee: Expression,
    pub checkee_type: QuestionMarkOrExpression,
    pub checkee_value: Option<QuestionMarkOrExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionMarkOrExpression {
    QuestionMark { start: TextPosition },
    Expression(Expression),
}
