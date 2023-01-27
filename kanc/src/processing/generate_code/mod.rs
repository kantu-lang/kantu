use crate::data::light_ast::*;

pub mod targets;

pub trait CompileTarget {
    type Options;
    type Ok;
    type Error;

    fn generate_code_with_options(
        registry: &NodeRegistry,
        file_item_list_id: Option<NonEmptyListId<FileItemNodeId>>,
        options: Self::Options,
    ) -> Result<Self::Ok, Self::Error>;

    fn generate_code(
        registry: &NodeRegistry,
        file_item_list_id: Option<NonEmptyListId<FileItemNodeId>>,
    ) -> Result<Self::Ok, Self::Error>
    where
        Self::Options: Default,
    {
        Self::generate_code_with_options(registry, file_item_list_id, Default::default())
    }
}
