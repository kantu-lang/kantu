use super::*;

impl Accept for UnfinishedDelimitedCallArg {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedDelimitedCallArg::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        *self = UnfinishedDelimitedCallArg::Colon(token);
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::StandardIdentifier => {
                        let identifier = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedDelimitedCallArg::Identifier {
                            first_token: token,
                            identifier,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _ => {
                        *self = UnfinishedDelimitedCallArg::Unlabeled;
                        AcceptResult::PushAndContinueReducingWithNewTop(
                            UnfinishedStackItem::UnfinishedDelimitedExpression(
                                UnfinishedDelimitedExpression::Empty,
                            ),
                            FinishedStackItem::Token(token),
                        )
                    }
                },

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedDelimitedCallArg::Colon(colon) => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::StandardIdentifier => {
                    let label = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::new(token.content),
                    };
                    *self = UnfinishedDelimitedCallArg::ColonIdentifier(colon.clone(), label);
                    AcceptResult::ContinueToNextToken
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedDelimitedCallArg::ColonIdentifier(colon, label_and_value) => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Ok(end_delimiter) => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::DelimitedCallArg(
                                colon.clone(),
                                CallArg {
                                    span: span_single(file_id, colon)
                                        .inclusive_merge(label_and_value.span),
                                    label_clause: Some(ParamLabelClause::Implicit),
                                    value: Expression::Identifier(label_and_value.clone()),
                                },
                                end_delimiter,
                            ),
                        ),
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::unexpected_token(original_token))
                        }
                    }
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedDelimitedCallArg::Identifier {
                first_token,
                identifier,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        *self = UnfinishedDelimitedCallArg::IdentifierColon(identifier.clone());
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _ => {
                        let token = ExpressionEndDelimiter::try_new(token);
                        match token {
                            Ok(end_delimiter) => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::DelimitedCallArg(
                                    first_token.clone(),
                                    CallArg {
                                        span: identifier.span,
                                        label_clause: None,
                                        value: Expression::Identifier(identifier.clone()),
                                    },
                                    end_delimiter,
                                ),
                            ),
                            Err(original_token) => {
                                let accept_result = AcceptResult::PushAndContinueReducingWithNewTop(
                                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                                        UnfinishedDelimitedExpression::WaitingForEndDelimiter(
                                            first_token.clone(),
                                            Expression::Identifier(identifier.clone()),
                                        ),
                                    ),
                                    FinishedStackItem::Token(original_token),
                                );
                                *self = UnfinishedDelimitedCallArg::Unlabeled;
                                accept_result
                            }
                        }
                    }
                },

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedDelimitedCallArg::IdentifierColon(label) => match item {
                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::DelimitedCallArg(
                        first_token,
                        CallArg {
                            span: label.span.inclusive_merge(expression.span()),
                            label_clause: Some(ParamLabelClause::Explicit(label.clone())),
                            value: expression,
                        },
                        end_delimiter,
                    ))
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedDelimitedCallArg::Unlabeled => match item {
                FinishedStackItem::DelimitedExpression(first_token, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::DelimitedCallArg(
                        first_token,
                        CallArg {
                            span: expression.span(),
                            label_clause: None,
                            value: expression,
                        },
                        end_delimiter,
                    ))
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
