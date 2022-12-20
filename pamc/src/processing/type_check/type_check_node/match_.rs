use super::*;

pub(in crate::processing::type_check) fn get_type_of_match_dirty(
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
    )
    .map_err(Tainted::new)?;

    let case_ids = state
        .registry
        .get_possibly_empty_list(match_.case_list_id)
        .to_vec();
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
    let ConstructedTerms {
        parameterized_matchee_id,
        parameterized_matchee_type_id,
        variant_arity,
    } = add_case_params_to_context_and_get_constructed_matchee_and_type_dirty(
        state,
        case_id,
        matchee_type,
    )??;

    let original_coercion_target_id = coercion_target_id;
    let coercion_target_id =
        coercion_target_id.map(|target_id| target_id.upshift(variant_arity, state.registry));

    let normalized_matchee_id = normalized_matchee_id.upshift(variant_arity, state.registry);
    let matchee_type_id = matchee_type_id.upshift(variant_arity, state.registry);

    state.substitution_context.push(SubstitutionContextEntry {
        context_len: state.context.len(),
        unadjusted_substitutions: vec![
            DynamicSubstitution(normalized_matchee_id, parameterized_matchee_id),
            DynamicSubstitution(matchee_type_id, parameterized_matchee_type_id),
        ],
    });

    let output_type_id = get_type_of_expression_dirty(state, coercion_target_id, case.output_id)?;

    if let Some(coercion_target_id) = coercion_target_id {
        let can_be_coerced =
            is_left_type_assignable_to_right_type(state, output_type_id, coercion_target_id);

        state.context.pop_n(variant_arity);
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
        };
    }

    state.context.pop_n(variant_arity);
    state.substitution_context.pop();

    match output_type_id.try_downshift(variant_arity, state.registry) {
        Ok(output_type_id) => Ok(output_type_id),
        Err(_) => tainted_err(TypeCheckError::AmbiguousOutputType { case_id }),
    }
}

#[derive(Debug, Clone)]
struct ConstructedTerms {
    parameterized_matchee_id: NormalFormId,
    parameterized_matchee_type_id: NormalFormId,
    variant_arity: usize,
}

fn add_case_params_to_context_and_get_constructed_matchee_and_type_dirty(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: NormalFormAdtExpression,
) -> Result<WithPushWarning<ConstructedTerms>, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    let variant_dbi =
        get_db_index_for_adt_variant_of_name(state, matchee_type, case.variant_name_id);
    let variant_type_id = state.context.get_type(variant_dbi, state.registry);
    let fully_qualified_variant_name_component_ids: NonEmptyVec<NodeId<Identifier>> = {
        let matchee_type_name = state.registry.get(matchee_type.type_name_id);
        let matchee_type_name_component_ids = state
            .registry
            .get_list(matchee_type_name.component_list_id)
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
            if case_param_list_id.non_zero_len() != expected_case_param_arity {
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
                let case_param_ids = match case_param_list_id {
                    NonEmptyMatchCaseParamListId::Unlabeled(param_list_id) => {
                        state.registry.get_list(param_list_id).to_non_empty_vec()
                    }
                    // TODO: Properly handle the labeled case.
                    NonEmptyMatchCaseParamListId::UniquelyLabeled {
                        param_list_id,
                        triple_dot: _,
                    } => state
                        .registry
                        .get_list(param_list_id)
                        .to_mapped(|&param_id| state.registry.get(param_id).name_id),
                };
                let case_param_arity = case_param_ids.len();
                let arg_ids = case_param_ids.as_non_empty_slice().enumerate_to_mapped(
                    |(index, &case_param_id)| {
                        ExpressionId::Name(add_name_expression(
                            state.registry,
                            NonEmptyVec::singleton(case_param_id),
                            DbIndex(case_param_arity - index - 1),
                        ))
                    },
                );
                // TODO: Properly construct parameterized matchee id
                // after we add support for labeled match case params.
                let arg_list_id =
                    NonEmptyCallArgListId::Unlabeled(state.registry.add_list(arg_ids));
                let parameterized_matchee_id = NormalFormId::unchecked_new(ExpressionId::Call(
                    state
                        .registry
                        .add(Call {
                            id: dummy_id(),
                            span: None,
                            callee_id,
                            arg_list_id,
                        })
                        .without_spans(state.registry),
                ));

                let output_substitutions: Vec<Substitution> = case_param_ids
                    .iter()
                    .copied()
                    .enumerate()
                    .map(|(raw_index, case_param_id)| {
                        let db_index = DbIndex(case_param_ids.len() - raw_index - 1);
                        let param_name_expression_id = ExpressionId::Name(add_name_expression(
                            state.registry,
                            NonEmptyVec::singleton(case_param_id),
                            db_index,
                        ));
                        Substitution {
                            // We don't care about the name of the `from` `NameExpression`
                            // because the comparison is only based on the `db_index`.
                            from: param_name_expression_id,
                            to: param_name_expression_id,
                        }
                    })
                    .collect();
                let substituted_output_id = normalized_forall
                    .output_id
                    .subst_all(&output_substitutions, &mut state.without_context());
                let parameterized_matchee_type_id =
                    NormalFormId::unchecked_new(substituted_output_id);

                (parameterized_matchee_id, parameterized_matchee_type_id)
            };

            Ok(with_push_warning(ConstructedTerms {
                parameterized_matchee_id,
                parameterized_matchee_type_id,
                variant_arity: normalized_forall.param_list_id.len(),
            }))
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
            Ok(with_push_warning(ConstructedTerms {
                parameterized_matchee_id,
                parameterized_matchee_type_id: variant_type_id,
                variant_arity: 0,
            }))
        }
        other => {
            // We could inline this constant directly into the `panic!()` call,
            // but then rustfmt will mysteriously stop working.
            // I don't know why this is, but it may have to do with the line length.
            // This is the only reason we use this constant.
            const MESSAGE: &str = "A variant's type should always either be a Forall or a Name, but it was actually a ";
            panic!("{}{:?}", MESSAGE, other)
        }
    }
}
