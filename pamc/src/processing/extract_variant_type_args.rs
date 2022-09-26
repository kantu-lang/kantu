use crate::data::{
    node_registry::NodeId, registered_ast::*, symbol_database::SymbolDatabase,
    variant_return_type::VariantReturnTypeTypeArgsMap,
};

#[derive(Clone, Debug)]
pub struct IllegalVariantReturnTypeError(pub NodeId<WrappedExpression>);

pub fn extract_variant_type_args_for_file(
    symbol_db: &SymbolDatabase,
    file: &File,
) -> Result<VariantReturnTypeTypeArgsMap, IllegalVariantReturnTypeError> {
    let mut map = VariantReturnTypeTypeArgsMap::empty();
    for item in &file.items {
        if let FileItem::Type(type_statement) = item {
            extract_variant_type_args_for_type_statement(symbol_db, &mut map, type_statement)?;
        }
    }
    Ok(map)
}

fn extract_variant_type_args_for_type_statement(
    symbol_db: &SymbolDatabase,
    map: &mut VariantReturnTypeTypeArgsMap,
    type_statement: &TypeStatement,
) -> Result<(), IllegalVariantReturnTypeError> {
    for variant in &type_statement.variants {
        let args = get_variant_type_args(symbol_db, type_statement, variant)?;
        map.insert_new(variant.id, args);
    }
    Ok(())
}

fn get_variant_type_args(
    symbol_db: &SymbolDatabase,
    type_statement: &TypeStatement,
    variant: &Variant,
) -> Result<Vec<NodeId<WrappedExpression>>, IllegalVariantReturnTypeError> {
    let type_symbol = symbol_db.identifier_symbols.get(type_statement.name.id);
    match &variant.return_type.expression {
        Expression::Identifier(identifier) => {
            let identifier_symbol = symbol_db.identifier_symbols.get(identifier.id);
            if identifier_symbol == type_symbol {
                Ok(vec![])
            } else {
                Err(IllegalVariantReturnTypeError(variant.return_type.id))
            }
        }
        Expression::Call(call) => match &call.callee.expression {
            Expression::Identifier(identifier) => {
                let identifier_symbol = symbol_db.identifier_symbols.get(identifier.id);
                if identifier_symbol == type_symbol {
                    Ok(call.args.iter().map(|arg| arg.id).collect())
                } else {
                    Err(IllegalVariantReturnTypeError(variant.return_type.id))
                }
            }
            _other_callee => Err(IllegalVariantReturnTypeError(variant.return_type.id)),
        },
        _other_variant_return_type => Err(IllegalVariantReturnTypeError(variant.return_type.id)),
    }
}
