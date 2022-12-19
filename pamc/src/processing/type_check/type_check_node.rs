
use super::*;

pub fn type_check_files(
    registry: &mut NodeRegistry,
    file_ids: &[FunRecursionValidated<NodeId<File>>],
) -> Result<Vec<TypeCheckWarning>, TypeCheckError> {
    let mut context = Context::with_builtins(registry);
    let mut substitution_context = SubstitutionContext::empty();
    let mut equality_checker = NodeEqualityChecker::new();
    let mut warnings = vec![];
    let mut state = State {
        context: &mut context,
        substitution_context: &mut substitution_context,
        registry,
        equality_checker: &mut equality_checker,
        warnings: &mut warnings,
    };
    for &id in file_ids {
        type_check_file(&mut state, id.raw())?;
    }
    Ok(warnings)
}

fn type_check_file(state: &mut State, file_id: NodeId<File>) -> Result<(), TypeCheckError> {
    untaint_err(state, file_id, type_check_file_dirty)
}

fn type_check_file_dirty(state: &mut State, file_id: NodeId<File>) -> Result<(), Tainted<TypeCheckError>> {
    let file = state.registry.get(file_id);
    let items = state.registry.get_possibly_empty_list(file.item_list_id).to_vec();
    for &item_id in &items {
        type_check_file_item_dirty(state, item_id)??;
    }
    state.context.pop_n(items.len());
    Ok(())
}

fn type_check_file_item_dirty(state: &mut State, item: FileItemNodeId) -> Result<PushWarning, Tainted<TypeCheckError>> {
    match item {
        FileItemNodeId::Type(type_statement) => type_check_type_statement_dirty(state, type_statement),
        FileItemNodeId::Let(let_statement) => type_check_let_statement_dirty(state, let_statement),
    }
}

fn type_check_type_statement_dirty(
    state: &mut State,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    type_check_type_constructor_dirty(state, type_statement_id)??;

    let type_statement = state.registry.get(type_statement_id);
    let variant_ids = state
        .registry.get_possibly_empty_list(type_statement.variant_list_id)
        .to_vec();
    for variant_id in variant_ids {
        type_check_type_variant_dirty(state, variant_id)??;
    }

    Ok(with_push_warning(()))
}

fn type_check_type_constructor_dirty(
    state: &mut State,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let type_statement = state.registry.get(type_statement_id).clone();
    let arity = type_statement.param_list_id.len();
    let normalized_param_list_id =
    normalize_optional_params_and_leave_params_in_context_dirty(state, type_statement.param_list_id)??;
    let type_constructor_type_id = NormalFormId::unchecked_new(
        PossiblyNullaryForall {
            id: dummy_id(),
            span: None,
            param_list_id: normalized_param_list_id,
            output_id: type0_expression(state).raw(),
        }
        .into_id(state.registry)
        .without_spans(state.registry),
    );
    state.context.pop_n(arity);

    let variant_name_list_id = {
        let variant_ids = state.registry.get_possibly_empty_list(type_statement.variant_list_id);
        let variant_name_ids: Vec<_> = variant_ids
            .iter()
            .map(|&variant_id| state.registry.get(variant_id).name_id)
            .collect();
        state.registry.add_possibly_empty_list(variant_name_ids)
    };
    Ok(state.context.push(ContextEntry {
        type_id: type_constructor_type_id,
        definition: ContextEntryDefinition::Adt {
            variant_name_list_id,
        },
    }))
}

pub(super) fn type_check_unlabeled_param_dirty(
    state: &mut State,
    param_id: NodeId<UnlabeledParam>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let param = state.registry.get(param_id).clone();
    let param_type_type_id = get_type_of_expression_dirty(state, None, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(state, param_type_type_id) {
        return tainted_err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(state, param.type_id);
    Ok(state.context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    }))
}

pub(super) fn type_check_labeled_param_dirty(
    state: &mut State,
    param_id: NodeId<LabeledParam>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let param = state.registry.get(param_id).clone();
    let param_type_type_id = get_type_of_expression_dirty(state, None, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(state, param_type_type_id) {
        return tainted_err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(state, param.type_id);
    Ok(state.context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    }))
}

fn type_check_type_variant_dirty(
    state: &mut State,
    variant_id: NodeId<Variant>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let variant = state.registry.get(variant_id).clone();
    let arity = variant.param_list_id.len();
    let normalized_param_list_id =
        normalize_optional_params_and_leave_params_in_context_dirty(state, variant.param_list_id)??;
    type_check_expression_dirty(state, None, variant.return_type_id)?;
    let return_type_id = evaluate_well_typed_expression(state, variant.return_type_id);
    let type_id = NormalFormId::unchecked_new(
        PossiblyNullaryForall {
            id: dummy_id(),
            span: None,
            param_list_id: normalized_param_list_id,
            output_id: return_type_id.raw(),
        }
        .into_id(state.registry)
        .without_spans(state.registry),
    );
    state.context.pop_n(arity);
    Ok(state.context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Variant {
            name_id: variant.name_id,
        },
    }))
}

