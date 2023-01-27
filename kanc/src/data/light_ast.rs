use crate::data::{fun_recursion_validation_result::IllegalFunRecursionError, text_span::*};

use bumpalo::collections::Vec as BumpVec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement<'a> {
    pub span: Option<TextSpan>,
    pub visibility: Visibility,
    pub name: &'a Identifier,
    pub param_list: Option<&'a NonEmptyParamVec<'a>>,
    pub variant_list: &'a [Variant<'a>],
}

pub use crate::data::bound_ast::ModScope;

pub use crate::data::bound_ast::Visibility;

pub use crate::data::bound_ast::Transparency;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NonEmptyParamVec<'a> {
    Unlabeled(BumpVec<'a, UnlabeledParam<'a>>),
    UniquelyLabeled(BumpVec<'a, LabeledParam<'a>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnlabeledParam<'a> {
    pub span: Option<TextSpan>,
    pub is_dashed: bool,
    pub name: &'a Identifier,
    pub type_: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledParam<'a> {
    pub span: Option<TextSpan>,
    pub label_clause: &'a ParamLabelClause<'a>,
    pub is_dashed: bool,
    pub name: &'a Identifier,
    pub type_: ExpressionRef<'a>,
}

impl LabeledParam<'_> {
    pub fn label(&self) -> &Identifier {
        match self.label_clause {
            ParamLabelClause::Implicit => self.name,
            ParamLabelClause::Explicit(label) => label,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParamLabelClause<'a> {
    Implicit,
    Explicit(&'a Identifier),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<'a> {
    pub span: Option<TextSpan>,
    pub name: &'a Identifier,
    pub param_list: Option<&'a NonEmptyParamVec<'a>>,
    pub return_type: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement<'a> {
    pub span: Option<TextSpan>,
    pub visibility: Visibility,
    pub transparency: Transparency,
    pub name: &'a Identifier,
    pub value: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionRef<'a> {
    Name(&'a NameExpression<'a>),
    Todo(&'a TodoExpression),
    Call(&'a Call<'a>),
    Fun(&'a Fun<'a>),
    Match(&'a Match<'a>),
    Forall(&'a Forall<'a>),
    Check(&'a Check<'a>),
}

impl ExpressionRef<'_> {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            ExpressionRef::Name(name) => name.span,
            ExpressionRef::Todo(todo) => todo.span,
            ExpressionRef::Call(call) => call.span,
            ExpressionRef::Fun(fun) => fun.span,
            ExpressionRef::Match(match_) => match_.span,
            ExpressionRef::Forall(forall) => forall.span,
            ExpressionRef::Check(check) => check.span,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression<'a> {
    pub span: Option<TextSpan>,
    pub component_list: &'a [Identifier],
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub span: Option<TextSpan>,
    pub name: IdentifierName,
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::UnreservedIdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TodoExpression {
    pub span: Option<TextSpan>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call<'a> {
    pub span: Option<TextSpan>,
    pub callee: ExpressionRef<'a>,
    pub arg_list: &'a NonEmptyCallArgVec<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NonEmptyCallArgVec<'a> {
    Unlabeled(BumpVec<'a, ExpressionRef<'a>>),
    UniquelyLabeled(BumpVec<'a, LabeledCallArg<'a>>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LabeledCallArg<'a> {
    Implicit {
        label: &'a Identifier,
        db_index: DbIndex,
    },
    Explicit {
        label: &'a Identifier,
        value: ExpressionRef<'a>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun<'a> {
    pub span: Option<TextSpan>,
    pub name: &'a Identifier,
    pub param_list: &'a NonEmptyParamVec<'a>,
    pub return_type: ExpressionRef<'a>,
    pub body: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match<'a> {
    pub span: Option<TextSpan>,
    pub matchee: ExpressionRef<'a>,
    pub case_list: &'a [MatchCase<'a>],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase<'a> {
    pub span: Option<TextSpan>,
    pub variant_name: &'a Identifier,
    pub param_list: Option<&'a NonEmptyMatchCaseParamVec<'a>>,
    pub output: MatchCaseOutput<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NonEmptyMatchCaseParamVec<'a> {
    Unlabeled(BumpVec<'a, Identifier>),
    UniquelyLabeled {
        params: BumpVec<'a, LabeledMatchCaseParam<'a>>,
        triple_dot: Option<TextSpan>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledMatchCaseParam<'a> {
    pub span: Option<TextSpan>,
    pub label_clause: ParamLabelClause<'a>,
    pub name: &'a Identifier,
}

impl<'a> LabeledMatchCaseParam<'a> {
    pub fn label(&self) -> &'a Identifier {
        match self.label_clause {
            ParamLabelClause::Implicit => self.name,
            ParamLabelClause::Explicit(label) => label,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MatchCaseOutput<'a> {
    Some(ExpressionRef<'a>),
    ImpossibilityClaim(Option<TextSpan>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall<'a> {
    pub span: Option<TextSpan>,
    pub param_list: NonEmptyParamVec<'a>,
    pub output: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Check<'a> {
    pub span: Option<TextSpan>,
    pub assertion_list: &'a [CheckAssertion<'a>],
    pub output: ExpressionRef<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckAssertion<'a> {
    pub span: Option<TextSpan>,
    pub kind: CheckAssertionKind,
    pub left: GoalKwOrPossiblyInvalidExpression<'a>,
    pub right: QuestionMarkOrPossiblyInvalidExpression<'a>,
}

pub use crate::data::bound_ast::CheckAssertionKind;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GoalKwOrPossiblyInvalidExpression<'a> {
    GoalKw { span: Option<TextSpan> },
    Expression(PossiblyInvalidExpressionRef<'a>),
}

impl GoalKwOrPossiblyInvalidExpression<'_> {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            GoalKwOrPossiblyInvalidExpression::GoalKw { span } => *span,
            GoalKwOrPossiblyInvalidExpression::Expression(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuestionMarkOrPossiblyInvalidExpression<'a> {
    QuestionMark { span: Option<TextSpan> },
    Expression(PossiblyInvalidExpressionRef<'a>),
}

impl QuestionMarkOrPossiblyInvalidExpression<'_> {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span } => *span,
            QuestionMarkOrPossiblyInvalidExpression::Expression(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PossiblyInvalidExpressionRef<'a> {
    Valid(ExpressionRef<'a>),
    Invalid(&'a InvalidExpression<'a>),
}

impl PossiblyInvalidExpressionRef<'_> {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            PossiblyInvalidExpressionRef::Valid(expression) => expression.span(),
            PossiblyInvalidExpressionRef::Invalid(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidExpression<'a> {
    SymbolicallyInvalid(SymbolicallyInvalidExpression),
    IllegalFunRecursion(IllegalFunRecursionExpression<'a>),
}

pub use crate::data::bound_ast::SymbolicallyInvalidExpression;

impl InvalidExpression<'_> {
    pub fn span(&self) -> Option<TextSpan> {
        match self {
            InvalidExpression::SymbolicallyInvalid(expression) => expression.span(),
            InvalidExpression::IllegalFunRecursion(expression) => expression.span(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IllegalFunRecursionExpression<'a> {
    pub expression: ExpressionRef<'a>,
    pub error: IllegalFunRecursionError<'a>,
    pub span_invalidated: bool,
}

impl IllegalFunRecursionExpression<'_> {
    pub fn span(&self) -> Option<TextSpan> {
        if self.span_invalidated {
            None
        } else {
            self.expression.span()
        }
    }
}
