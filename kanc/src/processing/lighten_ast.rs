use crate::data::{bound_ast as heavy, light_ast::*, text_span::*};

use bumpalo::{collections::Vec as BumpVec, Bump};

pub fn register_file_items(bump: &Bump, heavy: Vec<heavy::FileItem>) -> BumpVec<'_, FileItemRef> {
    heavy
        .into_iter()
        .map(|heavy| register_file_item(bump, heavy))
        .collect_in(bump)
}

pub fn register_file_item(bump: &Bump, heavy: heavy::FileItem) -> FileItemRef {
    match heavy {
        heavy::FileItem::Type(heavy) => FileItemRef::Type(register_type_statement(bump, heavy)),
        heavy::FileItem::Let(heavy) => FileItemRef::Let(register_let_statement(bump, heavy)),
    }
}

pub fn register_type_statement(bump: &Bump, heavy: heavy::TypeStatement) -> &TypeStatement {
    let name = register_identifier(bump, heavy.name);
    let params = register_optional_params(bump, heavy.params);
    let variants = heavy
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(bump, unregistered_variant))
        .collect_in(bump);
    bump.alloc(TypeStatement {
        span: heavy.span,
        visibility: heavy.visibility,
        name,
        params,
        variants,
    })
}

pub fn register_identifier(bump: &Bump, heavy: heavy::Identifier) -> &Identifier {
    bump.alloc(Identifier {
        span: heavy.span,
        name: heavy.name,
    })
}

pub fn register_optional_params(
    bump: &Bump,
    heavy: Option<heavy::NonEmptyParamVec>,
) -> Option<&NonEmptyParamVec> {
    heavy.map(|heavy| register_params(bump, heavy))
}

pub fn register_params(bump: &Bump, heavy: heavy::NonEmptyParamVec) -> &NonEmptyParamVec {
    match heavy {
        heavy::NonEmptyParamVec::Unlabeled(heavy) => {
            let params = heavy
                .into_iter()
                .map(|heavy| register_unlabeled_param(bump, heavy))
                .collect_in(bump);
            &NonEmptyParamVec::Unlabeled(params)
        }
        heavy::NonEmptyParamVec::UniquelyLabeled(heavy) => {
            let params = heavy
                .into_iter()
                .map(|heavy| register_labeled_param(bump, heavy))
                .collect_in(bump);
            &NonEmptyParamVec::UniquelyLabeled(params)
        }
    }
}

pub fn register_unlabeled_param(bump: &Bump, heavy: heavy::UnlabeledParam) -> &UnlabeledParam {
    let name = register_identifier(bump, heavy.name);
    let type_ = register_expression(bump, heavy.type_);
    bump.alloc(UnlabeledParam {
        span: heavy.span,
        is_dashed: heavy.is_dashed,
        name,
        type_,
    })
}

pub fn register_labeled_param(bump: &Bump, heavy: heavy::LabeledParam) -> &LabeledParam {
    let label_clause = register_param_label_clause(bump, heavy.label_clause);
    let name = register_identifier(bump, heavy.name);
    let type_ = register_expression(bump, heavy.type_);
    bump.alloc(LabeledParam {
        span: heavy.span,
        label_clause,
        is_dashed: heavy.is_dashed,
        name,
        type_,
    })
}

pub fn register_param_label_clause(
    bump: &Bump,
    heavy: heavy::ParamLabelClause,
) -> ParamLabelClauseRef {
    match heavy {
        heavy::ParamLabelClause::Implicit => ParamLabelClauseRef::Implicit,
        heavy::ParamLabelClause::Explicit(heavy) => {
            ParamLabelClauseRef::Explicit(register_identifier(bump, heavy))
        }
    }
}

pub fn register_variant(bump: &Bump, heavy: heavy::Variant) -> &Variant {
    let name = register_identifier(bump, heavy.name);
    let params = register_optional_params(bump, heavy.params);
    let return_type = register_expression(bump, heavy.return_type);
    bump.alloc(Variant {
        span: heavy.span,
        name,
        params,
        return_type,
    })
}

pub fn register_let_statement(bump: &Bump, heavy: heavy::LetStatement) -> &LetStatement {
    let name = register_identifier(bump, heavy.name);
    let value = register_expression(bump, heavy.value);
    bump.alloc(LetStatement {
        span: heavy.span,
        visibility: heavy.visibility,
        transparency: heavy.transparency,
        name,
        value,
    })
}

