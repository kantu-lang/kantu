use super::*;

pub fn type_check_file(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    variant_db: &VariantReturnTypeDatabase,
    file_id: NodeId<File>,
) -> Result<TypeMap, TypeError> {
    let file = registry.file(file_id);
    let file_item_ids = registry.file_item_list(file.item_list_id).to_vec();
    let wrapped_type0_identifier_id = {
        let type0_identifier_id = registry.add_identifier_and_overwrite_its_id(Identifier {
            id: dummy_id(),
            start: None,
            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
        });
        let type0_identifier = registry.identifier(type0_identifier_id).clone();
        let wrapped_id = registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
            id: dummy_id(),
            expression: Expression::Identifier(type0_identifier),
        });

        symbol_db
            .identifier_symbols
            .insert(type0_identifier_id, symbol_db.provider.type0_symbol());

        NormalFormNodeId(wrapped_id)
    };
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        variant_db,
        context: TypeCheckContext::new(),
        type0_identifier_id: wrapped_type0_identifier_id,
        sih_cache: NodeStructuralIdentityHashCache::empty(),
    };
    for item_id in file_item_ids {
        match item_id {
            FileItemNodeId::Type(type_id) => {
                type_check_type_statement(&mut state, type_id)?;
            }
            FileItemNodeId::Let(function_id) => {
                type_check_let_statement(&mut state, function_id)?;
            }
        }
    }
    Ok(state.context.bottom_type_map())
}

