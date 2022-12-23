use crate::data::{
    fun_recursion_validation_result::FunRecursionValidated,
    light_ast::*,
    node_registry::{LabeledCallArgId, NodeId, NodeRegistry, NonEmptyListId},
    non_empty_vec::NonEmptyVec,
    type_positivity_validation_result::*,
};

use context::*;
mod context;

use misc::*;
mod misc;

pub fn validate_type_positivity_in_file(
    registry: &mut NodeRegistry,
    file_id: FunRecursionValidated<NodeId<File>>,
) -> Result<TypePositivityValidated<NodeId<File>>, TypePositivityError> {
    let mut context = Context::with_builtins();
    let file = registry.get(file_id.raw()).clone();
    let item_ids = registry.get_possibly_empty_list(file.item_list_id).to_vec();
    for &item_id in &item_ids {
        validate_type_positivity_in_file_item(&mut context, registry, item_id)?;
    }
    Ok(TypePositivityValidated::unchecked_new(file.id))
}

fn validate_type_positivity_in_file_item(
    context: &mut Context,
    registry: &mut NodeRegistry,
    item_id: FileItemNodeId,
) -> Result<(), TypePositivityError> {
    match item_id {
        FileItemNodeId::Type(type_id) => {
            validate_type_positivity_in_type_statement(context, registry, type_id)
        }
        FileItemNodeId::Let(_) => {
            context.push(ContextEntryDefinition::Uninterpreted);
            Ok(())
        }
    }
}

fn validate_type_positivity_in_type_statement(
    context: &mut Context,
    registry: &mut NodeRegistry,
    type_id: NodeId<TypeStatement>,
) -> Result<(), TypePositivityError> {
    context.push(ContextEntryDefinition::Adt(type_id));

    let type_ = registry.get(type_id);
    let variant_ids = registry
        .get_possibly_empty_list(type_.variant_list_id)
        .to_vec();
    for (variant_index, variant_id) in variant_ids.iter().copied().enumerate() {
        let target = DbIndex(variant_index);
        validate_type_positivity_in_variant(context, registry, variant_id, target)?;
    }
    Ok(())
}

fn validate_type_positivity_in_variant(
    context: &mut Context,
    registry: &mut NodeRegistry,
    variant_id: NodeId<Variant>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let variant = registry.get(variant_id).clone();
    let param_type_ids = get_possibly_empty_param_type_ids(registry, variant.param_list_id);

    for (param_index, param_id) in param_type_ids.iter().copied().enumerate() {
        let shifted_target = DbIndex(target.0 + param_index);
        validate_type_positivity_in_variant_param_type(context, registry, param_id, shifted_target);
        context.push(ContextEntryDefinition::Uninterpreted);
    }
    context.pop_n(param_type_ids.len());

    context.push(ContextEntryDefinition::Variant(variant_id));

    Ok(())
}

fn validate_type_positivity_in_variant_param_type(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_type: ExpressionId,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    match param_type {
        ExpressionId::Name(_) => Ok(()),
        ExpressionId::Fun(fun_id) => Err(TypePositivityError::ExpectedTypeGotFun(fun_id)),

        ExpressionId::Call(call_id) => {
            validate_type_positivity_in_variant_param_type_call(context, registry, call_id, target)
        }
        ExpressionId::Match(match_id) => validate_type_positivity_in_variant_param_type_match(
            context, registry, match_id, target,
        ),
        ExpressionId::Forall(forall_id) => validate_type_positivity_in_variant_param_type_forall(
            context, registry, forall_id, target,
        ),
        ExpressionId::Check(check_id) => validate_type_positivity_in_variant_param_type_check(
            context, registry, check_id, target,
        ),
    }
}

fn validate_type_positivity_in_variant_param_type_call(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_type: NodeId<Call>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    unimplemented!()
}

fn validate_type_positivity_in_variant_param_type_match(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_type: NodeId<Match>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    unimplemented!()
}

fn validate_type_positivity_in_variant_param_type_forall(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_type: NodeId<Forall>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    let forall = registry.get(param_type).clone();
    verify_that_target_does_not_appear_in_any_param_type(registry, forall.param_list_id, target)?;

    let output_target = DbIndex(target.0 + forall.param_list_id.len());
    validate_type_positivity_in_variant_param_type(
        context,
        registry,
        forall.output_id,
        output_target,
    )?;
    Ok(())
}

fn validate_type_positivity_in_variant_param_type_check(
    context: &mut Context,
    registry: &mut NodeRegistry,
    param_type: NodeId<Check>,
    target: DbIndex,
) -> Result<(), TypePositivityError> {
    unimplemented!()
}
