use super::*;

pub fn type_check_files(
    registry: &mut NodeRegistry,
    file_ids: &[NodeId<File>],
) -> Result<(), TypeCheckError> {
    let mut context = Context::with_builtins(registry);
    for &id in file_ids {
        type_check_file(&mut context, registry, id)?;
    }
    Ok(())
}

fn type_check_file(
    context: &mut Context,
    registry: &mut NodeRegistry,
    file_id: NodeId<File>,
) -> Result<(), TypeCheckError> {
    let file = registry.file(file_id);
    let items = registry.file_item_list(file.item_list_id).to_vec();
    for &item_id in &items {
        type_check_file_item(context, registry, item_id)?;
    }
    context.pop_n(items.len());
    Ok(())
}

fn type_check_file_item(
    context: &mut Context,
    registry: &mut NodeRegistry,
    item: FileItemNodeId,
) -> Result<(), TypeCheckError> {
    match item {
        FileItemNodeId::Type(type_statement) => {
            type_check_type_statement(context, registry, type_statement)
        }
        FileItemNodeId::Let(let_statement) => {
            type_check_let_statement(context, registry, let_statement)
        }
    }
}

fn type_check_type_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeCheckError> {
    type_check_type_constructor(context, registry, type_statement_id)?;

    let type_statement = registry.type_statement(type_statement_id);
    let variant_ids = registry
        .variant_list(type_statement.variant_list_id)
        .to_vec();
    for variant_id in variant_ids {
        type_check_type_variant(context, registry, variant_id)?;
    }

    Ok(())
}

fn type_check_type_constructor(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeCheckError> {
    let type_statement = registry.type_statement(type_statement_id).clone();
    let normalized_param_list_id =
        normalize_params(context, registry, type_statement.param_list_id)?;
    let type_constructor_type_id = NormalFormId::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: type0_expression(context, registry).raw(),
        }
        .collapse_if_nullary(registry),
    );
    let variant_name_list_id = {
        let variant_ids = registry.variant_list(type_statement.variant_list_id);
        let variant_name_ids = variant_ids
            .iter()
            .map(|&variant_id| registry.variant(variant_id).name_id)
            .collect();
        registry.add_identifier_list(variant_name_ids)
    };
    context.push(ContextEntry {
        type_id: type_constructor_type_id,
        definition: ContextEntryDefinition::Adt {
            variant_name_list_id,
        },
    });
    Ok(())
}

pub fn type_check_param(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_id: NodeId<Param>,
) -> Result<(), TypeCheckError> {
    let param = registry.param(param_id).clone();
    let param_type_type_id = get_type_of_expression(context, registry, None, param.type_id)?;
    if !is_term_equal_to_type0_or_type1(context, registry, param_type_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(param.type_id));
    }

    let normalized_type_id = evaluate_well_typed_expression(context, registry, param.type_id);
    context.push(ContextEntry {
        type_id: normalized_type_id,
        definition: ContextEntryDefinition::Uninterpreted,
    });
    Ok(())
}

fn type_check_type_variant(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: NodeId<Variant>,
) -> Result<(), TypeCheckError> {
    let variant = registry.variant(variant_id).clone();
    let arity = variant.param_list_id.len;
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(context, registry, variant.param_list_id)?;
    type_check_expression(context, registry, None, variant.return_type_id)?;
    let return_type_id = evaluate_well_typed_expression(context, registry, variant.return_type_id);
    let type_id = NormalFormId::unchecked_new(
        Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: return_type_id.raw(),
        }
        .collapse_if_nullary(registry),
    );
    context.pop_n(arity);
    context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Variant {
            name_id: variant.name_id,
        },
    });
    Ok(())
}

fn type_check_let_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: NodeId<LetStatement>,
) -> Result<(), TypeCheckError> {
    let let_statement = registry.let_statement(let_statement_id).clone();
    let type_id = get_type_of_expression(context, registry, None, let_statement.value_id)?;
    let normalized_value_id =
        evaluate_well_typed_expression(context, registry, let_statement.value_id);
    context.push(ContextEntry {
        type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_value_id,
        },
    });
    Ok(())
}

