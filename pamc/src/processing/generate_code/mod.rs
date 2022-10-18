use crate::data::{
    node_registry::{NodeId, NodeRegistry},
    registered_sst::*,
    symbol_database::SymbolDatabase,
    variant_return_type::VariantReturnTypeDatabase,
};

pub mod targets;

pub trait CompileTarget {
    type Options;
    type Ok;
    type Error;

    fn generate_code_with_options(
        symbol_db: &SymbolDatabase,
        registry: &NodeRegistry,
        variant_db: &VariantReturnTypeDatabase,
        file_ids: &[NodeId<File>],
        options: Self::Options,
    ) -> Result<Self::Ok, Self::Error>;

    fn generate_code(
        symbol_db: &SymbolDatabase,
        registry: &NodeRegistry,
        variant_db: &VariantReturnTypeDatabase,
        file_ids: &[NodeId<File>],
    ) -> Result<Self::Ok, Self::Error>
    where
        Self::Options: Default,
    {
        Self::generate_code_with_options(
            symbol_db,
            registry,
            variant_db,
            file_ids,
            Default::default(),
        )
    }
}
