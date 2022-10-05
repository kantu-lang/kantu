use crate::data::{
    node_registry::{ListId, NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase, SymbolSource},
    type_map::{NormalFormNodeId, TypeMap},
    variant_return_type::{VariantReturnType, VariantReturnTypeDatabase},
};

#[derive(Clone, Debug)]
pub enum TypeError {
    IllegalParamType {
        param_id: NodeId<Param>,
        type_type_id: NormalFormNodeId,
    },
    CalleeNotAFunction {
        callee_id: NodeId<WrappedExpression>,
        callee_type_id: NormalFormNodeId,
    },
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
        param_arity: usize,
        arg_arity: usize,
    },
    WrongArgumentType {
        arg_id: NodeId<WrappedExpression>,
        param_type_id: NormalFormNodeId,
        arg_type_id: NormalFormNodeId,
    },
    IllegalReturnType {
        fun_id: NodeId<Fun>,
        return_type_type_id: NormalFormNodeId,
    },
    WrongBodyType {
        fun_id: NodeId<Fun>,
        normalized_return_type_id: NormalFormNodeId,
        body_type_id: NormalFormNodeId,
    },
    GoalMismatch {
        goal_id: NormalFormNodeId,
        actual_type_id: NormalFormNodeId,
    },
    IllegalMatcheeType {
        match_id: NodeId<Match>,
        matchee_type_id: NormalFormNodeId,
    },
    UnrecognizedVariant {
        adt_callee_id: NodeId<Identifier>,
        variant_name_id: NodeId<Identifier>,
    },
    DuplicateMatchCases {
        match_id: NodeId<Match>,
        first_case_id: NodeId<MatchCase>,
        second_case_id: NodeId<MatchCase>,
    },
    InconsistentMatchCases {
        match_id: NodeId<Match>,
        first_case_output_type_id: NormalFormNodeId,
        second_case_output_type_id: NormalFormNodeId,
    },
    UncoveredMatchCase {
        match_id: NodeId<Match>,
        uncovered_case: IdentifierName,
    },
    WrongNumberOfMatchCaseParams {
        case_id: NodeId<MatchCase>,
        variant_id: NodeId<Variant>,
        expected_arity: usize,
        actual_arity: usize,
    },
    IllegalForallOutputType {
        forall_id: NodeId<Forall>,
        output_type_id: NormalFormNodeId,
    },
}

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
        NormalFormNodeId(wrapped_id)
    };
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        variant_db,
        context: TypeCheckContext::new(),
        type0_identifier_id: wrapped_type0_identifier_id,
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