pub fn register_expression(bump: &Bump, heavy: heavy::Expression) -> ExpressionRef {
    match heavy {
        heavy::Expression::Name(heavy) => {
            let light = register_name_expression(bump, heavy);
            ExpressionRef::Name(light)
        }
        heavy::Expression::Todo(span) => {
            let light = register_todo_expression(bump, span);
            ExpressionRef::Todo(light)
        }
        heavy::Expression::Call(heavy) => {
            let light = register_call(bump, *heavy);
            ExpressionRef::Call(light)
        }
        heavy::Expression::Fun(heavy) => {
            let light = register_fun(bump, *heavy);
            ExpressionRef::Fun(light)
        }
        heavy::Expression::Match(heavy) => {
            let light = register_match(bump, *heavy);
            ExpressionRef::Match(light)
        }
        heavy::Expression::Forall(heavy) => {
            let light = register_forall(bump, *heavy);
            ExpressionRef::Forall(light)
        }
        heavy::Expression::Check(heavy) => {
            let light = register_check(bump, *heavy);
            ExpressionRef::Check(light)
        }
    }
}

pub fn register_name_expression(bump: &Bump, heavy: heavy::NameExpression) -> &NameExpression {
    let components = heavy
        .components
        .into_iter()
        .map(|heavy| register_identifier(bump, heavy))
        .collect_in(bump);
    bump.alloc(NameExpression {
        span: heavy.span,
        components,
        db_index: heavy.db_index,
    })
}

pub fn register_todo_expression(bump: &Bump, span: Option<TextSpan>) -> &TodoExpression {
    bump.alloc(TodoExpression { span })
}

pub fn register_call(bump: &Bump, heavy: heavy::Call) -> &Call {
    let callee = register_expression(bump, heavy.callee);
    let args = register_call_args(bump, heavy.args);
    let args = bump.alloc(args);
    bump.alloc(Call {
        span: heavy.span,
        callee,
        args,
    })
}

pub fn register_call_args(bump: &Bump, heavy: heavy::NonEmptyCallArgVec) -> NonEmptyCallArgVec {
    match heavy {
        heavy::NonEmptyCallArgVec::Unlabeled(heavy) => {
            let values = heavy
                .into_iter()
                .map(|heavy| register_expression(bump, heavy))
                .collect_in(bump);
            NonEmptyCallArgVec::Unlabeled(&values)
        }
        heavy::NonEmptyCallArgVec::UniquelyLabeled(heavy) => {
            let values = heavy
                .into_iter()
                .map(|heavy| register_labeled_call_arg(bump, heavy))
                .collect_in(bump);
            NonEmptyCallArgVec::UniquelyLabeled(&values)
        }
    }
}

pub fn register_labeled_call_arg(bump: &Bump, heavy: heavy::LabeledCallArg) -> LabeledCallArg {
    match heavy {
        heavy::LabeledCallArg::Implicit {
            label: value,
            db_index,
        } => {
            let label = register_identifier(bump, value);
            LabeledCallArg::Implicit { label, db_index }
        }
        heavy::LabeledCallArg::Explicit { label, value } => LabeledCallArg::Explicit {
            label: register_identifier(bump, label),
            value: register_expression(bump, value),
        },
    }
}

pub fn register_fun(bump: &Bump, heavy: heavy::Fun) -> &Fun {
    let name = register_identifier(bump, heavy.name);
    let params = register_params(bump, heavy.params);
    let return_type = register_expression(bump, heavy.return_type);
    let body = register_expression(bump, heavy.body);
    bump.alloc(Fun {
        span: heavy.span,
        name,
        params,
        return_type,
        body,
    })
}

pub fn register_match(bump: &Bump, heavy: heavy::Match) -> &Match {
    let matchee = register_expression(bump, heavy.matchee);
    let cases = heavy
        .cases
        .into_iter()
        .map(|heavy| register_match_case(bump, heavy))
        .collect_in(bump);
    bump.alloc(Match {
        span: heavy.span,
        matchee,
        cases,
    })
}

pub fn register_match_case(bump: &Bump, heavy: heavy::MatchCase) -> &MatchCase {
    let variant_name = register_identifier(bump, heavy.variant_name);
    let params = register_optional_match_case_params(bump, heavy.params);
    let output = register_match_case_output(bump, heavy.output);
    bump.alloc(MatchCase {
        span: heavy.span,
        variant_name,
        params,
        output,
    })
}

pub fn register_optional_match_case_params(
    bump: &Bump,
    heavy: Option<heavy::NonEmptyMatchCaseParamVec>,
) -> Option<NonEmptyMatchCaseParamVec> {
    heavy.map(|heavy| {
        let params = register_match_case_params(bump, heavy);
        bump.alloc(params)
    })
}

pub fn register_match_case_params(
    bump: &Bump,
    heavy: heavy::NonEmptyMatchCaseParamVec,
) -> NonEmptyMatchCaseParamVec {
    match heavy {
        heavy::NonEmptyMatchCaseParamVec::Unlabeled(heavy) => {
            let light = register_identifiers(bump, heavy);
            NonEmptyMatchCaseParamVec::Unlabeled(light)
        }
        heavy::NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot } => {
            let params = register_optional_labeled_match_case_params(bump, params);
            NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot }
        }
    }
}

