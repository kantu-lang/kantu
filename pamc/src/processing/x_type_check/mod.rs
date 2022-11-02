use crate::data::{
    x_light_ast::*,
    x_node_registry::{ListId, NodeId, NodeRegistry},
};

#[derive(Clone, Debug)]
pub enum TypeCheckError {
    IllegalTypeExpression(ExpressionId),
    BadCallee(ExpressionId),
    WrongNumberOfArguments {
        call_id: NodeId<Call>,
        expected: usize,
        actual: usize,
    },
    TypeMismatch {
        expression_id: ExpressionId,
        expected_type_id: NormalFormId,
        actual_type_id: NormalFormId,
    },
    NonAdtMatchee {
        matchee_id: ExpressionId,
        type_id: NormalFormId,
    },
    DuplicateMatchCase {
        existing_match_case_id: NodeId<MatchCase>,
        new_match_case_id: NodeId<MatchCase>,
    },
    MissingMatchCase {
        variant_name_id: NodeId<Identifier>,
    },
    ExtraneousMatchCase {
        case_id: NodeId<MatchCase>,
    },
}

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

fn normalize_params(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let normalized_list_id =
        normalize_params_and_leave_params_in_context(context, registry, param_list_id)?;
    context.pop_n(param_list_id.len);
    Ok(normalized_list_id)
}

