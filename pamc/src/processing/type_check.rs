use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    symbol_database::{IdentifierToSymbolMap, Symbol},
    type_map::TypeMap,
};

#[derive(Clone, Debug)]
pub enum TypeError {}

pub fn type_check_file(
    registry: &NodeRegistry,
    identifier_to_symbol_map: &IdentifierToSymbolMap,
    file: &File,
) -> Result<TypeMap, TypeError> {
    unimplemented!()
}
