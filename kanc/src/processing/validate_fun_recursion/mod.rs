use crate::data::{
    fun_recursion_validation_result::*,
    light_ast::*,
    node_registry::{LabeledCallArgId, NodeId, NodeRegistry, NonEmptyListId},
    non_empty_veclike::{NonEmptyVec, OptionalNonEmptyVecLen},
    variant_return_type_validation_result::VariantReturnTypesValidated,
};

use std::convert::Infallible;

use context::*;
mod context;

type TaintedIllegalFunRecursionError = Tainted<IllegalFunRecursionError>;

impl From<Tainted<Infallible>> for TaintedIllegalFunRecursionError {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

pub fn validate_fun_recursion_in_file_items(
    registry: &mut NodeRegistry,
    file_item_list_id: VariantReturnTypesValidated<Option<NonEmptyListId<FileItemNodeId>>>,
) -> Result<FunRecursionValidated<Option<NonEmptyListId<FileItemNodeId>>>, IllegalFunRecursionError>
{
    let file_item_list_id = file_item_list_id.raw();
    let mut context = Context::new();
    let item_ids = registry
        .get_possibly_empty_list(file_item_list_id)
        .to_vec()
        .into_iter()
        .map(|item_id| validate_fun_recursion_in_file_item(&mut context, registry, item_id))
        .collect::<Result<Vec<_>, _>>()?;
    let item_list_id = registry.add_possibly_empty_list(item_ids);
    Ok(FunRecursionValidated::unchecked_new(item_list_id))
}

// pub fn validate_fun_recursion_in_file(
//     registry: &mut NodeRegistry,
//     file_id: VariantReturnTypesValidated<&'a File<'a>>,
// ) -> Result<FunRecursionValidated<&'a File<'a>>, IllegalFunRecursionError> {
//     let file_id = file_id.raw();
//     let file = registry.get(file_id).clone();
//     let mut context = Context::new();
//     let item_ids = registry
//         .get_possibly_empty_list(file.item_list_id)
//         .to_vec()
//         .into_iter()
//         .map(|item_id| validate_fun_recursion_in_file_item(&mut context, registry, item_id))
//         .collect::<Result<Vec<_>, _>>()?;
//     let item_list_id = registry.add_possibly_empty_list(item_ids);
//     Ok(FunRecursionValidated::unchecked_new(
//         registry.add_and_overwrite_id(File {
//             id: dummy_id(),
//             span: file.span,
//             file_id: file.file_id,
//             item_list_id,
//         }),
//     ))
// }

fn validate_fun_recursion_in_file_item(
    context: &mut Context,
    registry: &mut NodeRegistry,
    item_id: FileItemNodeId,
) -> Result<FileItemNodeId, IllegalFunRecursionError> {
    Ok(match item_id {
        FileItemNodeId::Type(id) => FileItemNodeId::Type(validate_fun_recursion_in_type_statement(
            context, registry, id,
        )?),
        FileItemNodeId::Let(id) => FileItemNodeId::Let(validate_fun_recursion_in_let_statement(
            context, registry, id,
        )?),
    })
}

fn validate_fun_recursion_in_type_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: &'a TypeStatement<'a>,
) -> Result<&'a TypeStatement<'a>, IllegalFunRecursionError> {
    untaint_err(
        context,
        registry,
        type_statement_id,
        validate_fun_recursion_in_type_statement_dirty,
    )
}

