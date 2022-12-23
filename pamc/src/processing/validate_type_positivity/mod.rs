use crate::data::{
    fun_recursion_validation_result::FunRecursionValidated,
    light_ast::*,
    node_registry::{NodeId, NodeRegistry},
    type_positivity_validation_result::*,
};

use std::convert::Infallible;

use context::*;
mod context;

type TaintedTypePositivityError = Tainted<TypePositivityError>;

impl From<Tainted<Infallible>> for TaintedTypePositivityError {
    fn from(impossible: Tainted<Infallible>) -> Self {
        #[allow(unreachable_code)]
        match Infallible::from(impossible) {}
    }
}

pub fn validate_type_positivity_in_file(
    registry: &NodeRegistry,
    file_id: FunRecursionValidated<NodeId<File>>,
) -> Result<TypePositivityValidated<NodeId<File>>, TypePositivityError> {
    let mut context = Context::with_builtins();
    let file = registry.get(file_id.raw());
    let item_ids = registry.get_possibly_empty_list(file.item_list_id);
    for &item_id in item_ids {
        validate_type_positivity_in_file_item(&mut context, registry, item_id)?;
    }
    Ok(TypePositivityValidated::unchecked_new(file.id))
}

fn validate_type_positivity_in_file_item(
    context: &mut Context,
    registry: &NodeRegistry,
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
    registry: &NodeRegistry,
    type_id: NodeId<TypeStatement>,
) -> Result<(), TypePositivityError> {
    context.push(ContextEntryDefinition::Adt(type_id));

    let type_ = registry.get(type_id);
    let variant_ids = registry.get_possibly_empty_list(type_.variant_list_id);
    for &variant_id in variant_ids {
        validate_type_positivity_in_variant(context, registry, variant_id)?;
    }
    Ok(())
}

fn validate_type_positivity_in_variant(
    context: &mut Context,
    registry: &NodeRegistry,
    variant_id: NodeId<Variant>,
) -> Result<(), TypePositivityError> {
    let variant = registry.get(variant_id);

    todo!();

    context.push(ContextEntryDefinition::Variant(variant_id));

    Ok(())
}

fn dummy_id<T>() -> NodeId<T> {
    NodeId::new(0)
}
