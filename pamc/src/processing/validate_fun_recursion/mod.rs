use crate::data::{
    fun_recursion_validation_result::*,
    light_ast::*,
    node_registry::{NodeId, NodeRegistry, NonEmptyListId},
    non_empty_vec::OptionalNonEmptyVecLen,
    variant_return_type_validation_result::VariantReturnTypesValidated,
};

use std::convert::Infallible;

use context::*;
mod context;

type TaintedIllegalFunRecursionError = Tainted<IllegalFunRecursionError>;

impl From<Tainted<Infallible>> for TaintedIllegalFunRecursionError {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

pub fn validate_fun_recursion_in_file(
    registry: &mut NodeRegistry,
    file_id: VariantReturnTypesValidated<NodeId<File>>,
) -> Result<FunRecursionValidated<NodeId<File>>, IllegalFunRecursionError> {
    let file_id = file_id.raw();
    let file = registry.get(file_id).clone();
    let mut context = Context::new();
    let item_ids = registry
        .get_possibly_empty_list(file.item_list_id)
        .to_vec()
        .into_iter()
        .map(|item_id| validate_fun_recursion_in_file_item(&mut context, registry, item_id))
        .collect::<Result<Vec<_>, _>>()?;
    let item_list_id = registry.add_possibly_empty_list(item_ids);
    Ok(FunRecursionValidated::unchecked_new(registry.add(File {
        id: dummy_id(),
        span: file.span,
        file_id: file.file_id,
        item_list_id,
    })))
}

fn validate_fun_recursion_in_file_item(
    context: &mut Context,
    registry: &mut NodeRegistry,
    item_id: FileItemNodeId,
) -> Result<FileItemNodeId, IllegalFunRecursionError> {
    Ok(match item_id {
        FileItemNodeId::Type(id) => FileItemNodeId::Type(validate_fun_recursion_in_type_statement(
            context, registry, id,
        )?),
        FileItemNodeId::Let(id) => FileItemNodeId::Let(validate_fun_recursion_in_let_statement(
            context, registry, id,
        )?),
    })
}

fn validate_fun_recursion_in_type_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<NodeId<TypeStatement>, IllegalFunRecursionError> {
    untaint_err(
        context,
        registry,
        type_statement_id,
        validate_fun_recursion_in_type_statement_dirty,
    )
}

fn validate_fun_recursion_in_type_statement_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<NodeId<TypeStatement>, TaintedIllegalFunRecursionError> {
    let type_statement = registry.get(type_statement_id).clone();
    let param_list_id = validate_fun_recursion_in_optional_params_and_leave_in_context_dirty(
        context,
        registry,
        type_statement.param_list_id,
    )?;
    context.pop_n(type_statement.param_list_id.len());

    context.push(ContextEntry::NoInformation)?;

    let variant_ids = registry
        .get_possibly_empty_list(type_statement.variant_list_id)
        .to_vec()
        .into_iter()
        .map(|variant_id| validate_fun_recursion_in_variant_dirty(context, registry, variant_id))
        .collect::<Result<Vec<_>, _>>()?;
    let variant_list_id = registry.add_possibly_empty_list(variant_ids);

    Ok(registry.add(TypeStatement {
        id: dummy_id(),
        span: type_statement.span,
        name_id: type_statement.name_id,
        param_list_id,
        variant_list_id,
    }))
}

fn validate_fun_recursion_in_variant_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: NodeId<Variant>,
) -> Result<NodeId<Variant>, TaintedIllegalFunRecursionError> {
    let variant = registry.get(variant_id).clone();
    let arity = variant.param_list_id.len();
    let param_list_id = validate_fun_recursion_in_optional_params_and_leave_in_context_dirty(
        context,
        registry,
        variant.param_list_id,
    )?;
    let return_type_id =
        validate_fun_recursion_in_expression_dirty(context, registry, variant.return_type_id)?;
    context.pop_n(arity);

    context.push(ContextEntry::NoInformation)?;

    Ok(registry.add(Variant {
        id: dummy_id(),
        span: variant.span,
        name_id: variant.name_id,
        param_list_id,
        return_type_id,
    }))
}