fn validate_fun_recursion_in_type_statement_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_statement_id: &'a TypeStatement<'a>,
) -> Result<&'a TypeStatement<'a>, TaintedIllegalFunRecursionError> {
    let type_statement = registry.get(type_statement_id).clone();
    let param_list_id = validate_fun_recursion_in_optional_params_and_leave_in_context_dirty(
        context,
        registry,
        type_statement.param_list_id,
    )?;
    context.pop_n(type_statement.param_list_id.len());

    context.push(ContextEntry::NoInformation)?;

    let variant_ids = registry
        .get_possibly_empty_list(type_statement.variant_list_id)
        .to_vec()
        .into_iter()
        .map(|variant_id| validate_fun_recursion_in_variant_dirty(context, registry, variant_id))
        .collect::<Result<Vec<_>, _>>()?;
    let variant_list_id = registry.add_possibly_empty_list(variant_ids);

    Ok(registry.add_and_overwrite_id(TypeStatement {
        id: dummy_id(),
        span: type_statement.span,
        visibility: type_statement.visibility,
        name_id: type_statement.name_id,
        param_list_id,
        variant_list_id,
    }))
}

fn validate_fun_recursion_in_variant_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: &'a Variant<'a>,
) -> Result<&'a Variant<'a>, TaintedIllegalFunRecursionError> {
    let variant = registry.get(variant_id).clone();
    let arity = variant.param_list_id.len();
    let param_list_id = validate_fun_recursion_in_optional_params_and_leave_in_context_dirty(
        context,
        registry,
        variant.param_list_id,
    )?;
    let return_type_id =
        validate_fun_recursion_in_expression_dirty(context, registry, variant.return_type_id)?;
    context.pop_n(arity);

    context.push(ContextEntry::NoInformation)?;

    Ok(registry.add_and_overwrite_id(Variant {
        id: dummy_id(),
        span: variant.span,
        name_id: variant.name_id,
        param_list_id,
        return_type_id,
    }))
}

fn validate_fun_recursion_in_let_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: &'a LetStatement<'a>,
) -> Result<&'a LetStatement<'a>, IllegalFunRecursionError> {
    untaint_err(
        context,
        registry,
        let_statement_id,
        validate_fun_recursion_in_let_statement_dirty,
    )
}

fn validate_fun_recursion_in_let_statement_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    let_statement_id: &'a LetStatement<'a>,
) -> Result<&'a LetStatement<'a>, TaintedIllegalFunRecursionError> {
    let let_statement = registry.get(let_statement_id).clone();
    let value_id =
        validate_fun_recursion_in_expression_dirty(context, registry, let_statement.value_id)?;
    context.push(ContextEntry::NoInformation)?;
    Ok(registry.add_and_overwrite_id(LetStatement {
        id: dummy_id(),
        span: let_statement.span,
        visibility: let_statement.visibility,
        transparency: let_statement.transparency,
        name_id: let_statement.name_id,
        value_id,
    }))
}

fn validate_fun_recursion_in_expression(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: ExpressionRef<'a>,
) -> Result<ExpressionRef<'a>, IllegalFunRecursionError> {
    untaint_err(
        context,
        registry,
        expression_id,
        validate_fun_recursion_in_expression_dirty,
    )
}

fn validate_fun_recursion_in_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    expression_id: ExpressionRef<'a>,
) -> Result<ExpressionRef<'a>, TaintedIllegalFunRecursionError> {
    Ok(match expression_id {
        ExpressionRef<'a>::Name(id) => {
            validate_fun_recursion_in_name_dirty(context, registry, id).map(ExpressionRef<'a>::Name)?
        }
        ExpressionRef<'a>::Todo(id) => ExpressionRef<'a>::Todo(id),
        ExpressionRef<'a>::Call(id) => {
            validate_fun_recursion_in_call_dirty(context, registry, id).map(ExpressionRef<'a>::Call)?
        }
        ExpressionRef<'a>::Fun(id) => {
            validate_fun_recursion_in_fun_dirty(context, registry, id).map(ExpressionRef<'a>::Fun)?
        }
        ExpressionRef<'a>::Match(id) => {
            validate_fun_recursion_in_match_dirty(context, registry, id).map(ExpressionRef<'a>::Match)?
        }
        ExpressionRef<'a>::Forall(id) => validate_fun_recursion_in_forall_dirty(context, registry, id)
            .map(ExpressionRef<'a>::Forall)?,
        ExpressionRef<'a>::Check(id) => {
            validate_fun_recursion_in_check_dirty(context, registry, id).map(ExpressionRef<'a>::Check)?
        }
    })
}

