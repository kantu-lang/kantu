use super::*;

impl Accept for UnfinishedLetStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedLetStatement::EmptyString => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Let => {
                        *self = UnfinishedLetStatement::Keyword(token);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedLetStatement::Keyword(let_kw) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedLetStatement::Name(let_kw.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedLetStatement::Name(let_kw, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Equal => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Semicolon => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Let(
                                let_kw.clone(),
                                LetStatement {
                                    span: span_range_including_end(
                                        file_id,
                                        &let_kw,
                                        end_delimiter.raw(),
                                    ),
                                    name: name.clone(),
                                    value: expression,
                                },
                            ))
                        }
                        _ => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
