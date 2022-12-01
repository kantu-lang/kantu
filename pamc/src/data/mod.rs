pub mod bind_error;
pub mod bound_ast;
pub mod illegal_fun_recursion_error;
pub mod light_ast;
pub mod node_equality_checker;
pub mod node_registry;
pub mod simplified_ast;
pub mod token;
pub mod unsimplified_ast;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextPosition {
    pub file_id: FileId,
    pub index: usize,
}
