use super::*;

#[derive(Debug)]
struct EvalState<'a, 'b> {
    raw: &'a mut ContextlessState<'b>,
    context: OffsetContext<'b>,
}

#[derive(Clone, Debug)]
struct OffsetContext<'a> {
    raw: &'a Context,
    extra_entries_in_context: usize,
}

impl OffsetContext<'_> {
    fn push_uninterpreted(&mut self) {
        self.extra_entries_in_context += 1;
    }

    fn pop_n(&mut self, n: usize) {
        self.extra_entries_in_context -= n;
    }

    fn get_definition(
        &self,
        db_index: DbIndex,
        registry: &mut NodeRegistry,
    ) -> ContextEntryDefinition {
        let adjusted_db_index = db_index
            .0
            .checked_sub(self.extra_entries_in_context)
            .map(DbIndex);
        if let Some(adjusted_db_index) = adjusted_db_index {
            self.raw
                .get_definition(adjusted_db_index, registry)
                .upshift(self.extra_entries_in_context, registry)
        } else {
            ContextEntryDefinition::Uninterpreted
        }
    }
}

pub(super) fn evaluate_well_typed_expression(state: &mut State, id: ExpressionId) -> NormalFormId {
    let (context, mut contextless) = state.detach_context();
    let mut state = EvalState {
        raw: &mut contextless,
        context: OffsetContext {
            raw: context,
            extra_entries_in_context: 0,
        },
    };
    evaluate_expression(&mut state, id)
}

// As a convention, since every function in this module
// only deals with well-typed expressions,
// we'll omit the `well_typed` prefix
// from private function names,
// and leave it as implicit.

fn evaluate_expression(state: &mut EvalState, id: ExpressionId) -> NormalFormId {
    match id {
        ExpressionId::Name(name_id) => evaluate_name_expression(state, name_id),
        ExpressionId::Todo(todo_id) => evaluate_todo_expression(state, todo_id),
        ExpressionId::Call(call_id) => evaluate_call(state, call_id),
        ExpressionId::Fun(fun_id) => evaluate_fun(state, fun_id),
        ExpressionId::Match(match_id) => evaluate_match(state, match_id),
        ExpressionId::Forall(forall_id) => evaluate_forall(state, forall_id),
        ExpressionId::Check(check_id) => evaluate_check(state, check_id),
    }
}

fn evaluate_name_expression(
    state: &mut EvalState,
    name_id: NodeId<NameExpression>,
) -> NormalFormId {
    let name = state.raw.registry.get(name_id);
    let definition = state
        .context
        .get_definition(name.db_index, state.raw.registry);
    match definition {
        ContextEntryDefinition::Alias {
            value_id: alias_value_id,
            transparency,
            ..
        } => {
            let can_substitute = if let Some(required_transparency) =
                state.raw.required_transparency_for_substitution
            {
                is_left_at_least_as_permissive_as_right(
                    state.raw.file_tree,
                    transparency.0,
                    required_transparency.0,
                )
            } else {
                true
            };
            if can_substitute {
                alias_value_id
            } else {
                NormalFormId::unchecked_new(ExpressionId::Name(name_id))
            }
        }

        ContextEntryDefinition::Adt {
            variant_name_list_id: _,
            visibility: _,
        }
        | ContextEntryDefinition::Variant {
            name_id: _,
            visibility: _,
        }
        | ContextEntryDefinition::Uninterpreted => {
            NormalFormId::unchecked_new(ExpressionId::Name(name_id))
        }
    }
}

fn evaluate_todo_expression(_: &mut EvalState, todo_id: NodeId<TodoExpression>) -> NormalFormId {
    // `todo` expressions are, by definition, normal forms.
    NormalFormId::unchecked_new(ExpressionId::Todo(todo_id))
}

