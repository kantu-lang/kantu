pub mod bound_ast;
pub mod node_equality_checker;
pub mod simplified_ast;
pub mod token;
pub mod unsimplified_ast;

pub mod x_light_ast;
pub mod x_node_registry;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPosition {
    pub file_id: FileId,
    pub index: usize,
}
