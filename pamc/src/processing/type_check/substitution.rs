use super::*;

// TODO: Apply all the substitutions at once to speed things up.
pub fn apply_substitutions(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    target_id: ExpressionId,
    substitutions: impl IntoIterator<Item = Substitution>,
) -> ExpressionId {
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
    target_id: ExpressionId,
    substitutions: Substitution,
) -> ExpressionId {
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
    target_id: ExpressionId,
    substitution: Substitution,
) -> ExpressionId {
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
        SymbolSource::Type(id) => {
            let type_ = registry.type_statement(id);
            identifier_id_to_nfid(registry, type_.name_id)
        }
        SymbolSource::Variant(id) => {
            let variant = registry.variant(id);
            identifier_id_to_nfid(registry, variant.name_id)
        }
        SymbolSource::TypedParam(id) => {
            let param = registry.param(id);
            identifier_id_to_nfid(registry, param.name_id)
        }
        SymbolSource::UntypedParam(id) => identifier_id_to_nfid(registry, id),
        SymbolSource::Let(id) => {
            let let_ = registry.let_statement(id);
            identifier_id_to_nfid(registry, let_.name_id)
        }
        SymbolSource::Fun(id) => {
            let fun = registry.fun(id);
            identifier_id_to_nfid(registry, fun.name_id)
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
    target_id: ExpressionId,
    from_id: ExpressionId,
    to_id: ExpressionId,
) -> ExpressionId {
    // We can avoid capture avoiding by checking if `substitution.to` includes a bound variable
    // any time we enter a node with params (e.g., fun, forall, match case).
    // Or, we could just always substitute said params with new params to avoid capture.
    // In either case, we'll need to assign symbols accordingly.

    if are_expressions_equal_ignoring_ids(registry, symbol_db, sih_cache, target_id, from_id) {
        return to_id;
    }

    match target_id {
        ExpressionId::Name(_) => target_id,
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id);
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
                let old_arg_ids = registry.expression_list(old_arg_list_id).to_vec();
                let new_arg_ids: Vec<ExpressionId> = old_arg_ids
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
                    registry.add_expression_list(new_arg_ids)
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
                ExpressionId::Call(new_call_id)
            }
        }
        ExpressionId::Fun(fun_id) => {
            let fun = registry.fun(fun_id);
            let old_name_id = fun.name_id;
            let old_param_list_id = fun.param_list_id;
            let old_return_type_id = fun.return_type_id;
            let old_body_id = fun.body_id;

            let old_param_ids = registry.param_list(old_param_list_id).to_vec();

            let (freshening_pairs, freshening_substitutions): (
                Vec<(Symbol, Symbol)>,
                Vec<Substitution>,
            ) = {
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
                (freshening_pairs, freshening_substitutions)
            };

            let new_param_list_id = {
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
                            // TODO: We may need to move this up before calling `are_expressions_equal_ignoring_ids`
                            // since I'm not sure how things will play out with symbol source registration.
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

            let new_body_id = {
                let freshened_body_id = apply_substitutions(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    old_body_id,
                    freshening_substitutions.iter().copied(),
                );
                apply_single_substitution_using_lhs_expression(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    freshened_body_id,
                    from_id,
                    to_id,
                )
            };

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
                    let new_name_id = {
                        let component_list_id = registry.add_identifier_list(vec![new_name_id]);
                        registry.add_name_expression_and_overwrite_its_id(NameExpression {
                            id: dummy_id(),
                            component_list_id,
                        })
                    };
                    let wrapped_new_name_id = ExpressionId::Name(new_name_id);
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

                ExpressionId::Fun(new_fun_id)
            }
        }
        ExpressionId::Match(match_id) => {
            let match_ = registry.match_(match_id);
            let old_match = registry.match_(match_.id);
            let old_matchee_id = old_match.matchee_id;
            let old_case_list_id = old_match.case_list_id;
            let old_case_ids = registry.match_case_list(old_case_list_id).to_vec();

            let new_matchee_id = apply_single_substitution_using_lhs_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                old_matchee_id,
                from_id,
                to_id,
            );

            let new_case_list_id = {
                let new_case_ids: Vec<NodeId<MatchCase>> = old_case_ids
                    .iter()
                    .copied()
                    .map(|old_case_id| {
                        let old_case = registry.match_case(old_case_id);
                        let old_case_variant_name_id = old_case.variant_name_id;
                        let old_case_param_list_id = old_case.param_list_id;
                        let old_case_output_id = old_case.output_id;

                        let old_case_param_ids =
                            registry.identifier_list(old_case_param_list_id).to_vec();

                        let (freshening_pairs, freshening_substitutions) = {
                            let (from_free_variables, to_free_variables) = {
                                let node_info = (&*registry, &*symbol_db);
                                fv_cache.get_free_variables_2((from_id, to_id), node_info)
                            };
                            let captured_param_symbols: Vec<Symbol> = old_case_param_ids
                                .iter()
                                .copied()
                                .map(|param_id| symbol_db.identifier_symbols.get(param_id))
                                .filter(|s| {
                                    from_free_variables.contains(*s)
                                        || to_free_variables.contains(*s)
                                })
                                .collect();
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
                                    let fresh_param_identifier_id =
                                        get_wrapped_identifier_id_for_symbol(
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
                            (freshening_pairs, freshening_substitutions)
                        };

                        let new_param_list_id = {
                            let new_case_param_ids: Vec<NodeId<Identifier>> = old_case_param_ids
                                .iter()
                                .copied()
                                .map(|old_param_id| {
                                    let old_symbol = symbol_db.identifier_symbols.get(old_param_id);
                                    if let Some((_, corresponding_fresh_symbol)) = freshening_pairs
                                        .iter()
                                        .copied()
                                        .find(|(captured, _fresh)| *captured == old_symbol)
                                    {
                                        let old_param = registry.identifier(old_param_id);
                                        let old_param_start = old_param.start;
                                        let old_param_name = old_param.name.clone();
                                        let new_param_id = registry
                                            .add_identifier_and_overwrite_its_id(Identifier {
                                                id: dummy_id(),
                                                start: old_param_start,
                                                name: old_param_name,
                                            });

                                        symbol_db
                                            .identifier_symbols
                                            .insert(new_param_id, corresponding_fresh_symbol);

                                        new_param_id
                                    } else {
                                        old_param_id
                                    }
                                })
                                .collect();
                            if old_case_param_ids
                                .iter()
                                .copied()
                                .zip(new_case_param_ids.iter().copied())
                                .all(|(old_param_id, new_param_id)| old_param_id == new_param_id)
                            {
                                old_case_param_list_id
                            } else {
                                registry.add_identifier_list(new_case_param_ids)
                            }
                        };

                        let new_case_output_id = {
                            let freshened_case_output_id = apply_substitutions(
                                registry,
                                symbol_db,
                                sih_cache,
                                fv_cache,
                                type0_identifier_id,
                                old_case_output_id,
                                freshening_substitutions.iter().copied(),
                            );
                            apply_single_substitution_using_lhs_expression(
                                registry,
                                symbol_db,
                                sih_cache,
                                fv_cache,
                                type0_identifier_id,
                                freshened_case_output_id,
                                from_id,
                                to_id,
                            )
                        };

                        if old_case_param_list_id == new_param_list_id
                            && old_case_output_id == new_case_output_id
                        {
                            old_case_id
                        } else {
                            registry.add_match_case_and_overwrite_its_id(MatchCase {
                                id: dummy_id(),
                                variant_name_id: old_case_variant_name_id,
                                param_list_id: new_param_list_id,
                                output_id: new_case_output_id,
                            })
                        }
                    })
                    .collect();
                // TODO: Replace .all(...==...) with .eq(...)
                // in other parts of the codebase as well.
                if old_case_ids
                    .iter()
                    .copied()
                    .eq(new_case_ids.iter().copied())
                {
                    old_case_list_id
                } else {
                    registry.add_match_case_list(new_case_ids)
                }
            };

            if old_matchee_id == new_matchee_id && old_case_list_id == new_case_list_id {
                target_id
            } else {
                let new_match_id = registry.add_match_and_overwrite_its_id(Match {
                    id: dummy_id(),
                    matchee_id: new_matchee_id,
                    case_list_id: new_case_list_id,
                });
                ExpressionId::Match(new_match_id)
            }
        }
        ExpressionId::Forall(forall_id) => {
            let forall = registry.forall(forall_id);
            let old_param_list_id = forall.param_list_id;
            let old_output_id = forall.output_id;

            let old_param_ids = registry.param_list(old_param_list_id).to_vec();

            let (freshening_pairs, freshening_substitutions): (
                Vec<(Symbol, Symbol)>,
                Vec<Substitution>,
            ) = {
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
                (freshening_pairs, freshening_substitutions)
            };

            let new_param_list_id = {
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

            let new_output_id = {
                let freshened_output_id = apply_substitutions(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    old_output_id,
                    freshening_substitutions.iter().copied(),
                );
                apply_single_substitution_using_lhs_expression(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    freshened_output_id,
                    from_id,
                    to_id,
                )
            };

            if old_param_list_id == new_param_list_id && old_output_id == new_output_id {
                target_id
            } else {
                let new_forall_id = registry.add_forall_and_overwrite_its_id(Forall {
                    id: dummy_id(),
                    param_list_id: new_param_list_id,
                    output_id: new_output_id,
                });
                ExpressionId::Forall(new_forall_id)
            }
        }
    }
}
