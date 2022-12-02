use crate::data::{
    light_ast::*,
    node_registry::{NodeId, NodeRegistry},
};

#[derive(Clone, Debug)]
pub struct IllegalVariantReturnTypeError(pub ExpressionId);

pub fn validate_variant_return_types_in_file(
    registry: &NodeRegistry,
    file: &File,
) -> Result<(), IllegalVariantReturnTypeError> {
    let item_ids = registry.file_item_list(file.item_list_id);
    for item_id in item_ids {
        if let FileItemNodeId::Type(type_id) = item_id {
            let type_statement = registry.type_statement(*type_id);
            validate_variant_return_types_in_type_statement(registry, type_statement)?;
        }
    }
    Ok(())
}

fn validate_variant_return_types_in_type_statement(
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
) -> Result<(), IllegalVariantReturnTypeError> {
    let variant_ids = registry.variant_list(type_statement.variant_list_id);
    for (variant_index, variant_id) in variant_ids.iter().copied().enumerate() {
        let variant = registry.variant(variant_id);
        validate_return_type_of_variant(registry, variant, variant_index)?;
    }
    Ok(())
}

fn validate_return_type_of_variant(
    registry: &NodeRegistry,
    variant: &Variant,
    variant_index: usize,
) -> Result<(), IllegalVariantReturnTypeError> {
    fn validate_return_type_name_db_index(
        return_type_name_id: NodeId<NameExpression>,
        (registry, return_type_id, variant, variant_index): (
            &NodeRegistry,
            ExpressionId,
            &Variant,
            usize,
        ),
    ) -> Result<(), IllegalVariantReturnTypeError> {
        let adjusted_type_statement_db_index = DbIndex(variant_index + variant.param_list_id.len);
        let return_db_index = registry.name_expression(return_type_name_id).db_index;
        if adjusted_type_statement_db_index == return_db_index {
            Ok(())
        } else {
            Err(IllegalVariantReturnTypeError(return_type_id))
        }
    }

    let return_type_id = variant.return_type_id;
    match return_type_id {
        ExpressionId::Name(name_id) => validate_return_type_name_db_index(
            name_id,
            (registry, return_type_id, variant, variant_index),
        ),
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id);
            match call.callee_id {
                ExpressionId::Name(name_id) => validate_return_type_name_db_index(
                    name_id,
                    (registry, return_type_id, variant, variant_index),
                ),
                _other_callee => Err(IllegalVariantReturnTypeError(return_type_id)),
            }
        }
        _other_variant_return_type => Err(IllegalVariantReturnTypeError(return_type_id)),
    }
}
