//! We named this module `impl_parse` because it implements the `Parse` trait,
//! not because it actually implements the "real" parsing logic.
//! To see the real parsing logic, see the `accept` module.

use super::*;

impl Parse for File {
    fn initial_stack(_: FileId, first_token: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::File(Box::new(UnfinishedFile {
            first_token: first_token.clone(),
            pending_visibility: None,
            items: vec![],
        }))]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
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

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
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
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Param(UnfinishedParam::NoIdentifier {
            pending_tilde: None,
            pending_dash: None,
            is_dash_allowed: true,
        })]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
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
        vec![UnfinishedStackItem::Variant(UnfinishedVariant::Empty)]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
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

impl Parse for ParenthesizedWeakAncestor {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::ParenthesizedWeakAncestor(
            UnfinishedParenthesizedWeakAncestor::Empty,
        )]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::ParenthesizedWeakAncestor(_, ancestor) => Ok(ancestor),
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for UseStatement {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Use(UnfinishedUseStatement::Empty)]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::Use(_, use_statement) => Ok(use_statement),
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for ModStatement {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Mod(UnfinishedModStatement::Empty)]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::Mod(_, mod_statement) => Ok(mod_statement),
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for TypeStatement {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Type(UnfinishedTypeStatement::Empty)]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::Type(_, type_statement) => Ok(type_statement),
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}

impl Parse for LetStatement {
    fn initial_stack(_: FileId, _: &Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Let(UnfinishedLetStatement::Empty)]
    }

    fn finish(bottom_item: FinishedStackItem) -> Result<Self, ParseError> {
        match bottom_item {
            FinishedStackItem::Let(_, let_statement) => Ok(let_statement),
            other_item => Err(unexpected_finished_item_err(&other_item)),
        }
    }
}
