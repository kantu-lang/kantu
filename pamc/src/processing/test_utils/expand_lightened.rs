use crate::data::{
    bound_ast::*,
    light_ast::{self as light, ParamLabelId},
    node_registry::{
        FileItemNodeId, GoalKwOrPossiblyInvalidExpressionId, InvalidExpressionId, LabeledCallArgId,
        NodeId, NodeRegistry, NonEmptyCallArgListId, NonEmptyListId, NonEmptyMatchCaseParamListId,
        NonEmptyParamListId, PossiblyInvalidExpressionId,
        QuestionMarkOrPossiblyInvalidExpressionId,
    },
    non_empty_vec::{NonEmptyVec, OptionalNonEmptyToPossiblyEmpty},
};

pub fn expand_file(registry: &NodeRegistry, id: NodeId<light::File>) -> File {
    let light = registry.get(id);
    let items = expand_file_item_list(registry, light.item_list_id);
    File {
        id: light.file_id,
        span: light.span,
        items,
    }
}

pub fn expand_file_item_list(
    registry: &NodeRegistry,
    id: Option<NonEmptyListId<FileItemNodeId>>,
) -> Vec<FileItem> {
    registry
        .get_possibly_empty_list(id)
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
    let light = registry.get(id);
    let name = expand_identifier(registry, light.name_id);
    let params = expand_optional_param_list(registry, light.param_list_id);
    let variants =
        expand_optional_variant_list(registry, light.variant_list_id).into_possibly_empty();
    TypeStatement {
        span: light.span,
        name,
        params,
        variants,
    }
}

pub fn expand_identifier(registry: &NodeRegistry, id: NodeId<light::Identifier>) -> Identifier {
    let light = registry.get(id);
    Identifier {
        span: light.span,
        name: light.name.clone(),
    }
}

pub fn expand_optional_param_list(
    registry: &NodeRegistry,
    id: Option<NonEmptyParamListId>,
) -> Option<NonEmptyParamVec> {
    id.map(|id| expand_param_list(registry, id))
}

pub fn expand_param_list(registry: &NodeRegistry, id: NonEmptyParamListId) -> NonEmptyParamVec {
    match id {
        NonEmptyParamListId::Unlabeled(id) => {
            NonEmptyParamVec::Unlabeled(expand_unlabeled_param_list(registry, id))
        }
        NonEmptyParamListId::UniquelyLabeled(id) => {
            NonEmptyParamVec::UniquelyLabeled(expand_labeled_param_list(registry, id))
        }
    }
}

pub fn expand_unlabeled_param_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<light::UnlabeledParam>>,
) -> NonEmptyVec<UnlabeledParam> {
    registry
        .get_list(id)
        .to_mapped(|param_id| expand_unlabeled_param(registry, *param_id))
}

pub fn expand_unlabeled_param(
    registry: &NodeRegistry,
    id: NodeId<light::UnlabeledParam>,
) -> UnlabeledParam {
    let light = registry.get(id);
    let name = expand_identifier(registry, light.name_id);
    let type_ = expand_expression(registry, light.type_id);
    UnlabeledParam {
        span: light.span,
        is_dashed: light.is_dashed,
        name,
        type_,
    }
}

pub fn expand_labeled_param_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<light::LabeledParam>>,
) -> NonEmptyVec<LabeledParam> {
    registry
        .get_list(id)
        .to_mapped(|param_id| expand_labeled_param(registry, *param_id))
}

pub fn expand_labeled_param(
    registry: &NodeRegistry,
    id: NodeId<light::LabeledParam>,
) -> LabeledParam {
    let light = registry.get(id);
    let label = expand_param_label(registry, light.label_id);
    let name = expand_identifier(registry, light.name_id);
    let type_ = expand_expression(registry, light.type_id);
    LabeledParam {
        span: light.span,
        label,
        is_dashed: light.is_dashed,
        name,
        type_,
    }
}

pub fn expand_param_label(registry: &NodeRegistry, id: ParamLabelId) -> ParamLabel {
    match id {
        ParamLabelId::Implicit => ParamLabel::Implicit,
        ParamLabelId::Explicit(id) => ParamLabel::Explicit(expand_identifier(registry, id)),
    }
}

pub fn expand_optional_variant_list(
    registry: &NodeRegistry,
    id: Option<NonEmptyListId<NodeId<light::Variant>>>,
) -> Option<NonEmptyVec<Variant>> {
    id.map(|id| expand_variant_list(registry, id))
}

