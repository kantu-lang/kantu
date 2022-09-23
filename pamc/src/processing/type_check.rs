use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{Symbol, SymbolDatabase},
    type_map::TypeMap,
};

#[derive(Clone, Debug)]
pub enum TypeError {}

pub fn type_check_file(
    registry: &NodeRegistry,
    symbol_db: &SymbolDatabase,
    file: &File,
) -> Result<TypeMap, TypeError> {
    let mut type_map = TypeMap::empty();
    let mut state = TypeCheckState {
        registry,
        symbol_db,
        type_map: &mut type_map,
        context: TypeCheckContext::empty(),
    };
    for item in &file.items {
        match item {
            FileItem::Type(type_statement) => {
                type_check_type_statement(&mut state, type_statement)?;
            }
            FileItem::Let(let_statement) => {
                type_check_let_statement(&mut state, let_statement)?;
            }
        }
    }
    Ok(type_map)
}

fn type_check_type_statement(
    state: &mut TypeCheckState,
    type_statement: &TypeStatement,
) -> Result<(), TypeError> {
    for constructor in &type_statement.constructors {
        type_check_constructor(state, constructor)?;
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

fn type_check_constructor(
    state: &mut TypeCheckState,
    constructor: &Constructor,
) -> Result<(), TypeError> {
    for param in &constructor.params {
        type_check_param(state, param)?;
    }
    Ok(())
}

fn type_check_param(state: &mut TypeCheckState, param: &Param) -> Result<(), TypeError> {
    // let type_type = type_check_expression(state, &param.type_)?;
    unimplemented!()
}

#[derive(Debug)]
struct TypeCheckState<'a> {
    registry: &'a NodeRegistry,
    symbol_db: &'a SymbolDatabase,
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
