use crate::data::{
    bound_ast as heavy,
    light_ast::*,
    node_registry::{LabeledCallArgId, NodeId, NodeRegistry, NonEmptyListId},
    non_empty_veclike::NonEmptyVec,
    text_span::*,
};

fn dummy_id<T>() -> &'a T<'a> {
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
    let name_id = register_identifier(registry, unregistered.name);
    let param_list_id = register_optional_params(registry, unregistered.params);
    let variant_ids: Vec<_> = unregistered
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(registry, unregistered_variant))
        .collect();
    let variant_list_id = registry.add_possibly_empty_list(variant_ids);
    registry.add_and_overwrite_id(TypeStatement {
        id: dummy_id(),
        span: unregistered.span,
        visibility: unregistered.visibility,
        name_id,
        param_list_id,
        variant_list_id,
    })
}

pub fn register_identifier(
    registry: &mut NodeRegistry,
    unregistered: heavy::Identifier,
) -> &'a Identifier<'a> {
    registry.add_and_overwrite_id(Identifier {
        id: dummy_id(),
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
            let param_list_id = registry.add_list(param_ids);
            NonEmptyParamListId::Unlabeled(param_list_id)
        }
        heavy::NonEmptyParamVec::UniquelyLabeled(unregistered) => {
            let param_ids = unregistered
                .into_mapped(|unregistered| register_labeled_param(registry, unregistered));
            let param_list_id = registry.add_list(param_ids);
            NonEmptyParamListId::UniquelyLabeled(param_list_id)
        }
    }
}

pub fn register_unlabeled_param(
    registry: &mut NodeRegistry,
    unregistered: heavy::UnlabeledParam,
) -> &'a UnlabeledParam<'a> {
    let name_id = register_identifier(registry, unregistered.name);
    let type_id = register_expression(registry, unregistered.type_);
    registry.add_and_overwrite_id(UnlabeledParam {
        id: dummy_id(),
        span: unregistered.span,
        is_dashed: unregistered.is_dashed,
        name_id,
        type_id,
    })
}

