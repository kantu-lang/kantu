//! We named this module `impl_parse` because it implements the `Parse` trait,
//! not because it actually implements the "real" parsing logic.
//! To see the real parsing logic, see the `accept` module.

use super::*;

impl Parse for File {
    fn from_empty_str(file_id: FileId) -> Result<Self, ParseError> {
        Ok(File {
            span: TextSpan {
                file_id,
                start: 0,
                end: 0,
            },
            id: file_id,
            items: vec![],
        })
    }

    fn initial_stack(_: FileId, first_token: Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::File(Box::new(UnfinishedFile {
            first_token,
            items: vec![],
        }))]
    }

    fn finish(file_id: FileId, item: UnfinishedStackItem) -> Result<Self, ParseError> {
        match item {
            UnfinishedStackItem::File(file) => Ok(File {
                span: TextSpan {
                    file_id,
                    start: file.items[0].span().start,
                    end: file.items.last().expect("File should have at least one item.").span().end,
                },
                id: file_id,
                items: file.items,
            }),
            _ => panic!("The top item on the stack is not a file. This indicates a serious logic error with the parser.")
        }
    }
}
