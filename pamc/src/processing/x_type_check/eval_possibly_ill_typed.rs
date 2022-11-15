use super::*;

pub(super) fn evaluate_possibly_ill_typed_expression(
    state: &mut State,
    id: ExpressionId,
) -> ExpressionId {
    let out = match id {
        ExpressionId::Name(name_id) => evaluate_possibly_ill_typed_name_expression(state, name_id),
        ExpressionId::Call(call_id) => evaluate_possibly_ill_typed_call(state, call_id),
        ExpressionId::Fun(fun_id) => evaluate_possibly_ill_typed_fun(state, fun_id),
        ExpressionId::Match(match_id) => evaluate_possibly_ill_typed_match(state, match_id),
        ExpressionId::Forall(forall_id) => evaluate_possibly_ill_typed_forall(state, forall_id),
    };
    out
}

fn evaluate_possibly_ill_typed_name_expression(
    state: &mut State,
    name_id: NodeId<NameExpression>,
) -> ExpressionId {
    let name = state.registry.name_expression(name_id);
    let definition = state.context.get_definition(name.db_index, state.registry);
    match definition {
        ContextEntryDefinition::Alias { value_id } => value_id.raw(),

        ContextEntryDefinition::Adt {
            variant_name_list_id: _,
        }
        | ContextEntryDefinition::Variant { name_id: _ }
        | ContextEntryDefinition::Uninterpreted => ExpressionId::Name(name_id),
    }
}

fn evaluate_possibly_ill_typed_call(state: &mut State, call_id: NodeId<Call>) -> ExpressionId {
    fn register_normalized_nonsubstituted_fun(
        registry: &mut NodeRegistry,
        normalized_callee_id: ExpressionId,
        normalized_arg_ids: &[ExpressionId],
    ) -> ExpressionId {
        let normalized_arg_list_id = registry.add_expression_list(normalized_arg_ids.to_vec());
        let normalized_call_id = registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            callee_id: normalized_callee_id,
            arg_list_id: normalized_arg_list_id,
        });
        ExpressionId::Call(normalized_call_id)
    }

    let call = state.registry.call(call_id).clone();

    let normalized_callee_id = evaluate_possibly_ill_typed_expression(state, call.callee_id);

    let normalized_arg_ids: Vec<ExpressionId> = {
        let arg_ids = state.registry.expression_list(call.arg_list_id).to_vec();
        arg_ids
            .into_iter()
            .map(|arg_id| evaluate_possibly_ill_typed_expression(state, arg_id))
            .collect()
    };

    match normalized_callee_id {
        ExpressionId::Fun(fun_id) => {
            if !can_fun_be_applied(state, fun_id, &normalized_arg_ids) {
                return register_normalized_nonsubstituted_fun(
                    state.registry,
                    normalized_callee_id,
                    &normalized_arg_ids,
                );
            }

            let fun = state.registry.fun(fun_id).clone();
            let param_ids = state.registry.param_list(fun.param_list_id).to_vec();
            let arity = param_ids.len();
            let shifted_normalized_arg_ids = normalized_arg_ids
                .into_iter()
                .map(|arg_id| arg_id.upshift(arity + 1, state.registry))
                .collect::<Vec<_>>();
            let substitutions = {
                let shifted_fun_id = ExpressionId::Fun(fun_id.upshift(arity + 1, state.registry));
                const FUN_DB_INDEX: DbIndex = DbIndex(0);
                vec![Substitution {
                    from: ExpressionId::Name(add_name_expression(
                        state.registry,
                        vec![fun.name_id],
                        FUN_DB_INDEX,
                    )),
                    to: shifted_fun_id,
                }]
            }
            .into_iter()
            .chain(
                param_ids
                    .iter()
                    .copied()
                    .zip(shifted_normalized_arg_ids.iter().copied())
                    .enumerate()
                    .map(|(arg_index, (param_id, arg_id))| {
                        let param_name_id = state.registry.param(param_id).name_id;
                        let db_index = DbIndex(arity - arg_index);
                        let name = ExpressionId::Name(add_name_expression(
                            state.registry,
                            vec![param_name_id],
                            db_index,
                        ));
                        Substitution {
                            from: name,
                            to: arg_id,
                        }
                    }),
            )
            .collect::<Vec<_>>();

            let body_id = fun
                .body_id
                .subst_all(&substitutions, &mut state.without_context());
            let shifted_body_id = body_id.downshift(arity + 1, state.registry);
            evaluate_possibly_ill_typed_expression(state, shifted_body_id)
        }
        ExpressionId::Name(_) | ExpressionId::Call(_) | ExpressionId::Match(_) => {
            register_normalized_nonsubstituted_fun(
                state.registry,
                normalized_callee_id,
                &normalized_arg_ids,
            )
        }
        ExpressionId::Forall(_) => normalized_callee_id,
    }
}

