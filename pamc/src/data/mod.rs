pub mod free_variable_cache;
pub mod node_hash_cache;
pub mod node_registry;
pub mod registered_ast;
pub mod symbol_database;
pub mod token;
pub mod type_map;
pub mod unregistered_ast;
pub mod variant_return_type;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    pub file_id: FileId,
    pub index: usize,
}
