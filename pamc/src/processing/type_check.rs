use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
    type_map::{NormalFormId, TypeMap},
};

#[derive(Clone, Debug)]
pub enum TypeError {
    IllegalParamType {
        param: NodeId<Param>,
        type_type: NodeId<WrappedExpression>,
    },
}

pub fn type_check_file(
    registry: &mut NodeRegistry,
    symbol_db: &mut SymbolDatabase,
    file_id: NodeId<File>,
) -> Result<TypeMap, TypeError> {
    let file_item_ids = get_file_item_ids(registry, file_id);
    let mut type_map = TypeMap::empty();
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        type_map: &mut type_map,
        context: TypeCheckContext::empty(),
    };
    for item in file_item_ids {
        match item {
            FileItemId::Type(type_id) => {
                type_check_type_statement(&mut state, type_id)?;
            }
            FileItemId::Let(let_id) => {
                type_check_let_statement(&mut state, let_id)?;
            }
        }
    }
    Ok(type_map)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FileItemId {
    Type(NodeId<TypeStatement>),
    Let(NodeId<LetStatement>),
}

fn get_file_item_ids(registry: &NodeRegistry, file_id: NodeId<File>) -> Vec<FileItemId> {
    let file = registry.file(file_id);
    let mut ids = Vec::with_capacity(file.items.len());
    for item in &file.items {
        match item {
            FileItem::Type(type_statement_id) => ids.push(FileItemId::Type(type_statement_id.id)),
            FileItem::Let(let_statement_id) => ids.push(FileItemId::Let(let_statement_id.id)),
        }
    }
    ids
}

fn type_check_type_statement(
    state: &mut TypeCheckState,
    type_statement_id: NodeId<TypeStatement>,
) -> Result<(), TypeError> {
    let variant_ids: Vec<NodeId<Variant>> = state
        .registry
        .type_statement(type_statement_id)
        .variants
        .iter()
        .map(|v| v.id)
        .collect();
    for variant_id in variant_ids {
        type_check_variant(state, variant_id)?;
    }
    Ok(())
}

fn type_check_let_statement(
    _state: &mut TypeCheckState,
    _let_statement: NodeId<LetStatement>,
) -> Result<(), TypeError> {
    // TODO: Actually implement (or remove) type_check_let_statement
    Ok(())
}

fn type_check_variant(
    state: &mut TypeCheckState,
    variant: NodeId<Variant>,
) -> Result<(), TypeError> {
    let param_ids: Vec<NodeId<Param>> = state
        .registry
        .variant(variant)
        .params
        .iter()
        .map(|p| p.id)
        .collect();
    for param_id in param_ids {
        type_check_param(state, param_id)?;
    }
    Ok(())
}

fn type_check_param(state: &mut TypeCheckState, param_id: NodeId<Param>) -> Result<(), TypeError> {
    let type_id = state.registry.param(param_id).type_.id;
    let type_type_id = type_check_expression(state, type_id)?.0;
    let type_type = state.registry.wrapped_expression(type_type_id);
    match &type_type.expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            if !(symbol == state.symbol_db.provider.type0_symbol()
                || symbol == state.symbol_db.provider.type1_symbol())
            {
                return Err(TypeError::IllegalParamType {
                    param: param_id,
                    type_type: type_type_id,
                });
            }
        }
        _other_type_type => {
            return Err(TypeError::IllegalParamType {
                param: param_id,
                type_type: type_type_id,
            })
        }
    }

    let param_name_id = state.registry.param(param_id).name.id;
    let param_symbol = state.symbol_db.identifier_symbols.get(param_name_id);
    let type_normal_form_id = evaluate_well_typed_expression(state, type_id)?;
    state.type_map.insert_new(param_symbol, type_normal_form_id);

    Ok(())
}

fn type_check_expression(
    state: &mut TypeCheckState,
    id: NodeId<WrappedExpression>,
) -> Result<NormalFormId, TypeError> {
    match &state.registry.wrapped_expression(id).expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            let type_id = state.type_map.get(symbol);
            Ok(type_id)
        }
        Expression::Dot(dot) => {
            let symbol = state.symbol_db.identifier_symbols.get(dot.right.id);
            let type_id = state.type_map.get(symbol);
            Ok(type_id)
        }
        _ => unimplemented!(),
    }
}

fn evaluate_well_typed_expression(
    state: &mut TypeCheckState,
    expression: NodeId<WrappedExpression>,
) -> Result<NormalFormId, TypeError> {
    unimplemented!();
}

#[derive(Debug)]
struct TypeCheckState<'a> {
    registry: &'a mut NodeRegistry,
    symbol_db: &'a mut SymbolDatabase,
    type_map: &'a mut TypeMap,
    context: TypeCheckContext,
}

#[derive(Clone, Debug)]
struct TypeCheckContext {}

impl TypeCheckContext {
    fn empty() -> Self {
        Self {}
    }
}
