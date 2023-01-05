use crate::data::file_id::*;

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteIndex(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSpan {
    pub file_id: FileId,
    /// Inclusive TODO: Make this a ByteIndex
    pub start: usize,
    /// Exclusive TODO: Make this a ByteIndex
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

impl TextCoord {
    /// Returns `None` if `i` is out of bounds,
    /// or if `i` doesn't align with a character boundary.
    pub fn new(src: &str, i: ByteIndex) -> Option<Self> {
        let target = i.0;

        let mut line = 1;
        let mut col = 0;
        for (j, c) in src.char_indices() {
            if j == target {
                return Some(TextCoord { line, col });
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextBispan {
    pub file_id: FileId,
    /// Inclusive
    pub start: TextCoord,
    /// Exclusive
    pub end: TextCoord,
}

impl TextBispan {
    /// Returns `None` if either of `span`'s indices are out of bounds,
    /// or if either of `span`'s indices don't align with a character boundary.
    ///
    /// ## Panics
    ///
    /// Panics if `span.start > span.end`
    pub fn new(src: &str, span: TextSpan) -> Option<Self> {
        if span.start > span.end {
            panic!("Span start is after span end. {:?}", span);
        }

        let start = TextCoord::new(src, ByteIndex(span.start))?;
        let end = TextCoord::new(src, ByteIndex(span.end))?;
        Some(TextBispan {
            file_id: span.file_id,
            start,
            end,
        })
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
