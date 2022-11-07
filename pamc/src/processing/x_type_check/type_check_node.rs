use super::*;

pub fn type_check_files(
    registry: &mut NodeRegistry,
    file_ids: &[NodeId<File>],
) -> Result<(), TypeCheckError> {
    let mut context = Context::with_builtins(registry);
    let mut equality_checker = NodeEqualityChecker::new();
    let mut state = State {
        context: &mut context,
        registry,
        equality_checker: &mut equality_checker,
    };
    for &id in file_ids {
        type_check_file(&mut state, id)?;
    }
    Ok(())
}

fn type_check_file(state: &mut State, file_id: NodeId<File>) -> Result<(), TypeCheckError> {
    let file = state.registry.file(file_id);
    let items = state.registry.file_item_list(file.item_list_id).to_vec();
    for &item_id in &items {
        type_check_file_item(state, item_id)?;
    }
    state.context.pop_n(items.len());
    Ok(())
}

fn type_check_file_item(state: &mut State, item: FileItemNodeId) -> Result<(), TypeCheckError> {
    match item {
        FileItemNodeId::Type(type_statement) => type_check_type_statement(state, type_statement),
        FileItemNodeId::Let(let_statement) => type_check_let_statement(state, let_statement),
    }
}

fn type_check_type_statement(
    state: &mut State,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeCheckError> {
    type_check_type_constructor(state, type_statement_id)?;

    let type_statement = state.registry.type_statement(type_statement_id);
    let variant_ids = state
        .registry
        .variant_list(type_statement.variant_list_id)
        .to_vec();
    for variant_id in variant_ids {
        type_check_type_variant(state, variant_id)?;
    }

    Ok(())
}

fn type_check_type_constructor(
    state: &mut State,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeCheckError> {
    let type_statement = state.registry.type_statement(type_statement_id).clone();
    let normalized_param_list_id = normalize_params(state, type_statement.param_list_id)?;
    let type_constructor_type_id = NormalFormId::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: type0_expression(state).raw(),
        }
        .collapse_if_nullary(state.registry),
    );
    let variant_name_list_id = {
        let variant_ids = state.registry.variant_list(type_statement.variant_list_id);
        let variant_name_ids = variant_ids
            .iter()
            .map(|&variant_id| state.registry.variant(variant_id).name_id)
            .collect();
        state.registry.add_identifier_list(variant_name_ids)
    };
    state.context.push(ContextEntry {
        type_id: type_constructor_type_id,
        definition: ContextEntryDefinition::Adt {
            variant_name_list_id,
        },
    });
    Ok(())
}

pub(super) fn type_check_param(
    state: &mut State,
    param_id: NodeId<Param>,
) -> Result<(), TypeCheckError> {
    let param = state.registry.param(param_id).clone();
    let param_type_type_id = get_type_of_expression(state, None, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(state, param_type_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(state, param.type_id);
    state.context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    });
    Ok(())
}

fn type_check_type_variant(
    state: &mut State,
    variant_id: NodeId<Variant>,
) -> Result<(), TypeCheckError> {
    let variant = state.registry.variant(variant_id).clone();
    let arity = variant.param_list_id.len;
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(state, variant.param_list_id)?;
    type_check_expression(state, None, variant.return_type_id)?;
    let return_type_id = evaluate_well_typed_expression(state, variant.return_type_id);
    let type_id = NormalFormId::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: return_type_id.raw(),
        }
        .collapse_if_nullary(state.registry),
    );
    state.context.pop_n(arity);
    state.context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Variant {
            name_id: variant.name_id,
        },
    });
    Ok(())
}

fn type_check_let_statement(
    state: &mut State,
    let_statement_id: NodeId<LetStatement>,
) -> Result<(), TypeCheckError> {
    let let_statement = state.registry.let_statement(let_statement_id).clone();
    let type_id = get_type_of_expression(state, None, let_statement.value_id)?;
    let normalized_value_id = evaluate_well_typed_expression(state, let_statement.value_id);
    state.context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_value_id,
        },
    });
    Ok(())
}

