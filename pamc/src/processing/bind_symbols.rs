use crate::data::{
    identifier_to_symbol_map::IdentifierToSymbolMap,
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
};

pub fn bind_symbols_to_identifiers(
    registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> IdentifierToSymbolMap {
    unimplemented!();
}
