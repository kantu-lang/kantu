use crate::data::{
    bind_error::BindError,
    fun_recursion_validation_result::IllegalFunRecursionError,
    non_empty_vec::{NonEmptyVec, OptionalNonEmptyVecLen},
    simplified_ast as unbound, TextSpan,
};

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
    pub params: Option<NonEmptyParamVec>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyParamVec {
    Unlabeled(NonEmptyVec<UnlabeledParam>),
    UniquelyLabeled(NonEmptyVec<LabeledParam>),
}

impl NonEmptyParamVec {
    pub fn len(&self) -> usize {
        match self {
            NonEmptyParamVec::Unlabeled(vec) => vec.len(),
            NonEmptyParamVec::UniquelyLabeled(vec) => vec.len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnlabeledParam {
    pub span: Option<TextSpan>,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabeledParam {
    pub span: Option<TextSpan>,
    pub label: ParamLabel,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParamLabel {
    Implicit,
    Explicit(Identifier),
}

impl From<unbound::ParamLabel> for ParamLabel {
    fn from(label: unbound::ParamLabel) -> Self {
        match label {
            unbound::ParamLabel::Implicit => ParamLabel::Implicit,
            unbound::ParamLabel::Explicit(name) => ParamLabel::Explicit(name.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: Option<TextSpan>,
    pub name: Identifier,
    pub params: Option<NonEmptyParamVec>,
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
    Todo(Option<TextSpan>),
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
            Expression::Todo(span) => *span,
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
    pub components: NonEmptyVec<Identifier>,
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

pub use crate::data::simplified_ast::UnreservedIdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub span: Option<TextSpan>,
    pub callee: Expression,
    pub args: NonEmptyCallArgVec,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyCallArgVec {
    Unlabeled(NonEmptyVec<Expression>),
    UniquelyLabeled(NonEmptyVec<LabeledCallArg>),
}

impl NonEmptyCallArgVec {
    pub fn len(&self) -> usize {
        match self {
            NonEmptyCallArgVec::Unlabeled(vec) => vec.len(),
            NonEmptyCallArgVec::UniquelyLabeled(vec) => vec.len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LabeledCallArg {
    Implicit {
        label: Identifier,
        db_index: DbIndex,
    },
    Explicit {
        label: Identifier,
        value: Expression,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub span: Option<TextSpan>,
    pub name: Identifier,
    pub params: NonEmptyParamVec,
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
    pub params: Option<NonEmptyMatchCaseParamVec>,
    pub output: MatchCaseOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyMatchCaseParamVec {
    Unlabeled(NonEmptyVec<Identifier>),
    UniquelyLabeled {
        params: Option<NonEmptyVec<LabeledMatchCaseParam>>,
        triple_dot: Option<TextSpan>,
    },
}

impl OptionalNonEmptyVecLen for Option<NonEmptyMatchCaseParamVec> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.len()).unwrap_or(0)
    }
}

impl NonEmptyMatchCaseParamVec {
    pub fn len(&self) -> usize {
        match self {
            NonEmptyMatchCaseParamVec::Unlabeled(vec) => vec.len(),
            NonEmptyMatchCaseParamVec::UniquelyLabeled {
                params,
                triple_dot: _,
            } => params.len(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabeledMatchCaseParam {
    pub span: Option<TextSpan>,
    pub label: ParamLabel,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchCaseOutput {
    Some(Expression),
    ImpossibilityClaim(Option<TextSpan>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub span: Option<TextSpan>,
    pub params: NonEmptyParamVec,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub span: Option<TextSpan>,
    pub assertions: NonEmptyVec<CheckAssertion>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CheckAssertion {
    pub span: Option<TextSpan>,
    pub kind: CheckAssertionKind,
    pub left: GoalKwOrPossiblyInvalidExpression,
    pub right: QuestionMarkOrPossiblyInvalidExpression,
}

pub use crate::data::simplified_ast::CheckAssertionKind;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GoalKwOrPossiblyInvalidExpression {
    GoalKw { span: Option<TextSpan> },
    Expression(PossiblyInvalidExpression),
}

impl GoalKwOrPossiblyInvalidExpression {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            GoalKwOrPossiblyInvalidExpression::GoalKw { span } => *span,
            GoalKwOrPossiblyInvalidExpression::Expression(expression) => expression.span(),
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
            InvalidExpression::SymbolicallyInvalid(expression) => {
                Some(expression.expression.span())
            }
            InvalidExpression::IllegalFunRecursion(expression) => expression.expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolicallyInvalidExpression {
    pub expression: unbound::Expression,
    pub error: BindError,
    pub span_invalidated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IllegalFunRecursionExpression {
    pub expression: Expression,
    pub error: IllegalFunRecursionError,
    pub span_invalidated: bool,
}
