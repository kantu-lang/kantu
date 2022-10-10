use super::*;

#[derive(Clone, Debug)]
pub enum FusionResult {
    Exploded,
    Fused(Vec<Substitution>),
}

pub(super) fn compute_ltr_fusion_of_well_typed_expressions(
    state: &mut TypeCheckState,
    left_id: NodeId<WrappedExpression>,
    right_id: NodeId<WrappedExpression>,
) -> Result<FusionResult, TypeError> {
    let normalized_left_id = evaluate_well_typed_expression(
        &mut state.registry,
        &mut state.symbol_db,
        &mut state.sih_cache,
        &mut state.fv_cache,
        state.type0_identifier_id,
        left_id,
    )?;
    let normalized_right_id = evaluate_well_typed_expression(
        &mut state.registry,
        &mut state.symbol_db,
        &mut state.sih_cache,
        &mut state.fv_cache,
        state.type0_identifier_id,
        right_id,
    )?;
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
            &mut state.sih_cache,
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
    fn substitute_non_identical_terms_based_on_subterm_status(
        state: &mut TypeCheckState,
        left_id: NormalFormNodeId,
        right_id: NormalFormNodeId,
        happy_path_lhs: SubstitutionLhs,
    ) -> FusionResult {
        let left_subterm_right = is_term_a_non_strict_subterm(
            &state.registry,
            &state.symbol_db,
            &mut state.sih_cache,
            left_id.0,
            right_id.0,
        );
        let right_subterm_left = is_term_a_non_strict_subterm(
            &state.registry,
            &state.symbol_db,
            &mut state.sih_cache,
            right_id.0,
            left_id.0,
        );
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
        FusionCase::LeftReplacable { left_symbol } => {
            Ok(substitute_non_identical_terms_based_on_subterm_status(
                state,
                left_id,
                right_id,
                SubstitutionLhs::Symbol(left_symbol),
            ))
        }
        FusionCase::LeftIrreplacableRightReplacable { right_symbol } => {
            Ok(substitute_non_identical_terms_based_on_subterm_status(
                state,
                right_id,
                left_id,
                SubstitutionLhs::Symbol(right_symbol),
            ))
        }
        FusionCase::NeitherReplacable => {
            let raw_result = substitute_non_identical_terms_based_on_subterm_status(
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