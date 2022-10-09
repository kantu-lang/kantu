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
    _state: &TypeCheckState,
    _production_type_id: NormalFormNodeId,
    _requirement_type_id: NormalFormNodeId,
) -> bool {
    unimplemented!()
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

// TODO: Apply all the substitutions at once to speed things up.
pub fn apply_substitutions(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    target_id: NodeId<WrappedExpression>,
    substitutions: impl IntoIterator<Item = Substitution>,
) -> NodeId<WrappedExpression> {
    let mut target_id = target_id;
    for substitution in substitutions {
        target_id = apply_substitution(
            registry,
            symbol_db,
            sih_cache,
            fv_cache,
            type0_identifier_id,
            target_id,
            substitution,
        );
    }
    target_id
}

/// NOTE: "Applying" a substitution means to **repeatedly** substitute until
/// no more substitutions can be made.
/// This is distinct from a "single substitution", which, as the name implies,
/// is simply a single substitution.
///
/// TODO: There should be a limit (after which we panic) to
/// safeguard against infinite loops.
pub fn apply_substitution(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    target_id: NodeId<WrappedExpression>,
    substitutions: Substitution,
) -> NodeId<WrappedExpression> {
    let mut target_id = target_id;
    loop {
        let new_id = apply_single_substitution(
            registry,
            symbol_db,
            sih_cache,
            fv_cache,
            type0_identifier_id,
            target_id,
            substitutions,
        );
        if are_expressions_equal_ignoring_ids(registry, symbol_db, sih_cache, target_id, new_id) {
            return target_id;
        }
        target_id = new_id;
    }
}

fn apply_single_substitution(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    target_id: NodeId<WrappedExpression>,
    substitution: Substitution,
) -> NodeId<WrappedExpression> {
    match substitution.from {
        SubstitutionLhs::Expression(from) => apply_single_substitution_using_lhs_expression(
            registry,
            symbol_db,
            sih_cache,
            fv_cache,
            type0_identifier_id,
            target_id,
            from,
            substitution.to.0,
        ),
        SubstitutionLhs::Symbol(from_symbol) => {
            let wrapped_identifier_id = get_wrapped_identifier_id_for_symbol(
                registry,
                symbol_db,
                type0_identifier_id,
                from_symbol,
            );
            apply_single_substitution_using_lhs_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                target_id,
                wrapped_identifier_id.0,
                substitution.to.0,
            )
        }
    }
}

fn get_wrapped_identifier_id_for_symbol(
    registry: &mut NodeRegistry,
    symbol_db: &SymbolDatabase,
    type0_identifier_id: NormalFormNodeId,
    symbol: Symbol,
) -> NormalFormNodeId {
    let source = *symbol_db
        .symbol_sources
        .get(&symbol)
        .expect("Symbol not found");
    match source {
        // type variant typed_param untyped_param let fun builtin_type_title_case
        SymbolSource::Type(id) => {
            let type_ = registry.type_statement(id);
            let identifier = registry.identifier(type_.name_id).clone();
            NormalFormNodeId(registry.add_wrapped_expression_and_overwrite_its_id(
                WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(identifier),
                },
            ))
        }
        SymbolSource::Variant(id) => {
            let variant = registry.variant(id);
            let identifier = registry.identifier(variant.name_id).clone();
            NormalFormNodeId(registry.add_wrapped_expression_and_overwrite_its_id(
                WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(identifier),
                },
            ))
        }
        SymbolSource::TypedParam(id) => {
            let param = registry.param(id);
            let identifier = registry.identifier(param.name_id).clone();
            NormalFormNodeId(registry.add_wrapped_expression_and_overwrite_its_id(
                WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(identifier),
                },
            ))
        }
        SymbolSource::UntypedParam(id) => {
            let identifier = registry.identifier(id).clone();
            NormalFormNodeId(registry.add_wrapped_expression_and_overwrite_its_id(
                WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(identifier),
                },
            ))
        }
        SymbolSource::Let(id) => {
            let let_ = registry.let_statement(id);
            let identifier = registry.identifier(let_.name_id).clone();
            NormalFormNodeId(registry.add_wrapped_expression_and_overwrite_its_id(
                WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(identifier),
                },
            ))
        }
        SymbolSource::Fun(id) => {
            let fun = registry.fun(id);
            let identifier = registry.identifier(fun.name_id).clone();
            NormalFormNodeId(registry.add_wrapped_expression_and_overwrite_its_id(
                WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(identifier),
                },
            ))
        }
        SymbolSource::BuiltinTypeTitleCase => type0_identifier_id,
    }
}

