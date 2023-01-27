use crate::data::{bound_ast as heavy, light_ast::*, text_span::*};

use bumpalo::{collections::Vec as BumpVec, Bump};

pub fn register_file_items(
    bump: &Bump,
    unregistered: Vec<heavy::FileItem>,
) -> BumpVec<'_, FileItemRef> {
    unregistered
        .into_iter()
        .map(|unregistered| register_file_item(bump, unregistered))
        .collect_in(bump)
}

pub fn register_file_item(bump: &Bump, unregistered: heavy::FileItem) -> FileItemRef {
    match unregistered {
        heavy::FileItem::Type(unregistered) => {
            FileItemRef::Type(register_type_statement(bump, unregistered))
        }
        heavy::FileItem::Let(unregistered) => {
            FileItemRef::Let(register_let_statement(bump, unregistered))
        }
    }
}

pub fn register_type_statement(bump: &Bump, unregistered: heavy::TypeStatement) -> &TypeStatement {
    let name = register_identifier(bump, unregistered.name);
    let params = register_optional_params(bump, unregistered.params);
    let variants = unregistered
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(bump, unregistered_variant))
        .collect_in(bump);
    bump.alloc(TypeStatement {
        span: unregistered.span,
        visibility: unregistered.visibility,
        name,
        params,
        variants,
    })
}

pub fn register_identifier(bump: &Bump, unregistered: heavy::Identifier) -> &Identifier {
    bump.alloc(Identifier {
        span: unregistered.span,
        name: unregistered.name,
    })
}

pub fn register_optional_params(
    bump: &Bump,
    unregistered: Option<heavy::NonEmptyParamVec>,
) -> Option<&NonEmptyParamVec> {
    unregistered.map(|unregistered| register_params(bump, unregistered))
}

pub fn register_params(bump: &Bump, unregistered: heavy::NonEmptyParamVec) -> &NonEmptyParamVec {
    match unregistered {
        heavy::NonEmptyParamVec::Unlabeled(unregistered) => {
            let params = unregistered
                .into_iter()
                .map(|unregistered| register_unlabeled_param(bump, unregistered))
                .collect_in(bump);
            &NonEmptyParamVec::Unlabeled(params)
        }
        heavy::NonEmptyParamVec::UniquelyLabeled(unregistered) => {
            let params = unregistered
                .into_iter()
                .map(|unregistered| register_labeled_param(bump, unregistered))
                .collect_in(bump);
            &NonEmptyParamVec::UniquelyLabeled(params)
        }
    }
}

pub fn register_unlabeled_param(
    bump: &Bump,
    unregistered: heavy::UnlabeledParam,
) -> &UnlabeledParam {
    let name = register_identifier(bump, unregistered.name);
    let type_ = register_expression(bump, unregistered.type_);
    bump.alloc(UnlabeledParam {
        span: unregistered.span,
        is_dashed: unregistered.is_dashed,
        name,
        type_,
    })
}

pub fn register_labeled_param(bump: &Bump, unregistered: heavy::LabeledParam) -> &LabeledParam {
    let label_clause = register_param_label_clause(bump, unregistered.label_clause);
    let name = register_identifier(bump, unregistered.name);
    let type_ = register_expression(bump, unregistered.type_);
    bump.alloc(LabeledParam {
        span: unregistered.span,
        label_clause,
        is_dashed: unregistered.is_dashed,
        name,
        type_,
    })
}

pub fn register_param_label_clause(
    bump: &Bump,
    unregistered: heavy::ParamLabelClause,
) -> ParamLabelClauseRef {
    match unregistered {
        heavy::ParamLabelClause::Implicit => ParamLabelClauseRef::Implicit,
        heavy::ParamLabelClause::Explicit(unregistered) => {
            ParamLabelClauseRef::Explicit(register_identifier(bump, unregistered))
        }
    }
}

pub fn register_variant(bump: &Bump, unregistered: heavy::Variant) -> &Variant {
    let name = register_identifier(bump, unregistered.name);
    let params = register_optional_params(bump, unregistered.params);
    let return_type = register_expression(bump, unregistered.return_type);
    bump.alloc(Variant {
        span: unregistered.span,
        name,
        params,
        return_type,
    })
}

pub fn register_let_statement(bump: &Bump, unregistered: heavy::LetStatement) -> &LetStatement {
    let name = register_identifier(bump, unregistered.name);
    let value = register_expression(bump, unregistered.value);
    bump.alloc(LetStatement {
        span: unregistered.span,
        visibility: unregistered.visibility,
        transparency: unregistered.transparency,
        name,
        value,
    })
}

