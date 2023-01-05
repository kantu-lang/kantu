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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextBispan {
    pub file_id: FileId,
    /// Inclusive, 1-indexed
    pub start_line: usize,
    /// Inclusive, 0-indexed
    pub start_col: usize,
    /// Exclusive, 1-indexed
    pub end_line: usize,
    /// Exclusive, 0-indexed
    pub end_col: usize,
}

impl TextBispan {
    /// Returns `None` if either `span.start` or `span.end`
    /// is outside `src`'s bounds.
    pub fn new(src: &str, span: TextSpan) -> Option<Self> {
        unimplemented!()
    }
}

impl TextBispan {
    pub fn inclusive_merge(self, other: TextBispan) -> TextBispan {
        if self.file_id != other.file_id {
            panic!("Cannot merge spans from different files.");
        }

        let start_line = self.start_line;
        let start_col = self.start_col;
        let end_line = other.end_line;
        let end_col = other.end_col;
        if end_line < start_line || (end_line == start_line && end_col < start_col) {
            panic!("End of bispan is before start of bispan.");
        }

        TextBispan {
            file_id: self.file_id,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}
