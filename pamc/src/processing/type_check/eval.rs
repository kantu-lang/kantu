use super::*;

#[derive(Clone, Debug)]
pub enum EvalError {
    BadCallee(NormalFormId),
    NoMatchingCase(NodeId<Match>),
}

pub type EvalResult = Result<NormalFormId, (ExpressionId, EvalError)>;

pub trait BestEvalAttempt {
    fn best_eval_attempt(self) -> ExpressionId;
}

impl BestEvalAttempt for EvalResult {
    fn best_eval_attempt(self: EvalResult) -> ExpressionId {
        match self {
            Ok(nfid) => nfid.raw(),
            Err((best_attempt, _err)) => best_attempt,
        }
    }
}

type TaintedEvalResult = Result<NormalFormId, Tainted<(ExpressionId, EvalError)>>;

// TODO: Idea: Make this take a `type_id` param, to prove that the
// expression is well-typed.
pub(super) fn evaluate_well_typed_expression(state: &mut State, id: ExpressionId) -> NormalFormId {
    evaluate_possibly_ill_typed_expression(state, id).expect(
        "evaluate_possibly_ill_typed_expression should return Ok() if the expression is well-typed",
    )
}

pub(super) fn evaluate_possibly_ill_typed_expression(
    state: &mut State,
    id: ExpressionId,
) -> EvalResult {
    let out = match id {
        ExpressionId::Name(name_id) => evaluate_possibly_ill_typed_name_expression(state, name_id),
        ExpressionId::Call(call_id) => evaluate_possibly_ill_typed_call(state, call_id),
        ExpressionId::Fun(fun_id) => evaluate_possibly_ill_typed_fun(state, fun_id),
        ExpressionId::Match(match_id) => evaluate_possibly_ill_typed_match(state, match_id),
        ExpressionId::Forall(forall_id) => evaluate_possibly_ill_typed_forall(state, forall_id),
        ExpressionId::Check(check_id) => evaluate_possibly_ill_typed_check(state, check_id),
    };
    out
}

fn evaluate_possibly_ill_typed_name_expression(
    state: &mut State,
    name_id: NodeId<NameExpression>,
) -> EvalResult {
    let name = state.registry.get(name_id);
    let definition = state.context.get_definition(name.db_index, state.registry);
    match definition {
        ContextEntryDefinition::Alias { value_id } => Ok(value_id),

        ContextEntryDefinition::Adt {
            variant_name_list_id: _,
        }
        | ContextEntryDefinition::Variant { name_id: _ }
        | ContextEntryDefinition::Uninterpreted => {
            Ok(NormalFormId::unchecked_new(ExpressionId::Name(name_id)))
        }
    }
}

