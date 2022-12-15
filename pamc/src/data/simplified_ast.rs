use crate::data::{non_empty_vec::NonEmptyVec, FileId, TextSpan};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub span: TextSpan,
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

impl FileItem {
    pub fn span(&self) -> TextSpan {
        match self {
            FileItem::Type(type_) => type_.span,
            FileItem::Let(let_) => let_.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub span: TextSpan,
    pub name: Identifier,
    pub params: Option<NonEmptyParamVec>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyParamVec {
    Unlabeled(NonEmptyVec<UnlabeledParam>),
    Labeled(NonEmptyVec<LabeledParam>),
}

impl NonEmptyParamVec {
    pub fn len(&self) -> usize {
        match self {
            NonEmptyParamVec::Unlabeled(vec) => vec.len(),
            NonEmptyParamVec::Labeled(vec) => vec.len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnlabeledParam {
    pub span: TextSpan,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabeledParam {
    pub span: TextSpan,
    pub label: ParamLabel,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

pub use crate::data::unsimplified_ast::ParamLabel;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: TextSpan,
    pub name: Identifier,
    pub params: Option<NonEmptyParamVec>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub span: TextSpan,
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

impl Expression {
    pub fn span(&self) -> TextSpan {
        match self {
            Expression::Name(name) => name.span,
            Expression::Call(call) => call.span,
            Expression::Fun(fun) => fun.span,
            Expression::Match(match_) => match_.span,
            Expression::Forall(forall) => forall.span,
            Expression::Check(check) => check.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub span: TextSpan,
    pub components: NonEmptyVec<Identifier>,
}

pub use crate::data::unsimplified_ast::Identifier;

pub use crate::data::unsimplified_ast::IdentifierName;

pub use crate::data::unsimplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub span: TextSpan,
    pub callee: Expression,
    pub args: NonEmptyVec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub span: TextSpan,
    pub name: Identifier,
    pub params: NonEmptyParamVec,
    pub return_type: Expression,
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub span: TextSpan,
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub span: TextSpan,
    pub variant_name: Identifier,
    pub params: Option<NonEmptyVec<Identifier>>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub span: TextSpan,
    pub params: NonEmptyParamVec,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub span: TextSpan,
    pub assertions: NonEmptyVec<CheckAssertion>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CheckAssertion {
    pub span: TextSpan,
    pub kind: CheckAssertionKind,
    pub left: GoalKwOrExpression,
    pub right: QuestionMarkOrExpression,
}

pub use crate::data::unsimplified_ast::CheckAssertionKind;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GoalKwOrExpression {
    GoalKw { span: TextSpan },
    Expression(Expression),
}

impl GoalKwOrExpression {
    pub fn span(&self) -> TextSpan {
        match self {
            GoalKwOrExpression::GoalKw { span } => *span,
            GoalKwOrExpression::Expression(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionMarkOrExpression {
    QuestionMark { span: TextSpan },
    Expression(Expression),
}

impl QuestionMarkOrExpression {
    pub fn span(&self) -> TextSpan {
        match self {
            QuestionMarkOrExpression::QuestionMark { span } => *span,
            QuestionMarkOrExpression::Expression(expression) => expression.span(),
        }
    }
}