fn validate_fun_recursion_in_name_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    name_id: &'a NameExpression<'a>,
) -> Result<&'a NameExpression<'a>, TaintedIllegalFunRecursionError> {
    let name = registry.get(name_id);
    if context.reference_restriction(name.db_index).is_some() {
        return Err(Tainted::new(
            IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall {
                reference_id: name_id,
            },
        ));
    }
    Ok(name_id)
}

fn validate_fun_recursion_in_call_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    call_id: &'a Call<'a>,
) -> Result<&'a Call<'a>, TaintedIllegalFunRecursionError> {
    let call = registry.get(call_id).clone();

    let is_restricted = is_call_restricted(context, registry, call_id)?;

    // If the call is restricted (i.e., in the form
    // `f(x, y, ...z)`, where `f` is a Name referring to a restricted recursive function,
    // then we need to skip the callee validation (otherwise `f` will trigger
    // an error, since it is a recursive reference).
    let callee_id = if is_restricted {
        // We don't need to do any additional processing
        // (e.g., validate Check expressions)
        // since the callee is a Name expression.
        call.callee_id
    } else {
        validate_fun_recursion_in_expression_dirty(context, registry, call.callee_id)?
    };

    let arg_list_id =
        validate_fun_recursion_in_call_args_dirty(context, registry, call.arg_list_id)?;

    Ok(registry.add_and_overwrite_id(Call {
        id: dummy_id(),
        span: call.span,
        callee_id,
        arg_list_id,
    }))
}