fn type_check_let_statement_dirty(
    state: &mut State,
    let_statement_id: NodeId<LetStatement>,
) -> Result<PushWarning, Tainted<TypeCheckError>> {
    let let_statement = state.registry.get(let_statement_id).clone();
    let type_id = get_type_of_expression_dirty(state, None, let_statement.value_id)?;
    let normalized_value_id = evaluate_well_typed_expression(state, let_statement.value_id);
    Ok(state.context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_value_id,
        },
    }))
}

fn type_check_expression_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    expression: ExpressionId,
) -> Result<(), Tainted<TypeCheckError>> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression_dirty(state, coercion_target_id, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: ExpressionId,
) -> Result<NormalFormId, TypeCheckError> {
    fn f(
        state: &mut State,
        (coercion_target_id, id): (Option<NormalFormId>, ExpressionId),
    ) -> Result<NormalFormId, Tainted<TypeCheckError>> {
        get_type_of_expression_dirty(state, coercion_target_id, id)
    }

    untaint_err(state, (coercion_target_id, id), f)
}


fn get_type_of_expression_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: ExpressionId,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
     match id {
        ExpressionId::Name(name) => Ok(get_type_of_name(state, name)),
        ExpressionId::Call(call) => get_type_of_call_dirty(state, call),
        ExpressionId::Fun(fun) => get_type_of_fun_dirty(state, fun),
        ExpressionId::Match(match_) => get_type_of_match_dirty(state, coercion_target_id, match_),
        ExpressionId::Forall(forall) => get_type_of_forall_dirty(state, forall),
        ExpressionId::Check(check) => get_type_of_check_expression_dirty(state, coercion_target_id, check),
    }
}

fn get_type_of_name(state: &mut State, name_id: NodeId<NameExpression>) -> NormalFormId {
    let name = state.registry.get(name_id);
    state.context.get_type(name.db_index, state.registry)
}


fn get_type_of_call_dirty(
    state: &mut State,
    call_id: NodeId<Call>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    if let Some(corrected) = correct_call_arg_order_dirty(state, call_id)? {
        return get_type_of_call_dirty(state, corrected);
    }

    let call = state.registry.get(call_id).clone();
    let callee_type_id = get_type_of_expression_dirty(state, None, call.callee_id)?;
    let callee_type_id = if let ExpressionId::Forall(id) = callee_type_id.raw() {
        id
    } else {
        return tainted_err(TypeCheckError::IllegalCallee(call.callee_id));
    };
    let arg_ids = match call.arg_list_id {
        NonEmptyCallArgListId::Unlabeled(arg_list_id) => state.registry.get_list(arg_list_id).to_vec(),
        NonEmptyCallArgListId::UniquelyLabeled(arg_list_id) => {
            let arg_list = state.registry.get_list(arg_list_id).to_vec();
            arg_list.iter().map(|arg| arg.value_id(state.registry)).collect()
        }
    };
    let normalized_arg_ids: Vec<NormalFormId> = arg_ids
        .iter()
        .copied()
        .map(|arg_id| evaluate_well_typed_expression(state, arg_id))
        .collect();

    let callee_type = state.registry.get(callee_type_id).clone();
    {
        let expected_arity = callee_type.param_list_id.len();
        let actual_arity = arg_ids.len();
        if expected_arity != actual_arity {
            return tainted_err(TypeCheckError::WrongNumberOfArguments {
                call_id: call_id,
                expected: expected_arity,
                actual: actual_arity,
            });
        }
    }
    
    let (callee_type_param_name_ids, callee_type_param_type_ids) = get_names_and_types_of_params(state, callee_type.param_list_id);
    for (i, callee_type_param_type_id) in callee_type_param_type_ids
        .iter()
        .copied()
        .enumerate()
    {
        let substituted_param_type_id = {
            // This is safe because the param is the param of a normal
            // form Forall node, which guarantees that its type is a
            // normal form.
            let unsubstituted = NormalFormId::unchecked_new(callee_type_param_type_id);
            let substitutions: Vec<Substitution> = normalized_arg_ids[..i]
                .iter()
                .copied()
                .enumerate()
                .map(|(j, normalized_arg_id)| {
                    let db_index = DbIndex(i - j - 1);
                    let param_name_id = callee_type_param_name_ids[j];
                    Substitution {
                        from: ExpressionId::Name(add_name_expression(
                            state.registry,
                            NonEmptyVec::singleton(param_name_id),
                            db_index,
                        )),
                        to: normalized_arg_id.upshift(i, state.registry).raw(),
                    }
                })
                .collect();
            let substituted = unsubstituted
                .raw()
                .subst_all(&substitutions, &mut state.without_context())
                .downshift(i, state.registry);
            evaluate_well_typed_expression(state, substituted)
        };

        let arg_type_id = get_type_of_expression_dirty(state, Some(substituted_param_type_id), arg_ids[i])?;

        if !is_left_type_assignable_to_right_type(state, arg_type_id, substituted_param_type_id) {
            return tainted_err(TypeCheckError::TypeMismatch {
                expression_id: arg_ids[i],
                expected_type_id: substituted_param_type_id,
                actual_type_id: arg_type_id,
            });
        }
    }

    let substituted_output_id = {
        let unsubstituted = NormalFormId::unchecked_new(callee_type.output_id);
        let arity = callee_type.param_list_id.len();
        let substitutions: Vec<Substitution> = normalized_arg_ids
            .iter()
            .copied()
            .enumerate()
            .map(|(j, normalized_arg_id)| {
                let db_index = DbIndex(arity - j - 1);
                let param_name_id = callee_type_param_name_ids[j];
                Substitution {
                    from: ExpressionId::Name(add_name_expression(
                        state.registry,
                        NonEmptyVec::singleton(param_name_id),
                        db_index,
                    )),
                    to: normalized_arg_id.upshift(arity, state.registry).raw(),
                }
            })
            .collect();
        let substituted = unsubstituted
            .raw()
            .subst_all(&substitutions, &mut state.without_context())
            .downshift(arity, state.registry);
        evaluate_well_typed_expression(state, substituted)
    };
    Ok(substituted_output_id)
}