fn type_check_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    expression: ExpressionId,
) -> Result<(), TypeCheckError> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression(state, coercion_target_id, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: ExpressionId,
) -> Result<NormalFormId, TypeCheckError> {
    match id {
        ExpressionId::Name(name) => Ok(get_type_of_name(state, name)),
        ExpressionId::Call(call) => get_type_of_call(state, call),
        ExpressionId::Fun(fun) => get_type_of_fun(state, fun),
        ExpressionId::Match(match_) => get_type_of_match(state, coercion_target_id, match_),
        ExpressionId::Forall(forall) => get_type_of_forall(state, forall),
    }
}

fn get_type_of_name(state: &mut State, name_id: NodeId<NameExpression>) -> NormalFormId {
    let name = state.registry.name_expression(name_id);
    state.context.get_type(name.db_index, state.registry)
}

fn get_type_of_call(
    state: &mut State,
    call_id: NodeId<Call>,
) -> Result<NormalFormId, TypeCheckError> {
    let call = state.registry.call(call_id).clone();
    let callee_type_id = get_type_of_expression(state, None, call.callee_id)?;
    let callee_type_id = if let ExpressionId::Forall(id) = callee_type_id.raw() {
        id
    } else {
        return Err(TypeCheckError::BadCallee(call.callee_id));
    };
    let arg_ids = state.registry.expression_list(call.arg_list_id).to_vec();
    let arg_type_ids = arg_ids
        .iter()
        .copied()
        .map(|arg_id| {
            get_type_of_expression(
                state, /* TODO: Infer from call param types. */ None, arg_id,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let callee_type = state.registry.forall(callee_type_id).clone();
    // We use the params of the callee _type_ rather than the params of the
    // callee itself, since the callee type is a normal form, which guarantees
    // that its params are normal forms.
    let callee_type_param_ids = state
        .registry
        .param_list(callee_type.param_list_id)
        .to_vec();
    {
        let expected_arity = callee_type_param_ids.len();
        let actual_arity = arg_ids.len();
        if callee_type_param_ids.len() != arg_type_ids.len() {
            return Err(TypeCheckError::WrongNumberOfArguments {
                call_id: call_id,
                expected: expected_arity,
                actual: actual_arity,
            });
        }
    }
    for (i, (callee_type_param_id, arg_type_id)) in callee_type_param_ids
        .iter()
        .copied()
        .zip(arg_type_ids.iter().copied())
        .enumerate()
    {
        let substituted_param_type_id = {
            let callee_type_param = state.registry.param(callee_type_param_id);
            // This is safe because the param is the param of a normal
            // form Forall node, which guarantees that its type is a
            // normal form.
            let unsubstituted = NormalFormId::unchecked_new(callee_type_param.type_id);
            let substitutions: Vec<Substitution> =
                arg_type_ids[..i]
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(j, arg_type_id)| {
                        let db_index = DbIndex(i - j - 1);
                        let param_name_id = state.registry.param(callee_type_param_ids[j]).name_id;
                        Substitution {
                            from: NormalFormId::unchecked_new(ExpressionId::Name(
                                add_name_expression(state.registry, vec![param_name_id], db_index),
                            )),
                            to: arg_type_id.upshift(i, state.registry),
                        }
                    })
                    .collect();
            let substituted = unsubstituted
                .raw()
                .subst_all(&substitutions, state.registry)
                .0
                .downshift(i, state.registry);
            evaluate_well_typed_expression(state, substituted)
        };
        if !is_left_type_assignable_to_right_type(state, arg_type_id, substituted_param_type_id) {
            return Err(TypeCheckError::TypeMismatch {
                expression_id: arg_ids[i],
                expected_type_id: substituted_param_type_id,
                actual_type_id: arg_type_id,
            });
        }
    }

    let substituted_output_id = {
        let unsubstituted = NormalFormId::unchecked_new(callee_type.output_id);
        let arity = callee_type_param_ids.len();
        let substitutions: Vec<Substitution> = arg_type_ids
            .iter()
            .copied()
            .enumerate()
            .map(|(j, arg_type_id)| {
                let db_index = DbIndex(arity - j - 1);
                let param_name_id = state.registry.param(callee_type_param_ids[j]).name_id;
                Substitution {
                    from: NormalFormId::unchecked_new(ExpressionId::Name(add_name_expression(
                        state.registry,
                        vec![param_name_id],
                        db_index,
                    ))),
                    to: arg_type_id.upshift(arity, state.registry),
                }
            })
            .collect();
        let substituted = unsubstituted
            .raw()
            .subst_all(&substitutions, state.registry)
            .0
            .downshift(arity, state.registry);
        evaluate_well_typed_expression(state, substituted)
    };
    Ok(substituted_output_id)
}

fn get_type_of_fun(state: &mut State, fun_id: NodeId<Fun>) -> Result<NormalFormId, TypeCheckError> {
    let original_context_len = state.context.len();

    let fun = state.registry.fun(fun_id).clone();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(state, fun.param_list_id)?;
    {
        let return_type_type_id = get_type_of_expression(state, None, fun.return_type_id)?;
        if !is_term_equal_to_type0_or_type1(state, return_type_type_id) {
            return Err(TypeCheckError::IllegalTypeExpression(fun.return_type_id));
        }
    }
    let normalized_return_type_id = evaluate_well_typed_expression(state, fun.return_type_id);

    let fun_type_id = NormalFormId::unchecked_new(ExpressionId::Forall(
        state.registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: normalized_return_type_id.raw(),
        }),
    ));

    let shifted_fun_id = fun_id.upshift(state.context.len() - original_context_len, state.registry);
    let normalized_fun_id =
        evaluate_well_typed_expression(state, ExpressionId::Fun(shifted_fun_id));
    state.context.push(ContextEntry {
        type_id: fun_type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_fun_id,
        },
    });

    let normalized_body_type_id =
        get_type_of_expression(state, Some(normalized_return_type_id), fun.body_id)?;
    if !is_left_type_assignable_to_right_type(
        state,
        normalized_body_type_id,
        normalized_return_type_id,
    ) {
        return Err(TypeCheckError::TypeMismatch {
            expression_id: fun.body_id,
            expected_type_id: normalized_return_type_id,
            actual_type_id: normalized_body_type_id,
        });
    }

    state.context.pop_n(fun.param_list_id.len + 1);
    Ok(fun_type_id)
}