fn is_call_restricted(
    context: &Context,
    registry: &mut NodeRegistry,
    call_id: &'a Call<'a>,
) -> Result<bool, TaintedIllegalFunRecursionError> {
    let call = registry.get(call_id).clone();
    match call.callee_id {
        ExpressionRef<'a>::Name(callee_name_id) => {
            let callee_name = registry.get(callee_name_id);
            if let Some(restriction) = context.reference_restriction(callee_name.db_index) {
                match restriction {
                    ReferenceRestriction::MustCallWithSubstruct {
                        arg_position,
                        superstruct_db_level,
                        ..
                    } => match (arg_position, call.arg_list_id) {
                        (
                            IndexOrLabel::Index(index_of_arg_that_must_be_substruct),
                            NonEmptyCallArgListId::Unlabeled(arg_list_id),
                        ) => {
                            let arg_ids = registry.get_list(arg_list_id);
                            if index_of_arg_that_must_be_substruct < arg_ids.len() {
                                let expected_substruct_id =
                                    arg_ids[index_of_arg_that_must_be_substruct];
                                let err = Err(Tainted::new(
                                    IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                        callee_id: callee_name_id,
                                        arg_id: expected_substruct_id,
                                    },
                                ));
                                match expected_substruct_id {
                                    ExpressionRef<'a>::Name(expected_substruct_name_id) => {
                                        let expected_substruct =
                                            registry.get(expected_substruct_name_id);
                                        let expected_substruct_db_level =
                                            context.index_to_level(expected_substruct.db_index);
                                        if !context.is_left_strict_substruct_of_right(
                                            expected_substruct_db_level,
                                            superstruct_db_level,
                                        ) {
                                            return err;
                                        }
                                    }
                                    _ => return err,
                                }
                            }
                        }
                        (
                            IndexOrLabel::LabelId(
                                identifier_id_of_label_of_arg_that_must_be_substruct,
                            ),
                            NonEmptyCallArgListId::UniquelyLabeled(arg_list_id),
                        ) => {
                            let label_name_of_arg_that_must_be_substruct = registry
                                .get(identifier_id_of_label_of_arg_that_must_be_substruct)
                                .name
                                .clone();
                            let arg_ids = registry.get_list(arg_list_id);

                            let id_of_arg_that_is_supposed_to_be_substruct = arg_ids
                                .iter()
                                .find(|arg_id| {
                                    let label_name: &IdentifierName = match arg_id {
                                        LabeledCallArgId::Implicit {
                                            label_id: value_id, ..
                                        } => {
                                            let label_id = *value_id;
                                            &registry.get(label_id).name
                                        }
                                        LabeledCallArgId::Explicit { label_id, .. } => {
                                            &registry.get(*label_id).name
                                        }
                                    };
                                    *label_name == label_name_of_arg_that_must_be_substruct
                                })
                                .copied();
                            if let Some(id_of_arg_that_is_supposed_to_be_substruct) =
                                id_of_arg_that_is_supposed_to_be_substruct
                            {
                                match id_of_arg_that_is_supposed_to_be_substruct {
                                    LabeledCallArgId::Implicit { db_index, .. } => {
                                        let expected_substruct_db_level =
                                            context.index_to_level(db_index);
                                        if !context.is_left_strict_substruct_of_right(
                                            expected_substruct_db_level,
                                            superstruct_db_level,
                                        ) {
                                            let value_id =
                                                id_of_arg_that_is_supposed_to_be_substruct
                                                    .value_id();
                                            let err = Err(Tainted::new(
                                                IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                                    callee_id: callee_name_id,
                                                    arg_id: value_id,
                                                },
                                            ));
                                            return err;
                                        }
                                    }
                                    LabeledCallArgId::Explicit { value_id, .. } => {
                                        let err = Err(Tainted::new(
                                            IllegalFunRecursionError::NonSubstructPassedToDecreasingParam {
                                                callee_id: callee_name_id,
                                                arg_id: value_id,
                                            },
                                        ));
                                        match value_id {
                                            ExpressionRef<'a>::Name(expected_substruct_name_id) => {
                                                let expected_substruct =
                                                    registry.get(expected_substruct_name_id);
                                                let expected_substruct_db_level = context
                                                    .index_to_level(expected_substruct.db_index);
                                                if !context.is_left_strict_substruct_of_right(
                                                    expected_substruct_db_level,
                                                    superstruct_db_level,
                                                ) {
                                                    return err;
                                                }
                                            }
                                            _ => return err,
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            return Err(Tainted::new(
                                IllegalFunRecursionError::LabelednessMismatch(call_id),
                            ))
                        }
                    },
                    ReferenceRestriction::CannotCall { .. } => return Err(Tainted::new(
                        IllegalFunRecursionError::RecursivelyCalledFunctionWithoutDecreasingParam {
                            callee_id: callee_name.id,
                        },
                    )),
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _ => Ok(false),
    }
}

fn validate_fun_recursion_in_call_args_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    arg_list_id: NonEmptyCallArgListId,
) -> Result<NonEmptyCallArgListId, TaintedIllegalFunRecursionError> {
    Ok(match arg_list_id {
        NonEmptyCallArgListId::Unlabeled(id) => NonEmptyCallArgListId::Unlabeled(
            validate_fun_recursion_in_expression_list_dirty(context, registry, id)?,
        ),
        NonEmptyCallArgListId::UniquelyLabeled(id) => NonEmptyCallArgListId::UniquelyLabeled(
            validate_fun_recursion_in_labeled_call_arg_list_dirty(context, registry, id)?,
        ),
    })
}

fn validate_fun_recursion_in_expression_list_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    list_id: NonEmptyListId<ExpressionRef<'a>>,
) -> Result<NonEmptyListId<ExpressionRef<'a>>, TaintedIllegalFunRecursionError> {
    let expression_ids = registry
        .get_list(list_id)
        .to_non_empty_vec()
        .try_into_mapped(|id| validate_fun_recursion_in_expression_dirty(context, registry, id))?;
    Ok(registry.add_list(expression_ids))
}