fn can_fun_be_applied(
    state: &mut State,
    fun_id: NodeId<Fun>,
    normalized_arg_ids: &[ExpressionId],
) -> bool {
    let param_list_id = state.registry.fun(fun_id).param_list_id;
    let decreasing_param_index = state
        .registry
        .param_list(param_list_id)
        .iter()
        .copied()
        .position(|param_id| {
            let param = state.registry.param(param_id);
            param.is_dashed
        });
    let decreasing_param_index = if let Some(i) = decreasing_param_index {
        i
    } else {
        // If there is no decreasing parameter, the function is non-recursive,
        // so it can be safely applied without causing infinite expansion.
        return true;
    };

    let decreasing_arg_id = normalized_arg_ids[decreasing_param_index];
    is_variant_expression(state, decreasing_arg_id)
}

/// If the provided expression is has a variant at
/// the top level,this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
fn is_variant_expression(state: &mut State, expression_id: ExpressionId) -> bool {
    try_as_variant_expression(state, expression_id).is_some()
}

fn evaluate_possibly_ill_typed_fun(state: &mut State, fun_id: NodeId<Fun>) -> ExpressionId {
    let fun = state.registry.fun(fun_id).clone();
    let normalized_param_list_id =
        match normalize_params_and_leave_params_in_context(state, fun.param_list_id) {
            Ok(id) => id,
            Err(err) => {
                if let TypeCheckError::IllegalTypeExpression(expr_id) = &err {
                    println!(
                        "ILLEGAL_TYPE_EXPR: {:#?}",
                        crate::processing::x_expand_lightened::expand_expression(
                            state.registry,
                            *expr_id
                        )
                    );
                } else {
                    println!("DIFFERENT ERROR");
                }
                return ExpressionId::Fun(fun_id);
            }
        };
    let normalized_return_type_id =
        evaluate_possibly_ill_typed_expression(state, fun.return_type_id);
    state.context.pop_n(fun.param_list_id.len);

    ExpressionId::Fun(state.registry.add_fun_and_overwrite_its_id(Fun {
        id: dummy_id(),
        name_id: fun.name_id,
        param_list_id: normalized_param_list_id,
        return_type_id: normalized_return_type_id,
        body_id: fun.body_id,
        skip_type_checking_body: fun.skip_type_checking_body,
    }))
}

