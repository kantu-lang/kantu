use crate::data::{
    identifier_to_symbol_map::IdentifierToSymbolMap,
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
};

#[derive(Clone, Debug)]
pub enum BindError {
    CircularFileDependency(CircularFileDependencyError),
}

#[derive(Clone, Debug)]
pub struct CircularFileDependencyError {
    pub ids: Vec<NodeId<File>>,
}

impl From<CircularFileDependencyError> for BindError {
    fn from(error: CircularFileDependencyError) -> Self {
        Self::CircularFileDependency(error)
    }
}

pub fn bind_symbols_to_identifiers(
    registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> Result<IdentifierToSymbolMap, BindError> {
    let file_node_ids = sort_by_dependencies(registry, file_node_ids)?;
    unimplemented!();
}

fn sort_by_dependencies(
    _registry: &NodeRegistry,
    file_node_ids: Vec<NodeId<File>>,
) -> Result<Vec<NodeId<File>>, CircularFileDependencyError> {
    // TODO: Actually sort when
    Ok(file_node_ids)
}
