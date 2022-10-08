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

pub use type_check_node::type_check_file;
mod type_check_node;

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
    state: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    expression: NodeId<WrappedExpression>,
) -> Result<NormalFormNodeId, TypeError> {
    let mut current = expression;
    loop {
        let step_result = perform_eval_step_on_well_typed_expression(state, symbol_db, current)?;
        match step_result {
            EvalStepResult::Stepped(new_current) => current = new_current,
            EvalStepResult::CouldNotStepBecauseNormalForm(nfid) => break Ok(nfid),
        }
    }
}

#[derive(Clone, Debug)]
enum EvalStepResult {
    Stepped(NodeId<WrappedExpression>),
    CouldNotStepBecauseNormalForm(NormalFormNodeId),
}

fn perform_eval_step_on_well_typed_expression(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    expression_id: NodeId<WrappedExpression>,
) -> Result<EvalStepResult, TypeError> {
    fn perform_eval_step_on_identifier_or_dot_based_on_symbol(
        registry: &mut NodeRegistry,
        symbol_db: &mut SymbolDatabase,
        symbol: Symbol,
        original_expression_id: NodeId<WrappedExpression>,
    ) -> EvalStepResult {
        let source = *symbol_db
            .symbol_sources
            .get(&symbol)
            .expect("Symbol referenced in identifier expression should have a source.");
        match source {
            SymbolSource::Type(_)
            | SymbolSource::Variant(_)
            | SymbolSource::TypedParam(_)
            | SymbolSource::UntypedParam(_)
            | SymbolSource::Fun(_)
            | SymbolSource::BuiltinTypeTitleCase => {
                // This is safe because a identifier expression with
                // a symbol defined by a type, variant, param, fun, or Type0
                // is a normal form.
                EvalStepResult::CouldNotStepBecauseNormalForm(NormalFormNodeId(
                    original_expression_id,
                ))
            }
            SymbolSource::Let(let_id) => {
                let let_statement = registry.let_statement(let_id);
                EvalStepResult::Stepped(let_statement.value_id)
            }
        }
    }

    let wrapped = registry.wrapped_expression(expression_id);
    match &wrapped.expression {
        Expression::Identifier(identifier) => {
            let symbol = symbol_db.identifier_symbols.get(identifier.id);
            Ok(perform_eval_step_on_identifier_or_dot_based_on_symbol(
                registry,
                symbol_db,
                symbol,
                expression_id,
            ))
        }
        Expression::Dot(dot) => {
            let symbol = symbol_db.identifier_symbols.get(dot.right_id);
            Ok(perform_eval_step_on_identifier_or_dot_based_on_symbol(
                registry,
                symbol_db,
                symbol,
                expression_id,
            ))
        }
        Expression::Call(call) => {
            let callee_id = call.callee_id;
            let arg_list_id = call.arg_list_id;
            let callee_step_result =
                perform_eval_step_on_well_typed_expression(registry, symbol_db, callee_id)?;
            if let EvalStepResult::Stepped(stepped_callee_id) = callee_step_result {
                let stepped_call_id = registry.add_call_and_overwrite_its_id(Call {
                    id: dummy_id(),
                    callee_id: stepped_callee_id,
                    arg_list_id,
                });
                let stepped_call = registry.call(stepped_call_id).clone();
                let wrapped_stepped_id =
                    registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                        id: dummy_id(),
                        expression: Expression::Call(Box::new(stepped_call)),
                    });
                return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
            }

            let arg_ids = registry.wrapped_expression_list(arg_list_id).to_vec();
            let mut arg_nfids = Vec::with_capacity(arg_ids.len());
            for (arg_index, arg_id) in arg_ids.iter().copied().enumerate() {
                let arg_step_result =
                    perform_eval_step_on_well_typed_expression(registry, symbol_db, arg_id)?;
                match arg_step_result {
                    EvalStepResult::Stepped(stepped_arg_id) => {
                        let mut stepped_arg_ids = Vec::with_capacity(arg_ids.len());
                        stepped_arg_ids.extend(arg_ids[..arg_index].iter().copied());
                        stepped_arg_ids.push(stepped_arg_id);
                        stepped_arg_ids.extend(arg_ids[arg_index + 1..].iter().copied());
                        let stepped_arg_list_id =
                            registry.add_wrapped_expression_list(stepped_arg_ids);
                        let stepped_call_id = registry.add_call_and_overwrite_its_id(Call {
                            id: dummy_id(),
                            callee_id,
                            arg_list_id: stepped_arg_list_id,
                        });
                        let stepped_call = registry.call(stepped_call_id).clone();
                        let wrapped_stepped_id = registry
                            .add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                                id: dummy_id(),
                                expression: Expression::Call(Box::new(stepped_call)),
                            });
                        return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
                    }
                    EvalStepResult::CouldNotStepBecauseNormalForm(arg_nfid) => {
                        arg_nfids.push(arg_nfid);
                    }
                }
            }

            let callee = registry.wrapped_expression(callee_id);
            match &callee.expression {
                Expression::Identifier(callee_identifier) => {
                    let callee_symbol = symbol_db.identifier_symbols.get(callee_identifier.id);
                    let callee_source = *symbol_db.symbol_sources.get(&callee_symbol).expect("Symbol referenced in identifier expression should have a source.");
                    let callee_fun_id: NodeId<Fun> = match callee_source {
                        SymbolSource::Fun(fun_id) => fun_id,
                        other_source => panic!("Callee identifier symbol of call expression should be have a Fun source, but the source was `{:?}`.", other_source),
                    };

                    let can_substitute = can_apply_well_typed_fun_call(registry, symbol_db, callee_fun_id, &arg_nfids);
                    if !can_substitute {
                        return Ok(EvalStepResult::CouldNotStepBecauseNormalForm(
                            NormalFormNodeId(expression_id),
                        ));
                    }

                    let callee_fun = registry.fun(callee_fun_id);
                    let callee_param_list_id = callee_fun.param_list_id;
                    let callee_body_id = callee_fun.body_id;
                    let callee_param_ids = registry.param_list(callee_param_list_id).to_vec();
                    let substitutions: Vec<Substitution> = callee_param_ids
                        .iter()
                        .copied()
                        .zip(arg_nfids.iter().copied()).map(|(param_id, arg_nfid)| {
                            let param = registry.param(param_id);
                            let param_symbol = symbol_db.identifier_symbols.get(param.name_id);
                            Substitution {
                             from: SubstitutionLhs::Symbol(param_symbol),
                             to: arg_nfid
                            }
                        }).collect();
                    let application_result = apply_substitutions(registry, symbol_db, callee_body_id, substitutions);
                    Ok(EvalStepResult::Stepped(application_result))
                }
                other_normal_form_callee => panic!("A normal form callee in a well-typed Call expression should be an identifier, but was `{:?}`.", other_normal_form_callee),
            }
        }
        Expression::Fun(fun) => {
            let name_id = fun.name_id;
            let name = registry.identifier(name_id).clone();
            let wrapped_name_id =
                registry.add_wrapped_expression_and_overwrite_its_id(WrappedExpression {
                    id: dummy_id(),
                    expression: Expression::Identifier(name),
                });
            Ok(EvalStepResult::Stepped(wrapped_name_id))
        }
        _ => unimplemented!(),
    }
}