fn get_type_of_match(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    match_id: NodeId<Match>,
) -> Result<NormalFormId, TypeCheckError> {
    let match_ = state.registry.match_(match_id).clone();
    let matchee_type_id = get_type_of_expression(state, None, match_.matchee_id)?;
    let matchee_type = if let Some(t) = try_as_adt_expression(state, matchee_type_id) {
        t
    } else {
        return Err(TypeCheckError::NonAdtMatchee {
            matchee_id: match_.matchee_id,
            type_id: matchee_type_id,
        });
    };

    verify_variant_to_case_bijection(
        state.registry,
        matchee_type.variant_name_list_id,
        match_.case_list_id,
    )?;

    let case_ids = state.registry.match_case_list(match_.case_list_id).to_vec();
    let mut first_case_type_id = None;
    for case_id in case_ids {
        let case_type_id = get_type_of_match_case(
            state,
            coercion_target_id,
            case_id,
            matchee_type_id,
            matchee_type,
        )?;
        if let Some(first_case_type_id) = first_case_type_id {
            if !is_left_type_assignable_to_right_type(state, case_type_id, first_case_type_id) {
                let case = state.registry.match_case(case_id);
                return Err(TypeCheckError::TypeMismatch {
                    expression_id: case.output_id,
                    expected_type_id: first_case_type_id,
                    actual_type_id: case_type_id,
                });
            }
        } else {
            first_case_type_id = Some(case_type_id);
        }
    }

    if let Some(first_case_type_id) = first_case_type_id {
        Ok(first_case_type_id)
    } else {
        // If `first_case_type_id` is `None`, then `case_ids` is empty, which
        // means the matchee has any empty type.
        // Thus, the match should have an empty type.
        Ok(matchee_type_id)
    }
}

