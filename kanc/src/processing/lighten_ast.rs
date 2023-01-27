use crate::data::{bound_ast as heavy, light_ast::*, text_span::*};

fn dummy<T>() -> &'a T<'a> {
    NodeId::new(0)
}

pub fn register_file_items(
    registry: &mut NodeRegistry,
    unregistered: Vec<heavy::FileItem>,
) -> Option<NonEmptyListId<FileItemNodeId>> {
    let item_ids: Vec<_> = unregistered
        .into_iter()
        .map(|unregistered| register_file_item(registry, unregistered))
        .collect();
    registry.add_possibly_empty_list(item_ids)
}

pub fn register_file_item(
    registry: &mut NodeRegistry,
    unregistered: heavy::FileItem,
) -> FileItemNodeId {
    match unregistered {
        heavy::FileItem::Type(unregistered) => {
            FileItemNodeId::Type(register_type_statement(registry, unregistered))
        }
        heavy::FileItem::Let(unregistered) => {
            FileItemNodeId::Let(register_let_statement(registry, unregistered))
        }
    }
}

pub fn register_type_statement(
    registry: &mut NodeRegistry,
    unregistered: heavy::TypeStatement,
) -> &'a TypeStatement<'a> {
    let name = register_identifier(registry, unregistered.name);
    let params = register_optional_params(registry, unregistered.params);
    let variant_ids: Vec<_> = unregistered
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(registry, unregistered_variant))
        .collect();
    let variants = registry.add_possibly_empty_list(variant_ids);
    registry.add_and_overwrite(TypeStatement {
        span: unregistered.span,
        visibility: unregistered.visibility,
        name,
        params,
        variants,
    })
}

pub fn register_identifier(
    registry: &mut NodeRegistry,
    unregistered: heavy::Identifier,
) -> &'a Identifier<'a> {
    registry.add_and_overwrite(Identifier {
        span: unregistered.span,
        name: unregistered.name,
    })
}

pub fn register_optional_params(
    registry: &mut NodeRegistry,
    unregistered: Option<heavy::NonEmptyParamVec>,
) -> Option<NonEmptyParamListId> {
    unregistered.map(|unregistered| register_params(registry, unregistered))
}

pub fn register_params(
    registry: &mut NodeRegistry,
    unregistered: heavy::NonEmptyParamVec,
) -> NonEmptyParamListId {
    match unregistered {
        heavy::NonEmptyParamVec::Unlabeled(unregistered) => {
            let param_ids = unregistered
                .into_mapped(|unregistered| register_unlabeled_param(registry, unregistered));
            let params = registry.add_list(param_ids);
            NonEmptyParamListId::Unlabeled(params)
        }
        heavy::NonEmptyParamVec::UniquelyLabeled(unregistered) => {
            let param_ids = unregistered
                .into_mapped(|unregistered| register_labeled_param(registry, unregistered));
            let params = registry.add_list(param_ids);
            NonEmptyParamListId::UniquelyLabeled(params)
        }
    }
}

pub fn register_unlabeled_param(
    registry: &mut NodeRegistry,
    unregistered: heavy::UnlabeledParam,
) -> &'a UnlabeledParam<'a> {
    let name = register_identifier(registry, unregistered.name);
    let type_ = register_expression(registry, unregistered.type_);
    registry.add_and_overwrite(UnlabeledParam {
        span: unregistered.span,
        is_dashed: unregistered.is_dashed,
        name,
        type_,
    })
}

pub fn register_labeled_param(
    registry: &mut NodeRegistry,
    unregistered: heavy::LabeledParam,
) -> &'a LabeledParam<'a> {
    let label = register_param_label(registry, unregistered.label);
    let name = register_identifier(registry, unregistered.name);
    let type_ = register_expression(registry, unregistered.type_);
    registry.add_and_overwrite(LabeledParam {
        span: unregistered.span,
        label_clause: label,
        is_dashed: unregistered.is_dashed,
        name,
        type_,
    })
}

pub fn register_param_label(
    registry: &mut NodeRegistry,
    unregistered: heavy::ParamLabel,
) -> ParamLabelId {
    match unregistered {
        heavy::ParamLabel::Implicit => ParamLabelId::Implicit,
        heavy::ParamLabel::Explicit(unregistered) => {
            ParamLabelId::Explicit(register_identifier(registry, unregistered))
        }
    }
}

