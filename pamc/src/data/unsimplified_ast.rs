use crate::data::{non_empty_vec::NonEmptyVec, FileId, TextSpan};

use std::num::NonZeroUsize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub span: TextSpan,
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Use(UseStatement),
    Mod(ModStatement),
    Type(TypeStatement),
    Let(LetStatement),
}

impl FileItem {
    pub fn span(&self) -> TextSpan {
        match self {
            FileItem::Use(use_) => use_.span,
            FileItem::Mod(mod_) => mod_.span,
            FileItem::Type(type_) => type_.span,
            FileItem::Let(let_) => let_.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UseStatement {
    pub span: TextSpan,
    pub visibility: Option<PubClause>,
    pub first_component: UseStatementFirstComponent,
    pub other_components: Vec<Identifier>,
    pub import_modifier: Option<WildcardOrAlternateName>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UseStatementFirstComponent {
    pub span: TextSpan,
    pub kind: UseStatementFirstComponentKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UseStatementFirstComponentKind {
    Mod,
    Super(NonZeroUsize),
    Pack,
    Identifier(IdentifierName),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WildcardOrAlternateName {
    pub span: TextSpan,
    pub kind: WildcardOrAlternateNameKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WildcardOrAlternateNameKind {
    Wildcard,
    AlternateName(IdentifierName),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ModStatement {
    pub span: TextSpan,
    pub visibility: Option<PubClause>,
    pub name: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub span: TextSpan,
    pub visibility: Option<PubClause>,
    pub name: Identifier,
    pub params: Option<NonEmptyVec<Param>>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PubClause {
    pub span: TextSpan,
    pub ancestor: Option<ParenthesizedWeakAncestor>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParenthesizedWeakAncestor {
    pub span: TextSpan,
    pub kind: WeakAncestorKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum WeakAncestorKind {
    Global,
    Mod,
    Super(NonZeroUsize),
    PackRelative { path_after_pack_kw: Vec<Identifier> },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub span: TextSpan,
    pub label: Option<ParamLabel>,
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

impl Param {
    pub fn label_name(&self) -> Option<&IdentifierName> {
        self.label.as_ref().map(|label| match label {
            ParamLabel::Implicit => &self.name.name,
            ParamLabel::Explicit(name) => &name.name,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ParamLabel {
    Implicit,
    Explicit(Identifier),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub span: TextSpan,
    pub name: Identifier,
    pub params: Option<NonEmptyVec<Param>>,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Identifier(Identifier),
    Todo(TextSpan),
    Dot(Box<Dot>),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
    Check(Box<Check>),
}

impl Expression {
    pub fn span(&self) -> TextSpan {
        match self {
            Expression::Identifier(identifier) => identifier.span,
            Expression::Todo(span) => *span,
            Expression::Dot(dot) => dot.span,
            Expression::Call(call) => call.span,
            Expression::Fun(fun) => fun.span,
            Expression::Match(match_) => match_.span,
            Expression::Forall(forall) => forall.span,
            Expression::Check(check) => check.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub span: TextSpan,
    pub name: IdentifierName,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IdentifierName {
    Standard(UnreservedIdentifierName),
    Reserved(ReservedIdentifierName),
}

impl IdentifierName {
    pub fn new(s: String) -> Self {
        match s.as_str() {
            "Type" => IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
            "Type1" => IdentifierName::Reserved(ReservedIdentifierName::Type1),
            "Type2" => IdentifierName::Reserved(ReservedIdentifierName::Type2),
            "_" => IdentifierName::Reserved(ReservedIdentifierName::Underscore),
            "mod" => IdentifierName::Reserved(ReservedIdentifierName::Mod),
            "super" => IdentifierName::Reserved(ReservedIdentifierName::Super),
            "super2" => IdentifierName::Reserved(ReservedIdentifierName::Super2),
            "super3" => IdentifierName::Reserved(ReservedIdentifierName::Super3),
            "super4" => IdentifierName::Reserved(ReservedIdentifierName::Super4),
            "super5" => IdentifierName::Reserved(ReservedIdentifierName::Super5),
            "super6" => IdentifierName::Reserved(ReservedIdentifierName::Super6),
            "super7" => IdentifierName::Reserved(ReservedIdentifierName::Super7),
            "super8" => IdentifierName::Reserved(ReservedIdentifierName::Super8),
            "pack" => IdentifierName::Reserved(ReservedIdentifierName::Pack),
            _ => IdentifierName::Standard(UnreservedIdentifierName::unchecked_new(s)),
        }
    }
}

impl IdentifierName {
    pub fn src_str(&self) -> &str {
        match &self {
            IdentifierName::Standard(s) => s.raw(),
            IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase) => "Type",
            IdentifierName::Reserved(ReservedIdentifierName::Type1) => "Type1",
            IdentifierName::Reserved(ReservedIdentifierName::Type2) => "Type2",
            IdentifierName::Reserved(ReservedIdentifierName::Underscore) => "_",
            IdentifierName::Reserved(ReservedIdentifierName::Mod) => "mod",
            IdentifierName::Reserved(ReservedIdentifierName::Super) => "super",
            IdentifierName::Reserved(ReservedIdentifierName::Super2) => "super2",
            IdentifierName::Reserved(ReservedIdentifierName::Super3) => "super3",
            IdentifierName::Reserved(ReservedIdentifierName::Super4) => "super4",
            IdentifierName::Reserved(ReservedIdentifierName::Super5) => "super5",
            IdentifierName::Reserved(ReservedIdentifierName::Super6) => "super6",
            IdentifierName::Reserved(ReservedIdentifierName::Super7) => "super7",
            IdentifierName::Reserved(ReservedIdentifierName::Super8) => "super8",
            IdentifierName::Reserved(ReservedIdentifierName::Pack) => "pack",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UnreservedIdentifierName {
    raw: String,
}

impl UnreservedIdentifierName {
    pub fn unchecked_new(raw: String) -> Self {
        Self { raw }
    }
}

impl UnreservedIdentifierName {
    pub fn raw(&self) -> &str {
        &self.raw
    }
}

impl From<UnreservedIdentifierName> for String {
    fn from(name: UnreservedIdentifierName) -> Self {
        name.raw
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReservedIdentifierName {
    TypeTitleCase,
    Type1,
    Type2,
    Underscore,
    Mod,
    Super,
    Super2,
    Super3,
    Super4,
    Super5,
    Super6,
    Super7,
    Super8,
    Pack,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dot {
    pub span: TextSpan,
    pub left: Expression,
    pub right: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub span: TextSpan,
    pub callee: Expression,
    pub args: NonEmptyVec<CallArg>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallArg {
    pub span: TextSpan,
    pub label: Option<ParamLabel>,
    pub value: Expression,
}

impl CallArg {
    pub fn label_name(&self) -> Option<&IdentifierName> {
        self.label.as_ref().map(|label| match label {
            ParamLabel::Implicit => match &self.value {
                Expression::Identifier(identifier) => &identifier.name,
                _ => panic!("Implicit argument label must be an identifier"),
            },
            ParamLabel::Explicit(name) => &name.name,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub span: TextSpan,
    pub name: Identifier,
    pub params: NonEmptyVec<Param>,
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
    pub params: Option<NonEmptyVec<MatchCaseParam>>,
    pub triple_dot: Option<TextSpan>,
    pub output: MatchCaseOutput,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCaseParam {
    pub span: TextSpan,
    pub label: Option<ParamLabel>,
    pub name: Identifier,
}

impl MatchCaseParam {
    pub fn label_name(&self) -> Option<&IdentifierName> {
        self.label.as_ref().map(|label| match label {
            ParamLabel::Implicit => &self.name.name,
            ParamLabel::Explicit(name) => &name.name,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MatchCaseOutput {
    Some(Expression),
    ImpossibilityClaim(TextSpan),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub span: TextSpan,
    pub params: NonEmptyVec<Param>,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CheckAssertionKind {
    Type,
    NormalForm,
}

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