fn evaluate_possibly_ill_typed_call(state: &mut State, call_id: NodeId<Call>) -> EvalResult {
    fn register_normalized_nonsubstituted_fun(
        registry: &mut NodeRegistry,
        normalized_callee_id: NormalFormId,
        normalized_arg_ids: NonEmptySlice<NormalFormId>,
    ) -> EvalResult {
        let normalized_arg_ids = normalized_arg_ids.to_mapped(|id| NormalFormId::raw(*id));
        let normalized_arg_list_id = registry.add_list(normalized_arg_ids);
        let normalized_call_id = registry
            .add(Call {
                id: dummy_id(),
                span: None,
                callee_id: normalized_callee_id.raw(),
                arg_list_id: normalized_arg_list_id,
            })
            .without_spans(registry);
        Ok(NormalFormId::unchecked_new(ExpressionId::Call(
            normalized_call_id,
        )))
    }

    let call = state.registry.get(call_id).clone();

    let normalized_callee_id = match evaluate_possibly_ill_typed_expression(state, call.callee_id) {
        Ok(nfid) => nfid,
        Err((callee_best_attempt_id, err)) => {
            let best_attempt_id = ExpressionId::Call(
                state
                    .registry
                    .add(Call {
                        id: dummy_id(),
                        span: None,
                        callee_id: callee_best_attempt_id,
                        arg_list_id: call.arg_list_id,
                    })
                    .without_spans(state.registry),
            );
            return Err((best_attempt_id, err));
        }
    };

    let normalized_arg_ids: NonEmptyVec<NormalFormId> = {
        let arg_ids = state.registry.get_list(call.arg_list_id).to_non_empty_vec();
        let args_eval_result = eval_all(state, arg_ids.as_non_empty_slice());
        match args_eval_result {
            Ok(normalized_arg_ids) => normalized_arg_ids,
            Err((arg_ids_best_attempt, err)) => {
                let arg_ids_best_attempt = state.registry.add_list(arg_ids_best_attempt);
                let best_attempt_id = ExpressionId::Call(
                    state
                        .registry
                        .add(Call {
                            id: dummy_id(),
                            span: None,
                            callee_id: normalized_callee_id.raw(),
                            arg_list_id: arg_ids_best_attempt,
                        })
                        .without_spans(state.registry),
                );
                return Err((best_attempt_id, err));
            }
        }
    };

    match normalized_callee_id.raw() {
        ExpressionId::Fun(fun_id) => {
            if !can_fun_be_applied(state, fun_id, &normalized_arg_ids) {
                return register_normalized_nonsubstituted_fun(
                    state.registry,
                    normalized_callee_id,
                    normalized_arg_ids.as_non_empty_slice(),
                );
            }

            let fun = state.registry.get(fun_id).clone();
            let param_name_ids = get_param_name_ids(state, fun.param_list_id);
            let param_arity = fun.param_list_id.len();
            let shifted_normalized_arg_ids = normalized_arg_ids
                .into_iter()
                .map(|arg_id| arg_id.upshift(param_arity + 1, state.registry))
                .collect::<Vec<_>>();
            let substitutions = {
                let shifted_fun_id = NormalFormId::unchecked_new(ExpressionId::Fun(
                    fun_id.upshift(param_arity + 1, state.registry),
                ));
                const FUN_DB_INDEX: DbIndex = DbIndex(0);
                vec![Substitution {
                    from: ExpressionId::Name(add_name_expression(
                        state.registry,
                        NonEmptyVec::singleton(fun.name_id),
                        FUN_DB_INDEX,
                    )),
                    to: shifted_fun_id.raw(),
                }]
            }
            .into_iter()
            .chain(
                param_name_ids
                    .iter()
                    .copied()
                    .zip(shifted_normalized_arg_ids.iter().copied())
                    .enumerate()
                    .map(|(arg_index, (param_name_id, arg_id))| {
                        let db_index = DbIndex(param_arity - arg_index);
                        let name =
                            NormalFormId::unchecked_new(ExpressionId::Name(add_name_expression(
                                state.registry,
                                NonEmptyVec::singleton(param_name_id),
                                db_index,
                            )));
                        Substitution {
                            from: name.raw(),
                            to: arg_id.raw(),
                        }
                    }),
            )
            .collect::<Vec<_>>();

            let body_id = fun
                .body_id
                .subst_all(&substitutions, &mut state.without_context());
            let shifted_body_id = body_id.downshift(param_arity + 1, state.registry);
            evaluate_possibly_ill_typed_expression(state, shifted_body_id)
        }
        ExpressionId::Name(_) | ExpressionId::Call(_) | ExpressionId::Match(_) => {
            register_normalized_nonsubstituted_fun(
                state.registry,
                normalized_callee_id,
                normalized_arg_ids.as_non_empty_slice(),
            )
        }
        ExpressionId::Forall(_) => {
            let normalized_arg_list_id = state
                .registry
                .add_list(normalized_arg_ids.into_mapped(NormalFormId::raw));
            let best_attempt_id = ExpressionId::Call(
                state
                    .registry
                    .add(Call {
                        id: dummy_id(),
                        span: None,
                        callee_id: normalized_callee_id.raw(),
                        arg_list_id: normalized_arg_list_id,
                    })
                    .without_spans(state.registry),
            );
            Err((best_attempt_id, EvalError::BadCallee(normalized_callee_id)))
        }
        ExpressionId::Check(_) => {
            panic!("By definition, a check expression can never be a normal form.")
        }
    }
}

fn can_fun_be_applied(
    state: &mut State,
    fun_id: NodeId<Fun>,
    normalized_arg_ids: &[NormalFormId],
) -> bool {
    let param_list_id = state.registry.get(fun_id).param_list_id;
    let decreasing_param_index = get_decreasing_param_index(state, param_list_id);
    let decreasing_param_index = if let Some(i) = decreasing_param_index {
        i
    } else {
        // If there is no decreasing parameter, the function is non-recursive,
        // so it can be safely applied without causing infinite expansion.
        return true;
    };

    let decreasing_arg_id = normalized_arg_ids[decreasing_param_index];
    is_variant_expression(state, decreasing_arg_id)
}

