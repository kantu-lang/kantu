use crate::data::{
    light_ast::*,
    node_registry::{FileItemNodeId, NodeId, NodeRegistry, NonEmptyListId},
    non_empty_vec::OptionalNonEmptyVecLen,
    variant_return_type_validation_result::*,
};

/// For a given type `T` with type parameters `A_1, ..., A_n`,
/// every one of its variant's return type must be `T(x_1, ..., x_n)` for
/// some expressions `x_1, ..., x_n`.
/// A return type that is not of this form is considered _invalid_.
/// This function returns `Err` iff any variant's return type is invalid.
///
/// ### Return type validity examples:
///
/// **Invalid:**
///
/// ```kantu
/// type Bool {
///    .True: Nat,
///    .False: Nat,
/// }
/// ```
/// Since `Bool.True` and `Bool.False`
/// are variants of `Bool`, they can only return a `Bool`.
/// Since they return a `Nat` here, this example is invalid.
///
/// **Valid:**
///
/// ```kantu
/// type Bool {
///   .True: Bool,
///   .False: Bool,
/// }
/// ```
/// This is valid because since `Bool.True` and `Bool.False` both
/// return a `Bool`, which is the type they are variants of.
pub fn validate_variant_return_types_in_file_items(
    registry: &NodeRegistry,
    file_item_list_id: Option<NonEmptyListId<FileItemNodeId>>,
) -> Result<
    VariantReturnTypesValidated<Option<NonEmptyListId<FileItemNodeId>>>,
    IllegalVariantReturnTypeError,
> {
    let item_ids = registry.get_possibly_empty_list(file_item_list_id);
    for item_id in item_ids {
        if let FileItemNodeId::Type(type_id) = item_id {
            let type_statement = registry.get(*type_id);
            validate_variant_return_types_in_type_statement(registry, type_statement)?;
        }
    }
    Ok(VariantReturnTypesValidated::unchecked_new(
        file_item_list_id,
    ))
}

fn validate_variant_return_types_in_type_statement(
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
) -> Result<(), IllegalVariantReturnTypeError> {
    let variant_ids = registry.get_possibly_empty_list(type_statement.variant_list_id);
    for (variant_index, variant_id) in variant_ids.iter().copied().enumerate() {
        let variant = registry.get(variant_id);
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
        let adjusted_type_statement_db_index = DbIndex(variant_index + variant.param_list_id.len());
        let return_db_index = registry.get(return_type_name_id).db_index;
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
            let call = registry.get(call_id);
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
