use super::*;

pub fn evaluate_well_typed_expression(
    state: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    expression: ExpressionId,
) -> Result<NormalFormNodeId, TypeError> {
    let mut current = expression;
    loop {
        let step_result = perform_eval_step_on_well_typed_expression(
            state,
            symbol_db,
            sih_cache,
            fv_cache,
            type0_identifier_id,
            current,
        )?;
        match step_result {
            EvalStepResult::Stepped(new_current) => current = new_current,
            EvalStepResult::CouldNotStepBecauseNormalForm(nfid) => break Ok(nfid),
        }
    }
}

#[derive(Clone, Debug)]
pub enum EvalStepResult {
    Stepped(ExpressionId),
    CouldNotStepBecauseNormalForm(NormalFormNodeId),
}

fn perform_eval_step_on_well_typed_expression(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    sih_cache: &mut NodeStructuralIdentityHashCache,
    fv_cache: &mut NodeFreeVariableCache,
    type0_identifier_id: NormalFormNodeId,
    expression_id: ExpressionId,
) -> Result<EvalStepResult, TypeError> {
    fn perform_eval_step_on_identifier_or_dot_based_on_symbol(
        registry: &mut NodeRegistry,
        symbol_db: &mut SymbolDatabase,
        symbol: Symbol,
        original_expression_id: ExpressionId,
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

    match expression_id {
        ExpressionId::Identifier(identifier_id) => {
            let symbol = symbol_db.identifier_symbols.get(identifier_id);
            Ok(perform_eval_step_on_identifier_or_dot_based_on_symbol(
                registry,
                symbol_db,
                symbol,
                expression_id,
            ))
        }
        ExpressionId::Dot(dot_id) => {
            let dot = registry.dot(dot_id);
            let symbol = symbol_db.identifier_symbols.get(dot.right_id);
            Ok(perform_eval_step_on_identifier_or_dot_based_on_symbol(
                registry,
                symbol_db,
                symbol,
                expression_id,
            ))
        }
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id);
            let callee_id = call.callee_id;
            let arg_list_id = call.arg_list_id;
            let callee_step_result = perform_eval_step_on_well_typed_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                callee_id,
            )?;
            if let EvalStepResult::Stepped(stepped_callee_id) = callee_step_result {
                let stepped_call_id = registry.add_call_and_overwrite_its_id(Call {
                    id: dummy_id(),
                    callee_id: stepped_callee_id,
                    arg_list_id,
                });
                let wrapped_stepped_id = ExpressionId::Call(stepped_call_id);
                return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
            }

            let arg_ids = registry.expression_list(arg_list_id).to_vec();
            let mut arg_nfids = Vec::with_capacity(arg_ids.len());
            for (arg_index, arg_id) in arg_ids.iter().copied().enumerate() {
                let arg_step_result = perform_eval_step_on_well_typed_expression(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    arg_id,
                )?;
                match arg_step_result {
                    EvalStepResult::Stepped(stepped_arg_id) => {
                        let mut stepped_arg_ids = Vec::with_capacity(arg_ids.len());
                        stepped_arg_ids.extend(arg_ids[..arg_index].iter().copied());
                        stepped_arg_ids.push(stepped_arg_id);
                        stepped_arg_ids.extend(arg_ids[arg_index + 1..].iter().copied());
                        let stepped_arg_list_id = registry.add_expression_list(stepped_arg_ids);
                        let stepped_call_id = registry.add_call_and_overwrite_its_id(Call {
                            id: dummy_id(),
                            callee_id,
                            arg_list_id: stepped_arg_list_id,
                        });
                        let wrapped_stepped_id = ExpressionId::Call(stepped_call_id);
                        return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
                    }
                    EvalStepResult::CouldNotStepBecauseNormalForm(arg_nfid) => {
                        arg_nfids.push(arg_nfid);
                    }
                }
            }

            match callee_id {
                ExpressionId::Identifier(callee_identifier_id) => {
                    let callee_symbol = symbol_db.identifier_symbols.get(callee_identifier_id);
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
                    let application_result = apply_substitutions(registry, symbol_db, sih_cache, fv_cache, type0_identifier_id, callee_body_id, substitutions);
                    Ok(EvalStepResult::Stepped(application_result))
                }
                other_normal_form_callee => panic!("A normal form callee in a well-typed Call expression should be an identifier, but was `{:?}`.", other_normal_form_callee),
            }
        }
        ExpressionId::Fun(fun_id) => {
            let fun = registry.fun(fun_id);
            let name_id = fun.name_id;
            let wrapped_name_id = ExpressionId::Identifier(name_id);
            Ok(EvalStepResult::Stepped(wrapped_name_id))
        }
        ExpressionId::Match(match_id) => {
            let match_ = registry.match_(match_id);
            let matchee_id = match_.matchee_id;
            let case_list_id = match_.case_list_id;

            let matchee_step_result = perform_eval_step_on_well_typed_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                matchee_id,
            )?;
            let matchee_nfid = match matchee_step_result {
                EvalStepResult::Stepped(stepped_matchee_id) => {
                    let stepped_match_id = registry.add_match_and_overwrite_its_id(Match {
                        id: dummy_id(),
                        matchee_id: stepped_matchee_id,
                        case_list_id: case_list_id,
                    });
                    let wrapped_stepped_id = ExpressionId::Match(stepped_match_id);
                    return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
                }
                EvalStepResult::CouldNotStepBecauseNormalForm(matchee_nfid) => matchee_nfid,
            };

            let matchee_callee_symbol_and_source =
                as_variant_call(registry, symbol_db, matchee_nfid);
            match matchee_callee_symbol_and_source {
                None => Ok(EvalStepResult::CouldNotStepBecauseNormalForm(
                    NormalFormNodeId(expression_id),
                )),
                Some((called_variant_id, matchee_arg_list_id)) => {
                    let case_id = get_match_case_id_corresponding_to_variant(
                        registry,
                        called_variant_id,
                        match_id,
                    ).expect("A well-typed match expression should have a case corresponding to the variant of its matchee.");
                    let case = registry.match_case(case_id);
                    let case_output_id = case.output_id;
                    let substitutions: Vec<Substitution> = {
                        let case_param_list_id = case.param_list_id;
                        let case_param_ids = registry.identifier_list(case_param_list_id).to_vec();
                        let matchee_arg_ids =
                            registry.expression_list(matchee_arg_list_id).to_vec();
                        case_param_ids
                            .iter()
                            .copied()
                            .zip(matchee_arg_ids.iter().copied())
                            .map(|(param_id, arg_id)| {
                                let param_symbol = symbol_db.identifier_symbols.get(param_id);
                                Substitution {
                                    from: SubstitutionLhs::Symbol(param_symbol),
                                    // This is safe, since we know the matchee is a well-typed normal form Call expression.
                                    // Every argument of a normal form Call expression is a normal form, therefore it
                                    // is safe to assume that arg_id is a NormalFormNodeId.
                                    to: NormalFormNodeId(arg_id),
                                }
                            })
                            .collect()
                    };
                    let substituted_output = apply_substitutions(
                        registry,
                        symbol_db,
                        sih_cache,
                        fv_cache,
                        type0_identifier_id,
                        case_output_id,
                        substitutions.iter().copied(),
                    );
                    Ok(EvalStepResult::Stepped(substituted_output))
                }
            }
        }
        ExpressionId::Forall(forall_id) => {
            let forall = registry.forall(forall_id);
            let param_list_id = forall.param_list_id;
            let param_ids = registry.param_list(param_list_id).to_vec();
            let output_id = forall.output_id;

            for (param_index, param_id) in param_ids.iter().copied().enumerate() {
                let param = registry.param(param_id);
                let param_is_dashed = param.is_dashed;
                let param_name_id = param.name_id;
                let param_type_id = param.type_id;
                let param_type_step_result = perform_eval_step_on_well_typed_expression(
                    registry,
                    symbol_db,
                    sih_cache,
                    fv_cache,
                    type0_identifier_id,
                    param_type_id,
                )?;
                if let EvalStepResult::Stepped(stepped_param_type_id) = param_type_step_result {
                    let stepped_param_id = registry.add_param_and_overwrite_its_id(Param {
                        id: dummy_id(),
                        is_dashed: param_is_dashed,
                        name_id: param_name_id,
                        type_id: stepped_param_type_id,
                    });
                    let stepped_param_ids = {
                        let mut out = Vec::with_capacity(param_ids.len());
                        out.extend(param_ids[0..param_index].iter().copied());
                        out.push(stepped_param_id);
                        out.extend(param_ids[param_index + 1..].iter().copied());
                        out
                    };
                    let stepped_param_list_id = registry.add_param_list(stepped_param_ids);
                    let stepped_forall_id = registry.add_forall_and_overwrite_its_id(Forall {
                        id: dummy_id(),
                        param_list_id: stepped_param_list_id,
                        output_id: output_id,
                    });
                    let wrapped_stepped_id = ExpressionId::Forall(stepped_forall_id);
                    return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
                }
            }

            let output_step_result = perform_eval_step_on_well_typed_expression(
                registry,
                symbol_db,
                sih_cache,
                fv_cache,
                type0_identifier_id,
                output_id,
            )?;
            if let EvalStepResult::Stepped(stepped_output_id) = output_step_result {
                let stepped_forall_id = registry.add_forall_and_overwrite_its_id(Forall {
                    id: dummy_id(),
                    param_list_id: param_list_id,
                    output_id: stepped_output_id,
                });
                let wrapped_stepped_id = ExpressionId::Forall(stepped_forall_id);
                return Ok(EvalStepResult::Stepped(wrapped_stepped_id));
            }

            Ok(EvalStepResult::CouldNotStepBecauseNormalForm(
                NormalFormNodeId(expression_id),
            ))
        }
    }
}