fn type_check_type_statement(
    state: &mut TypeCheckState,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeError> {
    let type_statement = state.registry.type_statement(type_statement_id);
    let param_ids = state
        .registry
        .param_list(type_statement.param_list_id)
        .to_vec();
    for param_id in &param_ids {
        type_check_param(state, *param_id)?;
    }

    let type_name_type_id = if param_ids.is_empty() {
        state.type0_identifier_id
    } else {
        let normalized_param_list_id =
            normalize_type_checked_params(state, param_ids.iter().copied())?;
        let wrapped_forall_id =
            register_wrapped_forall(state, normalized_param_list_id, state.type0_identifier_id.0);
        NormalFormNodeId(wrapped_forall_id)
    };
    let type_statement = state.registry.type_statement(type_statement_id);
    let type_name_symbol = state
        .symbol_db
        .identifier_symbols
        .get(type_statement.name_id);
    state
        .context
        .insert_new(type_name_symbol, type_name_type_id);

    let variant_ids: Vec<NodeId<Variant>> = state
        .registry
        .variant_list(type_statement.variant_list_id)
        .to_vec();
    for variant_id in variant_ids {
        type_check_variant(state, variant_id)?;
    }

    Ok(())
}

fn type_check_variant(
    state: &mut TypeCheckState,
    variant_id: NodeId<Variant>,
) -> Result<(), TypeError> {
    let variant = state.registry.variant(variant_id);
    let variant_return_type_id = variant.return_type_id;
    let variant_name_id = variant.name_id;
    let param_ids = state.registry.param_list(variant.param_list_id).to_vec();
    for param_id in &param_ids {
        type_check_param(state, *param_id)?;
    }

    // This return type type will either be `Type` (i.e., type 0)
    // or it will not be well-typed at all.
    type_check_expression(state, variant_return_type_id, None)?;

    let normalized_return_type_id = evaluate_well_typed_expression(
        &mut state.registry,
        &mut state.symbol_db,
        &mut state.sih_cache,
        variant_return_type_id,
    )?;

    let variant_type_id = if param_ids.is_empty() {
        normalized_return_type_id
    } else {
        let normalized_param_list_id =
            normalize_type_checked_params(state, param_ids.iter().copied())?;
        let wrapped_forall_id =
            register_wrapped_forall(state, normalized_param_list_id, normalized_return_type_id.0);
        NormalFormNodeId(wrapped_forall_id)
    };

    let variant_symbol = state.symbol_db.identifier_symbols.get(variant_name_id);
    state.context.insert_new(variant_symbol, variant_type_id);

    Ok(())
}

fn type_check_param(state: &mut TypeCheckState, param_id: NodeId<Param>) -> Result<(), TypeError> {
    let type_id = state.registry.param(param_id).type_id;
    let type_type_id = type_check_expression(state, type_id, None)?;
    if !is_expression_type0_or_type1(state, type_type_id.0) {
        return Err(TypeError::IllegalParamType {
            param_id: param_id,
            type_type_id: type_type_id,
        });
    }

    let param_name_id = state.registry.param(param_id).name_id;
    let param_symbol = state.symbol_db.identifier_symbols.get(param_name_id);
    let type_normal_form_id = evaluate_well_typed_expression(
        &mut state.registry,
        &mut state.symbol_db,
        &mut state.sih_cache,
        type_id,
    )?;
    state.context.insert_new(param_symbol, type_normal_form_id);

    Ok(())
}

fn type_check_let_statement(
    _state: &mut TypeCheckState,
    _let_statement: NodeId<LetStatement>,
) -> Result<(), TypeError> {
    // TODO: Actually implement (or remove) type_check_let_statement
    Ok(())
}

fn type_check_expression(
    state: &mut TypeCheckState,
    id: NodeId<WrappedExpression>,
    goal: Option<NormalFormNodeId>,
) -> Result<NormalFormNodeId, TypeError> {
    match &state.registry.wrapped_expression(id).expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            let type_id = get_normalized_type(state, symbol)?;
            ok_unless_contradicts_goal(state, type_id, goal)
        }
        Expression::Dot(dot) => {
            let symbol = state.symbol_db.identifier_symbols.get(dot.right_id);
            let type_id = get_normalized_type(state, symbol)?;
            ok_unless_contradicts_goal(state, type_id, goal)
        }
        Expression::Call(call) => {
            let call_id = call.id;
            let callee_id = call.callee_id;
            let arg_list_id = call.arg_list_id;
            let callee_type_id = type_check_expression(state, callee_id, None)?;
            let callee_type: Forall = match &state
                .registry
                .wrapped_expression(callee_type_id.0)
                .expression
            {
                Expression::Forall(forall) => (**forall).clone(),
                _ => {
                    return Err(TypeError::CalleeNotAFunction {
                        callee_id: callee_id,
                        callee_type_id: callee_type_id,
                    })
                }
            };
            let param_ids = state.registry.param_list(callee_type.param_list_id);
            let arg_ids = state.registry.wrapped_expression_list(arg_list_id);
            if param_ids.len() != arg_ids.len() {
                return Err(TypeError::WrongNumberOfArguments {
                    call_id: call_id,
                    param_arity: param_ids.len(),
                    arg_arity: arg_ids.len(),
                });
            }

            let arg_ids_and_arg_type_ids: Vec<(NodeId<WrappedExpression>, NormalFormNodeId)> =
                arg_ids
                    .to_vec()
                    .iter()
                    .map(|arg_id| -> Result<(NodeId<WrappedExpression>, NormalFormNodeId), TypeError> {
                        // TODO: Infer arg goal using callee (i.e., current) goal
                        Ok((*arg_id, type_check_expression(state, *arg_id, None)?))
                    })
                    .collect::<Result<Vec<_>, TypeError>>()?;

            let param_ids = state
                .registry
                .param_list(callee_type.param_list_id)
                .to_vec();

            for (param_id, (arg_id, arg_type_id)) in param_ids
                .iter()
                .copied()
                .zip(arg_ids_and_arg_type_ids.iter().copied())
            {
                let param = state.registry.param(param_id);
                let param_symbol = state.symbol_db.identifier_symbols.get(param.name_id);
                let param_type_id = get_normalized_type(state, param_symbol)?;
                if !does_production_type_satisfy_required_type(state, arg_type_id, param_type_id) {
                    return Err(TypeError::WrongArgumentType {
                        arg_id,
                        param_type_id: param_type_id,
                        arg_type_id: arg_type_id,
                    });
                }
            }

            let substitutions: Vec<Substitution> = param_ids
                .iter()
                .copied()
                .zip(arg_ids_and_arg_type_ids.iter().copied())
                .map(
                    |(param_id, (arg_id, _))| -> Result<Substitution, TypeError> {
                        let normalized_arg_id = evaluate_well_typed_expression(
                            &mut state.registry,
                            &mut state.symbol_db,
                            &mut state.sih_cache,
                            arg_id,
                        )?;
                        let param = state.registry.param(param_id);
                        let param_symbol = state.symbol_db.identifier_symbols.get(param.name_id);
                        Ok(Substitution {
                            from: SubstitutionLhs::Symbol(param_symbol),
                            to: normalized_arg_id,
                        })
                    },
                )
                .collect::<Result<Vec<_>, TypeError>>()?;
            let unnormalized_return_type_id = apply_substitutions(
                &mut state.registry,
                &mut state.symbol_db,
                &mut state.sih_cache,
                callee_type.output_id,
                substitutions,
            );
            let return_type_id = evaluate_well_typed_expression(
                &mut state.registry,
                &mut state.symbol_db,
                &mut state.sih_cache,
                unnormalized_return_type_id,
            )?;

            ok_unless_contradicts_goal(state, return_type_id, goal)
        }
        Expression::Fun(fun) => {
            let fun_id = fun.id;
            let name_id = fun.name_id;
            let param_list_id = fun.param_list_id;
            let return_type_id = fun.return_type_id;
            let body_id = fun.body_id;

            let param_ids = state.registry.param_list(param_list_id).to_vec();
            for param_id in &param_ids {
                type_check_param(state, *param_id)?;
            }
            let normalized_param_list_id =
                normalize_type_checked_params(state, param_ids.iter().copied())?;

            let return_type_type_id = type_check_expression(state, return_type_id, None)?;
            if !is_expression_type0_or_type1(state, return_type_type_id.0) {
                return Err(TypeError::IllegalReturnType {
                    fun_id: fun_id,
                    return_type_type_id: return_type_type_id,
                });
            }

            let normalized_return_type_id = evaluate_well_typed_expression(
                &mut state.registry,
                &mut state.symbol_db,
                &mut state.sih_cache,
                return_type_id,
            )?;

            let goal_id = normalized_return_type_id;
            type_check_expression(state, body_id, Some(goal_id)).map_goal_mismatch_err(
                |actual_type_id, _| TypeError::WrongBodyType {
                    fun_id: fun_id,
                    normalized_return_type_id: normalized_return_type_id,
                    body_type_id: actual_type_id,
                },
            )?;

            let fun_type_id = state.registry.add_forall_and_overwrite_its_id(Forall {
                id: dummy_id(),
                param_list_id: normalized_param_list_id,
                output_id: normalized_return_type_id.0,
            });
            let fun_type = state.registry.forall(fun_type_id).clone();
            let wrapped_type_id =
                state
                    .registry
                    .add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                        id: dummy_id(),
                        expression: Expression::Forall(Box::new(fun_type)),
                    });
            // This is safe because the params and output are normalized, so
            // by definition, the Forall is a normal form.
            let wrapped_type_id = NormalFormNodeId(wrapped_type_id);

            let fun_symbol = state.symbol_db.identifier_symbols.get(name_id);
            state.context.insert_new(fun_symbol, wrapped_type_id);

            ok_unless_contradicts_goal(state, wrapped_type_id, goal)
        }
        Expression::Match(match_) => {
            let match_id = match_.id;
            let matchee_id = match_.matchee_id;
            let case_list_id = match_.case_list_id;

            let matchee_type_id = type_check_expression(state, matchee_id, None)?;
            let matchee_type = if let Some(t) = as_algebraic_data_type(state, matchee_type_id) {
                t
            } else {
                return Err(TypeError::IllegalMatcheeType {
                    match_id: match_id,
                    matchee_type_id: matchee_type_id,
                });
            };

            let case_ids = state.registry.match_case_list(case_list_id).to_vec();
            let mut covered_cases: Vec<(IdentifierName, NodeId<MatchCase>)> = vec![];
            if let Some(goal) = goal {
                for case_id in case_ids.iter().copied() {
                    type_check_uncovered_match_case(
                        state,
                        case_id,
                        match_id,
                        matchee_type,
                        &mut covered_cases,
                        Some(goal),
                    )?;
                }

                verify_all_match_cases_were_covered(state, match_id, matchee_type, &covered_cases)?;

                Ok(goal)
            } else {
                let mut first_case_output_type_id = None;
                for case_id in case_ids.iter().copied() {
                    let output_type_id = type_check_uncovered_match_case(
                        state,
                        case_id,
                        match_id,
                        matchee_type,
                        &mut covered_cases,
                        None,
                    )?;
                    if let Some(first_case_output_type_id) = first_case_output_type_id {
                        if !does_production_type_satisfy_required_type(
                            state,
                            output_type_id,
                            first_case_output_type_id,
                        ) {
                            return Err(TypeError::InconsistentMatchCases {
                                match_id: match_id,
                                first_case_output_type_id: first_case_output_type_id,
                                second_case_output_type_id: output_type_id,
                            });
                        }
                    } else {
                        first_case_output_type_id = Some(output_type_id);
                    }
                }

                verify_all_match_cases_were_covered(state, match_id, matchee_type, &covered_cases)?;

                match first_case_output_type_id {
                    Some(first_case_output_type_id) => Ok(first_case_output_type_id),
                    // If `first_case_output_type_id` is `None`, then there are no cases.
                    // Since `verify_all_match_cases_were_covered` returned `Ok`, the matchee type
                    // must have zero variants.
                    // This means the matchee type is equivalent to the empty type.
                    // The match expression's type is also equivalent to the empty type.
                    // Thus, we need to return the empty type.
                    // There is no built-in empty type, so we simply return the type of the
                    // matchee (which was proven above to be equivalent to the empty type).
                    None => Ok(matchee_type_id),
                }
            }
        }
        Expression::Forall(forall) => {
            let forall_id = forall.id;
            let param_list_id = forall.param_list_id;
            let output_id = forall.output_id;
            let param_ids = state.registry.param_list(param_list_id).to_vec();
            for param_id in param_ids {
                type_check_param(state, param_id)?;
            }

            let output_type_id = type_check_expression(state, output_id, None)?;
            if !is_expression_type0_or_type1(state, output_type_id.0) {
                return Err(TypeError::IllegalForallOutputType {
                    forall_id: forall_id,
                    output_type_id: output_type_id,
                });
            }

            Ok(state.type0_identifier_id)
        }
    }
}

