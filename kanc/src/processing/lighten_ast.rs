use crate::data::{bound_ast as heavy, light_ast::*, text_span::*};

use bumpalo::{collections::Vec as BumpVec, Bump};

pub fn register_file_items(
    bump: &Bump,
    unregistered: Vec<heavy::FileItem>,
) -> BumpVec<'_, FileItemRef<'_>> {
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

pub fn register_type_statement(
    bump: &Bump,
    unregistered: heavy::TypeStatement,
) -> &TypeStatement<'_> {
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
) -> &UnlabeledParam<'_> {
    let name = register_identifier(bump, unregistered.name);
    let type_ = register_expression(bump, unregistered.type_);
    bump.alloc(UnlabeledParam {
        span: unregistered.span,
        is_dashed: unregistered.is_dashed,
        name,
        type_,
    })
}

pub fn register_labeled_param(bump: &Bump, unregistered: heavy::LabeledParam) -> &LabeledParam<'_> {
    let label_clause = register_param_label(bump, unregistered.label_clause);
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

pub fn register_param_label(
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

pub fn register_variant(bump: &Bump, unregistered: heavy::Variant) -> &Variant<'_> {
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

pub fn register_let_statement(bump: &Bump, unregistered: heavy::LetStatement) -> &LetStatement<'_> {
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

pub fn register_expression(bump: &Bump, unregistered: heavy::Expression) -> ExpressionRef<'_> {
    match unregistered {
        heavy::Expression::Name(unregistered) => {
            let id = register_name_expression(bump, unregistered);
            ExpressionRef::Name(id)
        }
        heavy::Expression::Todo(span) => {
            let id = register_todo_expression(bump, span);
            ExpressionRef::Todo(id)
        }
        heavy::Expression::Call(unregistered) => {
            let id = register_call(bump, *unregistered);
            ExpressionRef::Call(id)
        }
        heavy::Expression::Fun(unregistered) => {
            let id = register_fun(bump, *unregistered);
            ExpressionRef::Fun(id)
        }
        heavy::Expression::Match(unregistered) => {
            let id = register_match(bump, *unregistered);
            ExpressionRef::Match(id)
        }
        heavy::Expression::Forall(unregistered) => {
            let id = register_forall(bump, *unregistered);
            ExpressionRef::Forall(id)
        }
        heavy::Expression::Check(unregistered) => {
            let id = register_check(bump, *unregistered);
            ExpressionRef::Check(id)
        }
    }
}

pub fn register_name_expression(
    bump: &Bump,
    unregistered: heavy::NameExpression,
) -> &NameExpression<'_> {
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

pub fn register_call(bump: &Bump, unregistered: heavy::Call) -> &Call<'_> {
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
            NonEmptyCallArgVec::Unlabeled(values)
        }
        heavy::NonEmptyCallArgVec::UniquelyLabeled(unregistered) => {
            let values = unregistered
                .into_iter()
                .map(|unregistered| register_labeled_call_arg(bump, unregistered))
                .collect_in(bump);
            NonEmptyCallArgVec::UniquelyLabeled(values)
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

pub fn register_fun(bump: &Bump, unregistered: heavy::Fun) -> &Fun<'_> {
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

pub fn register_match(bump: &Bump, unregistered: heavy::Match) -> &Match<'_> {
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

pub fn register_match_case(bump: &Bump, unregistered: heavy::MatchCase) -> &MatchCase<'_> {
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
            let id = register_identifiers(bump, unregistered);
            NonEmptyMatchCaseParamVec::Unlabeled(id)
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
        .map(|unregistered| register_identifier(bump, unregistered));
    registry.add_list(identifiers)
}

pub fn register_optional_labeled_match_case_params(
    bump: &Bump,
    unregistered: Option<Vec<heavy::LabeledMatchCaseParam>>,
) -> Option<NonEmptyListId<&'a LabeledMatchCaseParam<'a>>> {
    unregistered.map(|unregistered| register_labeled_match_case_params(bump, unregistered))
}

pub fn register_labeled_match_case_params(
    bump: &Bump,
    unregistered: Vec<heavy::LabeledMatchCaseParam>,
) -> NonEmptyListId<&'a LabeledMatchCaseParam<'a>> {
    let params = unregistered
        .into_iter()
        .map(|unregistered| register_labeled_match_case_param(bump, unregistered));
    registry.add_list(params)
}

