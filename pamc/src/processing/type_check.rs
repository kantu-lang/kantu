use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
    type_map::TypeMap,
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
    let mut type_map = TypeMap::empty();
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        type_map: &mut type_map,
        context: TypeCheckContext::empty(),
    };
    unimplemented!()
    // for item in &file.items {
    //     match item {
    //         FileItem::Type(type_statement) => {
    //             type_check_type_statement(&mut state, type_statement)?;
    //         }
    //         FileItem::Let(let_statement) => {
    //             type_check_let_statement(&mut state, let_statement)?;
    //         }
    //     }
    // }
    // Ok(type_map)
}

fn type_check_type_statement(
    state: &mut TypeCheckState,
    type_statement: &TypeStatement,
) -> Result<(), TypeError> {
    for variant in &type_statement.variants {
        type_check_variant(state, variant)?;
    }
    Ok(())
}

fn type_check_let_statement(
    state: &mut TypeCheckState,
    let_statement: &LetStatement,
) -> Result<(), TypeError> {
    // TODO: Actually implement (or remove) type_check_let_statement
    Ok(())
}

fn type_check_variant(state: &mut TypeCheckState, variant: &Variant) -> Result<(), TypeError> {
    for param in &variant.params {
        type_check_param(state, param)?;
    }
    Ok(())
}

fn type_check_param(state: &mut TypeCheckState, param: &Param) -> Result<(), TypeError> {
    let type_type_id = type_check_expression(state, &param.type_)?.0;
    let type_type = state.registry.wrapped_expression(type_type_id);
    match &type_type.expression {
        Expression::Identifier(identifier) => {
            let symbol = state.symbol_db.identifier_symbols.get(identifier.id);
            if !(symbol == state.symbol_db.provider.type0_symbol()
                || symbol == state.symbol_db.provider.type1_symbol())
            {
                return Err(TypeError::IllegalParamType {
                    param: param.id,
                    type_type: type_type_id,
                });
            }
        }
        _other_type_type => {
            return Err(TypeError::IllegalParamType {
                param: param.id,
                type_type: type_type_id,
            })
        }
    }

    let param_symbol = state.symbol_db.identifier_symbols.get(param.name.id);
    let type_normal_form = evaluate_well_typed_expression(state, &param.type_)?.0;
    state.type_map.insert_new(param_symbol, type_normal_form);

    Ok(())
}

fn type_check_expression(
    state: &mut TypeCheckState,
    expression: &WrappedExpression,
) -> Result<NormalForm, TypeError> {
    unimplemented!();
}

fn evaluate_well_typed_expression(
    state: &mut TypeCheckState,
    expression: &WrappedExpression,
) -> Result<NormalForm, TypeError> {
    unimplemented!();
}

#[derive(Clone, Copy, Debug)]
struct NormalForm(NodeId<WrappedExpression>);

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
