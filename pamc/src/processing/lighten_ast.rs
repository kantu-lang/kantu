use crate::data::{
    bound_ast as heavy,
    light_ast::*,
    node_registry::{LabeledCallArgId, NodeId, NodeRegistry},
};

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

pub fn lighten_file(registry: &mut NodeRegistry, unregistered: heavy::File) -> NodeId<File> {
    let item_ids: Vec<_> = unregistered
        .items
        .into_iter()
        .map(|unregistered| register_file_item(registry, unregistered))
        .collect();
    let item_list_id = registry.add_possibly_empty_list(item_ids);
    registry.add(File {
        id: dummy_id(),
        span: unregistered.span,
        file_id: unregistered.id,
        item_list_id,
    })
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
) -> NodeId<TypeStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_list_id = register_optional_params(registry, unregistered.params);
    let variant_ids: Vec<_> = unregistered
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(registry, unregistered_variant))
        .collect();
    let variant_list_id = registry.add_possibly_empty_list(variant_ids);
    registry.add(TypeStatement {
        id: dummy_id(),
        span: unregistered.span,
        name_id,
        param_list_id,
        variant_list_id,
    })
}

pub fn register_identifier(
    registry: &mut NodeRegistry,
    unregistered: heavy::Identifier,
) -> NodeId<Identifier> {
    registry.add(Identifier {
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
) -> NodeId<UnlabeledParam> {
    let name_id = register_identifier(registry, unregistered.name);
    let type_id = register_expression(registry, unregistered.type_);
    registry.add(UnlabeledParam {
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
) -> NodeId<LabeledParam> {
    let label_id = register_label(registry, unregistered.label);
    let name_id = register_identifier(registry, unregistered.name);
    let type_id = register_expression(registry, unregistered.type_);
    registry.add(LabeledParam {
        id: dummy_id(),
        span: unregistered.span,
        label_id,
        is_dashed: unregistered.is_dashed,
        name_id,
        type_id,
    })
}

pub fn register_label(
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
) -> NodeId<Variant> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_list_id = register_optional_params(registry, unregistered.params);
    let return_type_id = register_expression(registry, unregistered.return_type);
    registry.add(Variant {
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
) -> NodeId<LetStatement> {
    let name_id = register_identifier(registry, unregistered.name);
    let value_id = register_expression(registry, unregistered.value);
    registry.add(LetStatement {
        id: dummy_id(),
        span: unregistered.span,
        name_id,
        value_id,
    })
}

pub fn register_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::Expression,
) -> ExpressionId {
    match unregistered {
        heavy::Expression::Name(unregistered) => {
            let id = register_name_expression(registry, unregistered);
            ExpressionId::Name(id)
        }
        heavy::Expression::Call(unregistered) => {
            let id = register_call(registry, *unregistered);
            ExpressionId::Call(id)
        }
        heavy::Expression::Fun(unregistered) => {
            let id = register_fun(registry, *unregistered);
            ExpressionId::Fun(id)
        }
        heavy::Expression::Match(unregistered) => {
            let id = register_match(registry, *unregistered);
            ExpressionId::Match(id)
        }
        heavy::Expression::Forall(unregistered) => {
            let id = register_forall(registry, *unregistered);
            ExpressionId::Forall(id)
        }
        heavy::Expression::Check(unregistered) => {
            let id = register_check(registry, *unregistered);
            ExpressionId::Check(id)
        }
    }
}

pub fn register_name_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::NameExpression,
) -> NodeId<NameExpression> {
    let component_ids = unregistered
        .components
        .into_mapped(|unregistered| register_identifier(registry, unregistered));
    let component_list_id = registry.add_list(component_ids);
    registry.add(NameExpression {
        id: dummy_id(),
        span: unregistered.span,
        component_list_id,
        db_index: unregistered.db_index,
    })
}

pub fn register_call(registry: &mut NodeRegistry, unregistered: heavy::Call) -> NodeId<Call> {
    let callee_id = register_expression(registry, unregistered.callee);
    let arg_list_id = register_call_args(registry, unregistered.args);
    registry.add(Call {
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
        } => LabeledCallArgId::Implicit {
            label_id: register_identifier(registry, value),
            db_index,
        },
        heavy::LabeledCallArg::Explicit { label, value } => LabeledCallArgId::Explicit {
            label_id: register_identifier(registry, label),
            value_id: register_expression(registry, value),
        },
    }
}

pub fn register_fun(registry: &mut NodeRegistry, unregistered: heavy::Fun) -> NodeId<Fun> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_list_id = register_params(registry, unregistered.params);
    let return_type_id = register_expression(registry, unregistered.return_type);
    let body_id = register_expression(registry, unregistered.body);
    let skip_type_checking_body = unregistered.skip_type_checking_body;
    registry.add(Fun {
        id: dummy_id(),
        span: unregistered.span,
        name_id,
        param_list_id,
        return_type_id,
        body_id,
        skip_type_checking_body,
    })
}

pub fn register_match(registry: &mut NodeRegistry, unregistered: heavy::Match) -> NodeId<Match> {
    let matchee_id = register_expression(registry, unregistered.matchee);
    let case_ids: Vec<_> = unregistered
        .cases
        .into_iter()
        .map(|unregistered| register_match_case(registry, unregistered))
        .collect();
    let case_list_id = registry.add_possibly_empty_list(case_ids);
    registry.add(Match {
        id: dummy_id(),
        span: unregistered.span,
        matchee_id,
        case_list_id,
    })
}

pub fn register_forall(registry: &mut NodeRegistry, unregistered: heavy::Forall) -> NodeId<Forall> {
    let param_list_id = register_params(registry, unregistered.params);
    let output_id = register_expression(registry, unregistered.output);
    registry.add(Forall {
        id: dummy_id(),
        span: unregistered.span,
        param_list_id,
        output_id,
    })
}

pub fn register_check(registry: &mut NodeRegistry, unregistered: heavy::Check) -> NodeId<Check> {
    let assertion_ids = unregistered
        .assertions
        .into_mapped(|unregistered| register_check_assertion(registry, unregistered));
    let assertion_list_id = registry.add_list(assertion_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add(Check {
        id: dummy_id(),
        span: unregistered.span,
        assertion_list_id,
        output_id,
    })
}

pub fn register_check_assertion(
    registry: &mut NodeRegistry,
    unregistered: heavy::CheckAssertion,
) -> NodeId<CheckAssertion> {
    let left_id = register_goal_kw_or_expression(registry, unregistered.left);
    let right_id =
        register_question_mark_or_possibly_invalid_expression(registry, unregistered.right);
    registry.add(CheckAssertion {
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
) -> NodeId<SymbolicallyInvalidExpression> {
    registry.add(SymbolicallyInvalidExpression {
        id: dummy_id(),
        expression: unregistered.expression,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}

pub fn register_illegal_fun_recursion_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::IllegalFunRecursionExpression,
) -> NodeId<IllegalFunRecursionExpression> {
    let expression_id = register_expression(registry, unregistered.expression);
    registry.add(IllegalFunRecursionExpression {
        id: dummy_id(),
        expression_id,
        error: unregistered.error,
        span_invalidated: unregistered.span_invalidated,
    })
}

pub fn register_match_case(
    registry: &mut NodeRegistry,
    unregistered: heavy::MatchCase,
) -> NodeId<MatchCase> {
    let variant_name_id = register_identifier(registry, unregistered.variant_name);
    let param_ids: Vec<_> = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_identifier(registry, unregistered))
        .collect();
    let param_list_id = registry.add_possibly_empty_list(param_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add(MatchCase {
        id: dummy_id(),
        span: unregistered.span,
        variant_name_id,
        param_list_id,
        output_id,
    })
}