pub fn register_labeled_match_case_param(
    bump: &Bump,
    unregistered: heavy::LabeledMatchCaseParam,
) -> &'a LabeledMatchCaseParam<'a> {
    let label = register_param_label(bump, unregistered.label_clause);
    let name = register_identifier(bump, unregistered.name);
    bump.alloc(LabeledMatchCaseParam {
        span: unregistered.span,
        label,
        name,
    })
}

pub fn register_match_case_output(
    bump: &Bump,
    unregistered: heavy::MatchCaseOutput,
) -> MatchCaseOutputId {
    match unregistered {
        heavy::MatchCaseOutput::Some(unregistered) => {
            let light = register_expression(bump, unregistered);
            MatchCaseOutputId::Some(light)
        }
        heavy::MatchCaseOutput::ImpossibilityClaim(kw_span) => {
            MatchCaseOutputId::ImpossibilityClaim(kw_span)
        }
    }
}

pub fn register_forall(bump: &Bump, unregistered: heavy::Forall) -> &'a Forall<'a> {
    let params = register_params(bump, unregistered.params);
    let output = register_expression(bump, unregistered.output);
    bump.alloc(Forall {
        span: unregistered.span,
        params,
        output,
    })
}

pub fn register_check(bump: &Bump, unregistered: heavy::Check) -> &'a Check<'a> {
    let assertions = unregistered
        .assertions
        .into_iter()
        .map(|unregistered| register_check_assertion(bump, unregistered));
    let assertion_list = registry.add_list(assertions);
    let output = register_expression(bump, unregistered.output);
    bump.alloc(Check {
        span: unregistered.span,
        assertion_list,
        output,
    })
}

pub fn register_check_assertion(
    bump: &Bump,
    unregistered: heavy::CheckAssertion,
) -> &'a CheckAssertion<'a> {
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
) -> GoalKwOrPossiblyInvalidExpressionId {
    match unregistered {
        heavy::GoalKwOrPossiblyInvalidExpression::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start }
        }
        heavy::GoalKwOrPossiblyInvalidExpression::Expression(unregistered) => {
            let id = register_possibly_invalid_expression(bump, unregistered);
            GoalKwOrPossiblyInvalidExpressionId::Expression(id)
        }
    }
}

pub fn register_question_mark_or_possibly_invalid_expression(
    bump: &Bump,
    unregistered: heavy::QuestionMarkOrPossiblyInvalidExpression,
) -> QuestionMarkOrPossiblyInvalidExpressionId {
    match unregistered {
        heavy::QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start }
        }
        heavy::QuestionMarkOrPossiblyInvalidExpression::Expression(unregistered) => {
            let id = register_possibly_invalid_expression(bump, unregistered);
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(id)
        }
    }
}

pub fn register_possibly_invalid_expression(
    bump: &Bump,
    unregistered: heavy::PossiblyInvalidExpression,
) -> PossiblyInvalidExpressionId {
    match unregistered {
        heavy::PossiblyInvalidExpression::Valid(unregistered) => {
            let id = register_expression(bump, unregistered);
            PossiblyInvalidExpressionId::Valid(id)
        }
        heavy::PossiblyInvalidExpression::Invalid(invalid) => {
            let id = register_invalid_expression(bump, invalid);
            PossiblyInvalidExpressionId::Invalid(id)
        }
    }
}

pub fn register_invalid_expression(
    bump: &Bump,
    unregistered: heavy::InvalidExpression,
) -> InvalidExpressionId {
    match unregistered {
        heavy::InvalidExpression::SymbolicallyInvalid(id) => {
            let id = register_symbolically_invalid_expression(bump, id);
            InvalidExpressionId::SymbolicallyInvalid(id)
        }
        heavy::InvalidExpression::IllegalFunRecursion(id) => {
            let id = register_illegal_fun_recursion_expression(bump, id);
            InvalidExpressionId::IllegalFunRecursion(id)
        }
    }
}

pub fn register_symbolically_invalid_expression(
    bump: &Bump,
    unregistered: heavy::SymbolicallyInvalidExpression,
) -> &'a SymbolicallyInvalidExpression<'a> {
    bump.alloc(SymbolicallyInvalidExpression {
        expression: unregistered.expression,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}

pub fn register_illegal_fun_recursion_expression(
    bump: &Bump,
    unregistered: heavy::IllegalFunRecursionExpression,
) -> &'a IllegalFunRecursionExpression<'a> {
    let expression = register_expression(bump, unregistered.expression);
    bump.alloc(IllegalFunRecursionExpression {
        expression,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
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
