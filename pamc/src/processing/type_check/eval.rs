use super::*;

#[derive(Clone, Debug)]
pub enum EvalError {
    BadCallee(NormalFormId),
    IllTypedParams(ListId<NodeId<Param>>, TypeCheckError),
    NoMatchingCase(NodeId<Match>),
}

#[derive(Clone, Debug)]
pub struct EvalResult {
    pub best_attempt_id: ExpressionId,
    pub normal_form_id: Result<NormalFormId, EvalError>,
}

impl EvalResult {
    fn ok(id: NormalFormId) -> Self {
        Self {
            best_attempt_id: id.raw(),
            normal_form_id: Ok(id),
        }
    }
}

pub(super) fn evaluate_well_typed_expression(state: &mut State, id: ExpressionId) -> NormalFormId {
    evaluate_possibly_ill_typed_expression(state, id).normal_form_id.expect(
        "evaluate_possibly_ill_typed_expression should return EvalResult::ok() if the expression is well-typed",
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
    let name = state.registry.name_expression(name_id);
    let definition = state.context.get_definition(name.db_index, state.registry);
    match definition {
        ContextEntryDefinition::Alias { value_id } => EvalResult::ok(value_id),

        ContextEntryDefinition::Adt {
            variant_name_list_id: _,
        }
        | ContextEntryDefinition::Variant { name_id: _ }
        | ContextEntryDefinition::Uninterpreted => {
            EvalResult::ok(NormalFormId::unchecked_new(ExpressionId::Name(name_id)))
        }
    }
}

fn evaluate_possibly_ill_typed_call(state: &mut State, call_id: NodeId<Call>) -> EvalResult {
    fn register_normalized_nonsubstituted_fun(
        registry: &mut NodeRegistry,
        normalized_callee_id: NormalFormId,
        normalized_arg_ids: &[NormalFormId],
    ) -> EvalResult {
        let normalized_arg_ids = normalized_arg_ids
            .iter()
            .copied()
            .map(NormalFormId::raw)
            .collect();
        let normalized_arg_list_id = registry.add_expression_list(normalized_arg_ids);
        let normalized_call_id = registry.add_call_and_overwrite_its_id(Call {
            id: dummy_id(),
            callee_id: normalized_callee_id.raw(),
            arg_list_id: normalized_arg_list_id,
        });
        EvalResult::ok(NormalFormId::unchecked_new(ExpressionId::Call(
            normalized_call_id,
        )))
    }

    let call = state.registry.call(call_id).clone();

    let normalized_callee_id = match evaluate_possibly_ill_typed_expression(state, call.callee_id) {
        EvalResult {
            best_attempt_id: _,
            normal_form_id: Ok(nfid),
        } => nfid,
        EvalResult {
            best_attempt_id: callee_best_attempt_id,
            normal_form_id: Err(err),
        } => {
            let best_attempt_id =
                ExpressionId::Call(state.registry.add_call_and_overwrite_its_id(Call {
                    id: dummy_id(),
                    callee_id: callee_best_attempt_id,
                    arg_list_id: call.arg_list_id,
                }));
            return EvalResult {
                best_attempt_id,
                normal_form_id: Err(err),
            };
        }
    };

    let normalized_arg_ids: Vec<NormalFormId> = {
        let arg_ids = state.registry.expression_list(call.arg_list_id).to_vec();
        let args_eval_result = eval_all(state, &arg_ids);
        match args_eval_result {
            Ok(normalized_arg_ids) => normalized_arg_ids,
            Err((arg_ids_best_attempt, err)) => {
                let arg_ids_best_attempt = state.registry.add_expression_list(arg_ids_best_attempt);
                let best_attempt_id =
                    ExpressionId::Call(state.registry.add_call_and_overwrite_its_id(Call {
                        id: dummy_id(),
                        callee_id: normalized_callee_id.raw(),
                        arg_list_id: arg_ids_best_attempt,
                    }));
                return EvalResult {
                    best_attempt_id,
                    normal_form_id: Err(err),
                };
            }
        }
    };

    match normalized_callee_id.raw() {
        ExpressionId::Fun(fun_id) => {
            if !can_fun_be_applied(state, fun_id, &normalized_arg_ids) {
                return register_normalized_nonsubstituted_fun(
                    state.registry,
                    normalized_callee_id,
                    &normalized_arg_ids,
                );
            }

            let fun = state.registry.fun(fun_id).clone();
            let param_ids = state.registry.param_list(fun.param_list_id).to_vec();
            let arity = param_ids.len();
            let shifted_normalized_arg_ids = normalized_arg_ids
                .into_iter()
                .map(|arg_id| arg_id.upshift(arity + 1, state.registry))
                .collect::<Vec<_>>();
            let substitutions = {
                let shifted_fun_id = NormalFormId::unchecked_new(ExpressionId::Fun(
                    fun_id.upshift(arity + 1, state.registry),
                ));
                const FUN_DB_INDEX: DbIndex = DbIndex(0);
                vec![Substitution {
                    from: ExpressionId::Name(add_name_expression(
                        state.registry,
                        vec![fun.name_id],
                        FUN_DB_INDEX,
                    )),
                    to: shifted_fun_id.raw(),
                }]
            }
            .into_iter()
            .chain(
                param_ids
                    .iter()
                    .copied()
                    .zip(shifted_normalized_arg_ids.iter().copied())
                    .enumerate()
                    .map(|(arg_index, (param_id, arg_id))| {
                        let param_name_id = state.registry.param(param_id).name_id;
                        let db_index = DbIndex(arity - arg_index);
                        let name = NormalFormId::unchecked_new(ExpressionId::Name(
                            add_name_expression(state.registry, vec![param_name_id], db_index),
                        ));
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
            let shifted_body_id = body_id.downshift(arity + 1, state.registry);
            evaluate_possibly_ill_typed_expression(state, shifted_body_id)
        }
        ExpressionId::Name(_) | ExpressionId::Call(_) | ExpressionId::Match(_) => {
            register_normalized_nonsubstituted_fun(
                state.registry,
                normalized_callee_id,
                &normalized_arg_ids,
            )
        }
        ExpressionId::Forall(_) => {
            let normalized_arg_list_id = state.registry.add_expression_list(
                normalized_arg_ids
                    .into_iter()
                    .map(NormalFormId::raw)
                    .collect(),
            );
            let best_attempt_id =
                ExpressionId::Call(state.registry.add_call_and_overwrite_its_id(Call {
                    id: dummy_id(),
                    callee_id: normalized_callee_id.raw(),
                    arg_list_id: normalized_arg_list_id,
                }));
            EvalResult {
                best_attempt_id,
                normal_form_id: Err(EvalError::BadCallee(normalized_callee_id)),
            }
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
    let param_list_id = state.registry.fun(fun_id).param_list_id;
    let decreasing_param_index = state
        .registry
        .param_list(param_list_id)
        .iter()
        .copied()
        .position(|param_id| {
            let param = state.registry.param(param_id);
            param.is_dashed
        });
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

/// If the provided expression is has a variant at
/// the top level,this returns IDs for the variant name
/// and the variant's argument list.
/// Otherwise, returns `None`.
fn is_variant_expression(state: &mut State, expression_id: NormalFormId) -> bool {
    try_as_variant_expression(state, expression_id.raw()).is_some()
}

fn evaluate_possibly_ill_typed_fun(state: &mut State, fun_id: NodeId<Fun>) -> EvalResult {
    let original_len = state.context.len();
    evaluate_possibly_ill_typed_fun_dirty(state, fun_id).untaint(state.context, original_len)
}

fn evaluate_possibly_ill_typed_fun_dirty(
    state: &mut State,
    fun_id: NodeId<Fun>,
) -> Tainted<EvalResult> {
    let fun = state.registry.fun(fun_id).clone();
    let normalized_param_list_id =
        match normalize_params_and_leave_params_in_context(state, fun.param_list_id) {
            Ok((warning, id)) => {
                warning.drop_since_its_inside_tainted_fn();
                id
            }
            Err(err) => {
                let param_list_best_attempt =
                    normalize_params_as_much_as_possible_without_adding_to_context(
                        state,
                        fun.param_list_id,
                    );
                let best_attempt_id =
                    ExpressionId::Fun(state.registry.add_fun_and_overwrite_its_id(Fun {
                        id: dummy_id(),
                        name_id: fun.name_id,
                        param_list_id: param_list_best_attempt,
                        return_type_id: fun.return_type_id,
                        body_id: fun.body_id,
                        skip_type_checking_body: fun.skip_type_checking_body,
                    }));
                return Tainted::new(EvalResult {
                    best_attempt_id,
                    normal_form_id: Err(EvalError::IllTypedParams(fun.param_list_id, err)),
                });
            }
        };
    let normalized_return_type_id =
        match evaluate_possibly_ill_typed_expression(state, fun.return_type_id) {
            EvalResult {
                best_attempt_id: _,
                normal_form_id: Ok(nfid),
            } => nfid,
            EvalResult {
                best_attempt_id: return_type_best_attempt_id,
                normal_form_id: Err(err),
            } => {
                let best_attempt_id =
                    ExpressionId::Fun(state.registry.add_fun_and_overwrite_its_id(Fun {
                        id: dummy_id(),
                        name_id: fun.name_id,
                        param_list_id: normalized_param_list_id,
                        return_type_id: return_type_best_attempt_id,
                        body_id: fun.body_id,
                        skip_type_checking_body: fun.skip_type_checking_body,
                    }));
                return Tainted::new(EvalResult {
                    best_attempt_id,
                    normal_form_id: Err(err),
                });
            }
        };
    state.context.pop_n(fun.param_list_id.len);

    Tainted::new(EvalResult::ok(NormalFormId::unchecked_new(
        ExpressionId::Fun(state.registry.add_fun_and_overwrite_its_id(Fun {
            id: dummy_id(),
            name_id: fun.name_id,
            param_list_id: normalized_param_list_id,
            return_type_id: normalized_return_type_id.raw(),
            body_id: fun.body_id,
            skip_type_checking_body: fun.skip_type_checking_body,
        })),
    )))
}

fn normalize_params_as_much_as_possible_without_adding_to_context(
    state: &mut State,
    param_list_id: ListId<NodeId<Param>>,
) -> ListId<NodeId<Param>> {
    let original_len = state.context.len();
    normalize_params_as_much_as_possible_without_adding_to_context_dirty(state, param_list_id)
        .untaint(state.context, original_len)
}

fn normalize_params_as_much_as_possible_without_adding_to_context_dirty(
    state: &mut State,
    param_list_id: ListId<NodeId<Param>>,
) -> Tainted<ListId<NodeId<Param>>> {
    let mut normalized_param_ids = Vec::with_capacity(param_list_id.len);
    let param_ids = state.registry.param_list(param_list_id).to_vec();
    for (index, param_id) in param_ids.iter().copied().enumerate() {
        let param = state.registry.param(param_id).clone();
        let param_type_eval_res = evaluate_possibly_ill_typed_expression(state, param.type_id);

        normalized_param_ids.push(state.registry.add_param_and_overwrite_its_id(Param {
            id: dummy_id(),
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: param_type_eval_res.best_attempt_id,
        }));

        match param_type_eval_res.normal_form_id {
            Ok(normalized_type_id) => {
                state
                    .context
                    .push(ContextEntry {
                        type_id: normalized_type_id,
                        definition: ContextEntryDefinition::Uninterpreted,
                    })
                    .drop_since_its_inside_tainted_fn();
            }
            Err(_) => {
                normalized_param_ids.extend(param_ids[index + 1..].iter().copied());
                return Tainted::new(state.registry.add_param_list(normalized_param_ids));
            }
        }
    }
    Tainted::new(state.registry.add_param_list(normalized_param_ids))
}

fn evaluate_possibly_ill_typed_match(state: &mut State, match_id: NodeId<Match>) -> EvalResult {
    let match_ = state.registry.match_(match_id).clone();
    let normalized_matchee_id =
        match evaluate_possibly_ill_typed_expression(state, match_.matchee_id) {
            EvalResult {
                best_attempt_id: _,
                normal_form_id: Ok(nfid),
            } => nfid,
            EvalResult {
                best_attempt_id: matchee_best_attempt_id,
                normal_form_id: Err(err),
            } => {
                let best_attempt_id =
                    ExpressionId::Match(state.registry.add_match_and_overwrite_its_id(Match {
                        id: dummy_id(),
                        matchee_id: matchee_best_attempt_id,
                        case_list_id: match_.case_list_id,
                    }));
                return EvalResult {
                    best_attempt_id,
                    normal_form_id: Err(err),
                };
            }
        };

    let (normalized_matchee_variant_name_id, normalized_matchee_arg_list_id) =
        if let Some((variant_name_id, arg_list_id)) =
            try_as_variant_expression(state, normalized_matchee_id.raw())
        {
            (variant_name_id, arg_list_id)
        } else {
            return EvalResult::ok(NormalFormId::unchecked_new(ExpressionId::Match(
                state.registry.add_match_and_overwrite_its_id(Match {
                    id: dummy_id(),
                    matchee_id: normalized_matchee_id.raw(),
                    case_list_id: match_.case_list_id,
                }),
            )));
        };

    let case_id = state
        .registry
        .match_case_list(match_.case_list_id)
        .iter()
        .find(|case_id| {
            let case = state.registry.match_case(**case_id);
            let case_variant_name: &IdentifierName =
                &state.registry.identifier(case.variant_name_id).name;
            let matchee_variant_name: &IdentifierName = &state
                .registry
                .identifier(normalized_matchee_variant_name_id)
                .name;
            case_variant_name == matchee_variant_name
        })
        .copied();
    let case_id = match case_id {
        Some(id) => id,
        None => {
            let best_attempt_id =
                ExpressionId::Match(state.registry.add_match_and_overwrite_its_id(Match {
                    id: dummy_id(),
                    matchee_id: normalized_matchee_id.raw(),
                    case_list_id: match_.case_list_id,
                }));
            return EvalResult {
                best_attempt_id,
                normal_form_id: Err(EvalError::NoMatchingCase(match_id)),
            };
        }
    };

    let case = state.registry.match_case(case_id).clone();

    match normalized_matchee_arg_list_id {
        PossibleArgListId::Nullary => evaluate_possibly_ill_typed_expression(state, case.output_id),
        PossibleArgListId::Some(normalized_matchee_arg_list_id) => {
            let case_param_ids = state.registry.identifier_list(case.param_list_id).to_vec();
            let case_arity = case_param_ids.len();
            let matchee_arg_ids = state
                .registry
                .expression_list(normalized_matchee_arg_list_id)
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
                            vec![param_id],
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
    let original_len = state.context.len();
    evaluate_possibly_ill_typed_forall_dirty(state, forall_id).untaint(state.context, original_len)
}

fn evaluate_possibly_ill_typed_forall_dirty(
    state: &mut State,
    forall_id: NodeId<Forall>,
) -> Tainted<EvalResult> {
    let forall = state.registry.forall(forall_id).clone();
    let normalized_param_list_id =

        // TODO Don't use `normalize_params_and_leave_params_in_context`
        match normalize_params_and_leave_params_in_context(state, forall.param_list_id) {
            Ok((warning, id)) => {
                warning.drop_since_its_inside_tainted_fn();
                id
            }
            Err(err) => {
                let param_list_best_attempt =
                    normalize_params_as_much_as_possible_without_adding_to_context(
                        state,
                        forall.param_list_id,
                    );
                let best_attempt_id =
                    ExpressionId::Forall(state.registry.add_forall_and_overwrite_its_id(Forall {
                        id: dummy_id(),
                        param_list_id: param_list_best_attempt,
                        output_id: forall.output_id,
                    }));
                return Tainted::new(EvalResult {
                    best_attempt_id,
                    normal_form_id: Err(EvalError::IllTypedParams(forall.param_list_id, err)),
                });
            }
        };
    let normalized_output_id = match evaluate_possibly_ill_typed_expression(state, forall.output_id)
    {
        EvalResult {
            best_attempt_id: _,
            normal_form_id: Ok(nfid),
        } => nfid,
        EvalResult {
            best_attempt_id: output_best_attempt_id,
            normal_form_id: Err(err),
        } => {
            let best_attempt_id =
                ExpressionId::Forall(state.registry.add_forall_and_overwrite_its_id(Forall {
                    id: dummy_id(),
                    param_list_id: normalized_param_list_id,
                    output_id: output_best_attempt_id,
                }));
            return Tainted::new(EvalResult {
                best_attempt_id: best_attempt_id,
                normal_form_id: Err(err),
            });
        }
    };
    state.context.pop_n(forall.param_list_id.len);

    Tainted::new(EvalResult::ok(NormalFormId::unchecked_new(
        ExpressionId::Forall(state.registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: normalized_output_id.raw(),
        })),
    )))
}

fn evaluate_possibly_ill_typed_check(state: &mut State, check_id: NodeId<Check>) -> EvalResult {
    let check = state.registry.check(check_id);
    evaluate_possibly_ill_typed_expression(state, check.output_id)
}

fn eval_all(
    state: &mut State,
    ids: &[ExpressionId],
) -> Result<Vec<NormalFormId>, (Vec<ExpressionId>, EvalError)> {
    let mut nfids = Vec::new();
    for (index, id) in ids.iter().copied().enumerate() {
        let eval_res = evaluate_possibly_ill_typed_expression(state, id);
        let nfid = match eval_res.normal_form_id {
            Ok(nfid) => nfid,
            Err(err) => {
                let best_attempt: Vec<ExpressionId> = nfids
                    .into_iter()
                    .map(NormalFormId::raw)
                    .chain(std::iter::once(eval_res.best_attempt_id))
                    .chain(ids[index + 1..].iter().copied())
                    .collect();
                return Err((best_attempt, err));
            }
        };
        nfids.push(nfid);
    }
    Ok(nfids)
}

trait Untaint {
    type Output;
    fn untaint(self, context: &mut Context, original_len: usize) -> Self::Output;
}

impl<T> Untaint for Tainted<T> {
    type Output = T;
    fn untaint(self, context: &mut Context, original_len: usize) -> Self::Output {
        context.truncate(original_len);
        self.unchecked_raw()
    }
}
