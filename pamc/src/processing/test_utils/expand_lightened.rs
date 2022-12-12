use crate::data::{
    bound_ast::*,
    light_ast as light,
    node_registry::{
        CheckAssertionId, FileItemNodeId, GoalKwOrExpressionId, InvalidExpressionId, ListId,
        NodeId, NodeRegistry, PossiblyInvalidExpressionId,
        QuestionMarkOrPossiblyInvalidExpressionId,
    },
};

pub fn expand_file(registry: &NodeRegistry, id: NodeId<light::File>) -> File {
    let light = registry.file(id);
    let items = expand_file_item_list(registry, light.item_list_id);
    File {
        id: light.file_id,
        span: light.span,
        items,
    }
}

pub fn expand_file_item_list(registry: &NodeRegistry, id: ListId<FileItemNodeId>) -> Vec<FileItem> {
    registry
        .file_item_list(id)
        .iter()
        .map(|item_id| expand_file_item(registry, *item_id))
        .collect()
}

pub fn expand_file_item(registry: &NodeRegistry, id: FileItemNodeId) -> FileItem {
    match id {
        light::FileItemNodeId::Type(id) => FileItem::Type(expand_type_statement(registry, id)),
        light::FileItemNodeId::Let(id) => FileItem::Let(expand_let_statement(registry, id)),
    }
}

pub fn expand_type_statement(
    registry: &NodeRegistry,
    id: NodeId<light::TypeStatement>,
) -> TypeStatement {
    let light = registry.type_statement(id);
    let name = expand_identifier(registry, light.name_id);
    let params = expand_param_list(registry, light.param_list_id);
    let variants = expand_variant_list(registry, light.variant_list_id);
    TypeStatement {
        span: light.span,
        name,
        params,
        variants,
    }
}

pub fn expand_identifier(registry: &NodeRegistry, id: NodeId<light::Identifier>) -> Identifier {
    let light = registry.identifier(id);
    Identifier {
        span: light.span,
        name: light.name.clone(),
    }
}

pub fn expand_param_list(registry: &NodeRegistry, id: ListId<NodeId<light::Param>>) -> Vec<Param> {
    registry
        .param_list(id)
        .iter()
        .map(|param_id| expand_param(registry, *param_id))
        .collect()
}

pub fn expand_param(registry: &NodeRegistry, id: NodeId<light::Param>) -> Param {
    let light = registry.param(id);
    let name = expand_identifier(registry, light.name_id);
    let type_ = expand_expression(registry, light.type_id);
    Param {
        span: light.span,
        is_dashed: light.is_dashed,
        name,
        type_,
    }
}

pub fn expand_variant_list(
    registry: &NodeRegistry,
    id: ListId<NodeId<light::Variant>>,
) -> Vec<Variant> {
    registry
        .variant_list(id)
        .iter()
        .map(|variant_id| expand_variant(registry, *variant_id))
        .collect()
}

pub fn expand_variant(registry: &NodeRegistry, id: NodeId<light::Variant>) -> Variant {
    let light = registry.variant(id);
    let name = expand_identifier(registry, light.name_id);
    let params = expand_param_list(registry, light.param_list_id);
    let return_type = expand_expression(registry, light.return_type_id);
    Variant {
        span: light.span,
        name,
        params,
        return_type,
    }
}

pub fn expand_let_statement(
    registry: &NodeRegistry,
    id: NodeId<light::LetStatement>,
) -> LetStatement {
    let light = registry.let_statement(id);
    let name = expand_identifier(registry, light.name_id);
    let value = expand_expression(registry, light.value_id);
    LetStatement {
        span: light.span,
        name,
        value,
    }
}

pub fn expand_expression(registry: &NodeRegistry, id: light::ExpressionId) -> Expression {
    match id {
        light::ExpressionId::Name(id) => Expression::Name(expand_name_expression(registry, id)),
        light::ExpressionId::Call(id) => Expression::Call(Box::new(expand_call(registry, id))),
        light::ExpressionId::Fun(id) => Expression::Fun(Box::new(expand_fun(registry, id))),
        light::ExpressionId::Match(id) => Expression::Match(Box::new(expand_match(registry, id))),
        light::ExpressionId::Forall(id) => {
            Expression::Forall(Box::new(expand_forall(registry, id)))
        }
        light::ExpressionId::Check(id) => Expression::Check(Box::new(expand_check(registry, id))),
    }
}

