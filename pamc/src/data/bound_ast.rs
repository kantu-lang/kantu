use crate::data::{
    bind_error::BindError, fun_recursion_validation_result::IllegalFunRecursionError,
    simplified_ast as unbound, FileId, TextSpan,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub span: Option<TextSpan>,
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

impl FileItem {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            FileItem::Type(type_) => type_.span,
            FileItem::Let(let_) => let_.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub span: Option<TextSpan>,
    pub name: Identifier,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: Option<TextSpan>,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: Option<TextSpan>,
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub span: Option<TextSpan>,
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
    pub fn span(&self) -> Option<TextSpan> {
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
    pub span: Option<TextSpan>,
    pub components: Vec<Identifier>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}

/// De Bruijn index (zero-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DbIndex(pub usize);

/// De Bruijn level (zero-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DbLevel(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub span: Option<TextSpan>,
    pub name: IdentifierName,
}

impl From<crate::data::simplified_ast::Identifier> for Identifier {
    fn from(id: crate::data::simplified_ast::Identifier) -> Self {
        Self {
            span: Some(id.span),
            name: id.name,
        }
    }
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub span: Option<TextSpan>,
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub span: Option<TextSpan>,
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
    pub body: Expression,
    /// This is used by the type checker to
    /// determine whether it can skip type-checking
    /// the function's body.
    /// This is necessary to prevent infinite recursion
    /// when checking recursive functions.
    pub skip_type_checking_body: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub span: Option<TextSpan>,
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub span: Option<TextSpan>,
    pub variant_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub span: Option<TextSpan>,
    pub params: Vec<Param>,
    pub output: Expression,
}

// TODO: Cut

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub span: Option<TextSpan>,
    pub assertions: Vec<CheckAssertion>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CheckAssertion {
    Type(TypeAssertion),
    NormalForm(NormalFormAssertion),
}

impl CheckAssertion {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            CheckAssertion::Type(type_) => type_.span,
            CheckAssertion::NormalForm(normal_form) => normal_form.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeAssertion {
    pub span: Option<TextSpan>,
    pub left: Expression,
    pub right: QuestionMarkOrPossiblyInvalidExpression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NormalFormAssertion {
    pub span: Option<TextSpan>,
    pub left: GoalKwOrExpression,
    pub right: QuestionMarkOrPossiblyInvalidExpression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GoalKwOrExpression {
    GoalKw { span: Option<TextSpan> },
    Expression(Expression),
}

impl GoalKwOrExpression {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            GoalKwOrExpression::GoalKw { span } => *span,
            GoalKwOrExpression::Expression(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionMarkOrPossiblyInvalidExpression {
    QuestionMark { span: Option<TextSpan> },
    Expression(PossiblyInvalidExpression),
}

impl QuestionMarkOrPossiblyInvalidExpression {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span } => *span,
            QuestionMarkOrPossiblyInvalidExpression::Expression(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PossiblyInvalidExpression {
    Valid(Expression),
    Invalid(InvalidExpression),
}

impl PossiblyInvalidExpression {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            PossiblyInvalidExpression::Valid(expression) => expression.span(),
            PossiblyInvalidExpression::Invalid(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InvalidExpression {
    SymbolicallyInvalid(SymbolicallyInvalidExpression),
    IllegalFunRecursion(IllegalFunRecursionExpression),
}

impl InvalidExpression {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            InvalidExpression::SymbolicallyInvalid(expression) => expression.span,
            InvalidExpression::IllegalFunRecursion(expression) => expression.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolicallyInvalidExpression {
    pub span: Option<TextSpan>,
    pub expression: unbound::Expression,
    pub error: BindError,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IllegalFunRecursionExpression {
    pub span: Option<TextSpan>,
    pub expression: Expression,
    pub error: IllegalFunRecursionError,
}