pub fn expand_variant_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<light::Variant>>,
) -> NonEmptyVec<Variant> {
    registry
        .get_list(id)
        .to_mapped(|variant_id| expand_variant(registry, *variant_id))
}

pub fn expand_variant(registry: &NodeRegistry, id: NodeId<light::Variant>) -> Variant {
    let light = registry.get(id);
    let name = expand_identifier(registry, light.name_id);
    let params = expand_optional_param_list(registry, light.param_list_id);
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
    let light = registry.get(id);
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
    let light = registry.get(id);
    let components = expand_identifier_list(registry, light.component_list_id);
    NameExpression {
        span: light.span,
        components,
        db_index: light.db_index,
    }
}

pub fn expand_identifier_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<light::Identifier>>,
) -> NonEmptyVec<Identifier> {
    registry
        .get_list(id)
        .to_mapped(|id| expand_identifier(registry, *id))
}

pub fn expand_call(registry: &NodeRegistry, id: NodeId<light::Call>) -> Call {
    let light = registry.get(id);
    let callee = expand_expression(registry, light.callee_id);
    let args = expand_call_arg_list(registry, light.arg_list_id);
    Call {
        span: light.span,
        callee,
        args,
    }
}

pub fn expand_call_arg_list(
    registry: &NodeRegistry,
    id: NonEmptyCallArgListId,
) -> NonEmptyCallArgVec {
    match id {
        NonEmptyCallArgListId::Unlabeled(id) => {
            NonEmptyCallArgVec::Unlabeled(expand_expression_list(registry, id))
        }
        NonEmptyCallArgListId::UniquelyLabeled(id) => {
            NonEmptyCallArgVec::UniquelyLabeled(expand_labeled_call_arg_list(registry, id))
        }
    }
}

pub fn expand_labeled_call_arg_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<LabeledCallArgId>,
) -> NonEmptyVec<LabeledCallArg> {
    registry
        .get_list(id)
        .to_mapped(|id| expand_labeled_call_arg(registry, *id))
}

pub fn expand_labeled_call_arg(registry: &NodeRegistry, id: LabeledCallArgId) -> LabeledCallArg {
    match id {
        LabeledCallArgId::Implicit {
            label_id,
            db_index,
            value_id: _,
        } => LabeledCallArg::Implicit {
            label: expand_identifier(registry, label_id),
            db_index,
        },
        LabeledCallArgId::Explicit { label_id, value_id } => LabeledCallArg::Explicit {
            label: expand_identifier(registry, label_id),
            value: expand_expression(registry, value_id),
        },
    }
}

pub fn expand_expression_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<light::ExpressionId>,
) -> NonEmptyVec<Expression> {
    registry
        .get_list(id)
        .to_mapped(|id| expand_expression(registry, *id))
}

pub fn expand_fun(registry: &NodeRegistry, id: NodeId<light::Fun>) -> Fun {
    let light = registry.get(id);
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
    let light = registry.get(id);
    let matchee = expand_expression(registry, light.matchee_id);
    let cases = expand_optional_match_case_list(registry, light.case_list_id).into_possibly_empty();
    Match {
        span: light.span,
        matchee,
        cases,
    }
}

pub fn expand_optional_match_case_list(
    registry: &NodeRegistry,
    id: Option<NonEmptyListId<NodeId<light::MatchCase>>>,
) -> Option<NonEmptyVec<MatchCase>> {
    id.map(|id| expand_match_case_list(registry, id))
}

pub fn expand_match_case_list(
    registry: &NodeRegistry,
    id: NonEmptyListId<NodeId<light::MatchCase>>,
) -> NonEmptyVec<MatchCase> {
    registry
        .get_list(id)
        .to_mapped(|case_id| expand_match_case(registry, *case_id))
}

pub fn expand_match_case(registry: &NodeRegistry, id: NodeId<light::MatchCase>) -> MatchCase {
    let light = registry.get(id);
    let variant_name = expand_identifier(registry, light.variant_name_id);
    let params = expand_optional_match_case_param_list(registry, light.param_list_id);
    let output = expand_expression(registry, light.output_id);
    MatchCase {
        span: light.span,
        variant_name,
        params,
        output,
    }
}

