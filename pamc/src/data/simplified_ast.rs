use crate::data::{
    non_empty_vec::{NonEmptyVec, OptionalNonEmptyVecLen},
    FileId, TextSpan,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub span: TextSpan,
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    UseSingle(UseSingleStatement),
    UseWildcard(UseWildcardStatement),
    Mod(ModStatement),
    Type(TypeStatement),
    Let(LetStatement),
}

impl FileItem {
    pub fn span(&self) -> TextSpan {
        match self {
            FileItem::UseSingle(item) => item.span,
            FileItem::UseWildcard(item) => item.span,
            FileItem::Mod(item) => item.span,
            FileItem::Type(item) => item.span,
            FileItem::Let(item) => item.span,
        }
    }
}

pub use crate::data::unsimplified_ast::PubClause;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UseSingleStatement {
    pub span: TextSpan,
    pub visibility: Option<PubClause>,
    pub first_component: UseStatementFirstComponent,
    pub other_components: Vec<Identifier>,
    pub alternate_name: Option<Identifier>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UseWildcardStatement {
    pub span: TextSpan,
    pub visibility: Option<PubClause>,
    pub first_component: UseStatementFirstComponent,
    pub other_components: Vec<Identifier>,
    pub star_span: TextSpan,
}

pub use crate::data::unsimplified_ast::UseStatementFirstComponent;

pub use crate::data::unsimplified_ast::ModStatement;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub span: TextSpan,
    pub visibility: Option<PubClause>,
    pub name: Identifier,
    pub params: Option<NonEmptyParamVec>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyParamVec {
    Unlabeled(NonEmptyVec<UnlabeledParam>),
    UniquelyLabeled(NonEmptyVec<LabeledParam>),
}

impl OptionalNonEmptyVecLen for Option<NonEmptyParamVec> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.len()).unwrap_or(0)
    }
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
    pub visibility: Option<PubClause>,
    pub transparency: Option<ParenthesizedWeakAncestor>,
    pub name: Identifier,
    pub value: Expression,
}

pub use crate::data::unsimplified_ast::ParenthesizedWeakAncestor;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Name(NameExpression),
    Todo(TextSpan),
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
    pub span: TextSpan,
    pub components: NonEmptyVec<Identifier>,
}

pub use crate::data::unsimplified_ast::Identifier;

pub use crate::data::unsimplified_ast::IdentifierName;

pub use crate::data::unsimplified_ast::UnreservedIdentifierName;

pub use crate::data::unsimplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub span: TextSpan,
    pub callee: Expression,
    pub args: NonEmptyCallArgVec,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NonEmptyCallArgVec {
    Unlabeled(NonEmptyVec<Expression>),
    UniquelyLabeled(NonEmptyVec<LabeledCallArg>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LabeledCallArg {
    Implicit(Identifier),
    Explicit(Identifier, Expression),
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
pub enum MatchCaseOutput {
    Some(Expression),
    ImpossibilityClaim(TextSpan),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabeledMatchCaseParam {
    pub span: TextSpan,
    pub label: ParamLabel,
    pub name: Identifier,
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