fn get_decreasing_param_index(state: &State, param_list_id: NonEmptyParamListId) -> Option<usize> {
    match param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => state
            .registry
            .get_list(param_list_id)
            .iter()
            .copied()
            .position(|param_id| {
                let param = state.registry.get(param_id);
                param.is_dashed
            }),
        NonEmptyParamListId::Labeled(param_list_id) => state
            .registry
            .get_list(param_list_id)
            .iter()
            .copied()
            .position(|param_id| {
                let param = state.registry.get(param_id);
                param.is_dashed
            }),
    }
}

fn get_param_name_ids(
    state: &State,
    param_list_id: NonEmptyParamListId,
) -> NonEmptyVec<NodeId<Identifier>> {
    match param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => state
            .registry
            .get_list(param_list_id)
            .to_mapped(|&param_id| state.registry.get(param_id).name_id),
        NonEmptyParamListId::Labeled(param_list_id) => state
            .registry
            .get_list(param_list_id)
            .to_mapped(|&param_id| state.registry.get(param_id).name_id),
    }
}

fn evaluate_possibly_ill_typed_fun(state: &mut State, fun_id: NodeId<Fun>) -> EvalResult {
    untaint_err(state, fun_id, evaluate_possibly_ill_typed_fun_dirty)
}

fn evaluate_possibly_ill_typed_fun_dirty(
    state: &mut State,
    fun_id: NodeId<Fun>,
) -> TaintedEvalResult {
    let fun = state.registry.get(fun_id).clone();
    let normalized_param_list_id =
        match normalize_params_as_much_as_possible_and_leave_in_context(state, fun.param_list_id) {
            Ok(id) => id,
            Err(tainted) => {
                return Err(tainted.map(|(param_list_best_attempt, err)| {
                    let best_attempt_id = ExpressionId::Fun(
                        state
                            .registry
                            .add(Fun {
                                id: dummy_id(),
                                span: None,
                                name_id: fun.name_id,
                                param_list_id: param_list_best_attempt,
                                return_type_id: fun.return_type_id,
                                body_id: fun.body_id,
                                skip_type_checking_body: fun.skip_type_checking_body,
                            })
                            .without_spans(state.registry),
                    );
                    (best_attempt_id, err)
                }))
            }
        };

    let normalized_return_type_id =
        match evaluate_possibly_ill_typed_expression(state, fun.return_type_id) {
            Ok(nfid) => nfid,
            Err((return_type_best_attempt_id, err)) => {
                let best_attempt_id = ExpressionId::Fun(state.registry.add(Fun {
                    id: dummy_id(),
                    span: None,
                    name_id: fun.name_id,
                    param_list_id: normalized_param_list_id,
                    return_type_id: return_type_best_attempt_id,
                    body_id: fun.body_id,
                    skip_type_checking_body: fun.skip_type_checking_body,
                }));
                return tainted_err((best_attempt_id, err));
            }
        };
    state.context.pop_n(fun.param_list_id.len());

    Ok(NormalFormId::unchecked_new(ExpressionId::Fun(
        state
            .registry
            .add(Fun {
                id: dummy_id(),
                span: None,
                name_id: fun.name_id,
                param_list_id: normalized_param_list_id,
                return_type_id: normalized_return_type_id.raw(),
                body_id: fun.body_id,
                skip_type_checking_body: fun.skip_type_checking_body,
            })
            .without_spans(state.registry),
    )))
}