fn validate_fun_recursion_in_let_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: NodeId<LetStatement>,
) -> Result<NodeId<LetStatement>, IllegalFunRecursionError> {
    untaint_err(
        context,
        registry,
        let_statement_id,
        validate_fun_recursion_in_let_statement_dirty,
    )
}

fn validate_fun_recursion_in_let_statement_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: NodeId<LetStatement>,
) -> Result<NodeId<LetStatement>, TaintedIllegalFunRecursionError> {
    let let_statement = registry.get(let_statement_id).clone();
    let value_id =
        validate_fun_recursion_in_expression_dirty(context, registry, let_statement.value_id)?;
    context.push(ContextEntry::NoInformation)?;
    Ok(registry.add(LetStatement {
        id: dummy_id(),
        span: let_statement.span,
        name_id: let_statement.name_id,
        value_id,
    }))
}

fn validate_fun_recursion_in_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: ExpressionId,
) -> Result<ExpressionId, IllegalFunRecursionError> {
    untaint_err(
        context,
        registry,
        expression_id,
        validate_fun_recursion_in_expression_dirty,
    )
}

fn validate_fun_recursion_in_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: ExpressionId,
) -> Result<ExpressionId, TaintedIllegalFunRecursionError> {
    Ok(match expression_id {
        ExpressionId::Name(id) => {
            validate_fun_recursion_in_name_dirty(context, registry, id).map(ExpressionId::Name)?
        }
        ExpressionId::Call(id) => {
            validate_fun_recursion_in_call_dirty(context, registry, id).map(ExpressionId::Call)?
        }
        ExpressionId::Fun(id) => {
            validate_fun_recursion_in_fun_dirty(context, registry, id).map(ExpressionId::Fun)?
        }
        ExpressionId::Match(id) => {
            validate_fun_recursion_in_match_dirty(context, registry, id).map(ExpressionId::Match)?
        }
        ExpressionId::Forall(id) => validate_fun_recursion_in_forall_dirty(context, registry, id)
            .map(ExpressionId::Forall)?,
        ExpressionId::Check(id) => {
            validate_fun_recursion_in_check_dirty(context, registry, id).map(ExpressionId::Check)?
        }
    })
}

fn validate_fun_recursion_in_name_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> Result<NodeId<NameExpression>, TaintedIllegalFunRecursionError> {
    let name = registry.get(name_id);
    if context.reference_restriction(name.db_index).is_some() {
        return Err(Tainted::new(
            IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall { reference: name_id },
        ));
    }
    Ok(name_id)
}

fn validate_fun_recursion_in_call_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<NodeId<Call>, TaintedIllegalFunRecursionError> {
    let call = registry.get(call_id).clone();

    let is_restricted = is_call_restricted(context, registry, call_id)?;

    // If the call is restricted (i.e., in the form
    // `f(x, y, ...z)`, where `f` is a Name referring to a restricted recursive function,
    // then we need to skip the callee validation (otherwise `f` will trigger
    // an error, since it is a recursive reference).
    let callee_id = if is_restricted {
        // We don't need to do any additional processing
        // (e.g., validate Check expressions)
        // since the callee is a Name expression.
        call.callee_id
    } else {
        validate_fun_recursion_in_expression_dirty(context, registry, call.callee_id)?
    };

    let arg_ids = registry
        .get_list(call.arg_list_id)
        .to_non_empty_vec()
        .try_into_mapped(|arg_id| {
            validate_fun_recursion_in_expression_dirty(context, registry, arg_id)
        })?;
    let arg_list_id = registry.add_list(arg_ids);

    Ok(registry.add(Call {
        id: dummy_id(),
        span: call.span,
        callee_id,
        arg_list_id,
    }))
}

