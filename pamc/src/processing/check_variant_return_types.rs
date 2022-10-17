use crate::data::{
    node_registry::NodeRegistry,
    registered_ast::*,
    symbol_database::SymbolDatabase,
    variant_return_type::{VariantReturnType, VariantReturnTypeDatabase},
};

#[derive(Clone, Debug)]
pub struct IllegalVariantReturnTypeError(pub ExpressionId);

pub fn check_variant_return_types_for_file(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    file: &File,
) -> Result<VariantReturnTypeDatabase, IllegalVariantReturnTypeError> {
    let mut map = VariantReturnTypeDatabase::empty();
    let item_ids = registry.file_item_list(file.item_list_id);
    for item_id in item_ids {
        if let FileItemNodeId::Type(type_id) = item_id {
            let type_statement = registry.type_statement(*type_id);
            check_variant_return_types_for_type_statement(
                symbol_db,
                registry,
                &mut map,
                type_statement,
            )?;
        }
    }
    Ok(map)
}

fn check_variant_return_types_for_type_statement(
    symbol_db: &SymbolDatabase,
    registry: &NodeRegistry,
    map: &mut VariantReturnTypeDatabase,
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
) -> Result<VariantReturnType, IllegalVariantReturnTypeError> {
    let type_symbol = symbol_db.identifier_symbols.get(type_statement.name_id);
    let return_type_id = variant.return_type_id;
    match return_type_id {
        ExpressionId::Identifier(identifier_id) => {
            let identifier_symbol = symbol_db.identifier_symbols.get(identifier_id);
            if identifier_symbol == type_symbol {
                Ok(VariantReturnType::Identifier {
                    identifier_id: return_type_id,
                })
            } else {
                Err(IllegalVariantReturnTypeError(return_type_id))
            }
        }
        ExpressionId::Call(call_id) => {
            let call = registry.call(call_id);
            match call.callee_id {
                ExpressionId::Identifier(identifier_id) => {
                    let identifier_symbol = symbol_db.identifier_symbols.get(identifier_id);
                    if identifier_symbol == type_symbol {
                        Ok(VariantReturnType::Call {
                            callee_id: call.callee_id,
                            arg_list_id: call.arg_list_id,
                        })
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
