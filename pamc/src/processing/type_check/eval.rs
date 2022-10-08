use super::*;

pub fn evaluate_well_typed_expression(
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
pub enum EvalStepResult {
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
