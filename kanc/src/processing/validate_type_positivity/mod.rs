use crate::data::{
    fun_recursion_validation_result::FunRecursionValidated, light_ast::*,
    type_positivity_validation_result::*,
};

use context::*;
mod context;

use trust_cache::*;
mod trust_cache;

use misc::*;
mod misc;

// TODO: Clean up design.
//
// Currently, this module functions in a "tainted" way.
// No untainting occurs, so as soon as a function that takes
// mutable state
// (which usually happens to be `&mut Context` and `&mut TrustCache`,
// to be specific)
// and returns an error, that mutable state is left in a "tainted"
// state, meaning most of the normal invariants may no longer hold.

pub fn validate_type_positivity_in_file_items(
    registry: &NodeRegistry,
    file_item_list_id: FunRecursionValidated<Option<NonEmptyListId<FileItemNodeId>>>,
) -> Result<TypePositivityValidated<Option<NonEmptyListId<FileItemNodeId>>>, TypePositivityError> {
    let file_item_list_id = file_item_list_id.raw();
    let mut context = Context::with_builtins();
    let mut cache = TrustCache::empty();
    let item_ids = registry.get_possibly_empty_list(file_item_list_id).to_vec();
    for &item_id in &item_ids {
        validate_type_positivity_in_file_item(&mut context, &mut cache, registry, item_id)?;
    }
    Ok(TypePositivityValidated::unchecked_new(file_item_list_id))
}

fn validate_type_positivity_in_file_item(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    item_id: FileItemNodeId,
) -> Result<(), TypePositivityError> {
    match item_id {
        FileItemNodeId::Type(type_id) => {
            validate_type_positivity_in_type_statement(context, cache, registry, type_id)
        }
        FileItemNodeId::Let(_) => {
            context.push(ContextEntryDefinition::Uninterpreted);
            Ok(())
        }
    }
}

fn validate_type_positivity_in_type_statement(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    type_id: &'a TypeStatement<'a>,
) -> Result<(), TypePositivityError> {
    context.push(ContextEntryDefinition::Adt(type_id));

    let type_ = registry.get(type_id);
    let variant_ids = registry
        .get_possibly_empty_list(type_.variant_list_id)
        .to_vec();
    for (variant_index, variant_id) in variant_ids.iter().copied().enumerate() {
        let target = DbIndex(variant_index);
        validate_type_positivity_in_variant(context, cache, registry, variant_id, target)?;
    }
    Ok(())
}

fn validate_type_positivity_in_variant(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    variant_id: &'a Variant<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let variant = registry.get(variant_id);
    let param_type_ids = get_possibly_empty_param_type_ids(registry, variant.param_list_id);

    for (param_index, param_type_id) in param_type_ids.iter().copied().enumerate() {
        let shifted_target = DbIndex(target.0 + param_index);
        validate_type_positivity_in_expression(
            context,
            cache,
            registry,
            param_type_id,
            shifted_target,
        )?;
        context.push(ContextEntryDefinition::Uninterpreted);
    }
    context.pop_n(param_type_ids.len());

    context.push(ContextEntryDefinition::Variant(variant_id));

    Ok(())
}

fn validate_type_positivity_in_expression(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: ExpressionRef<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    match id {
        ExpressionRef::Name(_) => Ok(()),
        ExpressionRef::Todo(_) => Ok(()),
        ExpressionRef::Fun(fun_id) => Err(TypePositivityError::ExpectedTypeGotFun(fun_id)),

        ExpressionRef::Call(call_id) => {
            validate_type_positivity_in_call(context, cache, registry, call_id, target)
        }
        ExpressionRef::Match(match_id) => {
            validate_type_positivity_in_match(context, cache, registry, match_id, target)
        }
        ExpressionRef::Forall(forall_id) => {
            validate_type_positivity_in_forall(context, cache, registry, forall_id, target)
        }
        ExpressionRef::Check(check_id) => {
            validate_type_positivity_in_check_expression(context, cache, registry, check_id, target)
        }
    }
}