fn get_normalized_type(
    state: &mut TypeCheckState,
    symbol: Symbol,
) -> Result<NormalFormNodeId, TypeError> {
    let (unsubstituted_type_id, substitutions) = state.context.get(symbol);
    let unnormalized_type_id = apply_substitutions(
        &mut state.registry,
        &mut state.symbol_db,
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

fn can_apply_well_typed_fun_call(
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
                is_expression_a_variant_call(registry, symbol_db, decreasing_arg_nfid.0)
            }
            _ => false,
        },
    }
}

fn is_expression_a_variant_call(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    expression_id: NodeId<WrappedExpression>,
) -> bool {
    let wrapped = registry.wrapped_expression(expression_id);
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
                    matches!(symbol_source, SymbolSource::Variant(_))
                }
                _other_callee => false,
            }
        }
        _other_expression => false,
    }
}

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}

// TODO: Maybe context should be separate, since
// I feel like I'm passing `registry` and `symbol_db`
// a lot (since `context` is borrowed in such circumstances).
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

// TODO: Make this apply_capture_avoiding_substitutions
fn apply_substitutions(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
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
/// There should be a limit (after which we panic) to
/// safeguard against infinite loops.
fn apply_substitution(
    _registry: &mut NodeRegistry,
    _symbol_db: &mut SymbolDatabase,
    _type_id: NodeId<WrappedExpression>,
    _substitutions: Substitution,
) -> NodeId<WrappedExpression> {
    // We can avoid capture avoiding by checking if `substitution.to` includes a bound variable
    // any time we enter a node with params (e.g., fun, forall, match case).
    // Or, we could just always substitute said params with new params to avoid capture.
    // In either case, we'll need to assign symbols accordingly.
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