pub fn expand_optional_match_case_param_list(
    registry: &NodeRegistry,
    id: Option<NonEmptyMatchCaseParamListId>,
) -> Option<NonEmptyMatchCaseParamVec> {
    id.map(|id| expand_match_case_param_list(registry, id))
}

pub fn expand_match_case_param_list(
    registry: &NodeRegistry,
    id: NonEmptyMatchCaseParamListId,
) -> NonEmptyMatchCaseParamVec {
    match id {
        NonEmptyMatchCaseParamListId::Unlabeled(id) => {
            NonEmptyMatchCaseParamVec::Unlabeled(expand_identifier_list(registry, id))
        }
        NonEmptyMatchCaseParamListId::UniquelyLabeled {
            param_list_id,
            triple_dot,
        } => {
            let params = expand_optional_labeled_match_case_param_list(registry, param_list_id);
            NonEmptyMatchCaseParamVec::UniquelyLabeled { params, triple_dot }
        }
    }
}

pub fn expand_optional_labeled_match_case_param_list(
    registry: &NodeRegistry,
    param_list_id: Option<NonEmptyListId<NodeId<light::LabeledMatchCaseParam>>>,
) -> Option<NonEmptyVec<LabeledMatchCaseParam>> {
    param_list_id.map(|id| expand_labeled_match_case_param_list(registry, id))
}

pub fn expand_labeled_match_case_param_list(
    registry: &NodeRegistry,
    param_list_id: NonEmptyListId<NodeId<light::LabeledMatchCaseParam>>,
) -> NonEmptyVec<LabeledMatchCaseParam> {
    registry
        .get_list(param_list_id)
        .to_mapped(|id| expand_labeled_match_case_param(registry, *id))
}

pub fn expand_labeled_match_case_param(
    registry: &NodeRegistry,
    param_list_id: NodeId<light::LabeledMatchCaseParam>,
) -> LabeledMatchCaseParam {
    let light = registry.get(param_list_id);
    let label = expand_param_label(registry, light.label_id);
    let name = expand_identifier(registry, light.name_id);
    LabeledMatchCaseParam {
        span: light.span,
        label,
        name,
    }
}

pub fn expand_forall(registry: &NodeRegistry, id: NodeId<light::Forall>) -> Forall {
    let light = registry.get(id);
    let params = expand_param_list(registry, light.param_list_id);
    let output = expand_expression(registry, light.output_id);
    Forall {
        span: light.span,
        params,
        output,
    }
}

pub fn expand_check(registry: &NodeRegistry, id: NodeId<light::Check>) -> Check {
    let light = registry.get(id);
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
    id: NonEmptyListId<NodeId<light::CheckAssertion>>,
) -> NonEmptyVec<CheckAssertion> {
    registry
        .get_list(id)
        .to_mapped(|id| expand_check_assertion(registry, *id))
}

pub fn expand_check_assertion(
    registry: &NodeRegistry,
    id: NodeId<light::CheckAssertion>,
) -> CheckAssertion {
    let light = registry.get(id);
    let left = expand_goal_kw_or_expression(registry, light.left_id);
    let right = expand_question_mark_or_possibly_invalid_expression(registry, light.right_id);
    CheckAssertion {
        span: light.span,
        kind: light.kind,
        left,
        right,
    }
}

pub fn expand_goal_kw_or_expression(
    registry: &NodeRegistry,
    id: GoalKwOrPossiblyInvalidExpressionId,
) -> GoalKwOrPossiblyInvalidExpression {
    match id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { span } => {
            GoalKwOrPossiblyInvalidExpression::GoalKw { span }
        }
        GoalKwOrPossiblyInvalidExpressionId::Expression(expression) => {
            GoalKwOrPossiblyInvalidExpression::Expression(expand_possibly_invalid_expression(
                registry, expression,
            ))
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
    let light = registry.get(id);
    let expression = light.expression.clone();
    let error = light.error.clone();
    SymbolicallyInvalidExpression {
        expression,
        error,
        span_invalidated: light.span_invalidated,
    }
}

pub fn expand_illegal_fun_recursion_expression(
    registry: &NodeRegistry,
    id: NodeId<light::IllegalFunRecursionExpression>,
) -> IllegalFunRecursionExpression {
    let light = registry.get(id);
    let expression = expand_expression(registry, light.expression_id);
    let error = light.error.clone();
    IllegalFunRecursionExpression {
        expression,
        error,
        span_invalidated: light.span_invalidated,
    }
}