pub fn register_labeled_param(
    registry: &mut NodeRegistry,
    unregistered: heavy::LabeledParam,
) -> &'a LabeledParam<'a> {
    let label_id = register_param_label(registry, unregistered.label);
    let name_id = register_identifier(registry, unregistered.name);
    let type_id = register_expression(registry, unregistered.type_);
    registry.add_and_overwrite_id(LabeledParam {
        id: dummy_id(),
        span: unregistered.span,
        label_clause: label_id,
        is_dashed: unregistered.is_dashed,
        name_id,
        type_id,
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
    let name_id = register_identifier(registry, unregistered.name);
    let param_list_id = register_optional_params(registry, unregistered.params);
    let return_type_id = register_expression(registry, unregistered.return_type);
    registry.add_and_overwrite_id(Variant {
        id: dummy_id(),
        span: unregistered.span,
        name_id,
        param_list_id,
        return_type_id,
    })
}

pub fn register_let_statement(
    registry: &mut NodeRegistry,
    unregistered: heavy::LetStatement,
) -> &'a LetStatement<'a> {
    let name_id = register_identifier(registry, unregistered.name);
    let value_id = register_expression(registry, unregistered.value);
    registry.add_and_overwrite_id(LetStatement {
        id: dummy_id(),
        span: unregistered.span,
        visibility: unregistered.visibility,
        transparency: unregistered.transparency,
        name_id,
        value_id,
    })
}

pub fn register_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::Expression,
) -> ExpressionRef<'a> {
    match unregistered {
        heavy::Expression::Name(unregistered) => {
            let id = register_name_expression(registry, unregistered);
            ExpressionRef<'a>::Name(id)
        }
        heavy::Expression::Todo(span) => {
            let id = register_todo_expression(registry, span);
            ExpressionRef<'a>::Todo(id)
        }
        heavy::Expression::Call(unregistered) => {
            let id = register_call(registry, *unregistered);
            ExpressionRef<'a>::Call(id)
        }
        heavy::Expression::Fun(unregistered) => {
            let id = register_fun(registry, *unregistered);
            ExpressionRef<'a>::Fun(id)
        }
        heavy::Expression::Match(unregistered) => {
            let id = register_match(registry, *unregistered);
            ExpressionRef<'a>::Match(id)
        }
        heavy::Expression::Forall(unregistered) => {
            let id = register_forall(registry, *unregistered);
            ExpressionRef<'a>::Forall(id)
        }
        heavy::Expression::Check(unregistered) => {
            let id = register_check(registry, *unregistered);
            ExpressionRef<'a>::Check(id)
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
    let component_list_id = registry.add_list(component_ids);
    registry.add_and_overwrite_id(NameExpression {
        id: dummy_id(),
        span: unregistered.span,
        component_list_id,
        db_index: unregistered.db_index,
    })
}

pub fn register_todo_expression(
    registry: &mut NodeRegistry,
    span: Option<TextSpan>,
) -> &'a TodoExpression<'a> {
    registry.add_and_overwrite_id(TodoExpression {
        id: dummy_id(),
        span,
    })
}

pub fn register_call(registry: &mut NodeRegistry, unregistered: heavy::Call) -> &'a Call<'a> {
    let callee_id = register_expression(registry, unregistered.callee);
    let arg_list_id = register_call_args(registry, unregistered.args);
    registry.add_and_overwrite_id(Call {
        id: dummy_id(),
        span: unregistered.span,
        callee_id,
        arg_list_id,
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
            let label_id = register_identifier(registry, value);
            LabeledCallArgId::implicit(label_id, db_index, registry)
        }
        heavy::LabeledCallArg::Explicit { label, value } => LabeledCallArgId::Explicit {
            label_id: register_identifier(registry, label),
            value_id: register_expression(registry, value),
        },
    }
}

pub fn register_fun(registry: &mut NodeRegistry, unregistered: heavy::Fun) -> &'a Fun<'a> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_list_id = register_params(registry, unregistered.params);
    let return_type_id = register_expression(registry, unregistered.return_type);
    let body_id = register_expression(registry, unregistered.body);
    registry.add_and_overwrite_id(Fun {
        id: dummy_id(),
        span: unregistered.span,
        name_id,
        param_list_id,
        return_type_id,
        body_id,
    })
}

pub fn register_match(registry: &mut NodeRegistry, unregistered: heavy::Match) -> &'a Match<'a> {
    let matchee_id = register_expression(registry, unregistered.matchee);
    let case_ids: Vec<_> = unregistered
        .cases
        .into_iter()
        .map(|unregistered| register_match_case(registry, unregistered))
        .collect();
    let case_list_id = registry.add_possibly_empty_list(case_ids);
    registry.add_and_overwrite_id(Match {
        id: dummy_id(),
        span: unregistered.span,
        matchee_id,
        case_list_id,
    })
}

pub fn register_match_case(
    registry: &mut NodeRegistry,
    unregistered: heavy::MatchCase,
) -> &'a MatchCase<'a> {
    let variant_name_id = register_identifier(registry, unregistered.variant_name);
    let param_list_id = register_optional_match_case_params(registry, unregistered.params);
    let output_id = register_match_case_output(registry, unregistered.output);
    registry.add_and_overwrite_id(MatchCase {
        id: dummy_id(),
        span: unregistered.span,
        variant_name_id,
        param_list_id,
        output_id,
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
            let param_list_id = register_optional_labeled_match_case_params(registry, params);
            NonEmptyMatchCaseParamListId::UniquelyLabeled {
                param_list_id,
                triple_dot,
            }
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
    let label_id = register_param_label(registry, unregistered.label);
    let name_id = register_identifier(registry, unregistered.name);
    registry.add_and_overwrite_id(LabeledMatchCaseParam {
        id: dummy_id(),
        span: unregistered.span,
        label_id,
        name_id,
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
    let param_list_id = register_params(registry, unregistered.params);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_and_overwrite_id(Forall {
        id: dummy_id(),
        span: unregistered.span,
        param_list_id,
        output_id,
    })
}

pub fn register_check(registry: &mut NodeRegistry, unregistered: heavy::Check) -> &'a Check<'a> {
    let assertion_ids = unregistered
        .assertions
        .into_mapped(|unregistered| register_check_assertion(registry, unregistered));
    let assertion_list_id = registry.add_list(assertion_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_and_overwrite_id(Check {
        id: dummy_id(),
        span: unregistered.span,
        assertion_list_id,
        output_id,
    })
}

pub fn register_check_assertion(
    registry: &mut NodeRegistry,
    unregistered: heavy::CheckAssertion,
) -> &'a CheckAssertion<'a> {
    let left_id = register_goal_kw_or_expression(registry, unregistered.left);
    let right_id =
        register_question_mark_or_possibly_invalid_expression(registry, unregistered.right);
    registry.add_and_overwrite_id(CheckAssertion {
        id: dummy_id(),
        span: unregistered.span,
        kind: unregistered.kind,
        left_id,
        right_id,
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
    registry.add_and_overwrite_id(SymbolicallyInvalidExpression {
        id: dummy_id(),
        expression: unregistered.expression,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}

pub fn register_illegal_fun_recursion_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::IllegalFunRecursionExpression,
) -> &'a IllegalFunRecursionExpression<'a> {
    let expression_id = register_expression(registry, unregistered.expression);
    registry.add_and_overwrite_id(IllegalFunRecursionExpression {
        id: dummy_id(),
        expression_id,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}
