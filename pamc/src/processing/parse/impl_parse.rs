//! We named this module `impl_parse` because it implements the `Parse` trait,
//! not because it actually implements the "real" parsing logic.
//! To see the real parsing logic, see the `accept` module.

use super::*;

impl Parse for File {
    fn initial_stack(_: FileId, first_token: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::File(Box::new(UnfinishedFile {
            first_token: first_token.clone(),
            items: vec![],
        }))]
    }

    fn finish(file_id: FileId, bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::File(_, file) => Ok(file),
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for Expression {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::UnfinishedDelimitedExpression(
            UnfinishedDelimitedExpression::Empty,
        )]
    }

    fn finish(file_id: FileId, bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                if end_delimiter.raw().kind == TokenKind::Eoi {
                    Ok(expression)
                } else {
                    Err(ParseError::unexpected_token(end_delimiter.into_raw()))
                }
            }
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for Param {
    fn initial_stack(_: FileId, first_token: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Param(todo!())]
    }

    fn finish(file_id: FileId, bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::Param(_, param, end_delimiter) => {
                if end_delimiter.raw().kind == TokenKind::Eoi {
                    Ok(param)
                } else {
                    Err(ParseError::unexpected_token(end_delimiter.into_raw()))
                }
            }
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for Variant {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Variant(todo!())]
    }

    fn finish(file_id: FileId, bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::Variant(_, variant, end_delimiter) => {
                if end_delimiter.raw().kind == TokenKind::Eoi {
                    Ok(variant)
                } else {
                    Err(ParseError::unexpected_token(end_delimiter.into_raw()))
                }
            }
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for FileItem {
    fn initial_stack(_: FileId, first_token: &Token) -> Vec<UnfinishedStackItem> {
        vec![todo!()]
    }

    fn finish(file_id: FileId, bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        todo!()
    }
}

fn dummy_token() -> Token {
    Token {
        start_index: 0,
        content: "_".to_string(),
        kind: TokenKind::Underscore,
    }
}

fn dummy_identifier() -> Identifier {
    Identifier {
        span: TextSpan {
            file_id: FileId(0),
            start: 0,
            end: 1,
        },
        name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
    }
}
