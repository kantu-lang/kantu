use crate::data::{light_ast::*, variant_return_type_validation_result::*};

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
pub fn validate_variant_return_types_in_file_items<'a>(
    items: &'a [FileItemRef<'a>],
) -> Result<VariantReturnTypesValidated<&[FileItemRef]>, IllegalVariantReturnTypeError> {
    for item in items {
        if let FileItemRef::Type(type_) = item {
            validate_variant_return_types_in_type_statement(type_)?;
        }
    }
    Ok(VariantReturnTypesValidated::unchecked_new(items))
}

fn validate_variant_return_types_in_type_statement<'a>(
    type_statement: &'a TypeStatement<'a>,
) -> Result<(), IllegalVariantReturnTypeError> {
    for (variant_index, variant) in type_statement.variants.iter().copied().enumerate() {
        validate_return_type_of_variant(variant, variant_index)?;
    }
    Ok(())
}

fn validate_return_type_of_variant<'a>(
    variant: &'a Variant<'a>,
    variant_index: usize,
) -> Result<(), IllegalVariantReturnTypeError> {
    fn validate_return_type_name_db_index<'a>(
        return_type_name: &'a NameExpression<'a>,
        (return_type, variant, variant_index): (ExpressionRef<'a>, &'a Variant<'a>, usize),
    ) -> Result<(), IllegalVariantReturnTypeError<'a>> {
        let adjusted_type_statement_db_index = DbIndex(variant_index + variant.params.len());
        let return_db_index = return_type_name.db_index;
        if adjusted_type_statement_db_index == return_db_index {
            Ok(())
        } else {
            Err(IllegalVariantReturnTypeError(return_type))
        }
    }

    let return_type = variant.return_type;
    match return_type {
        ExpressionRef::Name(name) => {
            validate_return_type_name_db_index(name, (return_type, variant, variant_index))
        }
        ExpressionRef::Call(call) => match call.callee {
            ExpressionRef::Name(name) => {
                validate_return_type_name_db_index(name, (return_type, variant, variant_index))
            }
            _other_callee => Err(IllegalVariantReturnTypeError(return_type)),
        },
        _other_variant_return_type => Err(IllegalVariantReturnTypeError(return_type)),
    }
}