fn validate_fun_recursion_in_labeled_call_arg_list_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    list_id: NonEmptyListId<LabeledCallArgId>,
) -> Result<NonEmptyListId<LabeledCallArgId>, TaintedIllegalFunRecursionError> {
    let expression_ids = registry
        .get_list(list_id)
        .to_non_empty_vec()
        .try_into_mapped(|id| {
            validate_fun_recursion_in_labeled_call_arg_dirty(context, registry, id)
        })?;
    Ok(registry.add_list(expression_ids))
}

fn validate_fun_recursion_in_labeled_call_arg_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    arg_id: LabeledCallArgId,
) -> Result<LabeledCallArgId, TaintedIllegalFunRecursionError> {
    Ok(match arg_id {
        LabeledCallArgId::Implicit {
            label_id,
            db_index,
            value_id,
        } => {
            if context.reference_restriction(db_index).is_some() {
                let span = registry.get(label_id).span;
                let component_list_id = registry.add_list(NonEmptyVec::singleton(label_id));
                let name_id = registry.add_and_overwrite_id(NameExpression {
                    id: dummy_id(),
                    span,
                    component_list_id,
                    db_index,
                });
                return Err(Tainted::new(
                    IllegalFunRecursionError::RecursiveReferenceWasNotDirectCall {
                        reference_id: name_id,
                    },
                ));
            }
            LabeledCallArgId::Implicit {
                label_id,
                db_index,
                value_id,
            }
        }
        LabeledCallArgId::Explicit { label_id, value_id } => LabeledCallArgId::Explicit {
            label_id,
            value_id: validate_fun_recursion_in_expression_dirty(context, registry, value_id)?,
        },
    })
}

fn validate_fun_recursion_in_fun_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    fun_id: &'a Fun<'a>,
) -> Result<&'a Fun<'a>, TaintedIllegalFunRecursionError> {
    let fun = registry.get(fun_id).clone();
    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context_dirty(
        context,
        registry,
        fun.param_list_id,
    )?;
    let return_type_id =
        validate_fun_recursion_in_expression_dirty(context, registry, fun.return_type_id)?;

    let reference_restriction = match fun.param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => {
            let param_ids = registry.get_list(param_list_id);
            let decreasing_param_position = param_ids.iter().position(|param_id| {
                let param = registry.get(*param_id);
                param.is_dashed
            });
            match decreasing_param_position {
                Some(param_position) => {
                    let superstruct_db_index = DbIndex(param_ids.len() - param_position - 1);
                    let superstruct_db_level = context.index_to_level(superstruct_db_index);
                    ReferenceRestriction::MustCallWithSubstruct {
                        superstruct_db_level,
                        arg_position: IndexOrLabel::Index(param_position),
                    }
                }
                None => ReferenceRestriction::CannotCall,
            }
        }
        NonEmptyParamListId::UniquelyLabeled(param_list_id) => {
            let param_ids = registry.get_list(param_list_id);
            let decreasing_param_info =
                param_ids
                    .iter()
                    .enumerate()
                    .find_map(|(param_index, param_id)| {
                        let param = registry.get(*param_id);
                        if param.is_dashed {
                            Some(match param.label_id {
                                ParamLabelId::Explicit(label_id) => (param_index, label_id),
                                ParamLabelId::Implicit => (param_index, param.name_id),
                            })
                        } else {
                            None
                        }
                    });
            match decreasing_param_info {
                Some((decreasing_param_index, decreasing_param_label_id)) => {
                    let superstruct_db_index =
                        DbIndex(param_ids.len() - decreasing_param_index - 1);
                    let superstruct_db_level = context.index_to_level(superstruct_db_index);
                    ReferenceRestriction::MustCallWithSubstruct {
                        superstruct_db_level,
                        arg_position: IndexOrLabel::LabelId(decreasing_param_label_id),
                    }
                }
                None => ReferenceRestriction::CannotCall,
            }
        }
    };

    context.push(ContextEntry::Fun(reference_restriction))?;
    let body_id = validate_fun_recursion_in_expression_dirty(context, registry, fun.body_id)?;
    context.pop_n(param_list_id.len() + 1);

    Ok(registry.add_and_overwrite_id(Fun {
        id: dummy_id(),
        span: fun.span,
        name_id: fun.name_id,
        param_list_id,
        return_type_id,
        body_id,
    }))
}