/// If the params and args are both labeled AND the label order is correct,
/// this returns `Ok(None)`.
/// Otherwise, it tries to return `Ok(Some(new_call_id))` where `new_call_id`
/// is the result or correcting the arg order to match the param order.
/// If it cannot do this (e.g., the labeledness of the params and args doesn't match),
/// then it returns `Err(_)`.
fn correct_call_arg_order_dirty(state: &mut State, call_id: NodeId<Call>) -> Result<Option<NodeId<Call>>, Tainted<TypeCheckError>> {
    let call = state.registry.get(call_id).clone();
    let callee_type_id = get_type_of_expression_dirty(state, None, call.callee_id)?;
    let ExpressionId::Forall(callee_type_id) = callee_type_id.raw() else {
        return tainted_err(TypeCheckError::IllegalCallee(call.callee_id));
    };
    let callee_type = state.registry.get(callee_type_id).clone();

    match (callee_type.param_list_id, call.arg_list_id) {
        (NonEmptyParamListId::Unlabeled(param_list_id), NonEmptyCallArgListId::Unlabeled(arg_list_id)) => {
            let expected_arity = param_list_id.len.get();
            let actual_arity = arg_list_id.len.get();
            if expected_arity != actual_arity {
                tainted_err(TypeCheckError::WrongNumberOfArguments {
                    call_id: call_id,
                    expected: expected_arity,
                    actual: actual_arity,
                })
            } else {
                Ok(None)
            }
        }
        (NonEmptyParamListId::UniquelyLabeled(param_list_id), NonEmptyCallArgListId::UniquelyLabeled(arg_list_id)) => {
            correct_labeled_call_arg_order_dirty(state, call_id, param_list_id, arg_list_id)
        }
        _ => tainted_err(TypeCheckError::LabelednessMismatch { call_id })
    }
}

fn correct_labeled_call_arg_order_dirty(
    state: &mut State,
    call_id: NodeId<Call>,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
    arg_list_id: NonEmptyListId<LabeledCallArgId>,
) -> Result<Option<NodeId<Call>>, Tainted<TypeCheckError>> {
    // let param_ids = state.registry.get_list(param_list_id).to_non_empty_vec();
    // let arg_ids = state.registry.get_list(arg_list_id).to_non_empty_vec();

    // let mut out = NonEmptyVec::singleton(x);
    // for (param_index, param_id) in param_ids.iter().copied().enumerate() {
    //     //
    // }

    unimplemented!()
}

