use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_ast::*,
    unregistered_ast as ur,
};

pub fn register_file(registry: &mut NodeRegistry, file: ur::File) -> NodeId<File> {
    unimplemented!();
}
