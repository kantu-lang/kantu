use super::*;

/// Every param id yielded by `param_ids` **must** be a param that has been
/// type checked.
pub(super) fn normalize_type_checked_params(
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

pub(super) fn register_wrapped_forall(
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

pub(super) fn is_expression_type0_or_type1(
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

pub(super) fn get_variant_id_corresponding_to_match_case(
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

pub(super) fn get_match_case_id_corresponding_to_variant(
    registry: &NodeRegistry,
    target_variant_id: NodeId<Variant>,
    match_id: NodeId<Match>,
) -> Option<NodeId<MatchCase>> {
    let target_variant_name_id = registry.variant(target_variant_id).name_id;
    let target_variant_name: &IdentifierName = &registry.identifier(target_variant_name_id).name;
    let case_list_id = registry.match_(match_id).case_list_id;
    let case_ids = registry.match_case_list(case_list_id);
    case_ids.iter().copied().find(|case_id| {
        let case = registry.match_case(*case_id);
        let case_variant_name: &IdentifierName = &registry.identifier(case.variant_name_id).name;
        case_variant_name == target_variant_name
    })
}

pub(super) fn get_normalized_type(
    state: &mut TypeCheckState,
    symbol: Symbol,
) -> Result<NormalFormNodeId, TypeError> {
    let (unsubstituted_type_id, substitutions) = state.context.get(symbol);
    let unnormalized_type_id = apply_substitutions(
        &mut state.registry,
        &mut state.symbol_db,
        &mut state.sih_cache,
        &mut state.fv_cache,
        state.type0_identifier_id,
        unsubstituted_type_id.0,
        substitutions
            .iter()
            .flat_map(std::ops::Deref::deref)
            .copied(),
    );
    evaluate_well_typed_expression(
        &mut state.registry,
        &mut state.symbol_db,
        &mut state.sih_cache,
        &mut state.fv_cache,
        state.type0_identifier_id,
        unnormalized_type_id,
    )
}

pub(super) fn does_production_type_satisfy_required_type(
    state: &mut TypeCheckState,
    production_type_id: NormalFormNodeId,
    requirement_type_id: NormalFormNodeId,
) -> bool {
    return is_type_trivially_empty(state, production_type_id)
        || are_types_equivalent_up_to_renaming_of_forall_params(
            state,
            production_type_id,
            requirement_type_id,
        );
}

fn are_types_equivalent_up_to_renaming_of_forall_params(
    state: &mut TypeCheckState,
    production_type_id: NormalFormNodeId,
    requirement_type_id: NormalFormNodeId,
) -> bool {
    if are_expressions_equal_ignoring_ids(
        &state.registry,
        &state.symbol_db,
        &mut state.sih_cache,
        production_type_id.0,
        requirement_type_id.0,
    ) {
        return true;
    }

    let production_type = state.registry.wrapped_expression(production_type_id.0);
    let requirement_type = state.registry.wrapped_expression(requirement_type_id.0);
    match (&production_type.expression, &requirement_type.expression) {
        (Expression::Identifier(_), Expression::Identifier(_)) => {
            // The production and requirement identifiers must be different,
            // since `are_expressions_equal_ignoring_ids` returned false.
            false
        }
        (Expression::Call(production_call), Expression::Call(requirement_call)) => {
            let production_callee = state.registry.wrapped_expression(production_call.callee_id);
            let requirement_callee = state
                .registry
                .wrapped_expression(requirement_call.callee_id);
            match (
                &production_callee.expression,
                &requirement_callee.expression,
            ) {
                (
                    Expression::Identifier(production_callee_identifier),
                    Expression::Identifier(requirement_callee_identifier),
                ) => {
                    let production_callee_symbol = state
                        .symbol_db
                        .identifier_symbols
                        .get(production_callee_identifier.id);
                    let requirement_callee_symbol = state
                        .symbol_db
                        .identifier_symbols
                        .get(requirement_callee_identifier.id);
                    if production_callee_symbol != requirement_callee_symbol {
                        return false;
                    }

                    let production_arg_ids = state
                        .registry
                        .wrapped_expression_list(production_call.arg_list_id);
                    let requirement_arg_ids = state
                        .registry
                        .wrapped_expression_list(requirement_call.arg_list_id);
                    production_arg_ids
                        .iter()
                        .copied()
                        .zip(requirement_arg_ids.iter().copied())
                        .collect::<Vec<_>>()
                        .into_iter()
                        .all(|(production_argument_id, requirement_argument_id)| {
                            are_types_equivalent_up_to_renaming_of_forall_params(
                                state,
                                // These casts are safe because we know that the call
                                // is a normal form, and every argument of a normal form
                                // is itself (by definition) a normal form.
                                NormalFormNodeId(production_argument_id),
                                NormalFormNodeId(requirement_argument_id),
                            )
                        })
                }
                _ => false,
            }
        }
        (Expression::Forall(production_forall), Expression::Forall(requirement_forall)) => {
            let production_param_ids = state.registry.param_list(production_forall.param_list_id);
            let requirement_param_ids = state.registry.param_list(requirement_forall.param_list_id);
            if production_param_ids.len() != requirement_param_ids.len() {
                return false;
            }

            let fresh_symbols: Vec<Symbol> = production_param_ids
                .iter()
                .map(|_| state.symbol_db.provider.new_symbol())
                .collect();

            // let (production_substitutions, requirement_substitutions) = {

            //         let production_substitutions = production_param_ids.iter().copied().zip(fresh_symbols.iter().copied()).map(|(param_id,fresh_symbol)| {
            //             let param = state.registry.param(param_id);
            //             let symbol = state.symbol_db.identifier_symbols.get(param.name_id);
            //             Substitution {
            //                 from: SubstitutionLhs::Symbol(symbol),
            //                 to:
            //             }
            //         })
            // };

            // let renamed_production_forall_id = apply_substitutions(
            //     &mut state.registry,
            //     &mut state.symbol_db,
            //     &mut state.sih_cache,
            //     &mut state.fv_cache,
            //     state.type0_identifier_id,
            //     production_type_id.0,
            //     production_substitutions,
            // );
            // let renamed_requirement_forall_id = apply_substitutions(
            //     &mut state.registry,
            //     &mut state.symbol_db,
            //     &mut state.sih_cache,
            //     &mut state.fv_cache,
            //     state.type0_identifier_id,
            //     requirement_type_id.0,
            //     requirement_substitutions,
            // );

            unimplemented!()
        }
        _ => false,
    }
}

fn is_type_trivially_empty(state: &TypeCheckState, type_id: NormalFormNodeId) -> bool {
    let type_ = state.registry.wrapped_expression(type_id.0);
    match &type_.expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            let source = *state
                .symbol_db
                .symbol_sources
                .get(&symbol)
                .expect("Symbol should have a source defined.");
            let defining_type_statement_id = match source {
                SymbolSource::Type(type_statement_id) => type_statement_id,
                _ => return false,
            };
            let defining_type_statement = state.registry.type_statement(defining_type_statement_id);
            let defining_type_statement_variants = state
                .registry
                .variant_list(defining_type_statement.variant_list_id);
            defining_type_statement_variants.is_empty()
        }
        Expression::Call(call) => {
            let callee = state.registry.wrapped_expression(call.callee_id);
            match &callee.expression {
                Expression::Identifier(callee_identifier) => {
                    let symbol = state.symbol_db.identifier_symbols.get(callee_identifier.id);
                    let source = *state
                        .symbol_db
                        .symbol_sources
                        .get(&symbol)
                        .expect("Symbol should have a source defined.");
                    let defining_type_statement_id = match source {
                        SymbolSource::Type(type_statement_id) => type_statement_id,
                        _ => return false,
                    };
                    let defining_type_statement =
                        state.registry.type_statement(defining_type_statement_id);
                    let defining_type_statement_variants = state
                        .registry
                        .variant_list(defining_type_statement.variant_list_id);
                    defining_type_statement_variants.is_empty()
                }
                _ => false,
            }
        }
        _other_type => false,
    }
}

pub fn can_apply_well_typed_fun_call(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    fun_id: NodeId<Fun>,
    arg_nfids: &[NormalFormNodeId],
) -> bool {
    let fun = registry.fun(fun_id);
    let fun_param_ids = registry.param_list(fun.param_list_id);

    #[derive(Clone, Debug)]
    enum RecursionStatus {
        NonRecursive,
        Recursive { decreasing_param_index: usize },
    }
    let recursion_status = {
        let decreasing_param_index = fun_param_ids.iter().position(|param_id| {
            let param = registry.param(*param_id);
            param.is_dashed
        });
        match decreasing_param_index {
            Some(decreasing_param_index) => RecursionStatus::Recursive {
                decreasing_param_index,
            },
            None => RecursionStatus::NonRecursive,
        }
    };

    match recursion_status {
        RecursionStatus::NonRecursive => true,
        RecursionStatus::Recursive {
            decreasing_param_index,
        } => match arg_nfids.get(decreasing_param_index) {
            Some(decreasing_arg_nfid) => {
                is_normal_form_a_variant_call(registry, symbol_db, *decreasing_arg_nfid)
            }
            _ => false,
        },
    }
}

pub fn is_normal_form_a_variant_call(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    nfid: NormalFormNodeId,
) -> bool {
    as_variant_call(registry, symbol_db, nfid).is_some()
}

/// If the expression referenced by `nfid` is not a variant call, returns `None`.
pub fn as_variant_call(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    nfid: NormalFormNodeId,
) -> Option<(NodeId<Variant>, ListId<NodeId<WrappedExpression>>)> {
    let wrapped = registry.wrapped_expression(nfid.0);
    match &wrapped.expression {
        Expression::Call(call) => {
            let callee = registry.wrapped_expression(call.callee_id);
            match &callee.expression {
                Expression::Dot(callee_dot) => {
                    let symbol = symbol_db.identifier_symbols.get(callee_dot.right_id);
                    let symbol_source = symbol_db
                        .symbol_sources
                        .get(&symbol)
                        .expect("A symbol bound to a Dot expression RHS should have a source.");
                    match symbol_source {
                        SymbolSource::Variant(variant_id) => Some((*variant_id, call.arg_list_id)),
                        _other_source => None,
                    }
                }
                _other_callee => None,
            }
        }
        _other_expression => None,
    }
}

pub fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

/// This returns `Ok(nfid)` unless
/// `goal` equals `Some(g)` where `nfid` is **not** equal to `g` under
/// the definition type equality.
pub(super) fn ok_unless_contradicts_goal(
    state: &mut TypeCheckState,
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

#[derive(Clone, Copy, Debug)]
pub struct AlgebraicDataType {
    pub callee_id: NodeId<Identifier>,
    pub arg_list_id: ListId<NodeId<WrappedExpression>>,
}

pub(super) fn as_algebraic_data_type(
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

pub fn are_expressions_equal_ignoring_ids(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    a: NodeId<WrappedExpression>,
    b: NodeId<WrappedExpression>,
) -> bool {
    let node_info = (registry, symbol_db);
    let a_sih = sih_cache.get_structural_identity_hash(a, node_info);
    let b_sih = sih_cache.get_structural_identity_hash(b, node_info);
    a_sih == b_sih
}

pub fn is_term_a_non_strict_subterm(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    sub_id: NodeId<WrappedExpression>,
    super_id: NodeId<WrappedExpression>,
) -> bool {
    if are_expressions_equal_ignoring_ids(registry, symbol_db, sih_cache, sub_id, super_id) {
        return true;
    }

    // TODO: We should probably cache this too.

    let super_ = registry.wrapped_expression(super_id);
    match &super_.expression {
        Expression::Identifier(_) => false,
        Expression::Dot(dot) => {
            is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, dot.left_id)
        }
        Expression::Call(call) => {
            let arg_ids = registry.wrapped_expression_list(call.arg_list_id);
            is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, call.callee_id)
                || arg_ids.iter().copied().any(|arg_id| {
                    is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, arg_id)
                })
        }
        Expression::Fun(fun) => {
            let param_ids = registry.param_list(fun.param_list_id);

            param_ids.iter().copied().any(|param_id| {
                let param = registry.param(param_id);
                is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, param.type_id)
            }) || is_term_a_non_strict_subterm(
                registry,
                symbol_db,
                sih_cache,
                sub_id,
                fun.return_type_id,
            ) || is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, fun.body_id)
        }
        Expression::Match(match_) => {
            let case_ids = registry.match_case_list(match_.case_list_id);
            is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, match_.matchee_id)
                || case_ids.iter().copied().any(|case_id| {
                    let case = registry.match_case(case_id);
                    is_term_a_non_strict_subterm(
                        registry,
                        symbol_db,
                        sih_cache,
                        sub_id,
                        case.output_id,
                    )
                })
        }
        Expression::Forall(forall) => {
            let param_ids = registry.param_list(forall.param_list_id);
            param_ids.iter().copied().any(|param_id| {
                let param = registry.param(param_id);
                is_term_a_non_strict_subterm(registry, symbol_db, sih_cache, sub_id, param.type_id)
            }) || is_term_a_non_strict_subterm(
                registry,
                symbol_db,
                sih_cache,
                sub_id,
                forall.output_id,
            )
        }
    }
}
