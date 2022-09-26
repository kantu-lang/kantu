pub mod node_registry;
pub mod registered_ast;
pub mod symbol_database;
pub mod symbol_provider;
pub mod token;
pub mod type_map;
pub mod unregistered_ast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    pub file_id: FileId,
    pub index: usize,
}