fn is_call_restricted(
    context: &Context,
    registry: &NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<bool, TaintedIllegalFunRecursionError> {
    let call = registry.get(call_id).clone();
    match call.callee_id {
        ExpressionId::Name(callee_name_id) => {
            let callee_name = registry.get(callee_name_id);
            if let Some(restriction) = context.reference_restriction(callee_name.db_index) {
                match restriction {
                    ReferenceRestriction::MustCallWithSubstruct {
                        arg_position,
                        superstruct_db_level,
                        ..
                    } => {
                        let arg_ids = registry.get_list(call.arg_list_id);
                        if arg_position < arg_ids.len() {
                            let expected_substruct_id = arg_ids[arg_position];
                            let err = Err(Tainted::new(
                                IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                    callee: callee_name_id,
                                    arg: expected_substruct_id,
                                },
                            ));
                            match expected_substruct_id {
                                ExpressionId::Name(expected_substruct_name_id) => {
                                    let expected_substruct =
                                        registry.get(expected_substruct_name_id);
                                    let expected_substruct_db_level =
                                        context.index_to_level(expected_substruct.db_index);
                                    if !context.is_left_strict_substruct_of_right(
                                        expected_substruct_db_level,
                                        superstruct_db_level,
                                    ) {
                                        return err;
                                    }
                                }
                                _ => return err,
                            }
                        }
                    }
                    ReferenceRestriction::CannotCall { .. } => return Err(Tainted::new(
                        IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
                            callee: callee_name.id,
                        },
                    )),
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

fn validate_fun_recursion_in_fun_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: NodeId<Fun>,
) -> Result<NodeId<Fun>, TaintedIllegalFunRecursionError> {
    let fun = registry.get(fun_id).clone();
    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context_dirty(
        context,
        registry,
        fun.param_list_id,
    )?;
    let return_type_id =
        validate_fun_recursion_in_expression_dirty(context, registry, fun.return_type_id)?;

    let reference_restriction = match fun.param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => {
            let param_ids = registry.get_list(param_list_id);
            let decreasing_param_position = param_ids.iter().position(|param_id| {
                let param = registry.get(*param_id);
                param.is_dashed
            });
            match decreasing_param_position {
                Some(param_position) => {
                    let superstruct_db_index = DbIndex(param_ids.len() - param_position - 1);
                    let superstruct_db_level = context.index_to_level(superstruct_db_index);
                    ReferenceRestriction::MustCallWithSubstruct {
                        superstruct_db_level,
                        arg_position: param_position,
                    }
                }
                None => ReferenceRestriction::CannotCall,
            }
        }
        NonEmptyParamListId::Labeled(param_list_id) => {
            // TODO: We should use label instead of index.
            let param_ids = registry.get_list(param_list_id);
            let decreasing_param_position = param_ids.iter().position(|param_id| {
                let param = registry.get(*param_id);
                param.is_dashed
            });
            match decreasing_param_position {
                Some(param_position) => {
                    let superstruct_db_index = DbIndex(param_ids.len() - param_position - 1);
                    let superstruct_db_level = context.index_to_level(superstruct_db_index);
                    ReferenceRestriction::MustCallWithSubstruct {
                        superstruct_db_level,
                        arg_position: param_position,
                    }
                }
                None => ReferenceRestriction::CannotCall,
            }
        }
    };

    context.push(ContextEntry::Fun(reference_restriction))?;
    let body_id = validate_fun_recursion_in_expression_dirty(context, registry, fun.body_id)?;
    context.pop_n(param_list_id.len() + 1);

    Ok(registry.add(Fun {
        id: dummy_id(),
        span: fun.span,
        name_id: fun.name_id,
        param_list_id,
        return_type_id,
        body_id,
        skip_type_checking_body: fun.skip_type_checking_body,
    }))
}

fn validate_fun_recursion_in_optional_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: Option<NonEmptyParamListId>,
) -> Result<Option<NonEmptyParamListId>, TaintedIllegalFunRecursionError> {
    param_list_id
        .map(|param_list_id| {
            validate_fun_recursion_in_params_and_leave_in_context_dirty(
                context,
                registry,
                param_list_id,
            )
        })
        .transpose()
}

fn validate_fun_recursion_in_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: NonEmptyParamListId,
) -> Result<NonEmptyParamListId, TaintedIllegalFunRecursionError> {
    Ok(match param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => NonEmptyParamListId::Unlabeled(
            validate_fun_recursion_in_unlabeled_params_and_leave_in_context_dirty(
                context,
                registry,
                param_list_id,
            )?,
        ),
        NonEmptyParamListId::Labeled(param_list_id) => NonEmptyParamListId::Labeled(
            validate_fun_recursion_in_labeled_params_and_leave_in_context_dirty(
                context,
                registry,
                param_list_id,
            )?,
        ),
    })
}