fn validate_type_positivity_in_call(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    call_id: &'a Call<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    if !does_target_appear_in_expression(registry, ExpressionRef::Call(call_id), target) {
        return Ok(());
    }

    let call = registry.get(call_id);

    let ExpressionRef::Name(callee_id) = call.callee_id else {
        return Err(TypePositivityError::NonAdtCallee{
            call_id,
            callee_id: call.callee_id,
        });
    };

    let callee = registry.get(callee_id);
    let ContextEntryDefinition::Adt(callee_def_id) = context.get_definition(callee.db_index) else {
        return Err(TypePositivityError::NonAdtCallee{
            call_id,
            callee_id: call.callee_id,
        });
    };
    let callee_def_id = *callee_def_id;

    let indices_of_appearance: Vec<usize> = get_arg_values(registry, call.arg_list_id)
        .into_iter()
        .enumerate()
        .filter_map(|(index, arg_value_id)| {
            if does_target_appear_in_expression(registry, arg_value_id, target) {
                Some(index)
            } else {
                None
            }
        })
        .collect();

    let mut shortened_context = context.clone_up_to_excl(callee.db_index);

    for param_index in indices_of_appearance {
        verify_type_param_is_positive(
            &mut shortened_context,
            cache,
            registry,
            TypeParam {
                type_id: callee_def_id,
                param_index,
            },
        )?;
    }

    Ok(())
}

fn validate_type_positivity_in_match(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: &'a Match<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let match_ = registry.get(id);

    verify_that_target_does_not_appear_in_expression(registry, match_.matchee_id, target)?;

    validate_type_positivity_in_match_case_outputs(
        context,
        cache,
        registry,
        match_.case_list_id,
        target,
    )?;

    Ok(())
}

fn validate_type_positivity_in_match_case_outputs(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: Option<NonEmptyListId<&'a MatchCase<'a>>>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let case_ids = registry.get_possibly_empty_list(id).to_vec();
    for case_id in case_ids {
        validate_type_positivity_in_match_case(context, cache, registry, case_id, target)?;
    }
    Ok(())
}

fn validate_type_positivity_in_match_case(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: &'a MatchCase<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let case = registry.get(id);
    let explicit_arity = case
        .param_list_id
        .map(|list_id| list_id.explicit_len())
        .unwrap_or(0);
    let output_target = DbIndex(target.0 + explicit_arity);
    validate_type_positivity_in_match_case_output(
        context,
        cache,
        registry,
        case.output_id,
        output_target,
    )
}

fn validate_type_positivity_in_match_case_output(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: MatchCaseOutputId,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    match id {
        MatchCaseOutputId::Some(id) => {
            validate_type_positivity_in_expression(context, cache, registry, id, target)
        }
        MatchCaseOutputId::ImpossibilityClaim(_) => Ok(()),
    }
}

fn validate_type_positivity_in_forall(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: &'a Forall<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let forall = registry.get(id);
    let arity = forall.param_list_id.len();

    verify_that_target_does_not_appear_in_any_param_type(registry, forall.param_list_id, target)?;
    context.push_n_uninterpreted(arity);

    let output_target = DbIndex(target.0 + forall.param_list_id.len());
    validate_type_positivity_in_expression(
        context,
        cache,
        registry,
        forall.output_id,
        output_target,
    )?;

    context.pop_n(arity);

    Ok(())
}

fn validate_type_positivity_in_check_expression(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    id: &'a Check<'a>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let check = registry.get(id);
    validate_type_positivity_in_expression(context, cache, registry, check.output_id, target)
}

