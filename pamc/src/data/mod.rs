pub mod node_free_variable_cache;
pub mod node_hash_cache;
pub mod node_registry;
pub mod registered_ast;
pub mod symbol_database;
pub mod token;
pub mod type_map;
pub mod unregistered_ast;
/// "SST" stands for "simplified syntax tree".
pub mod unregistered_sst;
pub mod variant_return_type;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    pub file_id: FileId,
    pub index: usize,
}