fn validate_fun_recursion_in_unlabeled_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> Result<NonEmptyListId<NodeId<UnlabeledParam>>, TaintedIllegalFunRecursionError> {
    let param_ids = registry
        .get_list(param_list_id)
        .to_non_empty_vec()
        .try_into_mapped(|param_id| -> Result<_, TaintedIllegalFunRecursionError> {
            let param = registry.get(param_id).clone();
            let type_id =
                validate_fun_recursion_in_expression_dirty(context, registry, param.type_id)?;
            context.push(ContextEntry::NoInformation)?;
            Ok(registry.add(UnlabeledParam {
                id: dummy_id(),
                span: param.span,
                name_id: param.name_id,
                type_id,
                is_dashed: param.is_dashed,
            }))
        })?;

    Ok(registry.add_list(param_ids))
}

fn validate_fun_recursion_in_labeled_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
) -> Result<NonEmptyListId<NodeId<LabeledParam>>, TaintedIllegalFunRecursionError> {
    let param_ids = registry
        .get_list(param_list_id)
        .to_non_empty_vec()
        .try_into_mapped(|param_id| -> Result<_, TaintedIllegalFunRecursionError> {
            let param = registry.get(param_id).clone();
            let type_id =
                validate_fun_recursion_in_expression_dirty(context, registry, param.type_id)?;
            context.push(ContextEntry::NoInformation)?;
            Ok(registry.add(LabeledParam {
                id: dummy_id(),
                span: param.span,
                label_id: param.label_id,
                name_id: param.name_id,
                type_id,
                is_dashed: param.is_dashed,
            }))
        })?;

    Ok(registry.add_list(param_ids))
}

