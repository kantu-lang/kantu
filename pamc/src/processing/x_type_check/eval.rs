use super::*;

pub fn evaluate_well_typed_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: ExpressionId,
) -> NormalFormId {
    match id {
        ExpressionId::Name(name_id) => {
            evaluate_well_typed_name_expression(context, registry, name_id)
        }
        ExpressionId::Call(call_id) => evaluate_well_typed_call(context, registry, call_id),
        ExpressionId::Fun(fun_id) => evaluate_well_typed_fun(context, registry, fun_id),
        ExpressionId::Match(match_id) => evaluate_well_typed_match(context, registry, match_id),
        ExpressionId::Forall(forall_id) => {
            evaluate_well_typed_forall(context, registry, forall_id)
        }
    }
}

fn evaluate_well_typed_name_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> NormalFormId {
    let name = registry.name_expression(name_id);
    let definition = context.get_definition(name.db_index, registry);
    match definition {
        ContextEntryDefinition::Alias { value_id } => value_id,

        ContextEntryDefinition::Adt {
            variant_name_list_id: _,
        }
        | ContextEntryDefinition::Variant { name_id: _ }
        | ContextEntryDefinition::Uninterpreted => {
            NormalFormId::unchecked_new(ExpressionId::Name(name_id))
        }
    }
}

fn evaluate_well_typed_call(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: NodeId<Call>,
) -> NormalFormId {
    fn register_normalized_nonsubstituted_fun(
        registry: &mut NodeRegistry,
        normalized_callee_id: NormalFormId,
        normalized_arg_ids: &[NormalFormId],
    ) -> NormalFormId {
        let normalized_arg_ids = normalized_arg_ids
            .iter()
            .copied()
            .map(NormalFormId::raw)
            .collect();
        let normalized_arg_list_id = registry.add_expression_list(normalized_arg_ids);
        let normalized_call_id = registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            callee_id: normalized_callee_id.raw(),
            arg_list_id: normalized_arg_list_id,
        });
        NormalFormId::unchecked_new(ExpressionId::Call(normalized_call_id))
    }

    let call = registry.call(call_id).clone();

    let normalized_callee_id =
        evaluate_well_typed_expression(context, registry, call.callee_id);

    let normalized_arg_ids: Vec<NormalFormId> = {
        let arg_ids = registry.expression_list(call.arg_list_id).to_vec();
        arg_ids
            .into_iter()
            .map(|arg_id| evaluate_well_typed_expression(context, registry, arg_id))
            .collect()
    };

    match normalized_callee_id.raw() {
        ExpressionId::Fun(fun_id) => {
            if !can_fun_be_applied(context, registry, fun_id, &normalized_arg_ids) {
                return register_normalized_nonsubstituted_fun(
                    registry,
                    normalized_callee_id,
                    &normalized_arg_ids,
                );
            }

            let fun = registry.fun(fun_id).clone();
            let param_ids = registry.param_list(fun.param_list_id).to_vec();
            let arity = param_ids.len();
            let shifted_normalized_arg_ids = normalized_arg_ids
                .into_iter()
                .map(|arg_id| arg_id.upshift(arity + 1, registry))
                .collect::<Vec<_>>();
            let substitutions =
                {
                    let shifted_fun_id = NormalFormId::unchecked_new(ExpressionId::Fun(
                        fun_id.upshift(arity + 1, registry),
                    ));
                    const FUN_DB_INDEX: DbIndex = DbIndex(0);
                    vec![Substitution::Single {
                        from: NormalFormId::unchecked_new(ExpressionId::Name(
                            add_name_expression(registry, vec![fun.name_id], FUN_DB_INDEX),
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
                            let param_name_id = registry.param(param_id).name_id;
                            let db_index = DbIndex(arity - arg_index);
                            let name = NormalFormId::unchecked_new(ExpressionId::Name(
                                add_name_expression(registry, vec![param_name_id], db_index),
                            ));
                            Substitution::Single {
                                from: name,
                                to: arg_id,
                            }
                        }),
                )
                .collect::<Vec<_>>();

            let body_id = fun.body_id.subst_all(&substitutions, registry);
            let shifted_body_id = body_id.downshift(arity + 1, registry);
            evaluate_well_typed_expression(context, registry, shifted_body_id)
        }
        ExpressionId::Name(_) | ExpressionId::Call(_) | ExpressionId::Match(_) => {
            register_normalized_nonsubstituted_fun(
                registry,
                normalized_callee_id,
                &normalized_arg_ids,
            )
        }
        ExpressionId::Forall(_) => {
            panic!("A well-typed Call expression cannot have a Forall as its callee.")
        }
    }
}

fn can_fun_be_applied(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: NodeId<Fun>,
    normalized_arg_ids: &[NormalFormId],
) -> bool {
    let param_list_id = registry.fun(fun_id).param_list_id;
    let decreasing_param_index =
        registry
            .param_list(param_list_id)
            .iter()
            .copied()
            .position(|param_id| {
                let param = registry.param(param_id);
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
    is_variant_expression(context, registry, decreasing_arg_id)
}

/// If the provided expression is has a variant at
/// the top level,this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
fn is_variant_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: NormalFormId,
) -> bool {
    try_as_variant_expression(context, registry, expression_id).is_some()
}

fn evaluate_well_typed_fun(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: NodeId<Fun>,
) -> NormalFormId {
    let fun = registry.fun(fun_id).clone();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(context, registry, fun.param_list_id)
            .expect("A well-typed Fun should have well-typed params.");
    let normalized_return_type_id =
        evaluate_well_typed_expression(context, registry, fun.return_type_id);
    context.pop_n(fun.param_list_id.len);

    NormalFormId::unchecked_new(ExpressionId::Fun(registry.add_fun_and_overwrite_its_id(
        Fun {
            id: dummy_id(),
            name_id: fun.name_id,
            param_list_id: normalized_param_list_id,
            return_type_id: normalized_return_type_id.raw(),
            body_id: fun.body_id,
        },
    )))
}

fn evaluate_well_typed_match(
    context: &mut Context,
    registry: &mut NodeRegistry,
    match_id: NodeId<Match>,
) -> NormalFormId {
    let match_ = registry.match_(match_id).clone();
    let normalized_matchee_id =
        evaluate_well_typed_expression(context, registry, match_.matchee_id);

    let (normalized_matchee_variant_name_id, normalized_matchee_arg_list_id) =
        if let Some((variant_name_id, arg_list_id)) =
            try_as_variant_expression(context, registry, normalized_matchee_id)
        {
            (variant_name_id, arg_list_id)
        } else {
            return NormalFormId::unchecked_new(ExpressionId::Match(
                registry.add_match_and_overwrite_its_id(Match {
                    id: dummy_id(),
                    matchee_id: normalized_matchee_id.raw(),
                    case_list_id: match_.case_list_id,
                }),
            ));
        };

    let case_id = *registry
        .match_case_list(match_.case_list_id)
        .iter()
        .find(|case_id| {
            let case = registry.match_case(**case_id);
            case.variant_name_id == normalized_matchee_variant_name_id
        })
         .expect("A well-typed Match expression should have a case for every variant of its matchee's type.");

    let case = registry.match_case(case_id).clone();

    match normalized_matchee_arg_list_id {
        PossibleArgListId::Nullary => {
            evaluate_well_typed_expression(context, registry, case.output_id)
        }
        PossibleArgListId::Some(normalized_matchee_arg_list_id) => {
            let case_param_ids = registry.identifier_list(case.param_list_id).to_vec();
            let case_arity = case_param_ids.len();
            let matchee_arg_ids = registry
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
                    let shifted_arg_id =
                        NormalFormId::unchecked_new(arg_id).upshift(case_arity, registry);
                    Substitution::Single {
                        from: NormalFormId::unchecked_new(ExpressionId::Name(
                            add_name_expression(registry, vec![param_id], db_index),
                        )),
                        to: shifted_arg_id,
                    }
                })
                .collect();

            let substituted_body = case
                .output_id
                .subst_all(&substitutions, registry)
                .downshift(case_arity, registry);
            evaluate_well_typed_expression(context, registry, substituted_body)
        }
    }
}

