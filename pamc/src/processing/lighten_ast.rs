use crate::data::{
    bound_ast as heavy,
    light_ast::*,
    node_registry::{NodeId, NodeRegistry},
};

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

pub fn lighten_file(registry: &mut NodeRegistry, unregistered: heavy::File) -> NodeId<File> {
    let item_ids = unregistered
        .items
        .into_iter()
        .map(|unregistered| register_file_item(registry, unregistered))
        .collect();
    let item_list_id = registry.add_file_item_list(item_ids);
    registry.add_file_and_overwrite_its_id(File {
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
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let variant_ids = unregistered
        .variants
        .into_iter()
        .map(|unregistered_variant| register_variant(registry, unregistered_variant))
        .collect();
    let variant_list_id = registry.add_variant_list(variant_ids);
    registry.add_type_statement_and_overwrite_its_id(TypeStatement {
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
    registry.add_identifier_and_overwrite_its_id(Identifier {
        id: dummy_id(),
        span: unregistered.span,
        name: unregistered.name,
    })
}

pub fn register_param(registry: &mut NodeRegistry, unregistered: heavy::Param) -> NodeId<Param> {
    let name_id = register_identifier(registry, unregistered.name);
    let type_id = register_expression(registry, unregistered.type_);
    registry.add_param_and_overwrite_its_id(Param {
        id: dummy_id(),
        span: unregistered.span,
        is_dashed: unregistered.is_dashed,
        name_id,
        type_id,
    })
}

pub fn register_variant(
    registry: &mut NodeRegistry,
    unregistered: heavy::Variant,
) -> NodeId<Variant> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let return_type_id = register_expression(registry, unregistered.return_type);
    registry.add_variant_and_overwrite_its_id(Variant {
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
    registry.add_let_statement_and_overwrite_its_id(LetStatement {
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
        .into_iter()
        .map(|unregistered| register_identifier(registry, unregistered))
        .collect();
    let component_list_id = registry.add_identifier_list(component_ids);
    registry.add_name_expression_and_overwrite_its_id(NameExpression {
        id: dummy_id(),
        span: unregistered.span,
        component_list_id,
        db_index: unregistered.db_index,
    })
}

pub fn register_call(registry: &mut NodeRegistry, unregistered: heavy::Call) -> NodeId<Call> {
    let callee_id = register_expression(registry, unregistered.callee);
    let arg_ids = unregistered
        .args
        .into_iter()
        .map(|unregistered| register_expression(registry, unregistered))
        .collect();
    let arg_list_id = registry.add_expression_list(arg_ids);
    registry.add_call_and_overwrite_its_id(Call {
        id: dummy_id(),
        span: unregistered.span,
        callee_id,
        arg_list_id,
    })
}

pub fn register_fun(registry: &mut NodeRegistry, unregistered: heavy::Fun) -> NodeId<Fun> {
    let name_id = register_identifier(registry, unregistered.name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let return_type_id = register_expression(registry, unregistered.return_type);
    let body_id = register_expression(registry, unregistered.body);
    let skip_type_checking_body = unregistered.skip_type_checking_body;
    registry.add_fun_and_overwrite_its_id(Fun {
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
    let case_ids = unregistered
        .cases
        .into_iter()
        .map(|unregistered| register_match_case(registry, unregistered))
        .collect();
    let case_list_id = registry.add_match_case_list(case_ids);
    registry.add_match_and_overwrite_its_id(Match {
        id: dummy_id(),
        span: unregistered.span,
        matchee_id,
        case_list_id,
    })
}

pub fn register_forall(registry: &mut NodeRegistry, unregistered: heavy::Forall) -> NodeId<Forall> {
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_param(registry, unregistered))
        .collect();
    let param_list_id = registry.add_param_list(param_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_forall_and_overwrite_its_id(Forall {
        id: dummy_id(),
        span: unregistered.span,
        param_list_id,
        output_id,
    })
}

pub fn register_check(registry: &mut NodeRegistry, unregistered: heavy::Check) -> NodeId<Check> {
    let assertion_ids = unregistered
        .assertions
        .into_iter()
        .map(|unregistered| register_check_assertion(registry, unregistered))
        .collect();
    let assertion_list_id = registry.add_check_assertion_list(assertion_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_check_and_overwrite_its_id(Check {
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
    registry.add_check_assertion_and_overwrite_its_id(CheckAssertion {
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
    registry.add_symbolically_invalid_expression_and_overwrite_its_id(
        SymbolicallyInvalidExpression {
            id: dummy_id(),
            expression: unregistered.expression,
            error: unregistered.error,
            span_invalidated: unregistered.span_invalidated,
        },
    )
}

pub fn register_illegal_fun_recursion_expression(
    registry: &mut NodeRegistry,
    unregistered: heavy::IllegalFunRecursionExpression,
) -> NodeId<IllegalFunRecursionExpression> {
    let expression_id = register_expression(registry, unregistered.expression);
    registry.add_illegal_fun_recursion_expression_and_overwrite_its_id(
        IllegalFunRecursionExpression {
            id: dummy_id(),
            expression_id,
            error: unregistered.error,
            span_invalidated: unregistered.span_invalidated,
        },
    )
}

pub fn register_match_case(
    registry: &mut NodeRegistry,
    unregistered: heavy::MatchCase,
) -> NodeId<MatchCase> {
    let variant_name_id = register_identifier(registry, unregistered.variant_name);
    let param_ids = unregistered
        .params
        .into_iter()
        .map(|unregistered| register_identifier(registry, unregistered))
        .collect();
    let param_list_id = registry.add_identifier_list(param_ids);
    let output_id = register_expression(registry, unregistered.output);
    registry.add_match_case_and_overwrite_its_id(MatchCase {
        id: dummy_id(),
        span: unregistered.span,
        variant_name_id,
        param_list_id,
        output_id,
    })
}