pub fn register_expression(bump: &Bump, unregistered: heavy::Expression) -> ExpressionRef {
    match unregistered {
        heavy::Expression::Name(unregistered) => {
            let light = register_name_expression(bump, unregistered);
            ExpressionRef::Name(light)
        }
        heavy::Expression::Todo(span) => {
            let light = register_todo_expression(bump, span);
            ExpressionRef::Todo(light)
        }
        heavy::Expression::Call(unregistered) => {
            let light = register_call(bump, *unregistered);
            ExpressionRef::Call(light)
        }
        heavy::Expression::Fun(unregistered) => {
            let light = register_fun(bump, *unregistered);
            ExpressionRef::Fun(light)
        }
        heavy::Expression::Match(unregistered) => {
            let light = register_match(bump, *unregistered);
            ExpressionRef::Match(light)
        }
        heavy::Expression::Forall(unregistered) => {
            let light = register_forall(bump, *unregistered);
            ExpressionRef::Forall(light)
        }
        heavy::Expression::Check(unregistered) => {
            let light = register_check(bump, *unregistered);
            ExpressionRef::Check(light)
        }
    }
}

pub fn register_name_expression(
    bump: &Bump,
    unregistered: heavy::NameExpression,
) -> &NameExpression {
    let components = unregistered
        .components
        .into_iter()
        .map(|unregistered| register_identifier(bump, unregistered))
        .collect_in(bump);
    bump.alloc(NameExpression {
        span: unregistered.span,
        components,
        db_index: unregistered.db_index,
    })
}

pub fn register_todo_expression(bump: &Bump, span: Option<TextSpan>) -> &TodoExpression {
    bump.alloc(TodoExpression { span })
}

pub fn register_call(bump: &Bump, unregistered: heavy::Call) -> &Call {
    let callee = register_expression(bump, unregistered.callee);
    let args = register_call_args(bump, unregistered.args);
    let args = bump.alloc(args);
    bump.alloc(Call {
        span: unregistered.span,
        callee,
        args,
    })
}

pub fn register_call_args(
    bump: &Bump,
    unregistered: heavy::NonEmptyCallArgVec,
) -> NonEmptyCallArgVec {
    match unregistered {
        heavy::NonEmptyCallArgVec::Unlabeled(unregistered) => {
            let values = unregistered
                .into_iter()
                .map(|unregistered| register_expression(bump, unregistered))
                .collect_in(bump);
            NonEmptyCallArgVec::Unlabeled(&values)
        }
        heavy::NonEmptyCallArgVec::UniquelyLabeled(unregistered) => {
            let values = unregistered
                .into_iter()
                .map(|unregistered| register_labeled_call_arg(bump, unregistered))
                .collect_in(bump);
            NonEmptyCallArgVec::UniquelyLabeled(&values)
        }
    }
}

pub fn register_labeled_call_arg(
    bump: &Bump,
    unregistered: heavy::LabeledCallArg,
) -> LabeledCallArg {
    match unregistered {
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

pub fn register_fun(bump: &Bump, unregistered: heavy::Fun) -> &Fun {
    let name = register_identifier(bump, unregistered.name);
    let params = register_params(bump, unregistered.params);
    let return_type = register_expression(bump, unregistered.return_type);
    let body = register_expression(bump, unregistered.body);
    bump.alloc(Fun {
        span: unregistered.span,
        name,
        params,
        return_type,
        body,
    })
}

pub fn register_match(bump: &Bump, unregistered: heavy::Match) -> &Match {
    let matchee = register_expression(bump, unregistered.matchee);
    let cases = unregistered
        .cases
        .into_iter()
        .map(|unregistered| register_match_case(bump, unregistered))
        .collect_in(bump);
    bump.alloc(Match {
        span: unregistered.span,
        matchee,
        cases,
    })
}

pub fn register_match_case(bump: &Bump, unregistered: heavy::MatchCase) -> &MatchCase {
    let variant_name = register_identifier(bump, unregistered.variant_name);
    let params = register_optional_match_case_params(bump, unregistered.params);
    let output = register_match_case_output(bump, unregistered.output);
    bump.alloc(MatchCase {
        span: unregistered.span,
        variant_name,
        params,
        output,
    })
}

pub fn register_optional_match_case_params(
    bump: &Bump,
    unregistered: Option<heavy::NonEmptyMatchCaseParamVec>,
) -> Option<NonEmptyMatchCaseParamVec> {
    unregistered.map(|unregistered| {
        let params = register_match_case_params(bump, unregistered);
        bump.alloc(params)
    })
}

pub fn register_match_case_params(
    bump: &Bump,
    unregistered: heavy::NonEmptyMatchCaseParamVec,
) -> NonEmptyMatchCaseParamVec {
    match unregistered {
        heavy::NonEmptyMatchCaseParamVec::Unlabeled(unregistered) => {
            let light = register_identifiers(bump, unregistered);
            NonEmptyMatchCaseParamVec::Unlabeled(light)
        }
        heavy::NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot } => {
            let params = register_optional_labeled_match_case_params(bump, params);
            NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot }
        }
    }
}

