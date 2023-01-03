pub mod bind_error;
pub mod bound_ast;
pub mod file_tree;
pub mod fun_recursion_validation_result;
pub mod light_ast;
pub mod node_equality_checker;
pub mod node_registry;
pub mod non_empty_vec;
pub mod simplified_ast;
pub mod token;
pub mod type_positivity_validation_result;
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

impl TextSpan {
    pub fn inclusive_merge(self, other: TextSpan) -> TextSpan {
        if self.file_id != other.file_id {
            panic!("Cannot merge spans from different files.");
        }

        let start = self.start;
        let end = other.end;
        if end < start {
            panic!("End of span is before start of span.");
        }

        TextSpan {
            file_id: self.file_id,
            start,
            end,
        }
    }
}