fn evaluate_possibly_ill_typed_match(state: &mut State, match_id: NodeId<Match>) -> ExpressionId {
    let match_ = state.registry.match_(match_id).clone();
    let normalized_matchee_id = evaluate_possibly_ill_typed_expression(state, match_.matchee_id);

    let (normalized_matchee_variant_name_id, normalized_matchee_arg_list_id) =
        if let Some((variant_name_id, arg_list_id)) =
            try_as_variant_expression(state, normalized_matchee_id)
        {
            (variant_name_id, arg_list_id)
        } else {
            return ExpressionId::Match(state.registry.add_match_and_overwrite_its_id(Match {
                id: dummy_id(),
                matchee_id: normalized_matchee_id,
                case_list_id: match_.case_list_id,
            }));
        };

    if state
        .registry
        .match_case_list(match_.case_list_id)
        .iter()
        .find(|case_id| {
            let case = state.registry.match_case(**case_id);
            let case_variant_name: &IdentifierName =
                &state.registry.identifier(case.variant_name_id).name;
            let matchee_variant_name: &IdentifierName = &state
                .registry
                .identifier(normalized_matchee_variant_name_id)
                .name;
            case_variant_name == matchee_variant_name
        })
        .is_none()
    {
        println!(
            "TRIED_TO_EVAL_ILL_TYPED_MATCH(context_len={}, type0_dbi={:?}).match = {:#?}",
            state.context.len(),
            state.context.type0_dbi(),
            crate::processing::x_expand_lightened::expand_expression(
                state.registry,
                ExpressionId::Match(match_.id)
            )
        );
        println!(
            "TRIED_TO_EVAL_ILL_TYPED_MATCH(context_len={}, type0_dbi={:?}).variant_name = {:#?}",
            state.context.len(),
            state.context.type0_dbi(),
            &state
                .registry
                .identifier(normalized_matchee_variant_name_id)
                .name
        );
    }

    let case_id = if let Some(id) = state
        .registry
        .match_case_list(match_.case_list_id)
        .iter()
        .find(|case_id| {
            let case = state.registry.match_case(**case_id);
            let case_variant_name: &IdentifierName =
                &state.registry.identifier(case.variant_name_id).name;
            let matchee_variant_name: &IdentifierName = &state
                .registry
                .identifier(normalized_matchee_variant_name_id)
                .name;
            case_variant_name == matchee_variant_name
        })
        .copied()
    {
        id
    } else {
        return ExpressionId::Match(match_id);
    };

    let case = state.registry.match_case(case_id).clone();

    match normalized_matchee_arg_list_id {
        PossibleArgListId::Nullary => evaluate_possibly_ill_typed_expression(state, case.output_id),
        PossibleArgListId::Some(normalized_matchee_arg_list_id) => {
            let case_param_ids = state.registry.identifier_list(case.param_list_id).to_vec();
            let case_arity = case_param_ids.len();
            let matchee_arg_ids = state
                .registry
                .expression_list(normalized_matchee_arg_list_id)
                .to_vec();
            let substitutions: Vec<Substitution> = case_param_ids
                .iter()
                .copied()
                .zip(matchee_arg_ids.iter().copied())
                .enumerate()
                .map(|(param_index, (param_id, arg_id))| {
                    let db_index = DbIndex(case_arity - param_index - 1);
                    // We can safely call `unchecked_new` here because we know that each
                    // arg to a normal form Call is also a normal form.
                    let shifted_arg_id = arg_id.upshift(case_arity, state.registry);
                    Substitution {
                        from: ExpressionId::Name(add_name_expression(
                            state.registry,
                            vec![param_id],
                            db_index,
                        )),
                        to: shifted_arg_id,
                    }
                })
                .collect();

            let substituted_body = case
                .output_id
                .subst_all(&substitutions, &mut state.without_context())
                .downshift(case_arity, state.registry);
            evaluate_possibly_ill_typed_expression(state, substituted_body)
        }
    }
}

fn evaluate_possibly_ill_typed_forall(
    state: &mut State,
    forall_id: NodeId<Forall>,
) -> ExpressionId {
    let forall = state.registry.forall(forall_id).clone();
    let normalized_param_list_id =
        match normalize_params_and_leave_params_in_context(state, forall.param_list_id) {
            Ok(id) => id,
            Err(err) => {
                if let TypeCheckError::IllegalTypeExpression(expr_id) = &err {
                    println!("ILLEGAL_TYPE_EXPR.context: {:#?}", state.context,);
                    println!(
                        "ILLEGAL_TYPE_EXPR(context_len={}): {:#?}",
                        state.context.len(),
                        crate::processing::x_expand_lightened::expand_expression(
                            state.registry,
                            *expr_id
                        )
                    );
                } else {
                    println!("DIFFERENT ERROR");
                }
                return ExpressionId::Forall(forall_id);
            }
        };
    let normalized_output_id = evaluate_possibly_ill_typed_expression(state, forall.output_id);
    state.context.pop_n(forall.param_list_id.len);

    ExpressionId::Forall(state.registry.add_forall_and_overwrite_its_id(Forall {
        id: dummy_id(),
        param_list_id: normalized_param_list_id,
        output_id: normalized_output_id,
    }))
}