impl From<Tainted<Infallible>> for Tainted<(NonEmptyParamListId, EvalError)> {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

impl<T> From<Tainted<Infallible>> for Tainted<(NonEmptyListId<T>, EvalError)> {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

fn normalize_params_as_much_as_possible_and_leave_in_context(
    state: &mut State,
    param_list_id: NonEmptyParamListId,
) -> Result<NonEmptyParamListId, Tainted<(NonEmptyParamListId, EvalError)>> {
    Ok(match param_list_id {
        NonEmptyParamListId::Unlabeled(id) => NonEmptyParamListId::Unlabeled(
            normalize_unlabeled_params_as_much_as_possible_and_leave_in_context(state, id)
                .map_err(|tainted| {
                    tainted.map(|(best_attempt_param_list_id, err)| {
                        (
                            NonEmptyParamListId::Unlabeled(best_attempt_param_list_id),
                            err,
                        )
                    })
                })?,
        ),
        NonEmptyParamListId::Labeled(id) => NonEmptyParamListId::Labeled(
            normalize_labeled_params_as_much_as_possible_and_leave_in_context(state, id).map_err(
                |tainted| {
                    tainted.map(|(best_attempt_param_list_id, err)| {
                        (
                            NonEmptyParamListId::Labeled(best_attempt_param_list_id),
                            err,
                        )
                    })
                },
            )?,
        ),
    })
}

fn normalize_unlabeled_params_as_much_as_possible_and_leave_in_context(
    state: &mut State,
    param_list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> Result<
    NonEmptyListId<NodeId<UnlabeledParam>>,
    Tainted<(NonEmptyListId<NodeId<UnlabeledParam>>, EvalError)>,
> {
    let (&first_param_id, remaining_param_ids) = state.registry.get_list(param_list_id).to_cons();
    let normalized_first_param_id = {
        let first_param = state.registry.get(first_param_id).clone();
        let param_type_eval_res =
            evaluate_possibly_ill_typed_expression(state, first_param.type_id);
        match param_type_eval_res {
            Ok(normalized_param_type_id) => state.registry.add(UnlabeledParam {
                id: dummy_id(),
                span: None,
                is_dashed: first_param.is_dashed,
                name_id: first_param.name_id,
                type_id: normalized_param_type_id.raw(),
            }),
            Err((first_param_type_best_attempt, err)) => {
                let first_param_best_attempt_id = state.registry.add(UnlabeledParam {
                    id: dummy_id(),
                    span: None,
                    is_dashed: first_param.is_dashed,
                    name_id: first_param.name_id,
                    type_id: first_param_type_best_attempt,
                });
                let mut best_attempt = NonEmptyVec::singleton(first_param_best_attempt_id);
                best_attempt.extend(remaining_param_ids.iter().copied());
                return tainted_err((state.registry.add_list(best_attempt), err));
            }
        }
    };
    let mut normalized_param_ids = NonEmptyVec::singleton(normalized_first_param_id);
    let remaining_param_ids = remaining_param_ids.to_vec();
    for (index_in_remaining_param_ids, param_id) in remaining_param_ids.iter().copied().enumerate()
    {
        let param = state.registry.get(param_id).clone();
        let param_type_eval_res = evaluate_possibly_ill_typed_expression(state, param.type_id);

        match param_type_eval_res {
            Ok(normalized_param_type_id) => {
                normalized_param_ids.push(state.registry.add(UnlabeledParam {
                    id: dummy_id(),
                    span: None,
                    is_dashed: param.is_dashed,
                    name_id: param.name_id,
                    type_id: normalized_param_type_id.raw(),
                }));
                state.context.push(ContextEntry {
                    type_id: normalized_param_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                })?;
            }
            Err((param_type_best_attempt, err)) => {
                normalized_param_ids.push(state.registry.add(UnlabeledParam {
                    id: dummy_id(),
                    span: None,
                    is_dashed: param.is_dashed,
                    name_id: param.name_id,
                    type_id: param_type_best_attempt,
                }));
                normalized_param_ids.extend(
                    remaining_param_ids[index_in_remaining_param_ids + 1..]
                        .iter()
                        .copied(),
                );
                return tainted_err((state.registry.add_list(normalized_param_ids), err));
            }
        }
    }
    Ok(state.registry.add_list(normalized_param_ids))
}

fn normalize_labeled_params_as_much_as_possible_and_leave_in_context(
    state: &mut State,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
) -> Result<
    NonEmptyListId<NodeId<LabeledParam>>,
    Tainted<(NonEmptyListId<NodeId<LabeledParam>>, EvalError)>,
> {
    let (&first_param_id, remaining_param_ids) = state.registry.get_list(param_list_id).to_cons();
    let normalized_first_param_id = {
        let first_param = state.registry.get(first_param_id).clone();
        let param_type_eval_res =
            evaluate_possibly_ill_typed_expression(state, first_param.type_id);
        match param_type_eval_res {
            Ok(normalized_param_type_id) => state.registry.add(LabeledParam {
                id: dummy_id(),
                span: None,
                label_id: first_param.label_id,
                is_dashed: first_param.is_dashed,
                name_id: first_param.name_id,
                type_id: normalized_param_type_id.raw(),
            }),
            Err((first_param_type_best_attempt, err)) => {
                let first_param_best_attempt_id = state.registry.add(LabeledParam {
                    id: dummy_id(),
                    span: None,
                    label_id: first_param.label_id,
                    is_dashed: first_param.is_dashed,
                    name_id: first_param.name_id,
                    type_id: first_param_type_best_attempt,
                });
                let mut best_attempt = NonEmptyVec::singleton(first_param_best_attempt_id);
                best_attempt.extend(remaining_param_ids.iter().copied());
                return tainted_err((state.registry.add_list(best_attempt), err));
            }
        }
    };
    let mut normalized_param_ids = NonEmptyVec::singleton(normalized_first_param_id);
    let remaining_param_ids = remaining_param_ids.to_vec();
    for (index_in_remaining_param_ids, param_id) in remaining_param_ids.iter().copied().enumerate()
    {
        let param = state.registry.get(param_id).clone();
        let param_type_eval_res = evaluate_possibly_ill_typed_expression(state, param.type_id);

        match param_type_eval_res {
            Ok(normalized_param_type_id) => {
                normalized_param_ids.push(state.registry.add(LabeledParam {
                    id: dummy_id(),
                    span: None,
                    label_id: param.label_id,
                    is_dashed: param.is_dashed,
                    name_id: param.name_id,
                    type_id: normalized_param_type_id.raw(),
                }));
                state.context.push(ContextEntry {
                    type_id: normalized_param_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                })?;
            }
            Err((param_type_best_attempt, err)) => {
                normalized_param_ids.push(state.registry.add(LabeledParam {
                    id: dummy_id(),
                    span: None,
                    label_id: param.label_id,
                    is_dashed: param.is_dashed,
                    name_id: param.name_id,
                    type_id: param_type_best_attempt,
                }));
                normalized_param_ids.extend(
                    remaining_param_ids[index_in_remaining_param_ids + 1..]
                        .iter()
                        .copied(),
                );
                return tainted_err((state.registry.add_list(normalized_param_ids), err));
            }
        }
    }
    Ok(state.registry.add_list(normalized_param_ids))
}

fn evaluate_possibly_ill_typed_match(state: &mut State, match_id: NodeId<Match>) -> EvalResult {
    let match_ = state.registry.get(match_id).clone();
    let normalized_matchee_id =
        match evaluate_possibly_ill_typed_expression(state, match_.matchee_id) {
            Ok(nfid) => nfid,
            Err((matchee_best_attempt_id, err)) => {
                let best_attempt_id = ExpressionId::Match(state.registry.add(Match {
                    id: dummy_id(),
                    span: None,
                    matchee_id: matchee_best_attempt_id,
                    case_list_id: match_.case_list_id,
                }));
                return Err((best_attempt_id, err));
            }
        };

    let (normalized_matchee_variant_name_id, normalized_matchee_arg_list_id) =
        if let Some((variant_name_id, arg_list_id)) =
            try_as_variant_expression(state, normalized_matchee_id.raw())
        {
            (variant_name_id, arg_list_id)
        } else {
            return Ok(NormalFormId::unchecked_new(ExpressionId::Match(
                state
                    .registry
                    .add(Match {
                        id: dummy_id(),
                        span: None,
                        matchee_id: normalized_matchee_id.raw(),
                        case_list_id: match_.case_list_id,
                    })
                    .without_spans(state.registry),
            )));
        };

    let case_id = state
        .registry
        .get_possibly_empty_list(match_.case_list_id)
        .iter()
        .find(|case_id| {
            let case = state.registry.get(**case_id);
            let case_variant_name: &IdentifierName = &state.registry.get(case.variant_name_id).name;
            let matchee_variant_name: &IdentifierName =
                &state.registry.get(normalized_matchee_variant_name_id).name;
            case_variant_name == matchee_variant_name
        })
        .copied();
    let case_id = match case_id {
        Some(id) => id,
        None => {
            let best_attempt_id = ExpressionId::Match(
                state
                    .registry
                    .add(Match {
                        id: dummy_id(),
                        span: None,
                        matchee_id: normalized_matchee_id.raw(),
                        case_list_id: match_.case_list_id,
                    })
                    .without_spans(state.registry),
            );
            return Err((best_attempt_id, EvalError::NoMatchingCase(match_id)));
        }
    };

    let case = state.registry.get(case_id).clone();

    match normalized_matchee_arg_list_id {
        None => evaluate_possibly_ill_typed_expression(state, case.output_id),
        Some(normalized_matchee_arg_list_id) => {
            let case_param_ids = state
                .registry
                .get_possibly_empty_list(case.param_list_id)
                .to_vec();
            let case_arity = case_param_ids.len();
            let matchee_arg_ids = state
                .registry
                .get_list(normalized_matchee_arg_list_id)
                .to_vec();
            let substitutions: Vec<Substitution> = case_param_ids
                .iter()
                .copied()
                .zip(matchee_arg_ids.iter().copied())
                .enumerate()
                .map(|(param_index, (param_id, arg_id))| {
                    let db_index = DbIndex(case_arity - param_index - 1);
                    // We can safely call `unchecked_new` here because we know that each
                    // arg to a normal form Call is also a normal form.
                    let shifted_arg_id =
                        NormalFormId::unchecked_new(arg_id).upshift(case_arity, state.registry);
                    Substitution {
                        from: ExpressionId::Name(add_name_expression(
                            state.registry,
                            NonEmptyVec::singleton(param_id),
                            db_index,
                        )),
                        to: shifted_arg_id.raw(),
                    }
                })
                .collect();

            let substituted_body = case
                .output_id
                .subst_all(&substitutions, &mut state.without_context())
                .downshift(case_arity, state.registry);
            evaluate_possibly_ill_typed_expression(state, substituted_body)
        }
    }
}

fn evaluate_possibly_ill_typed_forall(state: &mut State, forall_id: NodeId<Forall>) -> EvalResult {
    untaint_err(state, forall_id, evaluate_possibly_ill_typed_forall_dirty)
}

fn evaluate_possibly_ill_typed_forall_dirty(
    state: &mut State,
    forall_id: NodeId<Forall>,
) -> TaintedEvalResult {
    let forall = state.registry.get(forall_id).clone();
    let normalized_param_list_id = match normalize_params_as_much_as_possible_and_leave_in_context(
        state,
        forall.param_list_id,
    ) {
        Ok(id) => id,
        Err(tainted) => {
            return Err(tainted.map(|(param_list_best_attempt, err)| {
                let best_attempt_id = ExpressionId::Forall(
                    state
                        .registry
                        .add(Forall {
                            id: dummy_id(),
                            span: None,
                            param_list_id: param_list_best_attempt,
                            output_id: forall.output_id,
                        })
                        .without_spans(state.registry),
                );
                (best_attempt_id, err)
            }))
        }
    };
    let normalized_output_id = match evaluate_possibly_ill_typed_expression(state, forall.output_id)
    {
        Ok(nfid) => nfid,
        Err((output_best_attempt_id, err)) => {
            let best_attempt_id = ExpressionId::Forall(
                state
                    .registry
                    .add(Forall {
                        id: dummy_id(),
                        span: None,
                        param_list_id: normalized_param_list_id,
                        output_id: output_best_attempt_id,
                    })
                    .without_spans(state.registry),
            );
            return tainted_err((best_attempt_id, err));
        }
    };
    state.context.pop_n(forall.param_list_id.len());

    Ok(NormalFormId::unchecked_new(ExpressionId::Forall(
        state
            .registry
            .add(Forall {
                id: dummy_id(),
                span: None,
                param_list_id: normalized_param_list_id,
                output_id: normalized_output_id.raw(),
            })
            .without_spans(state.registry),
    )))
}

fn evaluate_possibly_ill_typed_check(state: &mut State, check_id: NodeId<Check>) -> EvalResult {
    let check = state.registry.get(check_id);
    evaluate_possibly_ill_typed_expression(state, check.output_id)
}

fn eval_all(
    state: &mut State,
    original_ids: NonEmptySlice<ExpressionId>,
) -> Result<NonEmptyVec<NormalFormId>, (NonEmptyVec<ExpressionId>, EvalError)> {
    let (&first_id, remaining_ids) = original_ids.to_cons();
    let normalized_first_id = match evaluate_possibly_ill_typed_expression(state, first_id) {
        Ok(id) => id,
        Err((first_id_best_attempt, err)) => {
            let mut best_attempt = NonEmptyVec::singleton(first_id_best_attempt);
            best_attempt.extend(remaining_ids.iter().copied());
            return Err((best_attempt, err));
        }
    };
    let mut nfids = NonEmptyVec::singleton(normalized_first_id);
    for (index_in_remaining_ids, id) in remaining_ids.iter().copied().enumerate() {
        let eval_res = evaluate_possibly_ill_typed_expression(state, id);
        let nfid = match eval_res {
            Ok(nfid) => nfid,
            Err((best_attempt_id, err)) => {
                let mut best_attempt = nfids.into_mapped(NormalFormId::raw);
                best_attempt.push(best_attempt_id);
                best_attempt.extend(remaining_ids[index_in_remaining_ids + 1..].iter().copied());
                return Err((best_attempt, err));
            }
        };
        nfids.push(nfid);
    }
    Ok(nfids)
}
