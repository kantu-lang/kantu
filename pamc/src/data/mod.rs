pub mod bound_ast;
pub mod light_ast;
pub mod node_free_variable_cache;
pub mod node_hash_cache;
pub mod node_registry;
pub mod simplified_ast;
pub mod symbol_database;
pub mod token;
pub mod type_map;
pub mod unsimplified_ast;
pub mod variant_return_type;

pub mod x_light_ast;
pub mod x_node_registry;
pub mod x_stripped_ast;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPosition {
    pub file_id: FileId,
    pub index: usize,
}