fn get_type_of_fun_dirty(state: &mut State, fun_id: NodeId<Fun>) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    let fun = state.registry.get(fun_id).clone();
    // We call this "param arity" instead of simply "arity"
    // to convey the fact that it does **not** include the recursive
    // function.
    // For example, `fun f(a: A, b: B) -> C { ... }` has param arity 2,
    // even though `f` is also added to the context as a third entry
    // (to enable recursion).
    let param_arity = fun.param_list_id.len();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context_dirty(state, fun.param_list_id)??;
    {
        let return_type_type_id = get_type_of_expression_dirty(state, None, fun.return_type_id)?;
        if !is_term_equal_to_type0_or_type1(state, return_type_type_id) {
            return tainted_err(TypeCheckError::IllegalTypeExpression(fun.return_type_id));
        }
    }
    let normalized_return_type_id = evaluate_well_typed_expression(state, fun.return_type_id);

    let fun_type_id = NormalFormId::unchecked_new(ExpressionId::Forall(
        state.registry.add(Forall {
            id: dummy_id(),
            span: None,
            param_list_id: normalized_param_list_id,
            output_id: normalized_return_type_id.raw(),
        }).without_spans(state.registry),
    ));

    {
        let shifted_fun_type_id = fun_type_id.upshift(param_arity, state.registry);
        let shifted_fun_id = fun_id.upshift(param_arity, state.registry);
        let shifted_fun = state.registry.get(shifted_fun_id).clone();
        let body_skipped_fun_id = state.registry.add(Fun {
            skip_type_checking_body: true,
            ..shifted_fun
        });
        let normalized_fun_id =
            evaluate_well_typed_expression(state, ExpressionId::Fun(body_skipped_fun_id));
        state.context.push(ContextEntry {
            type_id: shifted_fun_type_id,
            definition: ContextEntryDefinition::Alias {
                value_id: normalized_fun_id,
            },
        })?;
    }

    // We need to upshift the return type by one level before comparing it
    // to the body type, to account for the fact that the function has been
    // added to the context.
    let normalized_return_type_id_relative_to_body = {
        let shifted_return_type_id = fun.return_type_id.upshift(1, state.registry);
        evaluate_well_typed_expression(state, shifted_return_type_id)
    };
    // Shadow the old variable to prevent it from being accidentally used.
    #[allow(unused_variables)]
    let normalized_return_type_id = ();

    if !fun.skip_type_checking_body {
        let normalized_body_type_id = get_type_of_expression_dirty(
            state,
            Some(normalized_return_type_id_relative_to_body),
            fun.body_id,
        )?;
        if !is_left_type_assignable_to_right_type(
            state,
            normalized_body_type_id,
            normalized_return_type_id_relative_to_body,
        ) {
            return tainted_err(TypeCheckError::TypeMismatch {
                expression_id: fun.body_id,
                expected_type_id: normalized_return_type_id_relative_to_body,
                actual_type_id: normalized_body_type_id,
            });
        }
    }

    state.context.pop_n(param_arity + 1);
    Ok(fun_type_id)
}