/// Every param id yielded by `param_ids` **must** be a param that has been
/// type checked.
fn normalize_type_checked_params(
    state: &mut TypeCheckState,
    param_ids: impl IntoIterator<Item = NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeError> {
    let normalized_param_ids: Vec<NodeId<Param>> = param_ids
        .into_iter()
        .map(|id| -> Result<NodeId<Param>, TypeError> {
            let param = state.registry.param(id);
            let param_symbol = state.symbol_db.identifier_symbols.get(param.name_id);
            let normalized_param_type_id = get_normalized_type(state, param_symbol)?;

            let param = state.registry.param(id);
            let normalized_param_with_dummy_id = Param {
                id: dummy_id(),
                is_dashed: param.is_dashed,
                name_id: param.name_id,
                // It's safe to call `evaluate_well_typed_expression`
                // because we type-checked it above.
                type_id: normalized_param_type_id.0,
            };
            Ok(state
                .registry
                .add_param_and_overwrite_its_id(normalized_param_with_dummy_id))
        })
        .collect::<Result<Vec<_>, TypeError>>()?;
    Ok(state.registry.add_param_list(normalized_param_ids))
}

fn register_wrapped_forall(
    state: &mut TypeCheckState,
    param_list_id: ListId<NodeId<Param>>,
    output_id: NodeId<WrappedExpression>,
) -> NodeId<WrappedExpression> {
    let forall_with_dummy_id = Forall {
        id: dummy_id(),
        param_list_id,
        output_id,
    };
    let forall_id = state
        .registry
        .add_forall_and_overwrite_its_id(forall_with_dummy_id);
    let registered_forall = state.registry.forall(forall_id).clone();
    let wrapped_with_dummy_id = WrappedExpression {
        id: dummy_id(),
        expression: Expression::Forall(Box::new(registered_forall)),
    };
    state
        .registry
        .add_wrapped_expression_and_overwrite_its_id(wrapped_with_dummy_id)
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
    let type_normal_form_id =
        evaluate_well_typed_expression(&mut state.registry, &mut state.symbol_db, type_id)?;
    state.context.insert_new(param_symbol, type_normal_form_id);

    Ok(())
}

fn is_expression_type0_or_type1(
    state: &TypeCheckState,
    type_id: NodeId<WrappedExpression>,
) -> bool {
    let type_ = state.registry.wrapped_expression(type_id);
    match &type_.expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            symbol == state.symbol_db.provider.type0_symbol()
                || symbol == state.symbol_db.provider.type1_symbol()
        }
        _other_type => false,
    }
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
                &state.symbol_db,
                callee_type.output_id,
                substitutions,
            );
            let return_type_id = evaluate_well_typed_expression(
                &mut state.registry,
                &mut state.symbol_db,
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

                verify_all_cases_were_covered(state, match_id, matchee_type, &covered_cases)?;

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

                verify_all_cases_were_covered(state, match_id, matchee_type, &covered_cases)?;

                match first_case_output_type_id {
                    Some(first_case_output_type_id) => Ok(first_case_output_type_id),
                    // If `first_case_output_type_id` is `None`, then there are no cases.
                    // Since `verify_all_cases_were_covered` returned `Ok`, the matchee type
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

fn verify_all_cases_were_covered(
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

fn type_check_match_case(
    state: &mut TypeCheckState,
    case_id: NodeId<MatchCase>,
    matchee_type: AlgebraicDataType,
    original_goal: Option<NormalFormNodeId>,
) -> Result<NormalFormNodeId, TypeError> {
    let mut goal = original_goal;
    let variant_id =
        if let Some(variant_id) = get_corresponding_variant_id(state, matchee_type, case_id) {
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
                            &state.symbol_db,
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
            &state.symbol_db,
            matchee_type_arg_id,
            type_arg_substitutions.iter().copied(),
        );
        let substituted_case_constructed_type_arg_id = apply_substitutions(
            &mut state.registry,
            &state.symbol_db,
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
                    &substitutions,
                )?;

                if let Some(goal) = goal.as_mut() {
                    let substituted_goal = apply_substitutions(
                        &mut state.registry,
                        &state.symbol_db,
                        goal.0,
                        substitutions,
                    );
                    let normalized_substituted_goal = evaluate_well_typed_expression(
                        &mut state.registry,
                        &mut state.symbol_db,
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

#[derive(Clone, Debug)]
enum FusionResult {
    Exploded,
    Fused(Vec<Substitution>),
}

fn compute_ltr_fusion_of_well_typed_expressions(
    state: &mut TypeCheckState,
    left_id: NodeId<WrappedExpression>,
    right_id: NodeId<WrappedExpression>,
) -> Result<FusionResult, TypeError> {
    let normalized_left_id =
        evaluate_well_typed_expression(&mut state.registry, &mut state.symbol_db, left_id)?;
    let normalized_right_id =
        evaluate_well_typed_expression(&mut state.registry, &mut state.symbol_db, right_id)?;
    compute_ltr_fusion_of_well_typed_normal_forms(state, normalized_left_id, normalized_right_id)
}

fn compute_ltr_fusion_of_well_typed_normal_forms(
    state: &mut TypeCheckState,
    left_id: NormalFormNodeId,
    right_id: NormalFormNodeId,
) -> Result<FusionResult, TypeError> {
    #[derive(Clone, Debug)]
    enum FusionCase {
        SyntacticallyIdentical,
        LeftReplacable { left_symbol: Symbol },
        LeftIrreplacableRightReplacable { right_symbol: Symbol },
        NeitherReplacable,
    }

    fn get_fusion_case(
        state: &mut TypeCheckState,
        left_id: NormalFormNodeId,
        right_id: NormalFormNodeId,
    ) -> FusionCase {
        if are_expressions_equal_ignoring_ids(
            &state.registry,
            &state.symbol_db,
            left_id.0,
            right_id.0,
        ) {
            return FusionCase::SyntacticallyIdentical;
        }

        fn get_fusion_case_assuming_left_is_irreplacable(
            state: &TypeCheckState,
            right: &WrappedExpression,
        ) -> FusionCase {
            match &right.expression {
                Expression::Identifier(right_identifier) => {
                    let right_symbol = state.symbol_db.identifier_symbols.get(right_identifier.id);
                    let right_source = *state
                        .symbol_db
                        .symbol_sources
                        .get(&right_symbol)
                        .expect("An identifier expression's symbol should have source.");
                    match right_source {
                        SymbolSource::Let(_) => {
                            panic!("A let-defined identifier should never appear in a normal form.")
                        }
                        SymbolSource::Type(_)
                        | SymbolSource::Variant(_)
                        | SymbolSource::Fun(_)
                        | SymbolSource::BuiltinTypeTitleCase => {
                            // `right` cannot be replaced.
                            FusionCase::NeitherReplacable
                        }
                        SymbolSource::TypedParam(_) | SymbolSource::UntypedParam(_) => {
                            FusionCase::LeftIrreplacableRightReplacable { right_symbol }
                        }
                    }
                }
                _other_right => FusionCase::NeitherReplacable,
            }
        }

        let left = state.registry.wrapped_expression(left_id.0);
        let right = state.registry.wrapped_expression(right_id.0);
        match &left.expression {
            Expression::Identifier(left_identifier) => {
                let left_symbol = state.symbol_db.identifier_symbols.get(left_identifier.id);
                let left_source = *state
                    .symbol_db
                    .symbol_sources
                    .get(&left_symbol)
                    .expect("An identifier expression's symbol should have source.");
                match left_source {
                    SymbolSource::Let(_) => {
                        panic!("A let-defined identifier should never appear in a normal form.")
                    }
                    SymbolSource::Type(_)
                    | SymbolSource::Variant(_)
                    | SymbolSource::Fun(_)
                    | SymbolSource::BuiltinTypeTitleCase => {
                        get_fusion_case_assuming_left_is_irreplacable(state, right)
                    }
                    SymbolSource::TypedParam(_) | SymbolSource::UntypedParam(_) => {
                        FusionCase::LeftReplacable { left_symbol }
                    }
                }
            }
            _other_left => get_fusion_case_assuming_left_is_irreplacable(state, right),
        }
    }

    /// Tries to execute `[happy_path_lhs -> right_id]`, but may change the
    /// direction of the arrows as needed depending on which term (if any)
    /// is a subterm of the other.
    fn substitute_based_on_subterm_status(
        state: &mut TypeCheckState,
        left_id: NormalFormNodeId,
        right_id: NormalFormNodeId,
        happy_path_lhs: SubstitutionLhs,
    ) -> FusionResult {
        let left_subterm_right =
            is_term_a_subterm(&state.registry, &state.symbol_db, left_id.0, right_id.0);
        let right_subterm_left =
            is_term_a_subterm(&state.registry, &state.symbol_db, right_id.0, left_id.0);
        match (left_subterm_right, right_subterm_left) {
            (false, false) => FusionResult::Fused(vec![Substitution {
                from: happy_path_lhs,
                to: right_id,
            }]),
            (true, true) => {
                panic!("Impossible: Two terms are mutually subterms of each other.")
            }
            (true, false) => FusionResult::Fused(vec![Substitution {
                from: SubstitutionLhs::Expression(right_id.0),
                to: left_id,
            }]),
            (false, true) => FusionResult::Fused(vec![Substitution {
                from: SubstitutionLhs::Expression(left_id.0),
                to: right_id,
            }]),
        }
    }

    impl FusionResult {
        fn chain(self, other: FusionResult) -> FusionResult {
            match (self, other) {
                (FusionResult::Exploded, _) | (_, FusionResult::Exploded) => FusionResult::Exploded,
                (
                    FusionResult::Fused(mut substitutions),
                    FusionResult::Fused(other_substitutions),
                ) => {
                    substitutions.extend(other_substitutions);
                    FusionResult::Fused(substitutions)
                }
            }
        }
    }

    match get_fusion_case(state, left_id, right_id) {
        FusionCase::SyntacticallyIdentical => Ok(FusionResult::Fused(vec![])),
        FusionCase::LeftReplacable { left_symbol } => Ok(substitute_based_on_subterm_status(
            state,
            left_id,
            right_id,
            SubstitutionLhs::Symbol(left_symbol),
        )),
        FusionCase::LeftIrreplacableRightReplacable { right_symbol } => {
            Ok(substitute_based_on_subterm_status(
                state,
                right_id,
                left_id,
                SubstitutionLhs::Symbol(right_symbol),
            ))
        }
        FusionCase::NeitherReplacable => {
            let raw_result = substitute_based_on_subterm_status(
                state,
                left_id,
                right_id,
                SubstitutionLhs::Expression(left_id.0),
            );
            let left = state.registry.wrapped_expression(left_id.0);
            let right = state.registry.wrapped_expression(right_id.0);
            let fusion_implied_by_constructors = match (&left.expression, &right.expression) {
                (Expression::Call(left_call), Expression::Call(right_call)) => {
                    let left_callee = state.registry.wrapped_expression(left_call.callee_id);
                    let right_callee = state.registry.wrapped_expression(right_call.callee_id);
                    match (&left_callee.expression, &right_callee.expression) {
                        (Expression::Dot(left_callee_dot), Expression::Dot(right_callee_dot)) => {
                            let left_callee_symbol = state
                                .symbol_db
                                .identifier_symbols
                                .get(left_callee_dot.right_id);
                            let left_callee_source = *state
                                .symbol_db
                                .symbol_sources
                                .get(&left_callee_symbol)
                                .expect("An dot RHS's symbol should have source.");
                            let right_callee_symbol = state
                                .symbol_db
                                .identifier_symbols
                                .get(right_callee_dot.right_id);
                            let right_callee_source = *state
                                .symbol_db
                                .symbol_sources
                                .get(&right_callee_symbol)
                                .expect("An dot RHS's symbol should have source.");
                            match (left_callee_source, right_callee_source) {
                                (SymbolSource::Variant(_), SymbolSource::Variant(_)) => {
                                    if left_callee_symbol == right_callee_symbol {
                                        let left_arg_ids = state
                                            .registry
                                            .wrapped_expression_list(left_call.arg_list_id)
                                            .to_vec();
                                        let right_arg_ids = state
                                            .registry
                                            .wrapped_expression_list(right_call.arg_list_id)
                                            .to_vec();
                                        let mut out = FusionResult::Fused(vec![]);
                                        for (left_arg_id, right_arg_id) in
                                            left_arg_ids.into_iter().zip(right_arg_ids)
                                        {
                                            let arg_fusion_result =
                                                compute_ltr_fusion_of_well_typed_expressions(
                                                    state,
                                                    left_arg_id,
                                                    right_arg_id,
                                                )?;
                                            out = out.chain(arg_fusion_result);
                                        }
                                        out
                                    } else {
                                        FusionResult::Exploded
                                    }
                                }
                                _ => FusionResult::Fused(vec![]),
                            }
                        }
                        _ => FusionResult::Fused(vec![]),
                    }
                }
                _ => FusionResult::Fused(vec![]),
            };

            Ok(raw_result.chain(fusion_implied_by_constructors))
        }
    }
}

fn get_corresponding_variant_id(
    state: &mut TypeCheckState,
    matchee_type: AlgebraicDataType,
    case_id: NodeId<MatchCase>,
) -> Option<NodeId<Variant>> {
    let case = state.registry.match_case(case_id);
    let callee_symbol = state
        .symbol_db
        .identifier_symbols
        .get(matchee_type.callee_id);
    let variant_name = &state.registry.identifier(case.variant_name_id).name;
    let target_symbol = state
        .symbol_db
        .symbol_dot_targets
        .get(callee_symbol, variant_name)?;
    let target_symbol_source = state
        .symbol_db
        .symbol_sources
        .get(&target_symbol)
        .expect("Variant symbol should have a source defined.");
    match target_symbol_source {
        SymbolSource::Variant(variant_id) => Some(*variant_id),
        other_source => panic!(
            "Variant symbol source should be of type `Variant`, but was `{:?}`.",
            other_source
        ),
    }
}

fn evaluate_well_typed_expression(
    _state: &mut NodeRegistry,
    _symbol_db: &mut SymbolDatabase,
    _expression: NodeId<WrappedExpression>,
) -> Result<NormalFormNodeId, TypeError> {
    unimplemented!();
}

fn get_normalized_type(
    state: &mut TypeCheckState,
    symbol: Symbol,
) -> Result<NormalFormNodeId, TypeError> {
    let (unsubstituted_type_id, substitutions) = state.context.get(symbol);
    let unnormalized_type_id = apply_substitutions(
        &mut state.registry,
        &state.symbol_db,
        unsubstituted_type_id.0,
        substitutions
            .iter()
            .flat_map(std::ops::Deref::deref)
            .copied(),
    );
    evaluate_well_typed_expression(
        &mut state.registry,
        &mut state.symbol_db,
        unnormalized_type_id,
    )
}

fn does_production_type_satisfy_required_type(
    _state: &TypeCheckState,
    _production_type_id: NormalFormNodeId,
    _requirement_type_id: NormalFormNodeId,
) -> bool {
    unimplemented!()
}

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

#[derive(Debug)]
struct TypeCheckState<'a> {
    registry: &'a mut NodeRegistry,
    symbol_db: &'a mut SymbolDatabase,
    variant_db: &'a VariantReturnTypeDatabase,
    context: TypeCheckContext,
    type0_identifier_id: NormalFormNodeId,
}

use context::*;
mod context {
    use super::*;

    use crate::data::symbol_database::Symbol;

    #[derive(Clone, Debug)]
    pub struct TypeCheckContext {
        stack: Vec<Scope>,
    }

    #[derive(Clone, Copy, Debug)]
    pub struct Substitution {
        pub from: SubstitutionLhs,
        pub to: NormalFormNodeId,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum SubstitutionLhs {
        Symbol(Symbol),
        Expression(NodeId<WrappedExpression>),
    }

    #[derive(Clone, Debug)]
    struct Scope {
        map: TypeMap,
        substitutions_applied_to_previous_scopes: Vec<Substitution>,
    }

    impl Scope {
        fn new() -> Self {
            Self {
                map: TypeMap::empty(),
                substitutions_applied_to_previous_scopes: Vec::new(),
            }
        }
    }

    impl TypeCheckContext {
        pub fn new() -> Self {
            Self {
                stack: vec![Scope::new()],
            }
        }
    }

    impl TypeCheckContext {
        pub fn get(&self, symbol: Symbol) -> (NormalFormNodeId, Vec<&[Substitution]>) {
            self.try_get(symbol).expect(&format!(
                "Tried to get the type of {:?}, but it was not in the type map.",
                symbol
            ))
        }

        fn try_get(&self, symbol: Symbol) -> Option<(NormalFormNodeId, Vec<&[Substitution]>)> {
            let mut substitution_list_stack: Vec<&[Substitution]> = vec![];
            for scope in self.stack.iter().rev() {
                if let Some(type_id) = scope.map.try_get(symbol) {
                    return Some((type_id, substitution_list_stack));
                }
                substitution_list_stack.push(&scope.substitutions_applied_to_previous_scopes);
            }
            None
        }

        pub fn insert_new(&mut self, symbol: Symbol, type_id: NormalFormNodeId) {
            if let Some((existing_type_id, substitutions)) = self.try_get(symbol) {
                panic!("Tried to insert new entry ({:?}, {:?}) into a context, when it already contained the entry ({:?}, {:?} + {} substitutions).", symbol, type_id, symbol, existing_type_id, substitutions.len());
            }
            self.stack.last_mut().expect("Error: Tried to insert an entry into a context with an empty scope stack. This indicates a serious logic error.").map.insert_new(symbol, type_id);
        }

        pub fn bottom_type_map(self) -> TypeMap {
            self.stack
                .into_iter()
                .next()
                .expect("Error: Tried to get the bottom type map from a context with an empty scope stack. This indicates a serious logic error.")
                .map
        }

        pub fn push_scope(&mut self) {
            self.stack.push(Scope::new());
        }

        pub fn pop_scope(&mut self) {
            self.stack.pop().expect("Error: Tried to pop a scope from a context with an empty scope stack. This indicates a serious logic error.");
        }

        pub fn apply_substitutions_to_top_scope(
            &mut self,
            registry: &mut NodeRegistry,
            symbol_db: &mut SymbolDatabase,
            substitutions: &[Substitution],
        ) -> Result<(), TypeError> {
            let top = self
                .stack
                .last_mut()
                .expect("Error: Tried to apply substitutions to the top scope of a context with an empty scope stack. This indicates a serious logic error.");
            apply_substitutions_to_map(registry, symbol_db, &mut top.map, substitutions)?;
            top.substitutions_applied_to_previous_scopes
                .extend(substitutions);
            Ok(())
        }
    }

    fn apply_substitutions_to_map(
        registry: &mut NodeRegistry,
        symbol_db: &mut SymbolDatabase,
        map: &mut TypeMap,
        substitutions: &[Substitution],
    ) -> Result<(), TypeError> {
        let keys = map.keys().collect::<Vec<_>>();
        for key in keys {
            let type_id = map.get(key);
            let substituted_type_id = apply_substitutions(
                registry,
                symbol_db,
                type_id.0,
                substitutions.iter().copied(),
            );
            let normalized_substituted_type_id =
                evaluate_well_typed_expression(registry, symbol_db, substituted_type_id)?;
            map.update(key, normalized_substituted_type_id);
        }

        Ok(())
    }
}

fn apply_substitutions(
    registry: &mut NodeRegistry,
    symbol_db: &SymbolDatabase,
    type_id: NodeId<WrappedExpression>,
    substitutions: impl IntoIterator<Item = Substitution>,
) -> NodeId<WrappedExpression> {
    let mut type_id = type_id;
    for substitution in substitutions {
        type_id = apply_substitution(registry, symbol_db, type_id, substitution);
    }
    type_id
}

/// NOTE: "Applying" a substitution means to **repeatedly** substitute until
/// no more substitutions can be made.
fn apply_substitution(
    _registry: &mut NodeRegistry,
    _symbol_db: &SymbolDatabase,
    _type_id: NodeId<WrappedExpression>,
    _substitutions: Substitution,
) -> NodeId<WrappedExpression> {
    unimplemented!()
}

use map_goal_mismatch_err::*;
mod map_goal_mismatch_err {
    use super::*;

    pub trait MapGoalMismatchErr {
        /// The `f` callback takes the params: `actual, goal`.
        fn map_goal_mismatch_err(
            self,
            f: impl FnOnce(NormalFormNodeId, NormalFormNodeId) -> TypeError,
        ) -> Self;
    }

    impl<T> MapGoalMismatchErr for Result<T, TypeError> {
        fn map_goal_mismatch_err(
            self,
            f: impl FnOnce(NormalFormNodeId, NormalFormNodeId) -> TypeError,
        ) -> Self {
            self.map_err(|err| match err {
                TypeError::GoalMismatch {
                    actual_type_id,
                    goal_id,
                } => f(actual_type_id, goal_id),
                _ => err,
            })
        }
    }
}

/// This returns `Ok(nfid)` unless
/// `goal` equals `Some(g)` where `nfid` is **not** equal to `g` under
/// the definition type equality.
fn ok_unless_contradicts_goal(
    state: &TypeCheckState,
    production_type_id: NormalFormNodeId,
    goal_id: Option<NormalFormNodeId>,
) -> Result<NormalFormNodeId, TypeError> {
    if let Some(goal_id) = goal_id {
        if does_production_type_satisfy_required_type(state, production_type_id, goal_id) {
            Ok(production_type_id)
        } else {
            Err(TypeError::GoalMismatch {
                actual_type_id: production_type_id,
                goal_id,
            })
        }
    } else {
        return Ok(production_type_id);
    }
}

#[derive(Clone, Copy, Debug)]
struct AlgebraicDataType {
    callee_id: NodeId<Identifier>,
    arg_list_id: ListId<NodeId<WrappedExpression>>,
}

fn as_algebraic_data_type(
    state: &mut TypeCheckState,
    term_id: NormalFormNodeId,
) -> Option<AlgebraicDataType> {
    let empty_list_id = state.registry.add_wrapped_expression_list(Vec::new());

    let term = state.registry.wrapped_expression(term_id.0);
    match &term.expression {
        Expression::Identifier(identifier) => Some(AlgebraicDataType {
            callee_id: identifier.id,
            arg_list_id: empty_list_id,
        }),
        Expression::Call(call) => {
            let callee = state.registry.wrapped_expression(call.callee_id);
            match &callee.expression {
                Expression::Identifier(callee_identifier) => Some(AlgebraicDataType {
                    callee_id: callee_identifier.id,
                    arg_list_id: call.arg_list_id,
                }),
                _other_callee => None,
            }
        }
        _other_term => None,
    }
}

// IDEA: Use hashing when registering the nodes to speed up
// equality checking.
fn are_expressions_equal_ignoring_ids(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    a: NodeId<WrappedExpression>,
    b: NodeId<WrappedExpression>,
) -> bool {
    unimplemented!();
}

fn is_term_a_subterm(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    sub: NodeId<WrappedExpression>,
    sup: NodeId<WrappedExpression>,
) -> bool {
    unimplemented!()
}