fn verify_all_match_cases_were_covered(
    state: &mut TypeCheckState,
    match_id: NodeId<Match>,
    matchee_type: AlgebraicDataType,
    covered_cases: &[(IdentifierName, NodeId<MatchCase>)],
) -> Result<(), TypeError> {
    let matchee_type_callee_symbol = state
        .symbol_db
        .identifier_symbols
        .get(matchee_type.callee_id);
    let uncovered_case = match state
        .symbol_db
        .symbol_dot_targets
        .get_all(matchee_type_callee_symbol)
    {
        Some(mut target_names) => target_names
            .find(|target_name| {
                let has_covered_target = covered_cases
                    .iter()
                    .any(|(covered_name, _)| *target_name == covered_name);
                !has_covered_target
            })
            .cloned(),
        None => None,
    };

    if let Some(uncovered_case) = uncovered_case {
        Err(TypeError::UncoveredMatchCase {
            match_id: match_id,
            uncovered_case,
        })
    } else {
        Ok(())
    }
}

fn type_check_uncovered_match_case(
    state: &mut TypeCheckState,
    case_id: NodeId<MatchCase>,
    match_id: NodeId<Match>,
    matchee_type: AlgebraicDataType,
    covered_cases: &mut Vec<(IdentifierName, NodeId<MatchCase>)>,
    goal: Option<NormalFormNodeId>,
) -> Result<NormalFormNodeId, TypeError> {
    let variant_name_id = state.registry.match_case(case_id).variant_name_id;
    let variant_name: IdentifierName = state.registry.identifier(variant_name_id).name.clone();
    if let Some((_, covered_case_id)) = covered_cases
        .iter()
        .find(|(covered_name, _)| *covered_name == variant_name)
    {
        return Err(TypeError::DuplicateMatchCases {
            match_id: match_id,
            first_case_id: *covered_case_id,
            second_case_id: case_id,
        });
    }

    covered_cases.push((variant_name, case_id));

    type_check_match_case(state, case_id, matchee_type, goal)
}