fn get_type_of_match_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    match_id: NodeId<Match>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    let match_ = state.registry.get(match_id).clone();
    let matchee_type_id = get_type_of_expression_dirty(state, None, match_.matchee_id)?;
    let matchee_type = if let Some(t) = try_as_normal_form_adt_expression(state, matchee_type_id) {
        t
    } else {
        return tainted_err(TypeCheckError::NonAdtMatchee {
            matchee_id: match_.matchee_id,
            type_id: matchee_type_id,
        });
    };
    let normalized_matchee_id = evaluate_well_typed_expression(state, match_.matchee_id);

    verify_variant_to_case_bijection(
        state.registry,
        matchee_type.variant_name_list_id,
        match_.case_list_id,
    ).map_err(Tainted::new)?;

    let case_ids = state.registry.get_possibly_empty_list(match_.case_list_id).to_vec();
    let mut first_case_type_id = None;
    for case_id in case_ids {
        let case_type_id = get_type_of_match_case_dirty(
            state,
            coercion_target_id,
            case_id,
            normalized_matchee_id,
            matchee_type_id,
            matchee_type,
        )?;
        if let Some(first_case_type_id) = first_case_type_id {
            if !is_left_type_assignable_to_right_type(state, case_type_id, first_case_type_id) {
                let case = state.registry.get(case_id);
                return tainted_err(TypeCheckError::TypeMismatch {
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

fn get_type_of_match_case_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    case_id: NodeId<MatchCase>,
    normalized_matchee_id: NormalFormId,
    matchee_type_id: NormalFormId,
    matchee_type: NormalFormAdtExpression,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    let case_arity = case.param_list_id.len();
    let (parameterized_matchee_id, parameterized_matchee_type_id) =
        add_case_params_to_context_and_get_constructed_matchee_and_type_dirty(
            state,
            case_id,
            matchee_type,
        )??;

    let original_coercion_target_id = coercion_target_id;
    let coercion_target_id =
        coercion_target_id.map(|target_id| target_id.upshift(case_arity, state.registry));

    let normalized_matchee_id = normalized_matchee_id.upshift(case_arity, state.registry);
    let matchee_type_id = matchee_type_id.upshift(case_arity, state.registry);

    state.substitution_context.push(SubstitutionContextEntry {
        context_len: state.context.len(),
        unadjusted_substitutions: vec![
            DynamicSubstitution(normalized_matchee_id, parameterized_matchee_id),
            DynamicSubstitution(matchee_type_id, parameterized_matchee_type_id),
        ],
    });

    let output_type_id = get_type_of_expression_dirty(state, coercion_target_id, case.output_id)?;

    if let Some(coercion_target_id) = coercion_target_id {
        let can_be_coerced = is_left_type_assignable_to_right_type(
            state,
            output_type_id,
            coercion_target_id,
        );

        state.context.pop_n(case_arity);
        state.substitution_context.pop();

        return if can_be_coerced {
            Ok(original_coercion_target_id.expect("original_coercion_target_id must be Some if normalized_substituted_coercion_target_id is Some"))
        } else {
            tainted_err(TypeCheckError::TypeMismatch {
                expression_id: case.output_id,
                actual_type_id: output_type_id,
                // TODO: This might be confusing to the user since it's
                // undergone substitution.
                // In the future, we'll include this in substitution
                // tracking (if we implement it).
                expected_type_id: coercion_target_id,
            })
        }
    }

    state.context.pop_n(case_arity);
    state.substitution_context.pop();

    match output_type_id.try_downshift(case_arity, state.registry) {
        Ok(output_type_id) => Ok(output_type_id),
        Err(_) => {
            tainted_err(TypeCheckError::AmbiguousOutputType { case_id })
        }
    }
}

fn add_case_params_to_context_and_get_constructed_matchee_and_type_dirty(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: NormalFormAdtExpression,
) -> Result<WithPushWarning<(NormalFormId, NormalFormId)>, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    let variant_dbi =
        get_db_index_for_adt_variant_of_name(state, matchee_type, case.variant_name_id);
    let variant_type_id = state.context.get_type(variant_dbi, state.registry);
    let fully_qualified_variant_name_component_ids: NonEmptyVec<NodeId<Identifier>> = {
        let matchee_type_name = state.registry.get(matchee_type.type_name_id);
        let matchee_type_name_component_ids = state
            .registry.get_list(matchee_type_name.component_list_id)
            .to_vec();
        NonEmptyVec::from_pushed(matchee_type_name_component_ids, case.variant_name_id)
    };
    match variant_type_id.raw() {
        ExpressionId::Forall(normalized_forall_id) => {
            let normalized_forall = state.registry.get(normalized_forall_id).clone();
            let expected_case_param_arity = normalized_forall.param_list_id.non_zero_len();
            let Some(case_param_list_id) = case.param_list_id else {
                return tainted_err(TypeCheckError::WrongNumberOfCaseParams {
                    case_id,
                    expected: expected_case_param_arity.get(),
                    actual: 0,
                });
            };
            if case_param_list_id.len != expected_case_param_arity {
                return tainted_err(TypeCheckError::WrongNumberOfCaseParams {
                    case_id,
                    expected: expected_case_param_arity.get(),
                    actual: case.param_list_id.len(),
                });
            }

            let param_type_ids = get_param_type_ids(state, normalized_forall.param_list_id)
                // This is safe because every param type of a normal form Forall
                // is also a normal form itself.
                .into_mapped(NormalFormId::unchecked_new);
            for &param_type_id in &param_type_ids {
                state.context.push(ContextEntry {
                    type_id: param_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                })?;
            }

            let (parameterized_matchee_id, parameterized_matchee_type_id) = {
                let shifted_variant_dbi = DbIndex(variant_dbi.0 + case.param_list_id.len());
                let callee_id = ExpressionId::Name(add_name_expression(
                    state.registry,
                    fully_qualified_variant_name_component_ids,
                    shifted_variant_dbi,
                ));
                let case_param_ids = state.registry.get_list(case_param_list_id).to_non_empty_vec();
                let case_param_arity = case_param_ids.len();
                let arg_ids = case_param_ids
                    .as_non_empty_slice()
                    .enumerate_to_mapped(|(index, &case_param_id)| {
                        ExpressionId::Name(add_name_expression(
                            state.registry,
                            NonEmptyVec::singleton(case_param_id),
                            DbIndex(case_param_arity - index - 1),
                        ))
                    });
                // TODO: Properly construct parameterized matchee id
                // after we add support for labeled match case params.
                let arg_list_id = NonEmptyCallArgListId::Unlabeled(state.registry.add_list(arg_ids));
                let parameterized_matchee_id = NormalFormId::unchecked_new(ExpressionId::Call(
                    state.registry.add(Call {
                        id: dummy_id(),
                        span: None,
                        callee_id,
                        arg_list_id,
                    }).without_spans(state.registry),
                ));

                let output_substitutions: Vec<Substitution> = case_param_ids
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(raw_index, case_param_id)| {
                        let db_index = DbIndex(case_param_ids.len() - raw_index - 1);
                        let param_name_expression_id = ExpressionId::Name(add_name_expression(state.registry, NonEmptyVec::singleton(case_param_id), db_index));
                        Substitution {
                            // We don't care about the name of the `from` `NameExpression`
                            // because the comparison is only based on the `db_index`.
                            from: param_name_expression_id,
                            to: param_name_expression_id,
                        }
                    })
                    .collect();
                let substituted_output_id = normalized_forall.output_id.subst_all(
                    &output_substitutions,
                    &mut state.without_context(),
                );
                let parameterized_matchee_type_id = NormalFormId::unchecked_new(substituted_output_id);

                (parameterized_matchee_id, parameterized_matchee_type_id)
            };
            
            Ok(with_push_warning((parameterized_matchee_id, parameterized_matchee_type_id)))
        }
        ExpressionId::Name(_) => {
            // In this case, the variant type is nullary.

            let expected_case_param_arity = 0;
            if case.param_list_id.len() != expected_case_param_arity {
                return tainted_err(TypeCheckError::WrongNumberOfCaseParams {
                    case_id,
                    expected: expected_case_param_arity,
                    actual: case.param_list_id.len(),
                });
            }
            // Since the case is nullary, we shift by zero.
            let shifted_variant_dbi = variant_dbi;
            let parameterized_matchee_id =
                NormalFormId::unchecked_new(ExpressionId::Name(add_name_expression(
                    state.registry,
                    fully_qualified_variant_name_component_ids,
                    shifted_variant_dbi,
                )));
            Ok(with_push_warning((parameterized_matchee_id, variant_type_id)))
        }
        other => panic!("A variant's type should always either be a Forall or a Name, but it was actually a {:?}", other),
    }
}

fn get_type_of_forall_dirty(
    state: &mut State,
    forall_id: NodeId<Forall>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    let forall = state.registry.get(forall_id).clone();
    let _param_list_id = normalize_params_and_leave_params_in_context_dirty(state, forall.param_list_id)??;

    let output_type_id = get_type_of_expression_dirty(state, None, forall.output_id)?;
    if !is_term_equal_to_type0_or_type1(state, output_type_id) {
        return tainted_err(TypeCheckError::IllegalTypeExpression(forall.output_id));
    }

    state.context.pop_n(forall.param_list_id.len());

    Ok(type0_expression(state))
}


fn get_type_of_check_expression_dirty(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    check_id: NodeId<Check>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    add_check_expression_warnings(state, coercion_target_id, check_id).map_err(Tainted::new)?;
    let check = state.registry.get(check_id).clone();
    get_type_of_expression_dirty(state, coercion_target_id, check.output_id)
}

fn add_check_expression_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    check_id: NodeId<Check>,
) -> Result<(), TypeCheckError> {
    let warnings = get_check_expression_warnings(state, coercion_target_id, check_id);
    state.warnings.extend(warnings);
    Ok(())
}

fn get_check_expression_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    check_id: NodeId<Check>,
) -> Vec<TypeCheckWarning> {
    let assertion_ids = {
        let check = state.registry.get(check_id);
        state.registry.get_list(check.assertion_list_id).to_vec()
    };
    assertion_ids
        .into_iter()
        .map(|assertion_id| get_check_assertion_warnings(state, coercion_target_id, assertion_id))
        .flatten()
        .collect()
}


