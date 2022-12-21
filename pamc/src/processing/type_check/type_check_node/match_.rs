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
    let ParameterizedTerms {
        matchee_id: parameterized_matchee_id,
        matchee_type_id: parameterized_matchee_type_id,
        case_output_substitutions,
    } = add_case_params_to_context_and_parameterize_terms_dirty(state, case_id, matchee_type)??;
    let variant_arity = case_output_substitutions.variant_arity;

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

    let shifted_output_id = apply_case_output_substitutions(
        &mut state.without_context(),
        case.output_id,
        &case_output_substitutions,
    );
    let output_type_id =
        get_type_of_expression_dirty(state, coercion_target_id, shifted_output_id)?;

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
struct ParameterizedTerms {
    matchee_id: NormalFormId,
    matchee_type_id: NormalFormId,
    case_output_substitutions: CaseOutputSubstitutions,
}

#[derive(Debug, Clone)]
struct CaseOutputSubstitutions {
    variant_arity: usize,
    case_explicit_arity: usize,
    substitutions: Vec<CaseOutputSubstitution>,
}

impl CaseOutputSubstitutions {
    fn noop_with_arity(arity: usize) -> Self {
        Self {
            variant_arity: arity,
            case_explicit_arity: arity,
            substitutions: vec![],
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct CaseOutputSubstitution {
    explicit_param_index: usize,
    explicit_param_name_id: NodeId<Identifier>,
    corresponding_variant_param_index: usize,
}

fn add_case_params_to_context_and_parameterize_terms_dirty(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: NormalFormAdtExpression,
) -> Result<WithPushWarning<ParameterizedTerms>, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    let variant_dbi =
        get_db_index_for_adt_variant_of_name(state, matchee_type, case.variant_name_id);
    let variant_type_id = state.context.get_type(variant_dbi, state.registry);

    match variant_type_id.raw() {
        ExpressionId::Forall(normalized_forall_id) => {
            let normalized_forall = state.registry.get(normalized_forall_id).clone();

            match normalized_forall.param_list_id {
                NonEmptyParamListId::Unlabeled(variant_type_param_list_id) => {
                    // We only need this alias to avoid breaking rustfmt.
                    // For some reason, when lines get really long, rustfmt mysteriously
                    // stops working.
                    // However, even though this alias has a really long line, rustfmt somehow
                    // still works.
                    // Thus, we use the alias.
                    use add_case_params_to_context_and_parameterize_terms_given_variant_is_unlabeled_dirty as handle_unlabeled;
                    handle_unlabeled(
                        state,
                        case_id,
                        matchee_type,
                        variant_dbi,
                        normalized_forall_id,
                        variant_type_param_list_id,
                    )
                }
                NonEmptyParamListId::UniquelyLabeled(variant_type_param_list_id) => {
                    add_case_params_to_context_and_parameterize_terms_given_variant_is_labeled_dirty(
                        state,
                        case_id,
                        matchee_type,
                        variant_dbi,
                        normalized_forall_id,
                        variant_type_param_list_id,
                    )
                }
            }
        }
        ExpressionId::Name(_) | ExpressionId::Call(_) => {
            add_case_params_to_context_and_parameterize_terms_given_variant_is_nullary_dirty(
                state,
                case_id,
                matchee_type,
                variant_dbi,
                variant_type_id,
            )
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

fn add_case_params_to_context_and_parameterize_terms_given_variant_is_unlabeled_dirty(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: NormalFormAdtExpression,
    variant_dbi: DbIndex,
    variant_type_id: NodeId<Forall>,
    variant_type_param_list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> Result<WithPushWarning<ParameterizedTerms>, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    let expected_case_param_arity = variant_type_param_list_id.len;
    let Some(case_param_list_id) = case.param_list_id else {
         return tainted_err(TypeCheckError::WrongNumberOfMatchCaseParams {
             case_id,
             expected: expected_case_param_arity.get(),
             actual: 0,
         });
     };
    let NonEmptyMatchCaseParamListId::Unlabeled(case_param_list_id) = case_param_list_id else {
        return tainted_err(TypeCheckError::MatchCaseLabelednessMismatch {
            case_id,
        });
     };
    if case_param_list_id.len != expected_case_param_arity {
        return tainted_err(TypeCheckError::WrongNumberOfMatchCaseParams {
            case_id,
            expected: expected_case_param_arity.get(),
            actual: case.param_list_id.len(),
        });
    }

    let param_type_ids = get_unlabeled_param_type_ids(state, variant_type_param_list_id)
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
        let fully_qualified_variant_name_component_ids: NonEmptyVec<NodeId<Identifier>> = {
            let matchee_type_name = state.registry.get(matchee_type.type_name_id);
            let matchee_type_name_component_ids = state
                .registry
                .get_list(matchee_type_name.component_list_id)
                .to_vec();
            NonEmptyVec::from_pushed(matchee_type_name_component_ids, case.variant_name_id)
        };
        let shifted_variant_dbi = DbIndex(variant_dbi.0 + case.param_list_id.len());
        let callee_id = ExpressionId::Name(add_name_expression(
            state.registry,
            fully_qualified_variant_name_component_ids,
            shifted_variant_dbi,
        ));
        let case_param_name_ids = state
            .registry
            .get_list(case_param_list_id)
            .to_non_empty_vec();
        let case_param_arity = case_param_name_ids.len();
        let arg_ids = case_param_name_ids
            .as_non_empty_slice()
            .enumerate_to_mapped(|(index, &case_param_id)| {
                ExpressionId::Name(add_name_expression(
                    state.registry,
                    NonEmptyVec::singleton(case_param_id),
                    DbIndex(case_param_arity - index - 1),
                ))
            });

        let arg_list_id = NonEmptyCallArgListId::Unlabeled(state.registry.add_list(arg_ids));
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

        let variant_type_output_substitutions: Vec<Substitution> = case_param_name_ids
            .iter()
            .copied()
            .enumerate()
            .map(|(raw_index, case_param_id)| {
                let db_index = DbIndex(case_param_name_ids.len() - raw_index - 1);
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
        let normalized_forall = state.registry.get(variant_type_id);
        let substituted_output_id = normalized_forall.output_id.subst_all(
            &variant_type_output_substitutions,
            &mut state.without_context(),
        );
        let parameterized_matchee_type_id = NormalFormId::unchecked_new(substituted_output_id);

        (parameterized_matchee_id, parameterized_matchee_type_id)
    };

    Ok(with_push_warning(ParameterizedTerms {
        matchee_id: parameterized_matchee_id,
        matchee_type_id: parameterized_matchee_type_id,
        case_output_substitutions: CaseOutputSubstitutions::noop_with_arity(
            variant_type_param_list_id.len.get(),
        ),
    }))
}

fn add_case_params_to_context_and_parameterize_terms_given_variant_is_labeled_dirty(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: NormalFormAdtExpression,
    variant_dbi: DbIndex,
    variant_type_id: NodeId<Forall>,
    variant_type_param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
) -> Result<WithPushWarning<ParameterizedTerms>, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    let Some(case_param_list_id) = case.param_list_id else {
        let missing_label_ids: NonEmptyVec<NodeId<Identifier>> = state
            .registry
            .get_list(variant_type_param_list_id)
            .to_mapped(|&param_id| state.registry.get(param_id).label_identifier_id());
        let missing_label_list_id = state.registry.add_list(missing_label_ids);
         return tainted_err(TypeCheckError::MissingLabeledMatchCaseParams {
             case_id,
             missing_label_list_id,
         });
     };
    let NonEmptyMatchCaseParamListId::UniquelyLabeled { param_list_id: explicit_case_param_list_id, triple_dot } = case_param_list_id else {
        return tainted_err(TypeCheckError::MatchCaseLabelednessMismatch {
            case_id,
        })
    };
    let explicit_case_param_ids = state
        .registry
        .get_possibly_empty_list(explicit_case_param_list_id)
        .to_vec();

    let variant_type_param_ids = state
        .registry
        .get_list(variant_type_param_list_id)
        .to_non_empty_vec();

    if triple_dot.is_none() {
        verify_case_param_bijection(
            state,
            case_id,
            &explicit_case_param_ids,
            variant_type_param_ids.as_non_empty_slice(),
        )?;
    }

    for &param_id in variant_type_param_ids.iter() {
        let param_type_id = state.registry.get(param_id).type_id;
        // This is safe because every param type of a normal form Forall
        // is also a normal form itself.
        let param_type_id = NormalFormId::unchecked_new(param_type_id);
        state.context.push(ContextEntry {
            type_id: param_type_id,
            definition: ContextEntryDefinition::Uninterpreted,
        })?;
    }

    let (parameterized_matchee_id, parameterized_matchee_type_id) = {
        let fully_qualified_variant_name_component_ids: NonEmptyVec<NodeId<Identifier>> = {
            let matchee_type_name = state.registry.get(matchee_type.type_name_id);
            let matchee_type_name_component_ids = state
                .registry
                .get_list(matchee_type_name.component_list_id)
                .to_vec();
            NonEmptyVec::from_pushed(matchee_type_name_component_ids, case.variant_name_id)
        };
        let shifted_variant_dbi = DbIndex(variant_dbi.0 + variant_type_param_list_id.len.get());
        let callee_id = ExpressionId::Name(add_name_expression(
            state.registry,
            fully_qualified_variant_name_component_ids,
            shifted_variant_dbi,
        ));

        let arg_name_ids = {
            let underscore_id = state.registry.add(Identifier {
                id: dummy_id(),
                span: None,
                name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
            });

            variant_type_param_ids
                .as_non_empty_slice()
                .to_mapped(|&variant_param_id| {
                    let variant_param = state.registry.get(variant_param_id);
                    let variant_param_label_name_id = variant_param.label_identifier_id();
                    let variant_param_label_name: &IdentifierName =
                        &state.registry.get(variant_param_label_name_id).name;
                    let corresponding_case_param_name_id =
                        explicit_case_param_ids.iter().find_map(|&case_param_id| {
                            let case_param = state.registry.get(case_param_id);
                            let case_param_label_name_id = case_param.label_identifier_id();
                            let case_param_label_name: &IdentifierName =
                                &state.registry.get(case_param_label_name_id).name;
                            if variant_param_label_name == case_param_label_name {
                                Some(case_param.name_id)
                            } else {
                                None
                            }
                        });

                    corresponding_case_param_name_id.unwrap_or(underscore_id)
                })
        };
        let arg_ids = variant_type_param_ids
            .as_non_empty_slice()
            .enumerate_to_mapped(|(variant_param_index, &variant_param_id)| {
                let variant_param = state.registry.get(variant_param_id);
                let variant_param_label_name_id = variant_param.label_identifier_id();
                let variant_param_label_name: &IdentifierName =
                    &state.registry.get(variant_param_label_name_id).name;

                let arg_name_id = arg_name_ids[variant_param_index];
                let does_arg_name_equal_param_label_name = {
                    let arg_name: &IdentifierName = &state.registry.get(arg_name_id).name;
                    arg_name == variant_param_label_name
                };

                let db_index = DbIndex(variant_type_param_ids.len() - variant_param_index - 1);

                if does_arg_name_equal_param_label_name {
                    LabeledCallArgId::Implicit {
                        label_id: variant_param_label_name_id,
                        db_index,
                    }
                } else {
                    let value_id = ExpressionId::Name(add_name_expression(
                        state.registry,
                        NonEmptyVec::singleton(arg_name_id),
                        db_index,
                    ));
                    LabeledCallArgId::Explicit {
                        label_id: variant_param_label_name_id,
                        value_id,
                    }
                }
            });
        let arg_list_id = NonEmptyCallArgListId::UniquelyLabeled(state.registry.add_list(arg_ids));
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

        let variant_type_output_substitutions: Vec<Substitution> = (0..variant_type_param_ids
            .len())
            .into_iter()
            .map(|variant_param_index| {
                let db_index = DbIndex(variant_type_param_ids.len() - variant_param_index - 1);
                let arg_name_id = arg_name_ids[variant_param_index];

                let to = ExpressionId::Name(add_name_expression(
                    state.registry,
                    NonEmptyVec::singleton(arg_name_id),
                    db_index,
                ));
                // We don't care what the name of `from` is as long as the
                // DB index is correct, so we may as well reuse `to`.
                let from = to;

                Substitution { from, to }
            })
            .collect();
        let normalized_forall = state.registry.get(variant_type_id);
        let substituted_output_id = normalized_forall.output_id.subst_all(
            &variant_type_output_substitutions,
            &mut state.without_context(),
        );
        let parameterized_matchee_type_id = NormalFormId::unchecked_new(substituted_output_id);

        (parameterized_matchee_id, parameterized_matchee_type_id)
    };

    let case_output_substitutions: Vec<CaseOutputSubstitution> = explicit_case_param_ids
        .iter()
        .copied()
        .enumerate()
        .map(
            |(case_param_index, case_param_id)| -> Result<CaseOutputSubstitution, _> {
                let case_param = state.registry.get(case_param_id);
                let case_param_name_id = case_param.name_id;
                let case_param_label_name_id = case_param.label_identifier_id();
                let case_param_label_name: &IdentifierName = &state.registry.get(case_param_label_name_id).name;
                let Some(corresponding_variant_param_index) = variant_type_param_ids
                    .iter().copied()
                    .position(|variant_type_param_id| {
                        let variant_type_param = state.registry.get(variant_type_param_id);
                        let variant_type_param_label_name_id = variant_type_param.label_identifier_id();
                        let variant_type_param_label_name: &IdentifierName = &state.registry.get(variant_type_param_label_name_id).name;
                        variant_type_param_label_name == case_param_label_name
                    })
                else {
                    return tainted_err(TypeCheckError::UndefinedLabeledMatchCaseParam { case_id, case_param_id });
                };
                Ok(CaseOutputSubstitution {
                    explicit_param_index: case_param_index,
                    explicit_param_name_id: case_param_name_id,
                    corresponding_variant_param_index,
                })
            },
        )
        .collect::<Result<Vec<_>, _>>()?;

    Ok(with_push_warning(ParameterizedTerms {
        matchee_id: parameterized_matchee_id,
        matchee_type_id: parameterized_matchee_type_id,
        case_output_substitutions: CaseOutputSubstitutions {
            variant_arity: variant_type_param_list_id.len.get(),
            case_explicit_arity: case_param_list_id.explicit_len(),
            substitutions: case_output_substitutions,
        },
    }))
}

fn add_case_params_to_context_and_parameterize_terms_given_variant_is_nullary_dirty(
    state: &mut State,
    case_id: NodeId<MatchCase>,
    matchee_type: NormalFormAdtExpression,
    variant_dbi: DbIndex,
    variant_type_id: NormalFormId,
) -> Result<WithPushWarning<ParameterizedTerms>, Tainted<TypeCheckError>> {
    let case = state.registry.get(case_id).clone();
    if let Some(case_param_list_id) = case.param_list_id {
        return tainted_err(match case_param_list_id {
            NonEmptyMatchCaseParamListId::Unlabeled(case_param_list_id) => {
                TypeCheckError::WrongNumberOfMatchCaseParams {
                    case_id,
                    expected: 0,
                    actual: case_param_list_id.len.get(),
                }
            }
            NonEmptyMatchCaseParamListId::UniquelyLabeled { .. } => {
                TypeCheckError::MatchCaseLabelednessMismatch { case_id }
            }
        });
    }

    let fully_qualified_variant_name_component_ids: NonEmptyVec<NodeId<Identifier>> = {
        let matchee_type_name = state.registry.get(matchee_type.type_name_id);
        let matchee_type_name_component_ids = state
            .registry
            .get_list(matchee_type_name.component_list_id)
            .to_vec();
        NonEmptyVec::from_pushed(matchee_type_name_component_ids, case.variant_name_id)
    };

    // Since the case is nullary, we shift by zero.
    let shifted_variant_dbi = variant_dbi;
    let parameterized_matchee_id =
        NormalFormId::unchecked_new(ExpressionId::Name(add_name_expression(
            state.registry,
            fully_qualified_variant_name_component_ids,
            shifted_variant_dbi,
        )));

    Ok(with_push_warning(ParameterizedTerms {
        matchee_id: parameterized_matchee_id,
        matchee_type_id: variant_type_id,
        case_output_substitutions: CaseOutputSubstitutions::noop_with_arity(0),
    }))
}

fn verify_case_param_bijection(
    _state: &mut State,
    _case_id: NodeId<MatchCase>,
    _explicit_case_param_ids: &[NodeId<LabeledMatchCaseParam>],
    _variant_type_param_ids: NonEmptySlice<NodeId<LabeledParam>>,
) -> Result<(), Tainted<TypeCheckError>> {
    todo!()
}

fn apply_case_output_substitutions(
    state: &mut ContextlessState,
    case_output_id: ExpressionId,
    subs: &CaseOutputSubstitutions,
) -> ExpressionId {
    // This case does not need to be handled specially,
    // (i.e., we could delete the whole `if` statement)
    // and the code would still be correct.
    // However, it's a low-hanging performance optimization.
    if subs.substitutions.len() == 0 && subs.case_explicit_arity == subs.variant_arity {
        return case_output_id;
    }

    let case_output_id = case_output_id.upshift(subs.variant_arity, state.registry);
    let concrete_subs: Vec<Substitution> = subs
        .substitutions
        .iter()
        .map(|sub| {
            let explicit_param_db_index =
                DbIndex(subs.case_explicit_arity - sub.explicit_param_index - 1);
            let variant_param_db_index =
                DbIndex(subs.variant_arity - sub.corresponding_variant_param_index - 1);
            let dummy = sub.explicit_param_name_id;
            let from = ExpressionId::Name(add_name_expression(
                state.registry,
                // This will never get used in the comparison,
                // so it doesn't matter what we put here.
                NonEmptyVec::singleton(dummy),
                DbIndex(explicit_param_db_index.0 + subs.variant_arity),
            ));
            let to = ExpressionId::Name(add_name_expression(
                state.registry,
                NonEmptyVec::singleton(sub.explicit_param_name_id),
                variant_param_db_index,
            ));
            Substitution { from, to }
        })
        .collect();
    let case_output_id = case_output_id.subst_all(&concrete_subs, state);
    case_output_id.downshift(subs.case_explicit_arity, state.registry)
}

// TODO: Perform `match` hunt.
