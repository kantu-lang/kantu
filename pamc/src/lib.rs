pub mod bind_file;
pub mod bound_ast;
pub mod lex;
pub mod parse;
pub mod unbound_ast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    file_id: FileId,
    index: usize,
}