pub fn register_variant(
    registry: &mut NodeRegistry,
    unregistered: heavy::Variant,
) -> &'a Variant<'a> {
    let name = register_identifier(registry, unregistered.name);
    let params = register_optional_params(registry, unregistered.params);
    let return_type = register_expression(registry, unregistered.return_type);
    registry.add_and_overwrite(Variant {
        span: unregistered.span,
        name,
        params,
        return_type,
    })
}

pub fn register_let_statement(
    registry: &mut NodeRegistry,
    unregistered: heavy::LetStatement,
) -> &'a LetStatement<'a> {
    let name = register_identifier(registry, unregistered.name);
    let value = register_expression(registry, unregistered.value);
    registry.add_and_overwrite(LetStatement {
        span: unregistered.span,
        visibility: unregistered.visibility,
        transparency: unregistered.transparency,
        name,
        value,
    })
}

pub fn register_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::Expression,
) -> ExpressionRef<'a> {
    match unregistered {
        heavy::Expression::Name(unregistered) => {
            let id = register_name_expression(registry, unregistered);
            ExpressionRef::Name(id)
        }
        heavy::Expression::Todo(span) => {
            let id = register_todo_expression(registry, span);
            ExpressionRef::Todo(id)
        }
        heavy::Expression::Call(unregistered) => {
            let id = register_call(registry, *unregistered);
            ExpressionRef::Call(id)
        }
        heavy::Expression::Fun(unregistered) => {
            let id = register_fun(registry, *unregistered);
            ExpressionRef::Fun(id)
        }
        heavy::Expression::Match(unregistered) => {
            let id = register_match(registry, *unregistered);
            ExpressionRef::Match(id)
        }
        heavy::Expression::Forall(unregistered) => {
            let id = register_forall(registry, *unregistered);
            ExpressionRef::Forall(id)
        }
        heavy::Expression::Check(unregistered) => {
            let id = register_check(registry, *unregistered);
            ExpressionRef::Check(id)
        }
    }
}

pub fn register_name_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::NameExpression,
) -> &'a NameExpression<'a> {
    let component_ids = unregistered
        .components
        .into_mapped(|unregistered| register_identifier(registry, unregistered));
    let components = registry.add_list(component_ids);
    registry.add_and_overwrite(NameExpression {
        span: unregistered.span,
        components,
        db_index: unregistered.db_index,
    })
}

pub fn register_todo_expression(
    registry: &mut NodeRegistry,
    span: Option<TextSpan>,
) -> &'a TodoExpression<'a> {
    registry.add_and_overwrite(TodoExpression { span })
}

pub fn register_call(registry: &mut NodeRegistry, unregistered: heavy::Call) -> &'a Call<'a> {
    let callee = register_expression(registry, unregistered.callee);
    let args = register_call_args(registry, unregistered.args);
    registry.add_and_overwrite(Call {
        span: unregistered.span,
        callee,
        args,
    })
}

pub fn register_call_args(
    registry: &mut NodeRegistry,
    unregistered: heavy::NonEmptyCallArgVec,
) -> NonEmptyCallArgListId {
    match unregistered {
        heavy::NonEmptyCallArgVec::Unlabeled(unregistered) => {
            let value_ids = unregistered
                .into_mapped(|unregistered| register_expression(registry, unregistered));
            NonEmptyCallArgListId::Unlabeled(registry.add_list(value_ids))
        }
        heavy::NonEmptyCallArgVec::UniquelyLabeled(unregistered) => {
            let value_ids = unregistered
                .into_mapped(|unregistered| register_labeled_call_arg(registry, unregistered));
            NonEmptyCallArgListId::UniquelyLabeled(registry.add_list(value_ids))
        }
    }
}

pub fn register_labeled_call_arg(
    registry: &mut NodeRegistry,
    unregistered: heavy::LabeledCallArg,
) -> LabeledCallArgId {
    match unregistered {
        heavy::LabeledCallArg::Implicit {
            label: value,
            db_index,
        } => {
            let label = register_identifier(registry, value);
            LabeledCallArgId::implicit(label, db_index, registry)
        }
        heavy::LabeledCallArg::Explicit { label, value } => LabeledCallArgId::Explicit {
            label: register_identifier(registry, label),
            value: register_expression(registry, value),
        },
    }
}