fn get_check_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion_id: NodeId<CheckAssertion>,
) -> Vec<TypeCheckWarning> {
    let assertion = state.registry.get(assertion_id).clone();
    match assertion.kind {
        CheckAssertionKind::Type => get_type_assertion_warnings(state, coercion_target_id, assertion).into_iter().map(TypeCheckWarning::TypeAssertion).collect(),
        CheckAssertionKind::NormalForm => get_normal_form_assertion_warnings(state, coercion_target_id, assertion),
    }
}

fn get_type_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
) -> Vec<TypeAssertionWarning> {
    match assertion.left_id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { .. } => vec![TypeAssertionWarning::GoalLhs(assertion.id)],
        GoalKwOrPossiblyInvalidExpressionId::Expression(expression_id) => get_non_goal_type_assertion_warnings(state, coercion_target_id, assertion, expression_id),
    }
}

fn get_non_goal_type_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
    left_id: PossiblyInvalidExpressionId,
) -> Vec<TypeAssertionWarning> {
    let left_correctness = get_type_correctness_of_possibly_invalid_expression(state, coercion_target_id, left_id);
    let right_correctness = get_type_correctness_of_question_mark_or_possibly_invalid_expression(state, coercion_target_id, assertion.right_id);
    
    match (left_correctness, right_correctness) {
        (Ok((left_expression_id, left_type_id)), QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(right_expression_id, _right_type_id)) => {
            let normalized_right_expression_id = evaluate_well_typed_expression(state, right_expression_id);
            match apply_substitutions_from_substitution_context(state, ((left_type_id,), (normalized_right_expression_id,))) {
                Ok(((rewritten_left_type_id,), (rewritten_right_id,),)) => {
                    if are_types_mutually_assignable(state, rewritten_left_type_id, rewritten_right_id) {
                        vec![]
                    } else {
                        vec![TypeAssertionWarning::TypesDoNotMatch {
                            left_id: left_expression_id,
                            rewritten_left_type_id,
                            original_and_rewritten_right_ids: Ok((right_expression_id, rewritten_right_id)),
                        }]
                    }
                },
                Err(Exploded) => vec![],
            }
        }
        (Ok((left_expression_id, left_type_id)), QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark) => {
            let (rewritten_left_type_id,) =
                match apply_substitutions_from_substitution_context(state, (left_type_id,)) {
                    Ok(rewritten) => rewritten,
                    Err(Exploded) => (left_type_id,),
                };
            vec![TypeAssertionWarning::TypesDoNotMatch {
                left_id: left_expression_id,
                rewritten_left_type_id,
                original_and_rewritten_right_ids: Err(RhsIsQuestionMark),
            }]
        }
        (other_left, other_right) => {
            let mut out = vec![];

            if let Err(reason) = other_left {
                out.push(TypeAssertionWarning::CompareeTypeCheckFailure(reason));
            }
            if let QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(reason) = other_right {
                out.push(TypeAssertionWarning::CompareeTypeCheckFailure(reason));
            }
            
            out
        }
    }
}

