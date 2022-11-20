use crate::data::{
    light_ast::*,
    node_registry::{NodeId, NodeRegistry},
};

pub mod targets;

pub trait CompileTarget {
    type Options;
    type Ok;
    type Error;

    fn generate_code_with_options(
        registry: &NodeRegistry,
        file_ids: &[NodeId<File>],
        options: Self::Options,
    ) -> Result<Self::Ok, Self::Error>;

    fn generate_code(
        registry: &NodeRegistry,
        file_ids: &[NodeId<File>],
    ) -> Result<Self::Ok, Self::Error>
    where
        Self::Options: Default,
    {
        Self::generate_code_with_options(registry, file_ids, Default::default())
    }
}
