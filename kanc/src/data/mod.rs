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

use std::cmp::Ordering;

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
    /// Inclusive
    pub start: TextCoord,
    /// Exclusive
    pub end: TextCoord,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextCoord {
    /// 1-indexed
    pub line: usize,
    /// 0-indexed
    pub col: usize,
}

impl Ord for TextCoord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.line.cmp(&other.line).then(self.col.cmp(&other.col))
    }
}

impl PartialOrd for TextCoord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TextBispan {
    /// Returns `None` if either of the span's indices are out of bounds,
    /// or if any of the span's indices don't align with a character boundary
    /// in `src`.
    ///
    /// ## Panics
    ///
    /// Panics if `span.start > span.end`
    pub fn new(src: &str, span: TextSpan) -> Option<Self> {
        if span.start > span.end {
            panic!("Span start is after span end. {:?}", span);
        }

        let mut line = 1;
        let mut col = 0;
        let mut start = None;
        for (i, c) in src.char_indices() {
            if i == span.start {
                start = Some(TextCoord { line, col });
            }
            if i == span.end {
                let start = start?;
                let end = TextCoord { line, col };
                return Some(TextBispan {
                    file_id: span.file_id,
                    start,
                    end,
                });
            }
            if c == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        None
    }
}

impl TextBispan {
    pub fn inclusive_merge(self, other: TextBispan) -> TextBispan {
        if self.file_id != other.file_id {
            panic!("Cannot merge spans from different files.");
        }

        let start = self.start;
        let end = other.end;
        if end < start {
            panic!("End of span is before start of span.");
        }

        TextBispan {
            file_id: self.file_id,
            start,
            end,
        }
    }
}