fn validate_fun_recursion_in_optional_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: Option<NonEmptyParamListId>,
) -> Result<Option<NonEmptyParamListId>, TaintedIllegalFunRecursionError> {
    param_list_id
        .map(|param_list_id| {
            validate_fun_recursion_in_params_and_leave_in_context_dirty(
                context,
                registry,
                param_list_id,
            )
        })
        .transpose()
}

fn validate_fun_recursion_in_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: NonEmptyParamListId,
) -> Result<NonEmptyParamListId, TaintedIllegalFunRecursionError> {
    Ok(match param_list_id {
        NonEmptyParamListId::Unlabeled(param_list_id) => NonEmptyParamListId::Unlabeled(
            validate_fun_recursion_in_unlabeled_params_and_leave_in_context_dirty(
                context,
                registry,
                param_list_id,
            )?,
        ),
        NonEmptyParamListId::UniquelyLabeled(param_list_id) => {
            NonEmptyParamListId::UniquelyLabeled(
                validate_fun_recursion_in_labeled_params_and_leave_in_context_dirty(
                    context,
                    registry,
                    param_list_id,
                )?,
            )
        }
    })
}

fn validate_fun_recursion_in_unlabeled_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: NonEmptyListId<&'a UnlabeledParam<'a>>,
) -> Result<NonEmptyListId<&'a UnlabeledParam<'a>>, TaintedIllegalFunRecursionError> {
    let param_ids = registry
        .get_list(param_list_id)
        .to_non_empty_vec()
        .try_into_mapped(|param_id| -> Result<_, TaintedIllegalFunRecursionError> {
            let param = registry.get(param_id).clone();
            let type_id =
                validate_fun_recursion_in_expression_dirty(context, registry, param.type_id)?;
            context.push(ContextEntry::NoInformation)?;
            Ok(registry.add_and_overwrite_id(UnlabeledParam {
                id: dummy_id(),
                span: param.span,
                name_id: param.name_id,
                type_id,
                is_dashed: param.is_dashed,
            }))
        })?;

    Ok(registry.add_list(param_ids))
}

fn validate_fun_recursion_in_labeled_params_and_leave_in_context_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_list_id: NonEmptyListId<&'a LabeledParam<'a>>,
) -> Result<NonEmptyListId<&'a LabeledParam<'a>>, TaintedIllegalFunRecursionError> {
    let param_ids = registry
        .get_list(param_list_id)
        .to_non_empty_vec()
        .try_into_mapped(|param_id| -> Result<_, TaintedIllegalFunRecursionError> {
            let param = registry.get(param_id).clone();
            let type_id =
                validate_fun_recursion_in_expression_dirty(context, registry, param.type_id)?;
            context.push(ContextEntry::NoInformation)?;
            Ok(registry.add_and_overwrite_id(LabeledParam {
                id: dummy_id(),
                span: param.span,
                label_clause: param.label_clause,
                name_id: param.name_id,
                type_id,
                is_dashed: param.is_dashed,
            }))
        })?;

    Ok(registry.add_list(param_ids))
}