fn get_normal_form_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
) -> Vec<TypeCheckWarning> {
    let nonwrapped = match assertion.left_id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { .. } => get_goal_normal_form_assertion_warnings(state, coercion_target_id, assertion),
        GoalKwOrPossiblyInvalidExpressionId::Expression(expression_id) => get_non_goal_normal_form_assertion_warnings(state, coercion_target_id, assertion, expression_id),
    };
    nonwrapped.into_iter().map(TypeCheckWarning::NormalFormAssertion).collect()
}

// TODO: DRY

fn get_goal_normal_form_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
) -> Vec<NormalFormAssertionWarning> {
    if let Some(coercion_target_id) = coercion_target_id {
        get_goal_normal_form_assertion_warnings_given_goal_exists(state, coercion_target_id, assertion)
    } else {
        vec![NormalFormAssertionWarning::NoGoalExists(assertion.id)]
    }
}

fn get_goal_normal_form_assertion_warnings_given_goal_exists(
    state: &mut State,
    goal_id: NormalFormId,
    assertion: CheckAssertion,
) -> Vec<NormalFormAssertionWarning> {
    let coercion_target_id = Some(goal_id);
    // TODO: DRY (this is copied from `get_non_goal_normal_form_assertion_warnings`)
    let right_correctness = get_type_correctness_of_question_mark_or_possibly_invalid_expression(state, coercion_target_id, assertion.right_id);
    
    match right_correctness {
        QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(right_expression_id, _right_type_id) => {
            let normalized_right_expression_id = evaluate_well_typed_expression(state, right_expression_id);
            match apply_substitutions_from_substitution_context(state, ((goal_id,), (normalized_right_expression_id,))) {
                Ok(((rewritten_goal_id,), (rewritten_right_expression_id,),)) => {
                    if are_types_mutually_assignable(state, rewritten_goal_id, rewritten_right_expression_id) {
                        vec![]
                    } else {
                        vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                            left_id: Err(LhsIsGoalKw),
                            rewritten_left_id: rewritten_goal_id,
                            original_and_rewritten_right_ids: Ok((right_expression_id, rewritten_right_expression_id)),
                        }]
                    }
                },
                Err(Exploded) => vec![],
            }
        }
        QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark => {
            let (rewritten_goal_id,) =
                match apply_substitutions_from_substitution_context(state, (goal_id,)) {
                    Ok(rewritten) => rewritten,
                    Err(Exploded) => (goal_id,),
                };
            vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                left_id: Err(LhsIsGoalKw),
                rewritten_left_id: rewritten_goal_id,
                original_and_rewritten_right_ids: Err(RhsIsQuestionMark),
            }]
        }
        other_right => {
            if let QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(reason) = other_right {
                vec![NormalFormAssertionWarning::CompareeTypeCheckFailure(reason)]
            } else {
                vec![]
            }
        }
    }
}

