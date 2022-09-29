use crate::data::{
    node_registry::{ListId, NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::SymbolDatabase,
    variant_return_type::VariantReturnTypeTypeArgsMap,
};

#[derive(Clone, Debug)]
pub struct IllegalVariantReturnTypeError(pub NodeId<WrappedExpression>);

pub fn extract_variant_type_args_for_file(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    file: &File,
) -> Result<VariantReturnTypeTypeArgsMap, IllegalVariantReturnTypeError> {
    let mut map = VariantReturnTypeTypeArgsMap::empty();
    let item_ids = registry.file_item_list(file.item_list_id);
    for item_id in item_ids {
        if let FileItemNodeId::Type(type_id) = item_id {
            let type_statement = registry.type_statement(*type_id);
            extract_variant_type_args_for_type_statement(
                symbol_db,
                registry,
                &mut map,
                type_statement,
            )?;
        }
    }
    Ok(map)
}

fn extract_variant_type_args_for_type_statement(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    map: &mut VariantReturnTypeTypeArgsMap,
    type_statement: &TypeStatement,
) -> Result<(), IllegalVariantReturnTypeError> {
    let variant_ids = registry.variant_list(type_statement.variant_list_id);
    for variant_id in variant_ids {
        let variant = registry.variant(*variant_id);
        let args = get_variant_type_args(symbol_db, registry, type_statement, variant)?;
        map.insert_new(variant.id, args);
    }
    Ok(())
}

fn get_variant_type_args(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    type_statement: &TypeStatement,
    variant: &Variant,
) -> Result<ListId<NodeId<WrappedExpression>>, IllegalVariantReturnTypeError> {
    let type_symbol = symbol_db.identifier_symbols.get(type_statement.name_id);
    let return_type_id = variant.return_type_id;
    let return_type = &registry.wrapped_expression(return_type_id).expression;
    match return_type {
        Expression::Identifier(identifier) => {
            let identifier_symbol = symbol_db.identifier_symbols.get(identifier.id);
            if identifier_symbol == type_symbol {
                Ok(vec![])
            } else {
                Err(IllegalVariantReturnTypeError(return_type_id))
            }
        }
        Expression::Call(call) => {
            let callee = &registry.wrapped_expression(call.callee_id).expression;
            match callee {
                Expression::Identifier(identifier) => {
                    let identifier_symbol = symbol_db.identifier_symbols.get(identifier.id);
                    if identifier_symbol == type_symbol {
                        Ok(call.arg_list_id)
                    } else {
                        Err(IllegalVariantReturnTypeError(return_type_id))
                    }
                }
                _other_callee => Err(IllegalVariantReturnTypeError(return_type_id)),
            }
        }
        _other_variant_return_type => Err(IllegalVariantReturnTypeError(return_type_id)),
    }
}