fn verify_type_param_is_positive(
    context_not_including_current_type_statement: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    param: TypeParam,
) -> Result<(), TypePositivityError> {
    if cache.is_trusted(param) {
        return Ok(());
    }

    cache.trust(param);

    let context = context_not_including_current_type_statement;
    let TypeParam {
        type_id,
        param_index,
    } = param;

    context.push(ContextEntryDefinition::Adt(type_id));

    let type_ = registry.get(type_id);
    let type_param_arity = type_.param_list_id.len();
    let variant_ids = registry
        .get_possibly_empty_list(type_.variant_list_id)
        .to_vec();

    for variant_id in variant_ids {
        verify_type_param_i_is_positive_in_variant(
            context,
            cache,
            registry,
            variant_id,
            param_index,
            type_param_arity,
        )?;
    }

    Ok(())
}

fn verify_type_param_i_is_positive_in_variant(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    variant_id: &'a Variant<'a>,
    i: usize,
    type_param_arity: usize,
) -> Result<(), TypePositivityError> {
    verify_type_param_i_is_positive_in_variant_without_pushing(
        context,
        cache,
        registry,
        variant_id,
        i,
        type_param_arity,
    )?;

    context.push(ContextEntryDefinition::Variant(variant_id));

    Ok(())
}

fn verify_type_param_i_is_positive_in_variant_without_pushing(
    context: &mut Context,
    cache: &mut TrustCache,
    registry: &NodeRegistry,
    variant_id: &'a Variant<'a>,
    i: usize,
    type_param_arity: usize,
) -> Result<(), TypePositivityError> {
    let target_index = i;
    let variant = registry.get(variant_id);
    let variant_arity = variant.param_list_id.len();

    let variant_return_type_id = match variant.return_type_id {
        ExpressionRef::Name(_) => {
            return Err(TypePositivityError::VariantReturnTypeTypeArgArityMismatch {
                actual: 0,
                expected: type_param_arity,
                return_type_id: variant.return_type_id,
            });
        }
        ExpressionRef::Call(variant_return_type_id) => {
            variant_return_type_id
        }
        _ => panic!("Impossible: The variant return type validator should have thrown an error on any return type that was neither a Name nor Call.")
    };
    let variant_return_type = registry.get(variant_return_type_id);

    let Some(type_arg_id) = get_ith_call_arg_value(registry, target_index, variant_return_type.arg_list_id) else {
        return Err(TypePositivityError::VariantReturnTypeTypeArgArityMismatch {
            actual: 0,
            expected: type_param_arity,
            return_type_id: variant.return_type_id,
        })
    };

    let does_any_variant_param_appear_in_type_arg =
        (0..variant_arity).into_iter().any(|raw_index| {
            let variant_param_db_index = DbIndex(raw_index);
            !does_target_appear_in_expression(registry, type_arg_id, variant_param_db_index)
        });
    if !does_any_variant_param_appear_in_type_arg {
        return Ok(());
    }

    let ExpressionRef::Name(type_arg_id) = type_arg_id else {
        // TODO: Enable "stack trace" (e.g., so we can see the original
        // type that required the variant return type to have a positive
        // type arg).
        return Err(TypePositivityError::VariantReturnTypeHadNonNameTypeArg {
            variant_id,
            type_arg_index: target_index,
        });
    };
    let type_arg_db_index_relative_to_return_type = registry.get(type_arg_id).db_index;

    let param_type_ids = get_possibly_empty_param_type_ids(registry, variant.param_list_id);
    for (param_index, param_type_id) in param_type_ids.iter().copied().enumerate() {
        let Some(shifted_type_arg_db_index) = (type_arg_db_index_relative_to_return_type.0 + param_index)
            .checked_sub(variant_arity) else {
                context.push(ContextEntryDefinition::Uninterpreted);
                continue;
            };
        let shifted_type_arg_db_index = DbIndex(shifted_type_arg_db_index);
        validate_type_positivity_in_expression(
            context,
            cache,
            registry,
            param_type_id,
            shifted_type_arg_db_index,
        )?;

        context.push(ContextEntryDefinition::Uninterpreted);
    }

    Ok(())
}
