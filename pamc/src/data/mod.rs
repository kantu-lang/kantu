pub mod bind_error;
pub mod bound_ast;
pub mod fun_recursion_validation_result;
pub mod light_ast;
pub mod node_equality_checker;
pub mod node_registry;
pub mod simplified_ast;
pub mod token;
pub mod unsimplified_ast;
pub mod variant_return_type_validation_result;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSpan {
    pub file_id: FileId,
    /// Inclusive
    pub start: usize,
    /// Exclusive
    pub end: usize,
}