fn validate_fun_recursion_in_match_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    match_id: &'a Match<'a>,
) -> Result<&'a Match<'a>, TaintedIllegalFunRecursionError> {
    let match_ = registry.get(match_id).clone();
    let matchee_id =
        validate_fun_recursion_in_expression_dirty(context, registry, match_.matchee_id)?;
    let matchee_db_index = match match_.matchee_id {
        ExpressionRef<'a>::Name(matchee_name_id) => {
            let matchee_name = registry.get(matchee_name_id);
            Some(matchee_name.db_index)
        }
        _ => None,
    };

    let case_ids = registry
        .get_possibly_empty_list(match_.case_list_id)
        .to_vec()
        .into_iter()
        .map(|case_id| {
            validate_fun_recursion_in_match_case_dirty(context, registry, case_id, matchee_db_index)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let case_list_id = registry.add_possibly_empty_list(case_ids);

    Ok(registry.add_and_overwrite_id(Match {
        id: dummy_id(),
        span: match_.span,
        matchee_id,
        case_list_id,
    }))
}

fn validate_fun_recursion_in_match_case_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    case_id: &'a MatchCase<'a>,
    matchee_db_index: Option<DbIndex>,
) -> Result<&'a MatchCase<'a>, TaintedIllegalFunRecursionError> {
    let case = registry.get(case_id).clone();
    let case_arity = case.param_list_id.len();

    if let Some(matchee_db_index) = matchee_db_index {
        let matchee_db_level = context.index_to_level(matchee_db_index);
        for _ in 0..case_arity {
            context.push(ContextEntry::Substruct {
                superstruct_db_level: matchee_db_level,
            })?;
        }
    } else {
        for _ in 0..case_arity {
            context.push(ContextEntry::NoInformation)?;
        }
    }

    // We don't need to do any validation since it's just a
    // list of identifiers.
    let param_list_id = case.param_list_id;

    let output_id =
        validate_fun_recursion_in_match_case_output_dirty(context, registry, case.output_id)?;
    context.pop_n(case_arity);

    Ok(registry.add_and_overwrite_id(MatchCase {
        id: dummy_id(),
        span: case.span,
        variant_name_id: case.variant_name_id,
        param_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_match_case_output_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: MatchCaseOutputId,
) -> Result<MatchCaseOutputId, TaintedIllegalFunRecursionError> {
    Ok(match id {
        MatchCaseOutputId::Some(id) => MatchCaseOutputId::Some(
            validate_fun_recursion_in_expression_dirty(context, registry, id)?,
        ),
        MatchCaseOutputId::ImpossibilityClaim(kw_span) => {
            MatchCaseOutputId::ImpossibilityClaim(kw_span)
        }
    })
}

fn validate_fun_recursion_in_forall_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    forall_id: &'a Forall<'a>,
) -> Result<&'a Forall<'a>, TaintedIllegalFunRecursionError> {
    let forall = registry.get(forall_id).clone();
    let arity = forall.param_list_id.len();

    let param_list_id = validate_fun_recursion_in_params_and_leave_in_context_dirty(
        context,
        registry,
        forall.param_list_id,
    )?;
    let output_id =
        validate_fun_recursion_in_expression_dirty(context, registry, forall.output_id)?;
    context.pop_n(arity);

    Ok(registry.add_and_overwrite_id(Forall {
        id: dummy_id(),
        span: forall.span,
        param_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_check_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    check_id: &'a Check<'a>,
) -> Result<&'a Check<'a>, TaintedIllegalFunRecursionError> {
    let check = registry.get(check_id).clone();
    let assertion_list_id = validate_fun_recursion_in_check_assertions_dirty(
        context,
        registry,
        check.assertion_list_id,
    )?;
    let output_id = validate_fun_recursion_in_expression_dirty(context, registry, check.output_id)?;
    Ok(registry.add_and_overwrite_id(Check {
        id: dummy_id(),
        span: check.span,
        assertion_list_id,
        output_id,
    }))
}

fn validate_fun_recursion_in_check_assertions_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: NonEmptyListId<&'a CheckAssertion<'a>>,
) -> Result<NonEmptyListId<&'a CheckAssertion<'a>>, TaintedIllegalFunRecursionError> {
    let assertion_ids =
        registry
            .get_list(id)
            .to_non_empty_vec()
            .try_into_mapped(|assertion_id| {
                validate_fun_recursion_in_check_assertion_dirty(context, registry, assertion_id)
            })?;
    Ok(registry.add_list(assertion_ids))
}

fn validate_fun_recursion_in_check_assertion_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: &'a CheckAssertion<'a>,
) -> Result<&'a CheckAssertion<'a>, TaintedIllegalFunRecursionError> {
    let assertion = registry.get(id).clone();
    let left_id = validate_fun_recursion_in_goal_kw_or_expression_dirty(
        context,
        registry,
        assertion.left_id,
    )?;
    let right_id = validate_fun_recursion_in_question_mark_or_possibly_invalid_expression_dirty(
        context,
        registry,
        assertion.right_id,
    )?;
    Ok(registry.add_and_overwrite_id(CheckAssertion {
        id: dummy_id(),
        span: assertion.span,
        kind: assertion.kind,
        left_id,
        right_id,
    }))
}

fn validate_fun_recursion_in_goal_kw_or_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: GoalKwOrPossiblyInvalidExpressionId,
) -> Result<GoalKwOrPossiblyInvalidExpressionId, TaintedIllegalFunRecursionError> {
    Ok(match id {
        GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start } => {
            GoalKwOrPossiblyInvalidExpressionId::GoalKw { span: start }
        }
        GoalKwOrPossiblyInvalidExpressionId::Expression(id) => {
            GoalKwOrPossiblyInvalidExpressionId::Expression(
                validate_fun_recursion_in_possibly_invalid_expression_dirty(context, registry, id),
            )
        }
    })
}

fn validate_fun_recursion_in_question_mark_or_possibly_invalid_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    id: QuestionMarkOrPossiblyInvalidExpressionId,
) -> Result<QuestionMarkOrPossiblyInvalidExpressionId, TaintedIllegalFunRecursionError> {
    Ok(match id {
        QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start } => {
            QuestionMarkOrPossiblyInvalidExpressionId::QuestionMark { span: start }
        }
        QuestionMarkOrPossiblyInvalidExpressionId::Expression(id) => {
            QuestionMarkOrPossiblyInvalidExpressionId::Expression(
                validate_fun_recursion_in_possibly_invalid_expression_dirty(context, registry, id),
            )
        }
    })
}