fn apply_single_substitution_using_lhs_expression(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    target_id: NodeId<WrappedExpression>,
    from_id: NodeId<WrappedExpression>,
    to_id: NodeId<WrappedExpression>,
) -> NodeId<WrappedExpression> {
    // We can avoid capture avoiding by checking if `substitution.to` includes a bound variable
    // any time we enter a node with params (e.g., fun, forall, match case).
    // Or, we could just always substitute said params with new params to avoid capture.
    // In either case, we'll need to assign symbols accordingly.

    if are_expressions_equal_ignoring_ids(registry, symbol_db, sih_cache, target_id, from_id) {
        return to_id;
    }

    let target = registry.wrapped_expression(target_id);
    match &target.expression {
        Expression::Identifier(_) => target_id,
        // TODO: In the future, if we allow arbitrary
        // expressions in the lhs of Dot expression,
        // we will need to handle that here.
        Expression::Dot(_) => target_id,
        Expression::Call(call) => {
            let old_callee_id = call.callee_id;
            let old_arg_list_id = call.arg_list_id;

            let new_callee_id = apply_single_substitution_using_lhs_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                old_callee_id,
                from_id,
                to_id,
            );
            let new_arg_list_id = {
                let old_arg_ids = registry.wrapped_expression_list(old_arg_list_id).to_vec();
                let new_arg_ids: Vec<NodeId<WrappedExpression>> = old_arg_ids
                    .iter()
                    .copied()
                    .map(|arg_id| {
                        apply_single_substitution_using_lhs_expression(
                            registry,
                            symbol_db,
                            sih_cache,
                            fv_cache,
                            type0_identifier_id,
                            arg_id,
                            from_id,
                            to_id,
                        )
                    })
                    .collect();
                if old_arg_ids
                    .iter()
                    .copied()
                    .zip(new_arg_ids.iter().copied())
                    .all(|(old_arg_id, new_arg_id)| {
                        are_expressions_equal_ignoring_ids(
                            registry, symbol_db, sih_cache, old_arg_id, new_arg_id,
                        )
                    })
                {
                    old_arg_list_id
                } else {
                    registry.add_wrapped_expression_list(new_arg_ids)
                }
            };
            if old_callee_id == new_callee_id && old_arg_list_id == new_arg_list_id {
                target_id
            } else {
                let new_call_id = registry.add_call_and_overwrite_its_id(Call {
                    id: dummy_id(),
                    callee_id: new_callee_id,
                    arg_list_id: new_arg_list_id,
                });
                let new_call = registry.call(new_call_id).clone();
                registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Call(Box::new(new_call)),
                })
            }
        }
        Expression::Fun(fun) => {
            let old_name_id = fun.name_id;
            let old_param_list_id = fun.param_list_id;
            let old_return_type_id = fun.return_type_id;
            let old_body_id = fun.body_id;

            let old_param_ids = registry.param_list(old_param_list_id).to_vec();

            // let type_subbed_param_ids: Vec<NodeId<Param>> = old_param_ids
            //     .iter()
            //     .copied()
            //     .map(|old_param_id| {
            //         let param = registry.param(old_param_id);
            //         let param_is_dashed = param.is_dashed;
            //         let param_name_id = param.name_id;
            //         let param_type_id = param.type_id;
            //         let new_param_type_id = apply_single_substitution_using_lhs_expression(
            //             registry,
            //             symbol_db,
            //             sih_cache,
            //             fv_cache,
            //             type0_identifier_id,
            //             param_type_id,
            //             from_id,
            //             to_id,
            //         );
            //         if are_expressions_equal_ignoring_ids(
            //             registry,
            //             symbol_db,
            //             sih_cache,
            //             param_type_id,
            //             new_param_type_id,
            //         ) {
            //             old_param_id
            //         } else {
            //             registry.add_param_and_overwrite_its_id(Param {
            //                 id: dummy_id(),
            //                 is_dashed: param_is_dashed,
            //                 name_id: param_name_id,
            //                 type_id: new_param_type_id,
            //             })
            //         }
            //     })
            //     .collect();

            // MARK

            let (from_free_variables, to_free_variables) = {
                let node_info = (&*registry, &*symbol_db);
                fv_cache.get_free_variables_2((from_id, to_id), node_info)
            };

            let captured_param_symbols: Vec<Symbol> = old_param_ids
                .iter()
                .copied()
                .map(|param_id| {
                    let param = registry.param(param_id);
                    symbol_db.identifier_symbols.get(param.name_id)
                })
                .filter(|s| from_free_variables.contains(*s) || to_free_variables.contains(*s))
                .collect();

            // TODO: Delete
            // let new_param_list_id = {
            //     if old_param_ids
            //         .iter()
            //         .copied()
            //         .zip(new_param_ids.iter().copied())
            //         .all(|(old_param_id, new_param_id)| old_param_id == new_param_id)
            //     {
            //         old_param_list_id
            //     } else {
            //         registry.add_param_list(new_param_ids)
            //     }
            // };

            let fresh_param_symbols: Vec<Symbol> = captured_param_symbols
                .iter()
                .map(|_| symbol_db.provider.new_symbol())
                .collect();
            let freshening_pairs: Vec<(Symbol, Symbol)> = captured_param_symbols
                .iter()
                .copied()
                .zip(fresh_param_symbols.iter().copied())
                .collect();
            let freshening_substitutions: Vec<Substitution> = freshening_pairs
                .iter()
                .copied()
                .map(|(captured_param_symbol, fresh_param_symbol)| {
                    let fresh_param_identifier_id = get_wrapped_identifier_id_for_symbol(
                        registry,
                        symbol_db,
                        type0_identifier_id,
                        fresh_param_symbol,
                    );
                    Substitution {
                        from: SubstitutionLhs::Symbol(captured_param_symbol),
                        to: fresh_param_identifier_id,
                    }
                })
                .collect();
            let new_param_ids: Vec<NodeId<Param>> = old_param_ids
                .iter()
                .copied()
                .map(|old_param_id| {
                    let old_param = registry.param(old_param_id);
                    let old_param_is_dashed = old_param.is_dashed;
                    let old_param_name_id = old_param.name_id;
                    let old_param_type_id = old_param.type_id;

                    let old_symbol = symbol_db.identifier_symbols.get(old_param.name_id);
                    let new_param_name_id = if let Some((_, corresponding_fresh_symbol)) =
                        freshening_pairs
                            .iter()
                            .copied()
                            .find(|(captured, _fresh)| *captured == old_symbol)
                    {
                        let old_param_name = registry.identifier(old_param_name_id);
                        let old_param_name_start = old_param_name.start;
                        let old_param_name_name = old_param_name.name.clone();
                        let new_param_name_id =
                            registry.add_identifier_and_overwrite_its_id(Identifier {
                                id: dummy_id(),
                                start: old_param_name_start,
                                name: old_param_name_name,
                            });

                        symbol_db
                            .identifier_symbols
                            .insert(new_param_name_id, corresponding_fresh_symbol);

                        new_param_name_id
                    } else {
                        old_param_name_id
                    };

                    let new_param_type_id = {
                        let freshened_param_type_id = apply_substitutions(
                            registry,
                            symbol_db,
                            sih_cache,
                            fv_cache,
                            type0_identifier_id,
                            old_param_type_id,
                            freshening_substitutions.iter().copied(),
                        );
                        apply_single_substitution_using_lhs_expression(
                            registry,
                            symbol_db,
                            sih_cache,
                            fv_cache,
                            type0_identifier_id,
                            freshened_param_type_id,
                            from_id,
                            to_id,
                        )
                    };

                    if old_param_name_id == new_param_name_id
                        && are_expressions_equal_ignoring_ids(
                            registry,
                            symbol_db,
                            sih_cache,
                            old_param_type_id,
                            new_param_type_id,
                        )
                    {
                        old_param_id
                    } else {
                        let new_param_id = registry.add_param_and_overwrite_its_id(Param {
                            id: dummy_id(),
                            is_dashed: old_param_is_dashed,
                            name_id: old_param_name_id,
                            type_id: new_param_type_id,
                        });

                        symbol_db.symbol_sources.insert(
                            symbol_db.identifier_symbols.get(new_param_name_id),
                            SymbolSource::TypedParam(new_param_id),
                        );

                        new_param_id
                    }
                })
                .collect();
            let new_param_list_id = {
                if old_param_ids
                    .iter()
                    .copied()
                    .zip(new_param_ids.iter().copied())
                    .all(|(old_param_id, new_param_id)| old_param_id == new_param_id)
                {
                    old_param_list_id
                } else {
                    registry.add_param_list(new_param_ids)
                }
            };

            let freshened_body_id = apply_substitutions(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                old_body_id,
                freshening_substitutions.iter().copied(),
            );
            let new_body_id = apply_single_substitution_using_lhs_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                freshened_body_id,
                from_id,
                to_id,
            );

            let new_return_type_id = {
                let freshened_return_type_id = apply_substitutions(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    old_return_type_id,
                    freshening_substitutions.iter().copied(),
                );
                apply_single_substitution_using_lhs_expression(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    freshened_return_type_id,
                    from_id,
                    to_id,
                )
            };

            if old_param_list_id == new_param_list_id
                && old_return_type_id == new_return_type_id
                && old_body_id == new_body_id
            {
                target_id
            } else {
                let new_name_id = {
                    let old_name = registry.identifier(old_name_id);
                    let old_name_start = old_name.start;
                    let old_name_name = old_name.name.clone();
                    let new_name_id = registry.add_identifier_and_overwrite_its_id(Identifier {
                        id: dummy_id(),
                        start: old_name_start,
                        name: old_name_name,
                    });
                    let new_fun_symbol = symbol_db.provider.new_symbol();

                    symbol_db
                        .identifier_symbols
                        .insert(new_name_id, new_fun_symbol);

                    new_name_id
                };
                let renamed_body_id = {
                    let new_name = registry.identifier(new_name_id).clone();
                    let wrapped_new_name_id =
                        registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                            id: dummy_id(),
                            expression: Expression::Identifier(new_name),
                        });
                    apply_single_substitution_using_lhs_expression(
                        registry,
                        symbol_db,
                        sih_cache,
                        fv_cache,
                        type0_identifier_id,
                        new_body_id,
                        target_id,
                        wrapped_new_name_id,
                    )
                };
                let new_fun_id = registry.add_fun_and_overwrite_its_id(Fun {
                    id: dummy_id(),
                    name_id: new_name_id,
                    param_list_id: new_param_list_id,
                    return_type_id: new_return_type_id,
                    body_id: renamed_body_id,
                });

                symbol_db.symbol_sources.insert(
                    symbol_db.identifier_symbols.get(new_name_id),
                    SymbolSource::Fun(new_fun_id),
                );

                let new_fun = registry.fun(new_fun_id).clone();
                registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Fun(Box::new(new_fun)),
                })
            }
        }
        _ => unimplemented!(),
    }
}