pub fn register_fun(registry: &mut NodeRegistry, unregistered: heavy::Fun) -> &'a Fun<'a> {
    let name = register_identifier(registry, unregistered.name);
    let params = register_params(registry, unregistered.params);
    let return_type = register_expression(registry, unregistered.return_type);
    let body = register_expression(registry, unregistered.body);
    registry.add_and_overwrite(Fun {
        span: unregistered.span,
        name,
        params,
        return_type,
        body,
    })
}

pub fn register_match(registry: &mut NodeRegistry, unregistered: heavy::Match) -> &'a Match<'a> {
    let matchee = register_expression(registry, unregistered.matchee);
    let case_ids: Vec<_> = unregistered
        .cases
        .into_iter()
        .map(|unregistered| register_match_case(registry, unregistered))
        .collect();
    let cases = registry.add_possibly_empty_list(case_ids);
    registry.add_and_overwrite(Match {
        span: unregistered.span,
        matchee,
        cases,
    })
}

pub fn register_match_case(
    registry: &mut NodeRegistry,
    unregistered: heavy::MatchCase,
) -> &'a MatchCase<'a> {
    let variant_name = register_identifier(registry, unregistered.variant_name);
    let params = register_optional_match_case_params(registry, unregistered.params);
    let output = register_match_case_output(registry, unregistered.output);
    registry.add_and_overwrite(MatchCase {
        span: unregistered.span,
        variant_name,
        params,
        output,
    })
}

pub fn register_optional_match_case_params(
    registry: &mut NodeRegistry,
    unregistered: Option<heavy::NonEmptyMatchCaseParamVec>,
) -> Option<NonEmptyMatchCaseParamListId> {
    unregistered.map(|unregistered| register_match_case_params(registry, unregistered))
}

pub fn register_match_case_params(
    registry: &mut NodeRegistry,
    unregistered: heavy::NonEmptyMatchCaseParamVec,
) -> NonEmptyMatchCaseParamListId {
    match unregistered {
        heavy::NonEmptyMatchCaseParamVec::Unlabeled(unregistered) => {
            let id = register_identifiers(registry, unregistered);
            NonEmptyMatchCaseParamListId::Unlabeled(id)
        }
        heavy::NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot } => {
            let params = register_optional_labeled_match_case_params(registry, params);
            NonEmptyMatchCaseParamListId::UniquelyLabeled { params, triple_dot }
        }
    }
}

pub fn register_identifiers(
    registry: &mut NodeRegistry,
    unregistered: NonEmptyVec<heavy::Identifier>,
) -> NonEmptyListId<&'a Identifier<'a>> {
    let ids = unregistered.into_mapped(|unregistered| register_identifier(registry, unregistered));
    registry.add_list(ids)
}

pub fn register_optional_labeled_match_case_params(
    registry: &mut NodeRegistry,
    unregistered: Option<NonEmptyVec<heavy::LabeledMatchCaseParam>>,
) -> Option<NonEmptyListId<&'a LabeledMatchCaseParam<'a>>> {
    unregistered.map(|unregistered| register_labeled_match_case_params(registry, unregistered))
}

pub fn register_labeled_match_case_params(
    registry: &mut NodeRegistry,
    unregistered: NonEmptyVec<heavy::LabeledMatchCaseParam>,
) -> NonEmptyListId<&'a LabeledMatchCaseParam<'a>> {
    let ids = unregistered
        .into_mapped(|unregistered| register_labeled_match_case_param(registry, unregistered));
    registry.add_list(ids)
}

pub fn register_labeled_match_case_param(
    registry: &mut NodeRegistry,
    unregistered: heavy::LabeledMatchCaseParam,
) -> &'a LabeledMatchCaseParam<'a> {
    let label = register_param_label(registry, unregistered.label);
    let name = register_identifier(registry, unregistered.name);
    registry.add_and_overwrite(LabeledMatchCaseParam {
        span: unregistered.span,
        label,
        name,
    })
}

