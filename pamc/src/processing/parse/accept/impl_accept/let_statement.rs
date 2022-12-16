use super::*;

impl Accept for UnfinishedLetStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
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
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedLetStatement::Name(let_kw, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Equal => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
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
                        _ => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}