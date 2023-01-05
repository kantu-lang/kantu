pub mod bind_error;
pub mod bound_ast;
pub mod file_id;
pub mod file_tree;
pub mod fun_recursion_validation_result;
pub mod light_ast;
pub mod node_equality_checker;
pub mod node_registry;
pub mod non_empty_vec;
pub mod simplified_ast;
pub mod text_span;
pub mod token;
pub mod type_positivity_validation_result;
pub mod unsimplified_ast;
pub mod variant_return_type_validation_result;

// TODO: Delete
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);