fn validate_fun_recursion_in_match_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    match_id: NodeId<Match>,
) -> Result<NodeId<Match>, TaintedIllegalFunRecursionError> {
    let match_ = registry.get(match_id).clone();
    let matchee_id =
        validate_fun_recursion_in_expression_dirty(context, registry, match_.matchee_id)?;
    let matchee_db_index = match match_.matchee_id {
        ExpressionId::Name(matchee_name_id) => {
            let matchee_name = registry.get(matchee_name_id);
            Some(matchee_name.db_index)
        }
        _ => None,
    };

    let case_ids = registry
        .get_possibly_empty_list(match_.case_list_id)
        .to_vec()
        .into_iter()
        .map(|case_id| {
            validate_fun_recursion_in_match_case_dirty(context, registry, case_id, matchee_db_index)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let case_list_id = registry.add_possibly_empty_list(case_ids);

    Ok(registry.add(Match {
        id: dummy_id(),
        span: match_.span,
        matchee_id,
        case_list_id,
    }))
}

fn validate_fun_recursion_in_match_case_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    case_id: NodeId<MatchCase>,
    matchee_db_index: Option<DbIndex>,
) -> Result<NodeId<MatchCase>, TaintedIllegalFunRecursionError> {
    let case = registry.get(case_id).clone();
    let case_arity = case.param_list_id.len();

    if let Some(matchee_db_index) = matchee_db_index {
        let matchee_db_level = context.index_to_level(matchee_db_index);
        for _ in 0..case_arity {
            context.push(ContextEntry::Substruct {
                superstruct_db_level: matchee_db_level,
            })?;
        }
    } else {
        for _ in 0..case_arity {
            context.push(ContextEntry::NoInformation)?;
        }
    }

    // We don't need to do any validation since it's just a
    // list of identifiers.
    let param_list_id = case.param_list_id;

    let output_id = validate_fun_recursion_in_expression_dirty(context, registry, case.output_id)?;
    context.pop_n(case_arity);

    Ok(registry.add(MatchCase {
        id: dummy_id(),
        span: case.span,
        variant_name_id: case.variant_name_id,
        param_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_forall_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<NodeId<Forall>, TaintedIllegalFunRecursionError> {
    let forall = registry.get(forall_id).clone();
    let arity = forall.param_list_id.len();

    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context_dirty(
        context,
        registry,
        forall.param_list_id,
    )?;
    let output_id =
        validate_fun_recursion_in_expression_dirty(context, registry, forall.output_id)?;
    context.pop_n(arity);

    Ok(registry.add(Forall {
        id: dummy_id(),
        span: forall.span,
        param_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_check_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    check_id: NodeId<Check>,
) -> Result<NodeId<Check>, TaintedIllegalFunRecursionError> {
    let check = registry.get(check_id).clone();
    let assertion_list_id = validate_fun_recursion_in_check_assertions_dirty(
        context,
        registry,
        check.assertion_list_id,
    )?;
    let output_id = validate_fun_recursion_in_expression_dirty(context, registry, check.output_id)?;
    Ok(registry.add(Check {
        id: dummy_id(),
        span: check.span,
        assertion_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_check_assertions_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: NonEmptyListId<NodeId<CheckAssertion>>,
) -> Result<NonEmptyListId<NodeId<CheckAssertion>>, TaintedIllegalFunRecursionError> {
    let assertion_ids =
        registry
            .get_list(id)
            .to_non_empty_vec()
            .try_into_mapped(|assertion_id| {
                validate_fun_recursion_in_check_assertion_dirty(context, registry, assertion_id)
            })?;
    Ok(registry.add_list(assertion_ids))
}

fn validate_fun_recursion_in_check_assertion_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: NodeId<CheckAssertion>,
) -> Result<NodeId<CheckAssertion>, TaintedIllegalFunRecursionError> {
    let assertion = registry.get(id).clone();
    let left_id = validate_fun_recursion_in_goal_kw_or_expression_dirty(
        context,
        registry,
        assertion.left_id,
    )?;
    let right_id = validate_fun_recursion_in_question_mark_or_possibly_invalid_expression_dirty(
        context,
        registry,
        assertion.right_id,
    )?;
    Ok(registry.add(CheckAssertion {
        id: dummy_id(),
        span: assertion.span,
        kind: assertion.kind,
        left_id,
        right_id,
    }))
}

fn validate_fun_recursion_in_goal_kw_or_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: GoalKwOrPossiblyInvalidExpressionId,
) -> Result<GoalKwOrPossiblyInvalidExpressionId, TaintedIllegalFunRecursionError> {
    Ok(match id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start }
        }
        GoalKwOrPossiblyInvalidExpressionId::Expression(id) => {
            GoalKwOrPossiblyInvalidExpressionId::Expression(
                validate_fun_recursion_in_possibly_invalid_expression_dirty(context, registry, id),
            )
        }
    })
}

fn validate_fun_recursion_in_question_mark_or_possibly_invalid_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
) -> Result<QuestionMarkOrPossiblyInvalidExpressionId, TaintedIllegalFunRecursionError> {
    Ok(match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start }
        }
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                validate_fun_recursion_in_possibly_invalid_expression_dirty(context, registry, id),
            )
        }
    })
}

fn validate_fun_recursion_in_possibly_invalid_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    original_id: PossiblyInvalidExpressionId,
) -> PossiblyInvalidExpressionId {
    match original_id {
        PossiblyInvalidExpressionId::Invalid(original_id) => {
            PossiblyInvalidExpressionId::Invalid(original_id)
        }
        PossiblyInvalidExpressionId::Valid(original_id) => {
            // We have to use `validate_fun_recursion_in_expression`
            // instead of `validate_fun_recursion_in_expression_dirty`
            // because we have to keep the context clean even in the case
            // where `validation_result` is `Err` (since we'll still ultimately
            // return `Ok`)
            let validation_result =
                validate_fun_recursion_in_expression(context, registry, original_id);
            match validation_result {
                Ok(validated_id) => PossiblyInvalidExpressionId::Valid(validated_id),
                Err(err) => {
                    PossiblyInvalidExpressionId::Invalid(InvalidExpressionId::IllegalFunRecursion(
                        registry.add(IllegalFunRecursionExpression {
                            id: dummy_id(),
                            expression_id: original_id,
                            error: err,
                            span_invalidated: false,
                        }),
                    ))
                }
            }
        }
    }
}

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}
