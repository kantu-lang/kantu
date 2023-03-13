use super::*;

pub(in crate::processing::type_check) fn get_type_of_call_dirty(
    state: &mut State,
    call_id: NodeId<Call>,
) -> Result<NormalFormId, Tainted<TypeCheckError>> {
    if let Some(corrected) = correct_call_arg_order_dirty(state, call_id)? {
        // TODO: Emit warning about incorrect arg order.
        return get_type_of_call_dirty(state, corrected);
    }

    let call = state.registry.get(call_id).clone();
    let callee_type_id = get_type_of_expression_dirty(state, None, call.callee_id)?;
    let callee_type_id = if let ExpressionId::Forall(id) = callee_type_id.raw() {
        id
    } else {
        return tainted_err(TypeCheckError::IllegalCallee {
            callee_id: call.callee_id,
            callee_type_id,
        });
    };
    let arg_ids = match call.arg_list_id {
        NonEmptyCallArgListId::Unlabeled(arg_list_id) => {
            state.registry.get_list(arg_list_id).to_vec()
        }
        NonEmptyCallArgListId::UniquelyLabeled(arg_list_id) => {
            let arg_list = state.registry.get_list(arg_list_id).to_vec();
            arg_list.iter().map(|arg| arg.value_id()).collect()
        }
    };

    // TODO: Delete

    // let normalized_arg_ids: Vec<NormalFormId> = arg_ids
    //     .iter()
    //     .copied()
    //     // TODO: Delete comment
    //     // BUG CAUSE: We call `evaluate_well_typed_expression` before
    //     // we check that the arguments are well-typed.
    //     .map(|arg_id| evaluate_well_typed_expression(state, arg_id))
    //     .collect();

    // let callee_type = state.registry.get(callee_type_id).clone();
    // {
    //     let expected_arity = callee_type.param_list_id.len();
    //     let actual_arity = arg_ids.len();
    //     if expected_arity != actual_arity {
    //         return tainted_err(TypeCheckError::WrongNumberOfArguments {
    //             call_id: call_id,
    //             expected: expected_arity,
    //             actual: actual_arity,
    //         });
    //     }
    // }

    // let (callee_type_param_name_ids, callee_type_param_type_ids) =
    //     get_names_and_types_of_params(state, callee_type.param_list_id);
    // for (i, callee_type_param_type_id) in callee_type_param_type_ids.iter().copied().enumerate() {
    //     let substituted_param_type_id = {
    //         // This is safe because the param is the param of a normal
    //         // form Forall node, which guarantees that its type is a
    //         // normal form.
    //         let unsubstituted = NormalFormId::unchecked_new(callee_type_param_type_id);
    //         let substitutions: Vec<Substitution> = normalized_arg_ids[..i]
    //             .iter()
    //             .copied()
    //             .enumerate()
    //             .map(|(j, normalized_arg_id)| {
    //                 let db_index = DbIndex(i - j - 1);
    //                 let param_name_id = callee_type_param_name_ids[j];
    //                 Substitution {
    //                     from: ExpressionId::Name(add_name_expression(
    //                         state.registry,
    //                         NonEmptyVec::singleton(param_name_id),
    //                         db_index,
    //                     )),
    //                     to: normalized_arg_id.upshift(i, state.registry).raw(),
    //                 }
    //             })
    //             .collect();
    //         let substituted = unsubstituted
    //             .raw()
    //             .subst_all(&substitutions, &mut state.without_context())
    //             .downshift(i, state.registry);
    //         evaluate_well_typed_expression(state, substituted)
    //     };

    //     let arg_type_id =
    //         get_type_of_expression_dirty(state, Some(substituted_param_type_id), arg_ids[i])?;

    //     let equality_status =
    //         get_rewritten_term_equality_status(state, arg_type_id, substituted_param_type_id);

    //     match equality_status {
    //         RewrittenTermEqualityStatus::Equal => (),
    //         RewrittenTermEqualityStatus::Exploded => {
    //             return tainted_err(TypeCheckError::UnreachableExpression(arg_ids[i]));
    //         }
    //         RewrittenTermEqualityStatus::NotEqual => {
    //             return tainted_err(TypeCheckError::TypeMismatch {
    //                 expression_id: arg_ids[i],
    //                 expected_type_id: substituted_param_type_id,
    //                 actual_type_id: arg_type_id,
    //             });
    //         }
    //     }
    // }

    // let substituted_output_id = {
    //     let unsubstituted = NormalFormId::unchecked_new(callee_type.output_id);
    //     let arity = callee_type.param_list_id.len();
    //     let substitutions: Vec<Substitution> = normalized_arg_ids
    //         .iter()
    //         .copied()
    //         .enumerate()
    //         .map(|(j, normalized_arg_id)| {
    //             let db_index = DbIndex(arity - j - 1);
    //             let param_name_id = callee_type_param_name_ids[j];
    //             Substitution {
    //                 from: ExpressionId::Name(add_name_expression(
    //                     state.registry,
    //                     NonEmptyVec::singleton(param_name_id),
    //                     db_index,
    //                 )),
    //                 to: normalized_arg_id.upshift(arity, state.registry).raw(),
    //             }
    //         })
    //         .collect();
    //     let substituted = unsubstituted
    //         .raw()
    //         .subst_all(&substitutions, &mut state.without_context())
    //         .downshift(arity, state.registry);
    //     evaluate_well_typed_expression(state, substituted)
    // };
    // Ok(substituted_output_id)

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

    let mut normalized_visited_arg_ids: Vec<NormalFormId> = Vec::with_capacity(arg_ids.len());

    let (callee_type_param_name_ids, callee_type_param_type_ids) =
        get_names_and_types_of_params(state, callee_type.param_list_id);
    for (i, callee_type_param_type_id) in callee_type_param_type_ids.iter().copied().enumerate() {
        let substituted_param_type_id = {
            // This is safe because the param is the param of a normal
            // form Forall node, which guarantees that its type is a
            // normal form.
            let unsubstituted = NormalFormId::unchecked_new(callee_type_param_type_id);
            let substitutions: Vec<Substitution> = normalized_visited_arg_ids
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

        let arg_type_id =
            get_type_of_expression_dirty(state, Some(substituted_param_type_id), arg_ids[i])?;

        normalized_visited_arg_ids.push(evaluate_well_typed_expression(state, arg_ids[i]));

        let equality_status =
            get_rewritten_term_equality_status(state, arg_type_id, substituted_param_type_id);

        match equality_status {
            RewrittenTermEqualityStatus::Equal => (),
            RewrittenTermEqualityStatus::Exploded => {
                return tainted_err(TypeCheckError::UnreachableExpression(arg_ids[i]));
            }
            RewrittenTermEqualityStatus::NotEqual => {
                return tainted_err(TypeCheckError::TypeMismatch {
                    expression_id: arg_ids[i],
                    expected_type_id: substituted_param_type_id,
                    actual_type_id: arg_type_id,
                });
            }
        }
    }

    let normalized_arg_ids = normalized_visited_arg_ids;

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
fn correct_call_arg_order_dirty(
    state: &mut State,
    call_id: NodeId<Call>,
) -> Result<Option<NodeId<Call>>, Tainted<TypeCheckError>> {
    let call = state.registry.get(call_id).clone();
    let callee_type_id = get_type_of_expression_dirty(state, None, call.callee_id)?;
    let ExpressionId::Forall(callee_type_id) = callee_type_id.raw() else {
        return tainted_err(TypeCheckError::IllegalCallee {
            callee_id:call.callee_id,
            callee_type_id,
        });
    };
    let callee_type = state.registry.get(callee_type_id).clone();

    match (callee_type.param_list_id, call.arg_list_id) {
        (
            NonEmptyParamListId::Unlabeled(param_list_id),
            NonEmptyCallArgListId::Unlabeled(arg_list_id),
        ) => {
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
        (
            NonEmptyParamListId::UniquelyLabeled(param_list_id),
            NonEmptyCallArgListId::UniquelyLabeled(arg_list_id),
        ) => correct_uniquely_labeled_call_arg_order_dirty(
            state,
            call_id,
            param_list_id,
            arg_list_id,
        ),
        _ => tainted_err(TypeCheckError::CallLabelednessMismatch { call_id }),
    }
}

fn correct_uniquely_labeled_call_arg_order_dirty(
    state: &mut State,
    call_id: NodeId<Call>,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
    arg_list_id: NonEmptyListId<LabeledCallArgId>,
) -> Result<Option<NodeId<Call>>, Tainted<TypeCheckError>> {
    verify_every_param_has_a_corresponding_arg(state, call_id, param_list_id, arg_list_id)?;
    verify_every_arg_has_a_corresponding_param(state, call_id, param_list_id, arg_list_id)?;

    let param_ids = state.registry.get_list(param_list_id);
    let (&first_param_id, remaining_param_ids) = param_ids.to_cons();
    let remaining_param_ids = remaining_param_ids.to_vec();
    let arg_ids = state.registry.get_list(arg_list_id).to_non_empty_vec();

    let mut are_any_args_out_of_place = false;
    let mut reordered_arg_ids = {
        let first_param_label_id = state.registry.get(first_param_id).label_identifier_id();
        let (arg_index, arg_id) = get_arg_corresponding_to_label(state, first_param_label_id, arg_ids.as_ref())
            .expect("Impossible: There is no corresponding arg even though we successfully verified that every param has a corresponding arg.");
        if arg_index != 0 {
            are_any_args_out_of_place = true;
        }
        NonEmptyVec::singleton(arg_id)
    };
    for (param_index_in_remaining_params, param_id) in
        remaining_param_ids.iter().copied().enumerate()
    {
        let param_index = 1 + param_index_in_remaining_params;
        let param_label_id = state.registry.get(param_id).label_identifier_id();
        let (arg_index, arg_id) = get_arg_corresponding_to_label(state, param_label_id, arg_ids.as_ref())
            .expect("Impossible: There is no corresponding arg even though we successfully verified that every param has a corresponding arg.");
        if arg_index != param_index {
            are_any_args_out_of_place = true;
        }
        reordered_arg_ids.push(arg_id);
    }

    if are_any_args_out_of_place {
        let callee_id = state.registry.get(call_id).callee_id;
        let reordered_arg_list_id = state.registry.add_list(reordered_arg_ids);
        let reordered = state
            .registry
            .add_and_overwrite_id(Call {
                id: dummy_id(),
                span: None,
                callee_id,
                arg_list_id: NonEmptyCallArgListId::UniquelyLabeled(reordered_arg_list_id),
            })
            .without_spans(state.registry);
        Ok(Some(reordered))
    } else {
        Ok(None)
    }
}

fn verify_every_param_has_a_corresponding_arg(
    state: &mut State,
    call_id: NodeId<Call>,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
    arg_list_id: NonEmptyListId<LabeledCallArgId>,
) -> Result<(), Tainted<TypeCheckError>> {
    let param_ids = state.registry.get_list(param_list_id);
    let arg_ids = state.registry.get_list(arg_list_id);
    let missing_param_label_name_ids: Vec<NodeId<Identifier>> = param_ids
        .iter()
        .copied()
        .filter_map(|param_id| {
            let param_label_name_id = state.registry.get(param_id).label_identifier_id();
            let param_label_name: &IdentifierName = &state.registry.get(param_label_name_id).name;

            let has_corresponding_arg = arg_ids.iter().copied().any(|arg_id| {
                let arg_label_id = arg_id.label_id();
                let arg_label_name: &IdentifierName = &state.registry.get(arg_label_id).name;
                arg_label_name == param_label_name
            });
            if !has_corresponding_arg {
                Some(param_label_name_id)
            } else {
                None
            }
        })
        .collect();

    if let Ok(missing_param_label_name_ids) = NonEmptyVec::try_from(missing_param_label_name_ids) {
        let missing_label_list_id = state.registry.add_list(missing_param_label_name_ids);
        return tainted_err(TypeCheckError::MissingLabeledCallArgs {
            call_id,
            missing_label_list_id,
        });
    }

    Ok(())
}

fn verify_every_arg_has_a_corresponding_param(
    state: &State,
    call_id: NodeId<Call>,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
    arg_list_id: NonEmptyListId<LabeledCallArgId>,
) -> Result<(), Tainted<TypeCheckError>> {
    let param_ids = state.registry.get_list(param_list_id);
    let arg_ids = state.registry.get_list(arg_list_id);
    for &arg_id in arg_ids.iter() {
        let arg_label_id = arg_id.label_id();
        let arg_label_name: &IdentifierName = &state.registry.get(arg_label_id).name;
        let has_corresponding_param = param_ids.iter().copied().any(|param_id| {
            let param_label_id = state.registry.get(param_id).label_identifier_id();
            let param_label_name: &IdentifierName = &state.registry.get(param_label_id).name;
            arg_label_name == param_label_name
        });
        if !has_corresponding_param {
            return tainted_err(TypeCheckError::ExtraneousLabeledCallArg { call_id, arg_id });
        }
    }
    Ok(())
}