pub fn expand_name_expression(
    registry: &NodeRegistry,
    id: NodeId<light::NameExpression>,
) -> NameExpression {
    let light = registry.name_expression(id);
    let components = expand_identifier_list(registry, light.component_list_id);
    NameExpression {
        span: light.span,
        components,
        db_index: light.db_index,
    }
}

pub fn expand_identifier_list(
    registry: &NodeRegistry,
    id: ListId<NodeId<light::Identifier>>,
) -> Vec<Identifier> {
    registry
        .identifier_list(id)
        .iter()
        .map(|id| expand_identifier(registry, *id))
        .collect()
}

pub fn expand_call(registry: &NodeRegistry, id: NodeId<light::Call>) -> Call {
    let light = registry.call(id);
    let callee = expand_expression(registry, light.callee_id);
    let args = expand_expression_list(registry, light.arg_list_id);
    Call {
        span: light.span,
        callee,
        args,
    }
}

pub fn expand_expression_list(
    registry: &NodeRegistry,
    id: ListId<light::ExpressionId>,
) -> Vec<Expression> {
    registry
        .expression_list(id)
        .iter()
        .map(|id| expand_expression(registry, *id))
        .collect()
}

pub fn expand_fun(registry: &NodeRegistry, id: NodeId<light::Fun>) -> Fun {
    let light = registry.fun(id);
    let name = expand_identifier(registry, light.name_id);
    let params = expand_param_list(registry, light.param_list_id);
    let return_type = expand_expression(registry, light.return_type_id);
    let body = expand_expression(registry, light.body_id);
    let skip_type_checking_body = light.skip_type_checking_body;
    Fun {
        span: light.span,
        name,
        params,
        return_type,
        body,
        skip_type_checking_body,
    }
}

pub fn expand_match(registry: &NodeRegistry, id: NodeId<light::Match>) -> Match {
    let light = registry.match_(id);
    let matchee = expand_expression(registry, light.matchee_id);
    let cases = expand_match_case_list(registry, light.case_list_id);
    Match {
        span: light.span,
        matchee,
        cases,
    }
}

pub fn expand_match_case_list(
    registry: &NodeRegistry,
    id: ListId<NodeId<light::MatchCase>>,
) -> Vec<MatchCase> {
    registry
        .match_case_list(id)
        .iter()
        .map(|case_id| expand_match_case(registry, *case_id))
        .collect()
}

pub fn expand_match_case(registry: &NodeRegistry, id: NodeId<light::MatchCase>) -> MatchCase {
    let light = registry.match_case(id);
    let variant_name = expand_identifier(registry, light.variant_name_id);
    let params = expand_identifier_list(registry, light.param_list_id);
    let output = expand_expression(registry, light.output_id);
    MatchCase {
        span: light.span,
        variant_name,
        params,
        output,
    }
}

pub fn expand_forall(registry: &NodeRegistry, id: NodeId<light::Forall>) -> Forall {
    let light = registry.forall(id);
    let params = expand_param_list(registry, light.param_list_id);
    let output = expand_expression(registry, light.output_id);
    Forall {
        span: light.span,
        params,
        output,
    }
}

pub fn expand_check(registry: &NodeRegistry, id: NodeId<light::Check>) -> Check {
    let light = registry.check(id);
    let assertions = expand_check_assertion_list(registry, light.assertion_list_id);
    let output = expand_expression(registry, light.output_id);
    Check {
        span: light.span,
        assertions,
        output,
    }
}

pub fn expand_check_assertion_list(
    registry: &NodeRegistry,
    id: ListId<CheckAssertionId>,
) -> Vec<CheckAssertion> {
    registry
        .check_assertion_list(id)
        .iter()
        .map(|id| expand_check_assertion(registry, *id))
        .collect()
}