/// If the provided expression is has a variant at
/// the top level,this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
fn try_as_variant_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: NormalFormId,
) -> Option<(NodeId<Identifier>, PossibleArgListId)> {
    match expression_id.raw() {
        ExpressionId::Name(name_id) => {
            let db_index = registry.name_expression(name_id).db_index;
            let definition = context.get_definition(db_index, registry);
            match definition {
                ContextEntryDefinition::Variant { name_id } => {
                    Some((name_id, PossibleArgListId::Nullary))
                }
                _ => None,
            }
        }
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id).clone();
            match call.callee_id {
                ExpressionId::Name(name_id) => {
                    let db_index = registry.name_expression(name_id).db_index;
                    let definition = context.get_definition(db_index, registry);
                    match definition {
                        ContextEntryDefinition::Variant { name_id } => {
                            Some((name_id, PossibleArgListId::Some(call.arg_list_id)))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn evaluate_well_typed_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: NodeId<Forall>,
) -> NormalFormId {
    let forall = registry.forall(forall_id).clone();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(context, registry, forall.param_list_id)
            .expect("A well-typed Fun should have well-typed params.");
    let normalized_output_id =
        evaluate_well_typed_expression(context, registry, forall.output_id);
    context.pop_n(forall.param_list_id.len);

    NormalFormId::unchecked_new(ExpressionId::Forall(
        registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: normalized_output_id.raw(),
        }),
    ))
}