fn type_check_match_case(
    state: &mut TypeCheckState,
    case_id: NodeId<MatchCase>,
    matchee_type: AlgebraicDataType,
    original_goal: Option<NormalFormNodeId>,
) -> Result<NormalFormNodeId, TypeError> {
    let mut goal = original_goal;
    let variant_id = if let Some(variant_id) =
        get_variant_id_corresponding_to_match_case(state, matchee_type, case_id)
    {
        variant_id
    } else {
        let case = state.registry.match_case(case_id);
        return Err(TypeError::UnrecognizedVariant {
            adt_callee_id: matchee_type.callee_id,
            variant_name_id: case.variant_name_id,
        });
    };

    let case_constructed_type_arg_ids: Vec<NodeId<WrappedExpression>> =
        match state.variant_db.get(variant_id) {
            VariantReturnType::Call {
                arg_list_id: variant_return_type_arg_list_id,
                ..
            } => {
                let variant_param_list_id = state.registry.variant(variant_id).param_list_id;
                let variant_param_ids = state.registry.param_list(variant_param_list_id).to_vec();
                let case_param_list_id = state.registry.match_case(case_id).param_list_id;
                let case_param_ids = state.registry.identifier_list(case_param_list_id).to_vec();
                if variant_param_ids.len() != case_param_ids.len() {
                    return Err(TypeError::WrongNumberOfMatchCaseParams {
                        case_id: case_id,
                        variant_id,
                        expected_arity: variant_param_ids.len(),
                        actual_arity: case_param_ids.len(),
                    });
                }
                let substitutions: Vec<Substitution> = variant_param_ids
                    .into_iter()
                    .zip(case_param_ids.into_iter())
                    .map(|(variant_param_id, case_param_id)| {
                        let variant_param = state.registry.param(variant_param_id);
                        let variant_param_symbol = state
                            .symbol_db
                            .identifier_symbols
                            .get(variant_param.name_id);
                        let case_param = state.registry.identifier(case_param_id).clone();
                        let wrapped_case_param_id = state
                            .registry
                            .add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                                id: dummy_id(),
                                expression: Expression::Identifier(case_param),
                            });
                        // This is safe because an identifier defined by a match case param declaration
                        // is always a normal form.
                        let wrapped_case_param_id = NormalFormNodeId(wrapped_case_param_id);
                        Substitution {
                            from: SubstitutionLhs::Symbol(variant_param_symbol),
                            to: wrapped_case_param_id,
                        }
                    })
                    .collect();

                let variant_return_type_arg_ids = state
                    .registry
                    .wrapped_expression_list(*variant_return_type_arg_list_id)
                    .to_vec();
                variant_return_type_arg_ids
                    .into_iter()
                    .map(|variant_return_type_arg_id| {
                        apply_substitutions(
                            &mut state.registry,
                            &mut state.symbol_db,
                            &mut state.sih_cache,
                            variant_return_type_arg_id,
                            substitutions.clone(),
                        )
                    })
                    .collect::<Vec<_>>()
            }
            VariantReturnType::Identifier { .. } => vec![],
        };

    let matchee_type_arg_ids = state
        .registry
        .wrapped_expression_list(matchee_type.arg_list_id)
        .to_vec();

    assert_eq!(matchee_type_arg_ids.len(), case_constructed_type_arg_ids.len(), "The number of type arguments of the matchee type and the number of type arguments of the constructed type should be the same. But they were different. This indicates a serious logic error.");

    state.context.push_scope();
    let mut type_arg_substitutions = vec![];
    for (matchee_type_arg_id, case_constructed_type_arg_id) in matchee_type_arg_ids
        .into_iter()
        .zip(case_constructed_type_arg_ids.into_iter())
    {
        let substituted_matchee_type_arg_id = apply_substitutions(
            &mut state.registry,
            &mut state.symbol_db,
            &mut state.sih_cache,
            matchee_type_arg_id,
            type_arg_substitutions.iter().copied(),
        );
        let substituted_case_constructed_type_arg_id = apply_substitutions(
            &mut state.registry,
            &mut state.symbol_db,
            &mut state.sih_cache,
            case_constructed_type_arg_id,
            type_arg_substitutions.iter().copied(),
        );
        let fusion_result = compute_ltr_fusion_of_well_typed_expressions(
            state,
            substituted_matchee_type_arg_id,
            substituted_case_constructed_type_arg_id,
        )?;
        match fusion_result {
            FusionResult::Exploded => {
                if let Some(original_goal) = original_goal {
                    return Ok(original_goal);
                }
                // TODO: Handle explosions in the case where there is no
                // goal.
            }
            FusionResult::Fused(substitutions) => {
                type_arg_substitutions.extend(substitutions.iter().copied());

                state.context.apply_substitutions_to_top_scope(
                    &mut state.registry,
                    &mut state.symbol_db,
                    &mut state.sih_cache,
                    &substitutions,
                )?;

                if let Some(goal) = goal.as_mut() {
                    let substituted_goal = apply_substitutions(
                        &mut state.registry,
                        &mut state.symbol_db,
                        &mut state.sih_cache,
                        goal.0,
                        substitutions,
                    );
                    let normalized_substituted_goal = evaluate_well_typed_expression(
                        &mut state.registry,
                        &mut state.symbol_db,
                        &mut state.sih_cache,
                        substituted_goal,
                    )?;
                    *goal = normalized_substituted_goal;
                }
            }
        }
    }
    let output_id = state.registry.match_case(case_id).output_id;
    let return_val = type_check_expression(state, output_id, goal);
    state.context.pop_scope();

    return_val
}