pub fn expand_check_assertion(registry: &NodeRegistry, id: CheckAssertionId) -> CheckAssertion {
    match id {
        CheckAssertionId::Type(id) => CheckAssertion::Type(expand_type_annotation(registry, id)),
        CheckAssertionId::NormalForm(id) => {
            CheckAssertion::NormalForm(expand_normal_form_annotation(registry, id))
        }
    }
}

pub fn expand_type_annotation(
    registry: &NodeRegistry,
    id: NodeId<light::TypeAssertion>,
) -> TypeAssertion {
    let light = registry.type_assertion(id);
    let left = expand_expression(registry, light.left_id);
    let right = expand_question_mark_or_possibly_invalid_expression(registry, light.right_id);
    TypeAssertion {
        span: light.span,
        left,
        right,
    }
}

pub fn expand_normal_form_annotation(
    registry: &NodeRegistry,
    id: NodeId<light::NormalFormAssertion>,
) -> NormalFormAssertion {
    let light = registry.normal_form_assertion(id);
    let left = expand_goal_kw_or_expression(registry, light.left_id);
    let right = expand_question_mark_or_possibly_invalid_expression(registry, light.right_id);
    NormalFormAssertion {
        span: light.span,
        left,
        right,
    }
}

pub fn expand_goal_kw_or_expression(
    registry: &NodeRegistry,
    id: GoalKwOrExpressionId,
) -> GoalKwOrExpression {
    match id {
        GoalKwOrExpressionId::GoalKw { span } => GoalKwOrExpression::GoalKw { span },
        GoalKwOrExpressionId::Expression(expression) => {
            GoalKwOrExpression::Expression(expand_expression(registry, expression))
        }
    }
}

pub fn expand_question_mark_or_possibly_invalid_expression(
    registry: &NodeRegistry,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
) -> QuestionMarkOrPossiblyInvalidExpression {
    match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpression::QuestionMark { span: start }
        }
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
            QuestionMarkOrPossiblyInvalidExpression::Expression(expand_possibly_invalid_expression(
                registry, id,
            ))
        }
    }
}

pub fn expand_possibly_invalid_expression(
    registry: &NodeRegistry,
    id: PossiblyInvalidExpressionId,
) -> PossiblyInvalidExpression {
    match id {
        PossiblyInvalidExpressionId::Valid(id) => {
            PossiblyInvalidExpression::Valid(expand_expression(registry, id))
        }
        PossiblyInvalidExpressionId::Invalid(id) => {
            PossiblyInvalidExpression::Invalid(expand_invalid_expression(registry, id))
        }
    }
}

pub fn expand_invalid_expression(
    registry: &NodeRegistry,
    id: InvalidExpressionId,
) -> InvalidExpression {
    match id {
        InvalidExpressionId::SymbolicallyInvalid(id) => InvalidExpression::SymbolicallyInvalid(
            expand_symbolically_invalid_expression(registry, id),
        ),
        InvalidExpressionId::IllegalFunRecursion(id) => InvalidExpression::IllegalFunRecursion(
            expand_illegal_fun_recursion_expression(registry, id),
        ),
    }
}

pub fn expand_symbolically_invalid_expression(
    registry: &NodeRegistry,
    id: NodeId<light::SymbolicallyInvalidExpression>,
) -> SymbolicallyInvalidExpression {
    let light = registry.symbolically_invalid_expression(id);
    let expression = light.expression.clone();
    let error = light.error.clone();
    SymbolicallyInvalidExpression {
        span: light.span,
        expression,
        error,
    }
}

pub fn expand_illegal_fun_recursion_expression(
    registry: &NodeRegistry,
    id: NodeId<light::IllegalFunRecursionExpression>,
) -> IllegalFunRecursionExpression {
    let light = registry.illegal_fun_recursion_expression(id);
    let expression = expand_expression(registry, light.expression_id);
    let error = light.error.clone();
    IllegalFunRecursionExpression {
        span: light.span,
        expression,
        error,
    }
}
