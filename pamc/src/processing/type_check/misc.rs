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

// IDEA: Use hashing when registering the nodes to speed up
// equality checking.
pub fn are_expressions_equal_ignoring_ids(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    a: NodeId<WrappedExpression>,
    b: NodeId<WrappedExpression>,
) -> bool {
    unimplemented!();
}

pub fn is_term_a_subterm(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    sub: NodeId<WrappedExpression>,
    sup: NodeId<WrappedExpression>,
) -> bool {
    unimplemented!()
}

// TODO: Make this apply_capture_avoiding_substitutions
pub fn apply_substitutions(
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
pub fn apply_substitution(
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