fn get_type_of_match_case(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    case_id: NodeId<MatchCase>,
    matchee_type_id: NormalFormId,
    matchee_type: AdtExpression,
) -> Result<NormalFormId, TypeCheckError> {
    let case = state.registry.match_case(case_id).clone();
    let case_arity = case.param_list_id.len;
    let parameterized_type_id =
        add_case_params_to_context_and_get_constructed_type(state, case_id, matchee_type)?;

    let original_coercion_target_id = coercion_target_id;
    let shifted_coercion_target_id = coercion_target_id
        .map(|coercion_target_id| coercion_target_id.upshift(case_arity, state.registry));

    let fusion = fuse_left_to_right(state, matchee_type_id, parameterized_type_id);
    if fusion.has_exploded {
        if let Some(coercion_target_id) = coercion_target_id {
            state.context.pop_n(case_arity);
            return Ok(coercion_target_id);
        }
    }

    let (mut substituted_context, substituted_coercion_target_id) =
        apply_dynamic_substitutions_with_compounding(
            state,
            fusion.substitutions,
            shifted_coercion_target_id.map(NormalFormId::raw),
        );

    let State {
        context: original_context,
        registry,
        equality_checker,
    } = state;
    let mut state = State {
        registry,
        context: &mut substituted_context,
        equality_checker,
    };
    let state = &mut state;

    let normalized_substituted_coercion_target_id = substituted_coercion_target_id
        .map(|target_id| evaluate_well_typed_expression(state, target_id));
    let output_type_id = get_type_of_expression(
        state,
        normalized_substituted_coercion_target_id,
        case.output_id,
    )?;

    let can_be_coerced = matches!(
        normalized_substituted_coercion_target_id,
        Some(normalized_substituted_coercion_target_id)
            if is_left_type_assignable_to_right_type(
                state,
                output_type_id,
                normalized_substituted_coercion_target_id,
            )
    );

    original_context.pop_n(case_arity);

    if can_be_coerced {
        Ok(original_coercion_target_id.expect("original_coercion_target_id must be Some if normalized_substituted_coercion_target_id is Some"))
    } else {
        match output_type_id.try_downshift(case_arity, state.registry) {
            Ok(shifted_output_type_id) => Ok(shifted_output_type_id),
            Err(_) => Err(TypeCheckError::AmbiguousOutputType { case_id }),
        }
    }
}

fn add_case_params_to_context_and_get_constructed_type(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: AdtExpression,
) -> Result<NormalFormId, TypeCheckError> {
    let case = state.registry.match_case(case_id).clone();
    let variant_dbi =
        get_db_index_for_adt_variant_of_name(state, matchee_type, case.variant_name_id);
    let variant_type_id = state.context.get_type(variant_dbi, state.registry);
    match variant_type_id.raw() {
        ExpressionId::Forall(normalized_forall_id) => {
            let normalized_forall = state.registry.forall(normalized_forall_id);
            let normalized_param_ids = state.registry
                .param_list(normalized_forall.param_list_id)
                .to_vec();
            for &normalized_param_id in &normalized_param_ids {
                let normalized_param = state.registry.param(normalized_param_id);
                let param_type_id = NormalFormId::unchecked_new(normalized_param.type_id);
                state.context.push(ContextEntry {
                    type_id: param_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                });
            }

            Ok(NormalFormId::unchecked_new(normalized_forall.output_id))
        }
        ExpressionId::Call(_) => {
            // In this case, the variant is nullary.
            Ok(variant_type_id)
        }
        other => panic!("A variant's type should always either be a Forall or a Call, but it was actually a {:?}", other),
    }
}

fn get_type_of_forall(
    state: &mut State,
    forall_id: NodeId<Forall>,
) -> Result<NormalFormId, TypeCheckError> {
    let forall = state.registry.forall(forall_id).clone();
    normalize_params_and_leave_params_in_context(state, forall.param_list_id)?;

    let output_type_id = get_type_of_expression(state, None, forall.output_id)?;
    if !is_term_equal_to_type0_or_type1(state, output_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(forall.output_id));
    }

    state.context.pop_n(forall.param_list_id.len);

    Ok(type0_expression(state))
}