pub fn register_identifiers(bump: &Bump, heavy: Vec<heavy::Identifier>) -> &[Identifier] {
    let identifiers = heavy
        .into_iter()
        .map(|heavy| register_identifier(bump, heavy))
        .collect_in(bump);
    &identifiers
}

pub fn register_optional_labeled_match_case_params(
    bump: &Bump,
    heavy: Option<Vec<heavy::LabeledMatchCaseParam>>,
) -> Option<NonEmptyMatchCaseParamVec> {
    heavy.map(|heavy| register_labeled_match_case_params(bump, heavy))
}

pub fn register_labeled_match_case_params(
    bump: &Bump,
    heavy: Vec<heavy::LabeledMatchCaseParam>,
) -> &[LabeledMatchCaseParam] {
    let params = heavy
        .into_iter()
        .map(|heavy| register_labeled_match_case_param(bump, heavy))
        .collect_in(bump);
    &params
}

pub fn register_labeled_match_case_param(
    bump: &Bump,
    heavy: heavy::LabeledMatchCaseParam,
) -> &LabeledMatchCaseParam {
    let label_clause = register_param_label_clause(bump, heavy.label_clause);
    let name = register_identifier(bump, heavy.name);
    bump.alloc(LabeledMatchCaseParam {
        span: heavy.span,
        label_clause,
        name,
    })
}

pub fn register_match_case_output(
    bump: &Bump,
    heavy: heavy::MatchCaseOutput,
) -> MatchCaseOutputRef {
    match heavy {
        heavy::MatchCaseOutput::Some(heavy) => {
            let light = register_expression(bump, heavy);
            MatchCaseOutputRef::Some(light)
        }
        heavy::MatchCaseOutput::ImpossibilityClaim(kw_span) => {
            MatchCaseOutputRef::ImpossibilityClaim(kw_span)
        }
    }
}

pub fn register_forall(bump: &Bump, heavy: heavy::Forall) -> &Forall {
    let params = register_params(bump, heavy.params);
    let output = register_expression(bump, heavy.output);
    bump.alloc(Forall {
        span: heavy.span,
        params,
        output,
    })
}

pub fn register_check(bump: &Bump, heavy: heavy::Check) -> &Check {
    let assertions = heavy
        .assertions
        .into_iter()
        .map(|heavy| register_check_assertion(bump, heavy))
        .collect_in(bump);
    let output = register_expression(bump, heavy.output);
    bump.alloc(Check {
        span: heavy.span,
        assertions,
        output,
    })
}

pub fn register_check_assertion(bump: &Bump, heavy: heavy::CheckAssertion) -> &CheckAssertion {
    let left = register_goal_kw_or_expression(bump, heavy.left);
    let right = register_question_mark_or_possibly_invalid_expression(bump, heavy.right);
    bump.alloc(CheckAssertion {
        span: heavy.span,
        kind: heavy.kind,
        left,
        right,
    })
}

pub fn register_goal_kw_or_expression(
    bump: &Bump,
    heavy: heavy::GoalKwOrPossiblyInvalidExpression,
) -> GoalKwOrPossiblyInvalidExpressionRef {
    match heavy {
        heavy::GoalKwOrPossiblyInvalidExpression::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpressionRef::GoalKw { span: start }
        }
        heavy::GoalKwOrPossiblyInvalidExpression::Expression(heavy) => {
            let light = register_possibly_invalid_expression(bump, heavy);
            GoalKwOrPossiblyInvalidExpressionRef::Expression(light)
        }
    }
}

pub fn register_question_mark_or_possibly_invalid_expression(
    bump: &Bump,
    heavy: heavy::QuestionMarkOrPossiblyInvalidExpression,
) -> QuestionMarkOrPossiblyInvalidExpressionRef {
    match heavy {
        heavy::QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpressionRef::QuestionMark { span: start }
        }
        heavy::QuestionMarkOrPossiblyInvalidExpression::Expression(heavy) => {
            let light = register_possibly_invalid_expression(bump, heavy);
            QuestionMarkOrPossiblyInvalidExpressionRef::Expression(light)
        }
    }
}

pub fn register_possibly_invalid_expression(
    bump: &Bump,
    heavy: heavy::PossiblyInvalidExpression,
) -> PossiblyInvalidExpressionRef {
    match heavy {
        heavy::PossiblyInvalidExpression::Valid(heavy) => {
            let light = register_expression(bump, heavy);
            PossiblyInvalidExpressionRef::Valid(light)
        }
        heavy::PossiblyInvalidExpression::Invalid(invalid) => {
            let invalid = bump.alloc(invalid);
            PossiblyInvalidExpressionRef::Invalid(invalid)
        }
    }
}

trait CollectInBump<T> {
    fn collect_in(self, bump: &Bump) -> BumpVec<'_, T>;
}

impl<T, I> CollectInBump<T> for I
where
    I: IntoIterator<Item = T>,
{
    fn collect_in(self, bump: &Bump) -> BumpVec<'_, T> {
        BumpVec::from_iter_in(self, bump)
    }
}