fn validate_fun_recursion_in_possibly_invalid_expression_dirty(
    context: &mut Context,
    registry: &mut NodeRegistry,
    original_id: PossiblyInvalidExpressionId,
) -> PossiblyInvalidExpressionId {
    match original_id {
        PossiblyInvalidExpressionId::Invalid(original_id) => {
            PossiblyInvalidExpressionId::Invalid(original_id)
        }
        PossiblyInvalidExpressionId::Valid(original_id) => {
            // We have to use `validate_fun_recursion_in_expression`
            // instead of `validate_fun_recursion_in_expression_dirty`
            // because we have to keep the context clean even in the case
            // where `validation_result` is `Err` (since we'll still ultimately
            // return `Ok`)
            let validation_result =
                validate_fun_recursion_in_expression(context, registry, original_id);
            match validation_result {
                Ok(validated_id) => PossiblyInvalidExpressionId::Valid(validated_id),
                Err(err) => {
                    PossiblyInvalidExpressionId::Invalid(InvalidExpressionId::IllegalFunRecursion(
                        registry.add_and_overwrite_id(IllegalFunRecursionExpression {
                            id: dummy_id(),
                            expression_id: original_id,
                            error: err,
                            span_invalidated: false,
                        }),
                    ))
                }
            }
        }
    }
}

fn dummy_id<T>() -> &'a T<'a> {
    NodeId::new(0)
}
