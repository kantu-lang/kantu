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

    fn finish(
        file_id: FileId,
        item: UnfinishedStackItem,
        _: Vec<UnfinishedStackItem>,
    ) -> Result<Self, ParseError> {
        match item {
            UnfinishedStackItem::File(file) => Ok(File {
                span: TextSpan {
                    file_id,
                    start: file.items[0].span().start,
                    end: file
                        .items
                        .last()
                        .expect("File should have at least one item.")
                        .span()
                        .end,
                },
                id: file_id,
                items: file.items,
            }),
            _ => Err(ParseError::UnexpectedEndOfInput),
        }
    }
}

impl Parse for Expression {
    fn from_empty_str(_: FileId) -> Result<Self, ParseError> {
        Err(ParseError::UnexpectedEndOfInput)
    }

    fn initial_stack(_: FileId, _: Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::UnfinishedDelimitedExpression(
            UnfinishedDelimitedExpression::Empty,
        )]
    }

    fn finish(
        _: FileId,
        item: UnfinishedStackItem,
        _: Vec<UnfinishedStackItem>,
    ) -> Result<Self, ParseError> {
        match item {
            UnfinishedStackItem::UnfinishedDelimitedExpression(
                UnfinishedDelimitedExpression::WaitingForEndDelimiter(_first_token, expression),
            ) => Ok(expression),
            _ => Err(ParseError::UnexpectedEndOfInput),
        }
    }
}

impl Parse for Param {
    fn from_empty_str(_: FileId) -> Result<Self, ParseError> {
        Err(ParseError::UnexpectedEndOfInput)
    }

    fn initial_stack(_: FileId, first_token: Token) -> Vec<UnfinishedStackItem> {
        vec![UnfinishedStackItem::Params(UnfinishedParams {
            first_token,
            maximum_dashed_params_allowed: 1,
            pending_dash: None,
            params: vec![],
        })]
    }

    fn before_handle_token(
        _: FileId,
        token: &Token,
        stack: &[UnfinishedStackItem],
    ) -> Result<(), ParseError> {
        if let Some(UnfinishedStackItem::Params(params)) = stack.get(0) {
            if params.params.is_empty() {
                return Ok(());
            }
        }
        Err(ParseError::UnexpectedToken(token.clone()))
    }

    fn finish(
        file_id: FileId,
        item: UnfinishedStackItem,
        mut remaining_stack: Vec<UnfinishedStackItem>,
    ) -> Result<Self, ParseError> {
        match (item, remaining_stack.pop()) {
            (
                UnfinishedStackItem::UnfinishedDelimitedExpression(
                    UnfinishedDelimitedExpression::WaitingForEndDelimiter(_, param_type),
                ),
                Some(UnfinishedStackItem::Param(UnfinishedParam::Name(
                    param_first_token,
                    is_param_dashed,
                    param_name,
                ))),
            ) => Ok(Param {
                span: span_single(file_id, &param_first_token).inclusive_merge(param_type.span()),
                is_dashed: is_param_dashed,
                name: param_name,
                type_: param_type,
            }),
            _ => Err(ParseError::UnexpectedEndOfInput),
        }
    }
}