pub fn register_identifiers(bump: &Bump, unregistered: Vec<heavy::Identifier>) -> &[Identifier] {
    let identifiers = unregistered
        .into_iter()
        .map(|unregistered| register_identifier(bump, unregistered))
        .collect_in(bump);
    &identifiers
}

pub fn register_optional_labeled_match_case_params(
    bump: &Bump,
    unregistered: Option<Vec<heavy::LabeledMatchCaseParam>>,
) -> Option<NonEmptyMatchCaseParamVec> {
    unregistered.map(|unregistered| register_labeled_match_case_params(bump, unregistered))
}

pub fn register_labeled_match_case_params(
    bump: &Bump,
    unregistered: Vec<heavy::LabeledMatchCaseParam>,
) -> &[LabeledMatchCaseParam] {
    let params = unregistered
        .into_iter()
        .map(|unregistered| register_labeled_match_case_param(bump, unregistered))
        .collect_in(bump);
    &params
}

pub fn register_labeled_match_case_param(
    bump: &Bump,
    unregistered: heavy::LabeledMatchCaseParam,
) -> &LabeledMatchCaseParam {
    let label_clause = register_param_label_clause(bump, unregistered.label_clause);
    let name = register_identifier(bump, unregistered.name);
    bump.alloc(LabeledMatchCaseParam {
        span: unregistered.span,
        label_clause,
        name,
    })
}

pub fn register_match_case_output(
    bump: &Bump,
    unregistered: heavy::MatchCaseOutput,
) -> MatchCaseOutputRef {
    match unregistered {
        heavy::MatchCaseOutput::Some(unregistered) => {
            let light = register_expression(bump, unregistered);
            MatchCaseOutputRef::Some(light)
        }
        heavy::MatchCaseOutput::ImpossibilityClaim(kw_span) => {
            MatchCaseOutputRef::ImpossibilityClaim(kw_span)
        }
    }
}

pub fn register_forall(bump: &Bump, unregistered: heavy::Forall) -> &Forall {
    let params = register_params(bump, unregistered.params);
    let output = register_expression(bump, unregistered.output);
    bump.alloc(Forall {
        span: unregistered.span,
        params,
        output,
    })
}

pub fn register_check(bump: &Bump, unregistered: heavy::Check) -> &Check {
    let assertions = unregistered
        .assertions
        .into_iter()
        .map(|unregistered| register_check_assertion(bump, unregistered))
        .collect_in(bump);
    let output = register_expression(bump, unregistered.output);
    bump.alloc(Check {
        span: unregistered.span,
        assertions,
        output,
    })
}

pub fn register_check_assertion(
    bump: &Bump,
    unregistered: heavy::CheckAssertion,
) -> &CheckAssertion {
    let left = register_goal_kw_or_expression(bump, unregistered.left);
    let right = register_question_mark_or_possibly_invalid_expression(bump, unregistered.right);
    bump.alloc(CheckAssertion {
        span: unregistered.span,
        kind: unregistered.kind,
        left,
        right,
    })
}

pub fn register_goal_kw_or_expression(
    bump: &Bump,
    unregistered: heavy::GoalKwOrPossiblyInvalidExpression,
) -> GoalKwOrPossiblyInvalidExpressionRef {
    match unregistered {
        heavy::GoalKwOrPossiblyInvalidExpression::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpressionRef::GoalKw { span: start }
        }
        heavy::GoalKwOrPossiblyInvalidExpression::Expression(unregistered) => {
            let light = register_possibly_invalid_expression(bump, unregistered);
            GoalKwOrPossiblyInvalidExpressionRef::Expression(light)
        }
    }
}

pub fn register_question_mark_or_possibly_invalid_expression(
    bump: &Bump,
    unregistered: heavy::QuestionMarkOrPossiblyInvalidExpression,
) -> QuestionMarkOrPossiblyInvalidExpressionRef {
    match unregistered {
        heavy::QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpressionRef::QuestionMark { span: start }
        }
        heavy::QuestionMarkOrPossiblyInvalidExpression::Expression(unregistered) => {
            let light = register_possibly_invalid_expression(bump, unregistered);
            QuestionMarkOrPossiblyInvalidExpressionRef::Expression(light)
        }
    }
}

pub fn register_possibly_invalid_expression(
    bump: &Bump,
    unregistered: heavy::PossiblyInvalidExpression,
) -> PossiblyInvalidExpressionRef {
    match unregistered {
        heavy::PossiblyInvalidExpression::Valid(unregistered) => {
            let light = register_expression(bump, unregistered);
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
