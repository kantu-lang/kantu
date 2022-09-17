pub mod lex;
pub mod node_registry;
pub mod parse;
pub mod registered_ast;
pub mod unregistered_ast;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    file_id: FileId,
    index: usize,
}
