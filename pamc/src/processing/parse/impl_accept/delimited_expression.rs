use super::*;

impl Accept for UnfinishedDelimitedExpression {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedDelimitedExpression::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::TypeTitleCase => {
                        let expression = Expression::Identifier(Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Reserved(ReservedIdentifierName::TypeTitleCase),
                        });
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            token, expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Underscore => {
                        let expression = Expression::Identifier(Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                        });
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            token, expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::StandardIdentifier => {
                        let expression = Expression::Identifier(Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        });
                        *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                            token, expression,
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Fun => {
                        AcceptResult::Push(UnfinishedStackItem::Fun(UnfinishedFun::Keyword(token)))
                    }
                    TokenKind::Match => AcceptResult::Push2(
                        UnfinishedStackItem::Match(UnfinishedMatch::Keyword(token)),
                        UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ),
                    ),
                    TokenKind::Forall => AcceptResult::Push(UnfinishedStackItem::Forall(
                        UnfinishedForall::Keyword(token),
                    )),
                    TokenKind::Check => AcceptResult::Push(UnfinishedStackItem::Check(
                        UnfinishedCheck::Keyword(token),
                    )),
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::DelimitedExpression(
                        first_token,
                        expression,
                        end_delimiter,
                    ))
                }
                FinishedStackItem::UndelimitedExpression(first_token, expression) => {
                    *self = UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                        first_token,
                        expression,
                    );
                    AcceptResult::ContinueToNextToken
                }
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedDelimitedExpression::WaitingForEndDelimiter(first_token, expression) => {
                match item {
                    FinishedStackItem::Token(token) => {
                        let token = ExpressionEndDelimiter::try_new(token);
                        let token = match token {
                            Err(original_token) => original_token,
                            Ok(wrapped_token) => {
                                return AcceptResult::PopAndContinueReducing(
                                    FinishedStackItem::DelimitedExpression(
                                        first_token.clone(),
                                        expression.clone(),
                                        wrapped_token,
                                    ),
                                )
                            }
                        };
                        match token.kind {
                            TokenKind::Dot => {
                                let unfinished = UnfinishedStackItem::Dot(UnfinishedDot {
                                    first_token: first_token.clone(),
                                    left: expression.clone(),
                                });
                                *self = UnfinishedDelimitedExpression::Empty;
                                AcceptResult::Push(unfinished)
                            }
                            TokenKind::LParen => {
                                let unfinished = UnfinishedStackItem::Call(UnfinishedCall {
                                    first_token: first_token.clone(),
                                    callee: expression.clone(),
                                    args: vec![],
                                });
                                *self = UnfinishedDelimitedExpression::Empty;
                                AcceptResult::Push2(
                                    unfinished,
                                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                                        UnfinishedDelimitedExpression::Empty,
                                    ),
                                )
                            }
                            _other_token_kind => {
                                AcceptResult::Error(ParseError::UnexpectedToken(token))
                            }
                        }
                    }
                    other_item => unexpected_finished_item(&other_item),
                }
            }
        }
    }
}