pub fn register_match_case_output(
    registry: &mut NodeRegistry,
    unregistered: heavy::MatchCaseOutput,
) -> MatchCaseOutputId {
    match unregistered {
        heavy::MatchCaseOutput::Some(unregistered) => {
            let id = register_expression(registry, unregistered);
            MatchCaseOutputId::Some(id)
        }
        heavy::MatchCaseOutput::ImpossibilityClaim(kw_span) => {
            MatchCaseOutputId::ImpossibilityClaim(kw_span)
        }
    }
}

pub fn register_forall(registry: &mut NodeRegistry, unregistered: heavy::Forall) -> &'a Forall<'a> {
    let params = register_params(registry, unregistered.params);
    let output = register_expression(registry, unregistered.output);
    registry.add_and_overwrite(Forall {
        span: unregistered.span,
        params,
        output,
    })
}

pub fn register_check(registry: &mut NodeRegistry, unregistered: heavy::Check) -> &'a Check<'a> {
    let assertion_ids = unregistered
        .assertions
        .into_mapped(|unregistered| register_check_assertion(registry, unregistered));
    let assertion_list = registry.add_list(assertion_ids);
    let output = register_expression(registry, unregistered.output);
    registry.add_and_overwrite(Check {
        span: unregistered.span,
        assertion_list,
        output,
    })
}

pub fn register_check_assertion(
    registry: &mut NodeRegistry,
    unregistered: heavy::CheckAssertion,
) -> &'a CheckAssertion<'a> {
    let left = register_goal_kw_or_expression(registry, unregistered.left);
    let right = register_question_mark_or_possibly_invalid_expression(registry, unregistered.right);
    registry.add_and_overwrite(CheckAssertion {
        span: unregistered.span,
        kind: unregistered.kind,
        left,
        right,
    })
}

pub fn register_goal_kw_or_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::GoalKwOrPossiblyInvalidExpression,
) -> GoalKwOrPossiblyInvalidExpressionId {
    match unregistered {
        heavy::GoalKwOrPossiblyInvalidExpression::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start }
        }
        heavy::GoalKwOrPossiblyInvalidExpression::Expression(unregistered) => {
            let id = register_possibly_invalid_expression(registry, unregistered);
            GoalKwOrPossiblyInvalidExpressionId::Expression(id)
        }
    }
}

pub fn register_question_mark_or_possibly_invalid_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::QuestionMarkOrPossiblyInvalidExpression,
) -> QuestionMarkOrPossiblyInvalidExpressionId {
    match unregistered {
        heavy::QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start }
        }
        heavy::QuestionMarkOrPossiblyInvalidExpression::Expression(unregistered) => {
            let id = register_possibly_invalid_expression(registry, unregistered);
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(id)
        }
    }
}

pub fn register_possibly_invalid_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::PossiblyInvalidExpression,
) -> PossiblyInvalidExpressionId {
    match unregistered {
        heavy::PossiblyInvalidExpression::Valid(unregistered) => {
            let id = register_expression(registry, unregistered);
            PossiblyInvalidExpressionId::Valid(id)
        }
        heavy::PossiblyInvalidExpression::Invalid(invalid) => {
            let id = register_invalid_expression(registry, invalid);
            PossiblyInvalidExpressionId::Invalid(id)
        }
    }
}

pub fn register_invalid_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::InvalidExpression,
) -> InvalidExpressionId {
    match unregistered {
        heavy::InvalidExpression::SymbolicallyInvalid(id) => {
            let id = register_symbolically_invalid_expression(registry, id);
            InvalidExpressionId::SymbolicallyInvalid(id)
        }
        heavy::InvalidExpression::IllegalFunRecursion(id) => {
            let id = register_illegal_fun_recursion_expression(registry, id);
            InvalidExpressionId::IllegalFunRecursion(id)
        }
    }
}

pub fn register_symbolically_invalid_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::SymbolicallyInvalidExpression,
) -> &'a SymbolicallyInvalidExpression<'a> {
    registry.add_and_overwrite(SymbolicallyInvalidExpression {
        expression: unregistered.expression,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}

pub fn register_illegal_fun_recursion_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::IllegalFunRecursionExpression,
) -> &'a IllegalFunRecursionExpression<'a> {
    let expression = register_expression(registry, unregistered.expression);
    registry.add_and_overwrite(IllegalFunRecursionExpression {
        expression,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}
