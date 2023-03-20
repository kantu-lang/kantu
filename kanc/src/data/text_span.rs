#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteIndex(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextSpan {
    pub file_id: FileId,
    pub start: ByteIndex,
    pub end: ByteIndex,
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
pub struct TextPosition {
    pub file_id: FileId,
    pub index: ByteIndex,
}