fn get_non_goal_normal_form_assertion_warnings(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    assertion: CheckAssertion,
    left_id: PossiblyInvalidExpressionId,
) -> Vec<NormalFormAssertionWarning> {
    let left_correctness = get_type_correctness_of_possibly_invalid_expression(state, coercion_target_id, left_id);
    let right_correctness = get_type_correctness_of_question_mark_or_possibly_invalid_expression(state, coercion_target_id, assertion.right_id);
    
    match (left_correctness, right_correctness) {
        (Ok((left_expression_id, _left_type_id)), QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(right_expression_id, _right_type_id)) => {
            let normalized_left_expression_id = evaluate_well_typed_expression(state, left_expression_id);
            let normalized_right_expression_id = evaluate_well_typed_expression(state, right_expression_id);
            match apply_substitutions_from_substitution_context(state, ((normalized_left_expression_id,), (normalized_right_expression_id,))) {
                Ok(((rewritten_left_expression_id,), (rewritten_right_expression_id,),)) => {
                    if are_types_mutually_assignable(state, rewritten_left_expression_id, rewritten_right_expression_id) {
                        vec![]
                    } else {
                        vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                            left_id: Ok(left_expression_id),
                            rewritten_left_id: rewritten_left_expression_id,
                            original_and_rewritten_right_ids: Ok((right_expression_id, rewritten_right_expression_id)),
                        }]
                    }
                },
                Err(Exploded) => vec![],
            }
        }
        (Ok((left_expression_id, left_type_id)), QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark) => {
            let (rewritten_left_type_id,) =
                match apply_substitutions_from_substitution_context(state, (left_type_id,)) {
                    Ok(rewritten) => rewritten,
                    Err(Exploded) => (left_type_id,),
                };
            vec![NormalFormAssertionWarning::CompareesDoNotMatch {
                left_id: Ok(left_expression_id),
                rewritten_left_id: rewritten_left_type_id,
                original_and_rewritten_right_ids: Err(RhsIsQuestionMark),
            }]
        }
        (other_left, other_right) => {
            let mut out = vec![];

            if let Err(reason) = other_left {
                out.push(NormalFormAssertionWarning::CompareeTypeCheckFailure(reason));
            }
            if let QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(reason) = other_right {
                out.push(NormalFormAssertionWarning::CompareeTypeCheckFailure(reason));
            }
            
            out
        }
    }
}

fn get_type_correctness_of_possibly_invalid_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: PossiblyInvalidExpressionId,
) -> Result<(ExpressionId, NormalFormId), TypeCheckFailureReason> {
    match id {
        PossiblyInvalidExpressionId::Invalid(untypecheckable) => Err(TypeCheckFailureReason::CannotTypeCheck(untypecheckable)),
        PossiblyInvalidExpressionId::Valid(expression_id) => {
            let type_id_or_err = get_type_of_expression(state, coercion_target_id, expression_id);
            match type_id_or_err {
                Ok(type_id) => Ok((expression_id, type_id)),
                Err(type_check_err) => Err(TypeCheckFailureReason::TypeCheckError(expression_id, type_check_err)),
            }
        }
    }
}

enum QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness {
    Correct(ExpressionId, NormalFormId),
    Incorrect(TypeCheckFailureReason),
    QuestionMark,
}

fn get_type_correctness_of_question_mark_or_possibly_invalid_expression(
    state: &mut State,
    coercion_target_id: Option<NormalFormId>,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
) -> QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness {
    match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { .. } => QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::QuestionMark,
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(possibly_typecheckable) => match possibly_typecheckable {
            PossiblyInvalidExpressionId::Invalid(untypecheckable) => QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(TypeCheckFailureReason::CannotTypeCheck(untypecheckable)),
            PossiblyInvalidExpressionId::Valid(typecheckable) => match get_type_of_expression(state, coercion_target_id, typecheckable) {
                Ok(type_id) => QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Correct(typecheckable, type_id),
                Err(err) => QuestionMarkOrPossiblyInvalidExpressionTypeCorrectness::Incorrect(TypeCheckFailureReason::TypeCheckError(typecheckable, err)),
            },
        }
    }
}