fn type_check_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    coercion_target_id: Option<NormalFormId>,
    expression: ExpressionId,
) -> Result<(), TypeCheckError> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression(context, registry, coercion_target_id, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    coercion_target_id: Option<NormalFormId>,
    id: ExpressionId,
) -> Result<NormalFormId, TypeCheckError> {
    match id {
        ExpressionId::Name(name) => Ok(get_type_of_name(context, registry, name)),
        ExpressionId::Call(call) => get_type_of_call(context, registry, call),
        ExpressionId::Fun(fun) => get_type_of_fun(context, registry, fun),
        ExpressionId::Match(match_) => {
            get_type_of_match(context, registry, coercion_target_id, match_)
        }
        ExpressionId::Forall(forall) => get_type_of_forall(context, registry, forall),
    }
}

fn get_type_of_name(
    context: &mut Context,
    registry: &mut NodeRegistry,
    name_id: NodeId<NameExpression>,
) -> NormalFormId {
    let name = registry.name_expression(name_id);
    context.get_type(name.db_index, registry)
}

fn get_type_of_call(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: NodeId<Call>,
) -> Result<NormalFormId, TypeCheckError> {
    let call = registry.call(call_id).clone();
    let callee_type_id = get_type_of_expression(context, registry, None, call.callee_id)?;
    let callee_type_id = if let ExpressionId::Forall(id) = callee_type_id.raw() {
        id
    } else {
        return Err(TypeCheckError::BadCallee(call.callee_id));
    };
    let arg_ids = registry.expression_list(call.arg_list_id).to_vec();
    let arg_type_ids = arg_ids
        .iter()
        .copied()
        .map(|arg_id| {
            get_type_of_expression(
                context, registry, /* TODO: Infer from call param types. */ None, arg_id,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let callee_type = registry.forall(callee_type_id);
    // We use the params of the callee _type_ rather than the params of the
    // callee itself, since the callee type is a normal form, which guarantees
    // that its params are normal forms.
    let callee_type_param_ids = registry.param_list(callee_type.param_list_id).to_vec();
    {
        let expected_arity = callee_type_param_ids.len();
        let actual_arity = arg_ids.len();
        if callee_type_param_ids.len() != arg_type_ids.len() {
            return Err(TypeCheckError::WrongNumberOfArguments {
                call_id: call_id,
                expected: expected_arity,
                actual: actual_arity,
            });
        }
    }
    for (i, (callee_type_param_id, arg_type_id)) in callee_type_param_ids
        .iter()
        .copied()
        .zip(arg_type_ids.iter().copied())
        .enumerate()
    {
        // TODO: Substitute the arg values into the param type, one-by-one.
        let callee_type_param = registry.param(callee_type_param_id);
        if !is_left_type_assignable_to_right_type(
            context,
            registry,
            arg_type_id,
            // This is safe because the param is the param of a normal
            // form Forall node, which guarantees that its type is a
            // normal form.
            NormalFormId::unchecked_new(callee_type_param.type_id),
        ) {
            return Err(TypeCheckError::TypeMismatch {
                expression_id: arg_ids[i],
                expected_type_id: NormalFormId::unchecked_new(callee_type_param.type_id),
                actual_type_id: arg_type_id,
            });
        }
    }
    // TODO: Substitute arg values into output type
    Ok(NormalFormId::unchecked_new(callee_type.output_id))
}

fn get_type_of_fun(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: NodeId<Fun>,
) -> Result<NormalFormId, TypeCheckError> {
    let original_context_len = context.len();

    let fun = registry.fun(fun_id).clone();
    let normalized_param_list_id =
        normalize_params_and_leave_params_in_context(context, registry, fun.param_list_id)?;
    {
        let return_type_type_id =
            get_type_of_expression(context, registry, None, fun.return_type_id)?;
        if !is_term_equal_to_type0_or_type1(context, registry, return_type_type_id) {
            return Err(TypeCheckError::IllegalTypeExpression(fun.return_type_id));
        }
    }
    let normalized_return_type_id =
        evaluate_well_typed_expression(context, registry, fun.return_type_id);

    let fun_type_id = NormalFormId::unchecked_new(ExpressionId::Forall(
        registry.add_forall_and_overwrite_its_id(Forall {
            id: dummy_id(),
            param_list_id: normalized_param_list_id,
            output_id: normalized_return_type_id.raw(),
        }),
    ));

    let shifted_fun_id = fun_id.upshift(context.len() - original_context_len, registry);
    let normalized_fun_id =
        evaluate_well_typed_expression(context, registry, ExpressionId::Fun(shifted_fun_id));
    context.push(ContextEntry {
        type_id: fun_type_id,
        definition: ContextEntryDefinition::Alias {
            value_id: normalized_fun_id,
        },
    });

    let normalized_body_type_id = get_type_of_expression(
        context,
        registry,
        Some(normalized_return_type_id),
        fun.body_id,
    )?;
    if !is_left_type_assignable_to_right_type(
        context,
        registry,
        normalized_body_type_id,
        normalized_return_type_id,
    ) {
        return Err(TypeCheckError::TypeMismatch {
            expression_id: fun.body_id,
            expected_type_id: normalized_return_type_id,
            actual_type_id: normalized_body_type_id,
        });
    }

    context.pop_n(fun.param_list_id.len + 1);
    Ok(fun_type_id)
}

fn get_type_of_match(
    context: &mut Context,
    registry: &mut NodeRegistry,
    coercion_target_id: Option<NormalFormId>,
    match_id: NodeId<Match>,
) -> Result<NormalFormId, TypeCheckError> {
    let match_ = registry.match_(match_id).clone();
    let matchee_type_id = get_type_of_expression(context, registry, None, match_.matchee_id)?;
    let matchee_type = if let Some(t) = try_as_adt_expression(context, registry, matchee_type_id) {
        t
    } else {
        return Err(TypeCheckError::NonAdtMatchee {
            matchee_id: match_.matchee_id,
            type_id: matchee_type_id,
        });
    };

    verify_variant_to_case_bijection(
        registry,
        matchee_type.variant_name_list_id,
        match_.case_list_id,
    )?;

    let case_ids = registry.match_case_list(match_.case_list_id).to_vec();
    let mut first_case_type_id = None;
    for case_id in case_ids {
        let case_type_id = get_type_of_match_case(
            context,
            registry,
            coercion_target_id,
            case_id,
            matchee_type_id,
            matchee_type,
        )?;
        if let Some(first_case_type_id) = first_case_type_id {
            if !is_left_type_assignable_to_right_type(
                context,
                registry,
                case_type_id,
                first_case_type_id,
            ) {
                let case = registry.match_case(case_id);
                return Err(TypeCheckError::TypeMismatch {
                    expression_id: case.output_id,
                    expected_type_id: first_case_type_id,
                    actual_type_id: case_type_id,
                });
            }
        } else {
            first_case_type_id = Some(case_type_id);
        }
    }

    if let Some(first_case_type_id) = first_case_type_id {
        Ok(first_case_type_id)
    } else {
        // If `first_case_type_id` is `None`, then `case_ids` is empty, which
        // means the matchee has any empty type.
        // Thus, the match should have an empty type.
        Ok(matchee_type_id)
    }
}

fn get_type_of_match_case(
    context: &mut Context,
    registry: &mut NodeRegistry,
    coercion_target_id: Option<NormalFormId>,
    case_id: NodeId<MatchCase>,
    matchee_type_id: NormalFormId,
    matchee_type: AdtExpression,
) -> Result<NormalFormId, TypeCheckError> {
    let case = registry.match_case(case_id).clone();
    let case_arity = case.param_list_id.len;
    let parameterized_type_id = add_case_params_to_context_and_get_constructed_type(
        context,
        registry,
        case_id,
        matchee_type,
    )?;

    let original_coercion_target_id = coercion_target_id;
    let shifted_coercion_target_id = coercion_target_id
        .map(|coercion_target_id| coercion_target_id.upshift(case_arity, registry));

    let substitutions =
        fuse_left_to_right(context, registry, matchee_type_id, parameterized_type_id);

    let mut substituted_context = context.clone().subst_all(&substitutions, registry);
    let substituted_coercion_target_id = shifted_coercion_target_id
        .map(|coercion_target_id| coercion_target_id.raw().subst_all(&substitutions, registry));
    let normalized_substituted_coercion_target_id =
        substituted_coercion_target_id.map(|coercion_target_id| {
            evaluate_well_typed_expression(context, registry, coercion_target_id)
        });
    let output_type_id = get_type_of_expression(
        &mut substituted_context,
        registry,
        normalized_substituted_coercion_target_id,
        case.output_id,
    )?;

    context.pop_n(case_arity);

    match normalized_substituted_coercion_target_id {
        Some(normalized_substituted_coercion_target_id)
            if is_left_type_assignable_to_right_type(
                &mut substituted_context,
                registry,
                output_type_id,
                normalized_substituted_coercion_target_id,
            ) =>
        {
            Ok(original_coercion_target_id.expect("original_coercion_target_id must be Some if normalized_substituted_coercion_target_id is some"))
        }
        _ => {
            match output_type_id.try_downshift(case_arity, registry) {
                Ok(shifted_output_type_id) => Ok(shifted_output_type_id),
                Err(_) => Err(TypeCheckError::AmbiguousOutputType {
                    case_id,
                }),
            }
        },
    }
}

fn add_case_params_to_context_and_get_constructed_type(
    context: &mut Context,
    registry: &mut NodeRegistry,
    case_id: NodeId<MatchCase>,
    matchee_type: AdtExpression,
) -> Result<NormalFormId, TypeCheckError> {
    let case = registry.match_case(case_id).clone();
    let variant_dbi =
        get_db_index_for_adt_variant_of_name(context, registry, matchee_type, case.variant_name_id);
    let variant_type_id = context.get_type(variant_dbi, registry);
    match variant_type_id.raw() {
        ExpressionId::Forall(forall_id) => {
            let forall = registry.forall(forall_id);
            let param_ids = registry.param_list(forall.param_list_id).to_vec();
            for &param_id in &param_ids {
                let param = registry.param(param_id);
                // We can safely call `unchecked_new` on the param type id
                // because the Forall which the param came from was a normal form.
                // We know this because we obtained the Forall from matching against
                // `variant_type_id.raw()`.
                let param_type_id = NormalFormId::unchecked_new(param.type_id);
                context.push(ContextEntry {
                    type_id: param_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                });
            }

            // We can safely call `unchecked_new` on the output id
               // because the Forall which the param came from was a normal form.
                // We know this because we obtained the Forall from matching against
                // `variant_type_id.raw()`.
            Ok( NormalFormId::unchecked_new(forall.output_id))
        }
        ExpressionId::Call(_) => {
            // In this case, the variant is nullary.
            Ok(variant_type_id)
        }
        other => panic!("A variant's type should always either be a Forall or a Call, but it was actually a {:?}", other),
    }
}

fn get_type_of_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<NormalFormId, TypeCheckError> {
    let forall = registry.forall(forall_id).clone();
    normalize_params_and_leave_params_in_context(context, registry, forall.param_list_id)?;

    let output_type_id = get_type_of_expression(context, registry, None, forall.output_id)?;
    if !is_term_equal_to_type0_or_type1(context, registry, output_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(forall.output_id));
    }

    context.pop_n(forall.param_list_id.len);

    Ok(type0_expression(context, registry))
}