fn evaluate_call(state: &mut EvalState, call_id: NodeId<Call>) -> NormalFormId {
    fn register_normalized_nonsubstituted_call(
        registry: &mut NodeRegistry,
        normalized_callee_id: NormalFormId,
        normalized_arg_list_id: NonEmptyCallArgListId,
    ) -> NormalFormId {
        let normalized_call_id = registry
            .add_and_overwrite_id(Call {
                id: dummy_id(),
                span: None,
                callee_id: normalized_callee_id.raw(),
                arg_list_id: normalized_arg_list_id,
            })
            .without_spans(registry);
        NormalFormId::unchecked_new(ExpressionId::Call(normalized_call_id))
    }

    let call = state.raw.registry.get(call_id).clone();

    let normalized_callee_id = evaluate_expression(state, call.callee_id);

    let normalized_arg_list_id = evaluate_call_arg_list(state, call.arg_list_id);

    match normalized_callee_id.raw() {
        ExpressionId::Fun(fun_id) => {
            if !can_fun_be_applied(state, fun_id, normalized_arg_list_id) {
                return register_normalized_nonsubstituted_call(
                    state.raw.registry,
                    normalized_callee_id,
                    normalized_arg_list_id,
                );
            }

            let fun = state.raw.registry.get(fun_id).clone();
            let param_arity = fun.param_list_id.len();
            let shifted_normalized_arg_list_id =
                normalized_arg_list_id.upshift(param_arity + 1, state.raw.registry);
            let substitutions: Vec<Substitution> = match shifted_normalized_arg_list_id {
                NonEmptyCallArgListId::Unlabeled(shifted_normalized_arg_list_id) => {
                    let param_name_ids = get_param_name_ids(state, fun.param_list_id);
                    let shifted_normalized_arg_ids = state
                        .raw
                        .registry
                        .get_list(shifted_normalized_arg_list_id)
                        .to_non_empty_vec();
                    {
                        let shifted_fun_id = NormalFormId::unchecked_new(ExpressionId::Fun(
                            fun_id.upshift(param_arity + 1, state.raw.registry),
                        ));
                        const FUN_DB_INDEX: DbIndex = DbIndex(0);
                        vec![Substitution {
                            from: ExpressionId::Name(add_name_expression(
                                state.raw.registry,
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
                                let name = NormalFormId::unchecked_new(ExpressionId::Name(
                                    add_name_expression(
                                        state.raw.registry,
                                        NonEmptyVec::singleton(param_name_id),
                                        db_index,
                                    ),
                                ));
                                Substitution {
                                    from: name.raw(),
                                    to: arg_id,
                                }
                            }),
                    )
                    .collect::<Vec<_>>()
                }

                NonEmptyCallArgListId::UniquelyLabeled(shifted_normalized_arg_list_id) => {
                    let recursive_fun_sub = {
                        let shifted_fun_id = NormalFormId::unchecked_new(ExpressionId::Fun(
                            fun_id.upshift(param_arity + 1, state.raw.registry),
                        ));
                        const FUN_DB_INDEX: DbIndex = DbIndex(0);
                        Substitution {
                            from: ExpressionId::Name(add_name_expression(
                                state.raw.registry,
                                NonEmptyVec::singleton(fun.name_id),
                                FUN_DB_INDEX,
                            )),
                            to: shifted_fun_id.raw(),
                        }
                    };

                    let shifted_normalized_arg_ids = state
                        .raw
                        .registry
                        .get_list(shifted_normalized_arg_list_id)
                        .to_non_empty_vec();

                    let param_ids = match fun.param_list_id {
                            NonEmptyParamListId::Unlabeled(_) => panic!("A well-typed Call with labeled arguments should have a callee with labeled params."),
                            NonEmptyParamListId::UniquelyLabeled(param_list_id) => {
                                state.raw.registry.get_list(param_list_id).to_non_empty_vec()
                            }
                        };

                    let mut subs = vec![recursive_fun_sub];
                    for &arg_id in &shifted_normalized_arg_ids {
                        let arg_label_name = &state.raw.registry.get(arg_id.label_id()).name;
                        let (param_index, param_name_id) = param_ids.iter().copied().enumerate().find_map(|(param_index, param_id)| {
                            let param = state.raw.registry.get(param_id);
                            let param_label_name = &state.raw.registry.get(param.label_identifier_id()).name;
                            if param_label_name == arg_label_name {
                                Some((param_index, param.name_id))
                            } else {
                                None
                            }

                        }).expect("A well-typed Call's callee should have a param for everyone one of the Call's args.");
                        let db_index = DbIndex(param_arity - param_index);
                        let name =
                            NormalFormId::unchecked_new(ExpressionId::Name(add_name_expression(
                                state.raw.registry,
                                NonEmptyVec::singleton(param_name_id),
                                db_index,
                            )));
                        subs.push(Substitution {
                            from: name.raw(),
                            to: arg_id.value_id(),
                        });
                    }
                    subs
                }
            };

            let body_id = fun.body_id.subst_all(&substitutions, state.raw);
            let shifted_body_id = body_id.downshift(param_arity + 1, state.raw.registry);
            evaluate_expression(state, shifted_body_id)
        }
        ExpressionId::Name(_)
        | ExpressionId::Call(_)
        | ExpressionId::Match(_)
        | ExpressionId::Todo(_) => register_normalized_nonsubstituted_call(
            state.raw.registry,
            normalized_callee_id,
            normalized_arg_list_id,
        ),
        ExpressionId::Forall(_) => {
            panic!("A well-typed Call cannot have a Forall as its callee.")
        }
        ExpressionId::Check(_) => {
            panic!("By definition, a check expression can never be a normal form.")
        }
    }
}

fn can_fun_be_applied(
    state: &mut EvalState,
    fun_id: NodeId<Fun>,
    normalized_arg_ids: NonEmptyCallArgListId,
) -> bool {
    let param_list_id = state.raw.registry.get(fun_id).param_list_id;
    match (param_list_id, normalized_arg_ids) {
        (NonEmptyParamListId::Unlabeled(param_list_id), NonEmptyCallArgListId::Unlabeled(normalized_arg_ids)) => can_unlabeled_fun_be_applied(state, param_list_id, normalized_arg_ids),
        (NonEmptyParamListId::UniquelyLabeled(param_list_id), NonEmptyCallArgListId::UniquelyLabeled(normalized_arg_ids)) => can_labeled_fun_be_applied(state, param_list_id, normalized_arg_ids),
        _ => panic!("A well-typed Call should have labeled args if and only if its callee has labeled params."),
    }
}

fn can_unlabeled_fun_be_applied(
    state: &mut EvalState,
    param_list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
    normalized_arg_list_id: NonEmptyListId<ExpressionId>,
) -> bool {
    let Some(decreasing_param_index) = get_decreasing_param_index(state, param_list_id) else {
        // If there is no decreasing parameter, the function is non-recursive,
        // so it can be safely applied without causing infinite expansion.
        return true;
    };

    let normalized_arg_ids = state.raw.registry.get_list(normalized_arg_list_id);
    let decreasing_arg_id = NormalFormId::unchecked_new(normalized_arg_ids[decreasing_param_index]);
    determine_whether_expression_is_variant_using_node_registry_and_definition_getter(
        state.raw.registry,
        |db_index, registry| state.context.get_definition(db_index, registry),
        decreasing_arg_id,
    )
}

fn get_decreasing_param_index(
    state: &EvalState,
    param_list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> Option<usize> {
    state
        .raw
        .registry
        .get_list(param_list_id)
        .iter()
        .copied()
        .position(|param_id| {
            let param = state.raw.registry.get(param_id);
            param.is_dashed
        })
}

fn can_labeled_fun_be_applied(
    state: &mut EvalState,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
    normalized_arg_list_id: NonEmptyListId<LabeledCallArgId>,
) -> bool {
    let Some(decreasing_param_label_id) = get_decreasing_param_label_id(state, param_list_id) else {
        // If there is no decreasing parameter, the function is non-recursive,
        // so it can be safely applied without causing infinite expansion.
        return true;
    };
    let decreasing_param_label_name = state
        .raw
        .registry
        .get(decreasing_param_label_id)
        .name
        .clone();

    let normalized_arg_ids = state
        .raw
        .registry
        .get_list(normalized_arg_list_id)
        .to_non_empty_vec();
    let decreasing_arg_id = normalized_arg_ids.iter().copied().find_map(|normalized_arg_id| {
        let arg_label_id = normalized_arg_id.label_id();
        let arg_label_name = &state.raw.registry.get(arg_label_id).name;
        if decreasing_param_label_name == *arg_label_name {
            let value_id = NormalFormId::unchecked_new(normalized_arg_id.value_id());
            Some(value_id)
        } else {
            None
        }
    }).expect(
        "A well-typed labeled Call should have a labeled arg corresponding to each param label.",
    );
    determine_whether_expression_is_variant_using_node_registry_and_definition_getter(
        state.raw.registry,
        |db_index, registry| state.context.get_definition(db_index, registry),
        decreasing_arg_id,
    )
}

fn get_decreasing_param_label_id(
    state: &EvalState,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
) -> Option<NodeId<Identifier>> {
    state
        .raw
        .registry
        .get_list(param_list_id)
        .iter()
        .copied()
        .find_map(|param_id| {
            let param = state.raw.registry.get(param_id);
            if param.is_dashed {
                Some(param.label_identifier_id())
            } else {
                None
            }
        })
}

fn evaluate_call_arg_list(
    state: &mut EvalState,
    arg_list_id: NonEmptyCallArgListId,
) -> NonEmptyCallArgListId {
    match arg_list_id {
        NonEmptyCallArgListId::Unlabeled(arg_list_id) => {
            let arg_ids = state
                .raw
                .registry
                .get_list(arg_list_id)
                .to_non_empty_vec()
                .into_mapped(|arg_id| evaluate_expression(state, arg_id).raw());
            NonEmptyCallArgListId::Unlabeled(state.raw.registry.add_list(arg_ids))
        }
        NonEmptyCallArgListId::UniquelyLabeled(arg_list_id) => {
            let arg_ids = state
                .raw
                .registry
                .get_list(arg_list_id)
                .to_non_empty_vec()
                .into_mapped(|arg_id| evaluate_labeled_call_arg(state, arg_id));
            NonEmptyCallArgListId::UniquelyLabeled(state.raw.registry.add_list(arg_ids))
        }
    }
}

fn evaluate_labeled_call_arg(state: &mut EvalState, arg_id: LabeledCallArgId) -> LabeledCallArgId {
    match arg_id {
        LabeledCallArgId::Implicit {
            label_id,
            db_index,
            value_id,
        } => {
            let definition = state.context.get_definition(db_index, state.raw.registry);
            if let ContextEntryDefinition::Alias {
                value_id: alias_value_id,
                transparency,
                ..
            } = definition
            {
                let can_substitute = if let Some(required_transparency) =
                    state.raw.required_transparency_for_substitution
                {
                    is_left_at_least_as_permissive_as_right(
                        state.raw.file_tree,
                        transparency.0,
                        required_transparency.0,
                    )
                } else {
                    true
                };
                if can_substitute {
                    return LabeledCallArgId::Explicit {
                        label_id,
                        value_id: alias_value_id.raw(),
                    };
                }
            }
            LabeledCallArgId::Implicit {
                label_id,
                db_index,
                value_id,
            }
        }
        LabeledCallArgId::Explicit { label_id, value_id } => LabeledCallArgId::Explicit {
            label_id,
            value_id: evaluate_expression(state, value_id).raw(),
        },
    }
}

fn get_param_name_ids(
    state: &EvalState,
    param_list_id: NonEmptyParamListId,
) -> NonEmptyVec<NodeId<Identifier>> {
    match param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => state
            .raw
            .registry
            .get_list(param_list_id)
            .to_mapped(|&param_id| state.raw.registry.get(param_id).name_id),
        NonEmptyParamListId::UniquelyLabeled(param_list_id) => state
            .raw
            .registry
            .get_list(param_list_id)
            .to_mapped(|&param_id| state.raw.registry.get(param_id).name_id),
    }
}

fn evaluate_fun(state: &mut EvalState, fun_id: NodeId<Fun>) -> NormalFormId {
    let fun = state.raw.registry.get(fun_id).clone();
    let normalized_param_list_id =
        normalize_params_as_much_as_possible_and_leave_in_context(state, fun.param_list_id);

    let normalized_return_type_id = evaluate_expression(state, fun.return_type_id);

    state.context.push_uninterpreted();
    let normalized_body_id = evaluate_expression(state, fun.body_id);

    state.context.pop_n(fun.param_list_id.len() + 1);

    NormalFormId::unchecked_new(ExpressionId::Fun(
        state
            .raw
            .registry
            .add_and_overwrite_id(Fun {
                id: dummy_id(),
                span: None,
                name_id: fun.name_id,
                param_list_id: normalized_param_list_id,
                return_type_id: normalized_return_type_id.raw(),
                body_id: normalized_body_id.raw(),
            })
            .without_spans(state.raw.registry),
    ))
}

fn normalize_params_as_much_as_possible_and_leave_in_context(
    state: &mut EvalState,
    param_list_id: NonEmptyParamListId,
) -> NonEmptyParamListId {
    match param_list_id {
        NonEmptyParamListId::Unlabeled(id) => {
            normalize_unlabeled_params_as_much_as_possible_and_leave_in_context(state, id)
        }
        NonEmptyParamListId::UniquelyLabeled(id) => {
            normalize_labeled_params_as_much_as_possible_and_leave_in_context(state, id)
        }
    }
}

fn normalize_unlabeled_params_as_much_as_possible_and_leave_in_context(
    state: &mut EvalState,
    param_list_id: NonEmptyListId<NodeId<UnlabeledParam>>,
) -> NonEmptyParamListId {
    let param_ids = state.raw.registry.get_list(param_list_id);
    let (&first_param_id, remaining_param_ids) = param_ids.to_cons();
    let remaining_param_ids = remaining_param_ids.to_vec();
    let normalized_first_param_id = {
        let first_param = state.raw.registry.get(first_param_id).clone();
        let normalized_param_type_id = evaluate_expression(state, first_param.type_id);
        state.context.push_uninterpreted();
        state.raw.registry.add_and_overwrite_id(UnlabeledParam {
            id: dummy_id(),
            span: None,
            is_dashed: first_param.is_dashed,
            name_id: first_param.name_id,
            type_id: normalized_param_type_id.raw(),
        })
    };
    let mut normalized_param_ids = NonEmptyVec::singleton(normalized_first_param_id);
    let remaining_param_ids = remaining_param_ids.to_vec();
    for param_id in remaining_param_ids.iter().copied() {
        let param = state.raw.registry.get(param_id).clone();
        let normalized_param_type_id = evaluate_expression(state, param.type_id);
        normalized_param_ids.push(state.raw.registry.add_and_overwrite_id(UnlabeledParam {
            id: dummy_id(),
            span: None,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: normalized_param_type_id.raw(),
        }));
        state.context.push_uninterpreted();
    }
    NonEmptyParamListId::Unlabeled(state.raw.registry.add_list(normalized_param_ids))
}

fn normalize_labeled_params_as_much_as_possible_and_leave_in_context(
    state: &mut EvalState,
    param_list_id: NonEmptyListId<NodeId<LabeledParam>>,
) -> NonEmptyParamListId {
    let param_ids = state.raw.registry.get_list(param_list_id);
    let (&first_param_id, remaining_param_ids) = param_ids.to_cons();
    let remaining_param_ids = remaining_param_ids.to_vec();
    let normalized_first_param_id = {
        let first_param = state.raw.registry.get(first_param_id).clone();
        let normalized_param_type_id = evaluate_expression(state, first_param.type_id);
        state.context.push_uninterpreted();
        state.raw.registry.add_and_overwrite_id(LabeledParam {
            id: dummy_id(),
            span: None,
            label_id: first_param.label_id,
            is_dashed: first_param.is_dashed,
            name_id: first_param.name_id,
            type_id: normalized_param_type_id.raw(),
        })
    };
    let mut normalized_param_ids = NonEmptyVec::singleton(normalized_first_param_id);
    let remaining_param_ids = remaining_param_ids.to_vec();
    for param_id in remaining_param_ids.iter().copied() {
        let param = state.raw.registry.get(param_id).clone();
        let normalized_param_type_id = evaluate_expression(state, param.type_id);
        normalized_param_ids.push(state.raw.registry.add_and_overwrite_id(LabeledParam {
            id: dummy_id(),
            span: None,
            label_id: param.label_id,
            is_dashed: param.is_dashed,
            name_id: param.name_id,
            type_id: normalized_param_type_id.raw(),
        }));
        state.context.push_uninterpreted();
    }
    NonEmptyParamListId::UniquelyLabeled(state.raw.registry.add_list(normalized_param_ids))
}

fn evaluate_match(state: &mut EvalState, match_id: NodeId<Match>) -> NormalFormId {
    let match_ = state.raw.registry.get(match_id).clone();
    let normalized_matchee_id = evaluate_expression(state, match_.matchee_id);

    let (normalized_matchee_variant_name_id, normalized_matchee_arg_list_id) =
        if let Some((variant_name_id, arg_list_id)) =
            try_as_variant_expression_with_node_registry_and_definition_getter(
                state.raw.registry,
                |db_index, registry| state.context.get_definition(db_index, registry),
                normalized_matchee_id.raw(),
            )
        {
            (variant_name_id, arg_list_id)
        } else {
            let normalized_case_list_id =
                normalize_possibly_empty_match_case_list(state, match_.case_list_id);
            return NormalFormId::unchecked_new(ExpressionId::Match(
                state
                    .raw
                    .registry
                    .add_and_overwrite_id(Match {
                        id: dummy_id(),
                        span: None,
                        matchee_id: normalized_matchee_id.raw(),
                        case_list_id: normalized_case_list_id,
                    })
                    .without_spans(state.raw.registry),
            ));
        };

    let case_id = state
        .raw
        .registry
        .get_possibly_empty_list(match_.case_list_id)
        .iter()
        .find(|case_id| {
            let case = state.raw.registry.get(**case_id);
            let case_variant_name: &IdentifierName =
                &state.raw.registry.get(case.variant_name_id).name;
            let matchee_variant_name: &IdentifierName = &state
                .raw
                .registry
                .get(normalized_matchee_variant_name_id)
                .name;
            case_variant_name == matchee_variant_name
        })
        .copied();
    let case_id = match case_id {
        Some(id) => id,
        None => panic!("Impossible: Cannot find matching MatchCase in well-typed Match expression"),
    };

    let case = state.raw.registry.get(case_id).clone();

    let substitutions: Vec<Substitution> = match (
        normalized_matchee_arg_list_id,
        case.param_list_id,
    ) {
        (None, None) => vec![],
        (
            Some(NonEmptyCallArgListId::Unlabeled(normalized_matchee_arg_list_id)),
            Some(NonEmptyMatchCaseParamListId::Unlabeled(case_param_list_id)),
        ) => {
            let case_param_ids = state.raw.registry.get_list(case_param_list_id).to_vec();
            let case_arity = case_param_ids.len();
            let matchee_arg_ids: Vec<_> = state
                .raw
                .registry
                .get_list(normalized_matchee_arg_list_id)
                .to_vec();

            case_param_ids
                .iter()
                .copied()
                .zip(matchee_arg_ids.iter().copied())
                .enumerate()
                .map(|(param_index, (param_id, arg_id))| {
                    let db_index = DbIndex(case_arity - param_index - 1);
                    // We can safely call `unchecked_new` here because we know that each
                    // arg to a normal form Call is also a normal form.
                    let shifted_arg_id =
                        NormalFormId::unchecked_new(arg_id).upshift(case_arity, state.raw.registry);
                    Substitution {
                        from: ExpressionId::Name(add_name_expression(
                            state.raw.registry,
                            NonEmptyVec::singleton(param_id),
                            db_index,
                        )),
                        to: shifted_arg_id.raw(),
                    }
                })
                .collect()
        }
        (
            Some(NonEmptyCallArgListId::UniquelyLabeled(normalized_matchee_arg_list_id)),
            Some(NonEmptyMatchCaseParamListId::UniquelyLabeled {
                param_list_id: explicit_param_list_id,
                triple_dot: _,
            }),
        ) => {
            let explicit_param_ids = state
                .raw
                .registry
                .get_possibly_empty_list(explicit_param_list_id)
                .to_vec();
            let explicit_arity = explicit_param_ids.len();
            let matchee_arg_ids = state
                .raw
                .registry
                .get_list(normalized_matchee_arg_list_id)
                .to_non_empty_vec();

            explicit_param_ids
                .iter()
                .copied().enumerate()
                .map(|(explicit_param_index, explicit_param_id)| {
                    let explicit_param_label_name_id =
                        state.raw.registry.get(explicit_param_id).label_identifier_id();
                    let explicit_param_label_name =
                        &state.raw.registry.get(explicit_param_label_name_id).name;
                        let corresponding_arg_id = matchee_arg_ids
                            .iter().copied()
                            .find(|&arg_id| {
                                let arg_label_name_id = arg_id.label_id();
                                let arg_label_name = &state.raw.registry.get(arg_label_name_id).name;
                                IdentifierName::eq(arg_label_name, explicit_param_label_name)
                            }).expect("Impossible: well-typed Match expression has a case param with no corresponding matchee arg.");

                    let explicit_param_name_id =
                        state.raw.registry.get(explicit_param_id).name_id;
                    let param_value_id = ExpressionId::Name(add_name_expression(
                        state.raw.registry,
                        NonEmptyVec::singleton(explicit_param_name_id),
                        DbIndex(explicit_arity - explicit_param_index - 1),
                    ));
                    let arg_value_id = corresponding_arg_id
                        .value_id()
                        .upshift(explicit_arity, state.raw.registry);
                    Substitution { from: param_value_id, to: arg_value_id }
                })
                .collect()
        }
        other => panic!(
            "Impossible: a well-typed Match expression has a labeledness mismatch. {:?}",
            other
        ),
    };

    let explicit_arity = case
        .param_list_id
        .map(|param_list_id| param_list_id.explicit_len())
        .unwrap_or(0);

    let substituted_body = case
        .output_id
        .subst_all(&substitutions, state.raw)
        .downshift(explicit_arity, state.raw.registry);
    let substituted_body = match substituted_body {
        MatchCaseOutputId::Some(id) => id,
        MatchCaseOutputId::ImpossibilityClaim(kw_span) => {
            panic!("Impossible: A well-typed Match expression ended up evaluating to one of the cases that was supposedly impossible. `impossible` keyword span: {:?}", kw_span)
        }
    };

    evaluate_expression(state, substituted_body)
}

fn normalize_possibly_empty_match_case_list(
    state: &mut EvalState,
    list_id: Option<NonEmptyListId<NodeId<MatchCase>>>,
) -> Option<NonEmptyListId<NodeId<MatchCase>>> {
    list_id.map(|list_id| normalize_match_case_list(state, list_id))
}

fn normalize_match_case_list(
    state: &mut EvalState,
    list_id: NonEmptyListId<NodeId<MatchCase>>,
) -> NonEmptyListId<NodeId<MatchCase>> {
    let ids = state.raw.registry.get_list(list_id).to_non_empty_vec();
    let normalized_ids = ids.into_mapped(|id| normalize_match_case(state, id));
    state.raw.registry.add_list(normalized_ids)
}

fn normalize_match_case(
    state: &mut EvalState,
    match_case_list_id: NodeId<MatchCase>,
) -> NodeId<MatchCase> {
    let case = state.raw.registry.get(match_case_list_id).clone();

    let explicit_arity = case
        .param_list_id
        .map(|param_list_id| param_list_id.explicit_len())
        .unwrap_or(0);
    for _ in 0..explicit_arity {
        state.context.push_uninterpreted();
    }
    let normalized_output_id = match case.output_id {
        MatchCaseOutputId::Some(case_output_id) => {
            MatchCaseOutputId::Some(evaluate_expression(state, case_output_id).raw())
        }
        MatchCaseOutputId::ImpossibilityClaim(kw_span) => {
            MatchCaseOutputId::ImpossibilityClaim(kw_span)
        }
    };
    state.context.pop_n(explicit_arity);

    state.raw.registry.add_and_overwrite_id(MatchCase {
        id: dummy_id(),
        span: None,
        variant_name_id: case.variant_name_id,
        param_list_id: case.param_list_id,
        output_id: normalized_output_id,
    })
}

fn evaluate_forall(state: &mut EvalState, forall_id: NodeId<Forall>) -> NormalFormId {
    let forall = state.raw.registry.get(forall_id).clone();
    let normalized_param_list_id =
        normalize_params_as_much_as_possible_and_leave_in_context(state, forall.param_list_id);
    let normalized_output_id = evaluate_expression(state, forall.output_id);
    state.context.pop_n(forall.param_list_id.len());

    NormalFormId::unchecked_new(ExpressionId::Forall(
        state
            .raw
            .registry
            .add_and_overwrite_id(Forall {
                id: dummy_id(),
                span: None,
                param_list_id: normalized_param_list_id,
                output_id: normalized_output_id.raw(),
            })
            .without_spans(state.raw.registry),
    ))
}

fn evaluate_check(state: &mut EvalState, check_id: NodeId<Check>) -> NormalFormId {
    let check = state.raw.registry.get(check_id);
    evaluate_expression(state, check.output_id)
}