fn normalize_params_and_leave_params_in_context(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: ListId<NodeId<Param>>,
) -> Result<ListId<NodeId<Param>>, TypeCheckError> {
    let param_ids = registry.param_list(param_list_id).to_vec();
    let normalized_ids = param_ids
        .iter()
        .copied()
        .map(|param_id| {
            type_check_param(context, registry, param_id)?;
            let type_id: ExpressionId = context.get_type(DbIndex(0), registry).raw();
            let old_param = registry.param(param_id);
            let normalized_param_with_dummy_id = Param {
                id: dummy_id(),
                is_dashed: old_param.is_dashed,
                name_id: old_param.name_id,
                type_id,
            };
            let normalized_id =
                registry.add_param_and_overwrite_its_id(normalized_param_with_dummy_id);
            Ok(normalized_id)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(registry.add_param_list(normalized_ids))
}

fn type_check_param(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_id: NodeId<Param>,
) -> Result<(), TypeCheckError> {
    let param = registry.param(param_id).clone();
    let param_type_type_id = get_type_of_expression(context, registry, param.type_id)?;
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
    type_check_expression(context, registry, variant.return_type_id)?;
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
    let type_id = get_type_of_expression(context, registry, let_statement.value_id)?;
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
    expression: ExpressionId,
) -> Result<(), TypeCheckError> {
    // In the future, we could implement a version of this that skips the
    // allocations required by `get_type_of_expression`, since we don't
    // actually use the returned type.
    // But for now, we'll just reuse the existing code, for the sake of
    // simplicity.
    get_type_of_expression(context, registry, expression).map(std::mem::drop)
}

fn get_type_of_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: ExpressionId,
) -> Result<NormalFormId, TypeCheckError> {
    match id {
        ExpressionId::Name(name) => Ok(get_type_of_name(context, registry, name)),
        ExpressionId::Call(call) => get_type_of_call(context, registry, call),
        ExpressionId::Fun(fun) => get_type_of_fun(context, registry, fun),
        ExpressionId::Match(match_) => get_type_of_match(context, registry, match_),
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
    let callee_type_id = get_type_of_expression(context, registry, call.callee_id)?;
    let callee_type_id = if let ExpressionId::Forall(id) = callee_type_id.raw() {
        id
    } else {
        return Err(TypeCheckError::BadCallee(call.callee_id));
    };
    let arg_ids = registry.expression_list(call.arg_list_id).to_vec();
    let arg_type_ids = arg_ids
        .iter()
        .copied()
        .map(|arg_id| get_type_of_expression(context, registry, arg_id))
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
        let return_type_type_id = get_type_of_expression(context, registry, fun.return_type_id)?;
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

    let normalized_body_type_id = get_type_of_expression(context, registry, fun.body_id)?;
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
    match_id: NodeId<Match>,
) -> Result<NormalFormId, TypeCheckError> {
    let match_ = registry.match_(match_id).clone();
    let matchee_type_id = get_type_of_expression(context, registry, match_.matchee_id)?;
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
        let case_type_id =
            get_type_of_match_case(context, registry, case_id, matchee_type_id, matchee_type)?;
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

fn verify_variant_to_case_bijection(
    registry: &NodeRegistry,
    variant_name_list_id: ListId<NodeId<Identifier>>,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    verify_there_are_no_duplicate_cases(registry, case_list_id)?;
    verify_that_every_variant_has_a_case(registry, variant_name_list_id, case_list_id)?;
    verify_that_every_case_has_a_variant(registry, variant_name_list_id, case_list_id)?;
    Ok(())
}

fn verify_there_are_no_duplicate_cases(
    registry: &NodeRegistry,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    let mut visited_cases: Vec<NodeId<MatchCase>> = Vec::with_capacity(case_list_id.len);

    let case_ids = registry.match_case_list(case_list_id);

    for &case_id in case_ids {
        let case = registry.match_case(case_id);
        let case_variant_name = &registry.identifier(case.variant_name_id).name;

        if let Some(existing_case_id) = visited_cases
            .iter()
            .find(|&&existing_case_id| {
                let existing_case = registry.match_case(existing_case_id);
                let existing_case_variant_name =
                    &registry.identifier(existing_case.variant_name_id).name;
                existing_case_variant_name == case_variant_name
            })
            .copied()
        {
            return Err(TypeCheckError::DuplicateMatchCase {
                existing_match_case_id: existing_case_id,
                new_match_case_id: case_id,
            });
        }

        visited_cases.push(case_id);
    }

    Ok(())
}

fn verify_that_every_variant_has_a_case(
    registry: &NodeRegistry,
    variant_name_list_id: ListId<NodeId<Identifier>>,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    let variant_name_ids = registry.identifier_list(variant_name_list_id);
    let case_ids = registry.match_case_list(case_list_id);

    for &variant_name_id in variant_name_ids {
        let variant_name = &registry.identifier(variant_name_id).name;
        if !case_ids.iter().any(|&case_id| {
            let case = registry.match_case(case_id);
            let case_variant_name = &registry.identifier(case.variant_name_id).name;
            case_variant_name == variant_name
        }) {
            return Err(TypeCheckError::MissingMatchCase { variant_name_id });
        }
    }
    Ok(())
}

fn verify_that_every_case_has_a_variant(
    registry: &NodeRegistry,
    variant_name_list_id: ListId<NodeId<Identifier>>,
    case_list_id: ListId<NodeId<MatchCase>>,
) -> Result<(), TypeCheckError> {
    let variant_name_ids = registry.identifier_list(variant_name_list_id);
    let case_ids = registry.match_case_list(case_list_id);

    for &case_id in case_ids {
        let case = registry.match_case(case_id);
        let case_variant_name = &registry.identifier(case.variant_name_id).name;
        if !variant_name_ids.iter().any(|&variant_name_id| {
            let variant_name = &registry.identifier(variant_name_id).name;
            case_variant_name == variant_name
        }) {
            return Err(TypeCheckError::ExtraneousMatchCase { case_id });
        }
    }
    Ok(())
}

fn get_type_of_match_case(
    context: &mut Context,
    registry: &mut NodeRegistry,
    case_id: NodeId<MatchCase>,
    matchee_type_id: NormalFormId,
    matchee_type: AdtExpression,
) -> Result<NormalFormId, TypeCheckError> {
    let case = registry.match_case(case_id).clone();
    let parameterized_type_id = add_case_params_to_context_and_get_constructed_type(
        context,
        registry,
        case_id,
        matchee_type,
    )?;

    let substitutions =
        fuse_left_to_right(context, registry, matchee_type_id, parameterized_type_id);

    let mut substituted_context = context.clone().subst_all(&substitutions, registry);
    let output_type_id =
        get_type_of_expression(&mut substituted_context, registry, case.output_id)?;

    context.pop_n(case.param_list_id.len);

    Ok(output_type_id)
}

fn add_case_params_to_context_and_get_constructed_type(
    context: &mut Context,
    registry: &mut NodeRegistry,
    case_id: NodeId<MatchCase>,
    matchee_type: AdtExpression,
) -> Result<NormalFormId, TypeCheckError> {
    let case = registry.match_case(case_id).clone();
    let _variant_dbi =
        get_db_index_for_adt_variant_of_name(context, registry, matchee_type, case.variant_name_id);
    unimplemented!()
}

fn get_db_index_for_adt_variant_of_name(
    context: &Context,
    registry: &mut NodeRegistry,
    adt_expression: AdtExpression,
    target_variant_name_id: NodeId<Identifier>,
) -> DbIndex {
    let type_dbi = registry
        .name_expression(adt_expression.type_name_id)
        .db_index;
    let variant_name_list_id = match context.get_definition(type_dbi, registry) {
        ContextEntryDefinition::Adt {
            variant_name_list_id,
        } => variant_name_list_id,
        _ => panic!("An ADT's NameExpression should always point to an ADT definition"),
    };

    let target_variant_name = &registry.identifier(target_variant_name_id).name;
    let variant_index = registry
        .identifier_list(variant_name_list_id)
        .iter()
        .position(|&variant_name_id| {
            let variant_name = &registry.identifier(variant_name_id).name;
            variant_name == target_variant_name
        })
        .expect("The target variant name should always be found in the ADT's variant name list");
    DbIndex(type_dbi.0 + 1 + variant_index)
}

fn fuse_left_to_right(
    _context: &mut Context,
    _registry: &mut NodeRegistry,
    _left: NormalFormId,
    _right: NormalFormId,
) -> Vec<Substitution> {
    unimplemented!()
}

fn get_type_of_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: NodeId<Forall>,
) -> Result<NormalFormId, TypeCheckError> {
    let forall = registry.forall(forall_id).clone();
    normalize_params_and_leave_params_in_context(context, registry, forall.param_list_id)?;

    let output_type_id = get_type_of_expression(context, registry, forall.output_id)?;
    if !is_term_equal_to_type0_or_type1(context, registry, output_type_id) {
        return Err(TypeCheckError::IllegalTypeExpression(forall.output_id));
    }

    context.pop_n(forall.param_list_id.len);

    Ok(type0_expression(context, registry))
}

use eval::*;
mod eval {
    use super::*;

    pub fn evaluate_well_typed_expression(
        context: &mut Context,
        registry: &mut NodeRegistry,
        id: ExpressionId,
    ) -> NormalFormId {
        match id {
            ExpressionId::Name(name_id) => {
                evaluate_well_typed_name_expression(context, registry, name_id)
            }
            ExpressionId::Call(call_id) => evaluate_well_typed_call(context, registry, call_id),
            ExpressionId::Fun(fun_id) => evaluate_well_typed_fun(context, registry, fun_id),
            ExpressionId::Match(match_id) => evaluate_well_typed_match(context, registry, match_id),
            ExpressionId::Forall(forall_id) => {
                evaluate_well_typed_forall(context, registry, forall_id)
            }
        }
    }

    fn evaluate_well_typed_name_expression(
        context: &mut Context,
        registry: &mut NodeRegistry,
        name_id: NodeId<NameExpression>,
    ) -> NormalFormId {
        let name = registry.name_expression(name_id);
        let definition = context.get_definition(name.db_index, registry);
        match definition {
            ContextEntryDefinition::Alias { value_id } => value_id,

            ContextEntryDefinition::Adt {
                variant_name_list_id: _,
            }
            | ContextEntryDefinition::Variant { name_id: _ }
            | ContextEntryDefinition::Uninterpreted => {
                NormalFormId::unchecked_new(ExpressionId::Name(name_id))
            }
        }
    }

    fn evaluate_well_typed_call(
        context: &mut Context,
        registry: &mut NodeRegistry,
        call_id: NodeId<Call>,
    ) -> NormalFormId {
        fn register_normalized_nonsubstituted_fun(
            registry: &mut NodeRegistry,
            normalized_callee_id: NormalFormId,
            normalized_arg_ids: &[NormalFormId],
        ) -> NormalFormId {
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
            NormalFormId::unchecked_new(ExpressionId::Call(normalized_call_id))
        }

        let call = registry.call(call_id).clone();

        let normalized_callee_id =
            evaluate_well_typed_expression(context, registry, call.callee_id);

        let normalized_arg_ids: Vec<NormalFormId> = {
            let arg_ids = registry.expression_list(call.arg_list_id).to_vec();
            arg_ids
                .into_iter()
                .map(|arg_id| evaluate_well_typed_expression(context, registry, arg_id))
                .collect()
        };

        match normalized_callee_id.raw() {
            // TODO: Only unwrap if decreasing argument has a variant at the top,
            // or if there is no decreasing argument (i.e., the function is non-recursive).
            ExpressionId::Fun(fun_id) => {
                if !can_fun_be_applied(context, registry, fun_id, &normalized_arg_ids) {
                    return register_normalized_nonsubstituted_fun(
                        registry,
                        normalized_callee_id,
                        &normalized_arg_ids,
                    );
                }

                let fun = registry.fun(fun_id).clone();
                let param_ids = registry.param_list(fun.param_list_id).to_vec();
                let arity = param_ids.len();
                let shifted_normalized_arg_ids = normalized_arg_ids
                    .into_iter()
                    .map(|arg_id| arg_id.upshift(arity + 1, registry))
                    .collect::<Vec<_>>();
                let substitutions =
                    {
                        let shifted_fun_id = NormalFormId::unchecked_new(ExpressionId::Fun(
                            fun_id.upshift(arity + 1, registry),
                        ));
                        const FUN_DB_INDEX: DbIndex = DbIndex(0);
                        vec![Substitution::Single {
                            from: NormalFormId::unchecked_new(ExpressionId::Name(
                                add_name_expression(registry, vec![fun.name_id], FUN_DB_INDEX),
                            )),
                            to: shifted_fun_id,
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
                                let param_name_id = registry.param(param_id).name_id;
                                let db_index = DbIndex(arity - arg_index);
                                let name = NormalFormId::unchecked_new(ExpressionId::Name(
                                    add_name_expression(registry, vec![param_name_id], db_index),
                                ));
                                Substitution::Single {
                                    from: name,
                                    to: arg_id,
                                }
                            }),
                    )
                    .collect::<Vec<_>>();

                let body_id = fun.body_id.subst_all(&substitutions, registry);
                let shifted_body_id = body_id.downshift(arity + 1, registry);
                evaluate_well_typed_expression(context, registry, shifted_body_id)
            }
            ExpressionId::Name(_) | ExpressionId::Call(_) | ExpressionId::Match(_) => {
                register_normalized_nonsubstituted_fun(
                    registry,
                    normalized_callee_id,
                    &normalized_arg_ids,
                )
            }
            ExpressionId::Forall(_) => {
                panic!("A well-typed Call expression cannot have a Forall as its callee.")
            }
        }
    }

    fn can_fun_be_applied(
        context: &mut Context,
        registry: &mut NodeRegistry,
        fun_id: NodeId<Fun>,
        normalized_arg_ids: &[NormalFormId],
    ) -> bool {
        let param_list_id = registry.fun(fun_id).param_list_id;
        let decreasing_param_index =
            registry
                .param_list(param_list_id)
                .iter()
                .copied()
                .position(|param_id| {
                    let param = registry.param(param_id);
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
        is_variant_expression(context, registry, decreasing_arg_id)
    }

    /// If the provided expression is has a variant at
    /// the top level,this returns IDs for the variant name
    /// and the variant's argument list.
    /// Otherwise, returns `None`.
    fn is_variant_expression(
        context: &mut Context,
        registry: &mut NodeRegistry,
        expression_id: NormalFormId,
    ) -> bool {
        try_as_variant_expression(context, registry, expression_id).is_some()
    }

    fn evaluate_well_typed_fun(
        context: &mut Context,
        registry: &mut NodeRegistry,
        fun_id: NodeId<Fun>,
    ) -> NormalFormId {
        let fun = registry.fun(fun_id).clone();
        let normalized_param_list_id =
            normalize_params_and_leave_params_in_context(context, registry, fun.param_list_id)
                .expect("A well-typed Fun should have well-typed params.");
        let normalized_return_type_id =
            evaluate_well_typed_expression(context, registry, fun.return_type_id);
        context.pop_n(fun.param_list_id.len);

        NormalFormId::unchecked_new(ExpressionId::Fun(registry.add_fun_and_overwrite_its_id(
            Fun {
                id: dummy_id(),
                name_id: fun.name_id,
                param_list_id: normalized_param_list_id,
                return_type_id: normalized_return_type_id.raw(),
                body_id: fun.body_id,
            },
        )))
    }

    fn evaluate_well_typed_match(
        context: &mut Context,
        registry: &mut NodeRegistry,
        match_id: NodeId<Match>,
    ) -> NormalFormId {
        let match_ = registry.match_(match_id).clone();
        let normalized_matchee_id =
            evaluate_well_typed_expression(context, registry, match_.matchee_id);

        let (normalized_matchee_variant_name_id, normalized_matchee_arg_list_id) =
            if let Some((variant_name_id, arg_list_id)) =
                try_as_variant_expression(context, registry, normalized_matchee_id)
            {
                (variant_name_id, arg_list_id)
            } else {
                return NormalFormId::unchecked_new(ExpressionId::Match(
                    registry.add_match_and_overwrite_its_id(Match {
                        id: dummy_id(),
                        matchee_id: normalized_matchee_id.raw(),
                        case_list_id: match_.case_list_id,
                    }),
                ));
            };

        let case_id = *registry
            .match_case_list(match_.case_list_id)
            .iter()
            .find(|case_id| {
                let case = registry.match_case(**case_id);
                case.variant_name_id == normalized_matchee_variant_name_id
            })
             .expect("A well-typed Match expression should have a case for every variant of its matchee's type.");

        let case = registry.match_case(case_id).clone();

        match normalized_matchee_arg_list_id {
            PossibleArgListId::Nullary => {
                evaluate_well_typed_expression(context, registry, case.output_id)
            }
            PossibleArgListId::Some(normalized_matchee_arg_list_id) => {
                let case_param_ids = registry.identifier_list(case.param_list_id).to_vec();
                let case_arity = case_param_ids.len();
                let matchee_arg_ids = registry
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
                            NormalFormId::unchecked_new(arg_id).upshift(case_arity, registry);
                        Substitution::Single {
                            from: NormalFormId::unchecked_new(ExpressionId::Name(
                                add_name_expression(registry, vec![param_id], db_index),
                            )),
                            to: shifted_arg_id,
                        }
                    })
                    .collect();

                let substituted_body = case
                    .output_id
                    .subst_all(&substitutions, registry)
                    .downshift(case_arity, registry);
                evaluate_well_typed_expression(context, registry, substituted_body)
            }
        }
    }

    /// If the provided expression is has a variant at
    /// the top level,this returns IDs for the variant name
    /// and the variant's argument list.
    /// Otherwise, returns `None`.
    fn try_as_variant_expression(
        context: &mut Context,
        registry: &mut NodeRegistry,
        expression_id: NormalFormId,
    ) -> Option<(NodeId<Identifier>, PossibleArgListId)> {
        match expression_id.raw() {
            ExpressionId::Name(name_id) => {
                let db_index = registry.name_expression(name_id).db_index;
                let definition = context.get_definition(db_index, registry);
                match definition {
                    ContextEntryDefinition::Variant { name_id } => {
                        Some((name_id, PossibleArgListId::Nullary))
                    }
                    _ => None,
                }
            }
            ExpressionId::Call(call_id) => {
                let call = registry.call(call_id).clone();
                match call.callee_id {
                    ExpressionId::Name(name_id) => {
                        let db_index = registry.name_expression(name_id).db_index;
                        let definition = context.get_definition(db_index, registry);
                        match definition {
                            ContextEntryDefinition::Variant { name_id } => {
                                Some((name_id, PossibleArgListId::Some(call.arg_list_id)))
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn evaluate_well_typed_forall(
        context: &mut Context,
        registry: &mut NodeRegistry,
        forall_id: NodeId<Forall>,
    ) -> NormalFormId {
        let forall = registry.forall(forall_id).clone();
        let normalized_param_list_id =
            normalize_params_and_leave_params_in_context(context, registry, forall.param_list_id)
                .expect("A well-typed Fun should have well-typed params.");
        let normalized_output_id =
            evaluate_well_typed_expression(context, registry, forall.output_id);
        context.pop_n(forall.param_list_id.len);

        NormalFormId::unchecked_new(ExpressionId::Forall(
            registry.add_forall_and_overwrite_its_id(Forall {
                id: dummy_id(),
                param_list_id: normalized_param_list_id,
                output_id: normalized_output_id.raw(),
            }),
        ))
    }
}

use context::*;
mod context {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct Context {
        /// Each type in the stack is expressed "locally" (i.e., relative
        /// to its position within the stack).
        ///
        /// For example, consider the scenario where `local_type_stack[1] == NameExpression { db_index: 0 }`.
        /// The local De Bruijn index `0` refers to the first symbol counting right-to-left _from position 1_.
        /// Thus, if `local_type_stack.len() == 3`, for example, then the global De Bruijn index for `local_type_stack[1]` is `2`.
        ///
        /// If an illustration would help, consider the following:
        /// ```text
        /// Type1: DNE
        /// Type0: Type1
        /// Nat: Type0
        ///
        /// ----------------------
        ///
        /// local_type_stack: [Type1, Type0, Nat] = [DNE, 0, 0]
        ///
        /// ----------------------
        ///
        /// local_type(Type0) = Type1 = 0
        /// // Why? - Count backwards from Type0 (not including Type0 itself):
        ///
        /// vvv
        /// (0)
        /// [Type1, Type0, Nat]
        ///
        /// ----------------------
        ///
        /// global_type(Type0) = Type1 = 2
        /// // Why? - Count backwards from the end of the stack (including the last item):
        ///
        /// vvv
        /// (2)     (1)    (0)
        /// [Type1, Type0, Nat]
        /// ```
        ///
        local_type_stack: Vec<ContextEntry>,
    }

    #[derive(Clone, Debug)]
    pub struct ContextEntry {
        pub type_id: NormalFormId,
        pub definition: ContextEntryDefinition,
    }

    #[derive(Clone, Copy, Debug)]
    pub enum ContextEntryDefinition {
        Alias {
            value_id: NormalFormId,
        },
        /// Algebraic data type
        Adt {
            variant_name_list_id: ListId<NodeId<Identifier>>,
        },
        Variant {
            name_id: NodeId<Identifier>,
        },
        Uninterpreted,
    }

    const TYPE1_LEVEL: DbLevel = DbLevel(0);
    const TYPE0_LEVEL: DbLevel = DbLevel(1);

    impl Context {
        pub fn with_builtins(registry: &mut NodeRegistry) -> Self {
            // We should will never retrieve the type of `Type1`, since it is undefined.
            // However, we need to store _some_ object in the stack, so that the indices
            // of the other types are correct.
            let type1_entry = {
                let dummy_type1_type_id = NormalFormId::unchecked_new(ExpressionId::Name(
                    add_name_expression_and_overwrite_component_ids(
                        registry,
                        vec![Identifier {
                            id: dummy_id(),
                            name: IdentifierName::Standard("Type2".to_owned()),
                            start: None,
                        }],
                        DbIndex(0),
                    ),
                ));
                ContextEntry {
                    type_id: dummy_type1_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                }
            };
            let type0_entry = {
                let type0_type_id = NormalFormId::unchecked_new(ExpressionId::Name(
                    add_name_expression_and_overwrite_component_ids(
                        registry,
                        vec![Identifier {
                            id: dummy_id(),
                            name: IdentifierName::Standard("Type1".to_owned()),
                            start: None,
                        }],
                        DbIndex(0),
                    ),
                ));
                ContextEntry {
                    type_id: type0_type_id,
                    definition: ContextEntryDefinition::Uninterpreted,
                }
            };
            Self {
                local_type_stack: vec![type1_entry, type0_entry],
            }
        }
    }

    impl Context {
        /// Panics if `n > self.len()`.
        pub fn pop_n(&mut self, n: usize) {
            if n > self.len() {
                panic!(
                    "Tried to pop {} elements from a context with only {} elements",
                    n,
                    self.len()
                );
            }
            self.local_type_stack.truncate(self.len() - n);
        }

        pub fn push(&mut self, entry: ContextEntry) {
            self.local_type_stack.push(entry);
        }

        pub fn len(&self) -> usize {
            self.local_type_stack.len()
        }
    }

    impl Context {
        /// Returns the De Bruijn index of the `Type0` expression.
        pub fn type0_dbi(&self) -> DbIndex {
            self.level_to_index(TYPE0_LEVEL)
        }

        /// Returns the De Bruijn index of the `Type1` expression.
        pub fn type1_dbi(&self) -> DbIndex {
            self.level_to_index(TYPE1_LEVEL)
        }
    }

    impl Context {
        fn level_to_index(&self, level: DbLevel) -> DbIndex {
            DbIndex(self.len() - level.0 - 1)
        }

        fn index_to_level(&self, index: DbIndex) -> DbLevel {
            DbLevel(self.len() - index.0 - 1)
        }
    }

    impl Context {
        pub fn get_type(&self, index: DbIndex, registry: &mut NodeRegistry) -> NormalFormId {
            let level = self.index_to_level(index);
            if level == TYPE1_LEVEL {
                panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
            }
            self.local_type_stack[level.0]
                .type_id
                .upshift(index.0 + 1, registry)
        }

        pub fn get_definition(
            &self,
            index: DbIndex,
            registry: &mut NodeRegistry,
        ) -> ContextEntryDefinition {
            let level = self.index_to_level(index);
            if level == TYPE1_LEVEL {
                panic!("Type1 has no type. We may add support for infinite type hierarchies in the future. However, for now, Type1 is the \"limit\" type.");
            }
            self.local_type_stack[level.0]
                .definition
                .upshift(index.0 + 1, registry)
        }
    }

    impl Substitute for Context {
        type Output = Self;

        fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
            unimplemented!();
        }
    }
}

use misc::*;
mod misc {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct NormalFormId(ExpressionId);

    impl NormalFormId {
        pub fn unchecked_new(expression: ExpressionId) -> Self {
            Self(expression)
        }
    }

    impl NormalFormId {
        pub fn raw(self) -> ExpressionId {
            self.0
        }
    }

    pub fn type0_expression(context: &Context, registry: &mut NodeRegistry) -> NormalFormId {
        let name_id = add_name_expression_and_overwrite_component_ids(
            registry,
            vec![Identifier {
                id: dummy_id(),
                name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                start: None,
            }],
            context.type0_dbi(),
        );
        NormalFormId::unchecked_new(ExpressionId::Name(name_id))
    }

    pub fn add_name_expression_and_overwrite_component_ids(
        registry: &mut NodeRegistry,
        components: Vec<Identifier>,
        db_index: DbIndex,
    ) -> NodeId<NameExpression> {
        let component_ids = components
            .into_iter()
            .map(|component| registry.add_identifier_and_overwrite_its_id(component))
            .collect();
        let component_list_id = registry.add_identifier_list(component_ids);
        registry.add_name_expression_and_overwrite_its_id(NameExpression {
            id: dummy_id(),
            component_list_id,
            db_index,
        })
    }

    pub fn add_name_expression(
        registry: &mut NodeRegistry,
        component_ids: Vec<NodeId<Identifier>>,
        db_index: DbIndex,
    ) -> NodeId<NameExpression> {
        let component_list_id = registry.add_identifier_list(component_ids);
        registry.add_name_expression_and_overwrite_its_id(NameExpression {
            id: dummy_id(),
            component_list_id,
            db_index,
        })
    }

    pub fn dummy_id<T>() -> NodeId<T> {
        NodeId::new(0)
    }

    impl Forall {
        pub fn collapse_if_nullary(self, registry: &mut NodeRegistry) -> ExpressionId {
            if self.param_list_id.len == 0 {
                self.output_id
            } else {
                let forall_id = registry.add_forall_and_overwrite_its_id(self);
                ExpressionId::Forall(forall_id)
            }
        }
    }

    pub fn is_term_equal_to_type0_or_type1(
        context: &Context,
        registry: &NodeRegistry,
        term: NormalFormId,
    ) -> bool {
        if let ExpressionId::Name(name_id) = term.raw() {
            let name = registry.name_expression(name_id);
            let i = name.db_index;
            i == context.type0_dbi() || i == context.type1_dbi()
        } else {
            false
        }
    }

    pub fn is_left_type_assignable_to_right_type(
        _context: &Context,
        _registry: &NodeRegistry,
        _left: NormalFormId,
        _right: NormalFormId,
    ) -> bool {
        unimplemented!()
    }
}

use shift::*;
mod shift {
    use super::*;

    pub trait ShiftDbIndices {
        type Output;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            amount: A,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self::Output;

        fn upshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
        where
            Self: Sized,
        {
            self.shift_with_cutoff(Upshift(amount), 0, registry)
        }

        fn downshift(self, amount: usize, registry: &mut NodeRegistry) -> Self::Output
        where
            Self: Sized,
        {
            self.shift_with_cutoff(Downshift(amount), 0, registry)
        }
    }

    pub trait ShiftAmount {
        fn apply(&self, i: DbIndex) -> DbIndex;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Upshift(usize);

    impl ShiftAmount for Upshift {
        fn apply(&self, i: DbIndex) -> DbIndex {
            DbIndex(i.0 + self.0)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct Downshift(usize);

    impl ShiftAmount for Downshift {
        fn apply(&self, i: DbIndex) -> DbIndex {
            DbIndex(i.0 - self.0)
        }
    }

    impl ShiftDbIndices for ContextEntryDefinition {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            amount: A,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self {
            match self {
                ContextEntryDefinition::Alias { value_id } => ContextEntryDefinition::Alias {
                    value_id: value_id.shift_with_cutoff(amount, cutoff, registry),
                },

                ContextEntryDefinition::Adt {
                    variant_name_list_id: _,
                }
                | ContextEntryDefinition::Variant { name_id: _ }
                | ContextEntryDefinition::Uninterpreted => self,
            }
        }
    }

    impl ShiftDbIndices for NormalFormId {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            amount: A,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self {
            Self::unchecked_new(self.raw().shift_with_cutoff(amount, cutoff, registry))
        }
    }

    impl ShiftDbIndices for ExpressionId {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            amount: A,
            cutoff: usize,
            registry: &mut NodeRegistry,
        ) -> Self {
            match self {
                ExpressionId::Name(name_id) => {
                    ExpressionId::Name(name_id.shift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Call(call_id) => {
                    ExpressionId::Call(call_id.shift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Fun(fun_id) => {
                    ExpressionId::Fun(fun_id.shift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Match(match_id) => {
                    ExpressionId::Match(match_id.shift_with_cutoff(amount, cutoff, registry))
                }
                ExpressionId::Forall(forall_id) => {
                    ExpressionId::Forall(forall_id.shift_with_cutoff(amount, cutoff, registry))
                }
            }
        }
    }

    impl ShiftDbIndices for NodeId<NameExpression> {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            _amount: A,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Call> {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            _amount: A,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Fun> {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            _amount: A,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Match> {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            _amount: A,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    impl ShiftDbIndices for NodeId<Forall> {
        type Output = Self;

        fn shift_with_cutoff<A: ShiftAmount>(
            self,
            _amount: A,
            _cutoff: usize,
            _registry: &mut NodeRegistry,
        ) -> Self {
            unimplemented!()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PossibleArgListId {
        Nullary,
        Some(ListId<ExpressionId>),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct AdtExpression {
        pub type_name_id: NodeId<NameExpression>,
        pub variant_name_list_id: ListId<NodeId<Identifier>>,
        pub arg_list_id: PossibleArgListId,
    }

    /// If the provided expression is has a variant at
    /// the top level,this returns IDs for the variant name
    /// and the variant's argument list.
    /// Otherwise, returns `None`.
    pub fn try_as_adt_expression(
        context: &mut Context,
        registry: &mut NodeRegistry,
        expression_id: NormalFormId,
    ) -> Option<AdtExpression> {
        match expression_id.raw() {
            ExpressionId::Name(name_id) => {
                let db_index = registry.name_expression(name_id).db_index;
                let definition = context.get_definition(db_index, registry);
                match definition {
                    ContextEntryDefinition::Adt {
                        variant_name_list_id,
                    } => Some(AdtExpression {
                        type_name_id: name_id,
                        variant_name_list_id,
                        arg_list_id: PossibleArgListId::Nullary,
                    }),
                    _ => None,
                }
            }
            ExpressionId::Call(call_id) => {
                let call = registry.call(call_id).clone();
                match call.callee_id {
                    ExpressionId::Name(name_id) => {
                        let db_index = registry.name_expression(name_id).db_index;
                        let definition = context.get_definition(db_index, registry);
                        match definition {
                            ContextEntryDefinition::Adt {
                                variant_name_list_id,
                            } => Some(AdtExpression {
                                type_name_id: name_id,
                                variant_name_list_id: variant_name_list_id,
                                arg_list_id: PossibleArgListId::Some(call.arg_list_id),
                            }),
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

use substitute::*;
mod substitute {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Substitution {
        Single {
            from: NormalFormId,
            to: NormalFormId,
        },
    }

    pub trait Substitute {
        type Output;

        fn subst(self, substitution: Substitution, registry: &mut NodeRegistry) -> Self::Output;

        fn subst_all(
            self,
            substitutions: &[Substitution],
            registry: &mut NodeRegistry,
        ) -> Self::Output
        where
            Self: Sized + Substitute<Output = Self>,
        {
            let mut result = self;
            for &subst in substitutions {
                result = result.subst(subst, registry);
            }
            result
        }
    }

    impl Substitute for ExpressionId {
        type Output = Self;

        fn subst(self, substitution: Substitution, registry: &mut NodeRegistry) -> Self {
            match self {
                ExpressionId::Name(name_id) => {
                    ExpressionId::Name(name_id.subst(substitution, registry))
                }
                ExpressionId::Call(call_id) => {
                    ExpressionId::Call(call_id.subst(substitution, registry))
                }
                ExpressionId::Fun(fun_id) => {
                    ExpressionId::Fun(fun_id.subst(substitution, registry))
                }
                ExpressionId::Match(match_id) => {
                    ExpressionId::Match(match_id.subst(substitution, registry))
                }
                ExpressionId::Forall(forall_id) => {
                    ExpressionId::Forall(forall_id.subst(substitution, registry))
                }
            }
        }
    }

    impl Substitute for NodeId<NameExpression> {
        type Output = Self;

        fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
            unimplemented!()
        }
    }

    impl Substitute for NodeId<Call> {
        type Output = Self;

        fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
            unimplemented!()
        }
    }

    impl Substitute for NodeId<Fun> {
        type Output = Self;

        fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
            unimplemented!()
        }
    }

    impl Substitute for NodeId<Match> {
        type Output = Self;

        fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
            unimplemented!()
        }
    }

    impl Substitute for NodeId<Forall> {
        type Output = Self;

        fn subst(self, _substitution: Substitution, _registry: &mut NodeRegistry) -> Self {
            unimplemented!()
        }
    }
}
