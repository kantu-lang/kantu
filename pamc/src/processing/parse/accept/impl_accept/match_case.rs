use super::*;

impl Accept for UnfinishedMatchCase {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedMatchCase::Dot(dot_token) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedMatchCase::VariantName(dot_token.clone(), name);
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedMatchCase::VariantName(dot_token, variant_name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        *self = UnfinishedMatchCase::ParamsInProgress(
                            dot_token.clone(),
                            variant_name.clone(),
                            vec![],
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::FatArrow => {
                        *self = UnfinishedMatchCase::AwaitingOutput {
                            dot_token: dot_token.clone(),
                            variant_name: variant_name.clone(),
                            params: None,
                            triple_dot: None,
                        };
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedMatchCase::ParamsInProgress(dot_token, variant_name, params) => match item {
                FinishedStackItem::MatchCaseParam(_, param, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Comma => {
                            params.push(param);
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::RParen => {
                            let params = NonEmptyVec::from_pushed(params.clone(), param);
                            *self = UnfinishedMatchCase::AwaitingOutput {
                                dot_token: dot_token.clone(),
                                variant_name: variant_name.clone(),
                                params: Some(params),
                                triple_dot: None,
                            };
                            AcceptResult::ContinueToNextToken
                        }
                        _ => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }

                FinishedStackItem::DelimitedTripleDot(triple_dot, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::RParen => {
                            let params = NonEmptyVec::try_from(params.clone()).ok();
                            *self = UnfinishedMatchCase::AwaitingOutput {
                                dot_token: dot_token.clone(),
                                variant_name: variant_name.clone(),
                                params,
                                triple_dot: Some(span_single(file_id, &triple_dot)),
                            };
                            AcceptResult::ContinueToNextToken
                        }
                        _ => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }

                FinishedStackItem::Token(token) if token.kind == TokenKind::RParen => {
                    let Ok(params) = NonEmptyVec::try_from(params.clone()) else {
                        return AcceptResult::Error(ParseError::unexpected_token(token));
                    };
                    *self = UnfinishedMatchCase::AwaitingOutput {
                        dot_token: dot_token.clone(),
                        variant_name: variant_name.clone(),
                        params: Some(params),
                        triple_dot: None,
                    };
                    AcceptResult::ContinueToNextToken
                }

                FinishedStackItem::Token(token) if token.kind == TokenKind::TripleDot => {
                    AcceptResult::PushAndContinueReducingWithNewTop(
                        UnfinishedStackItem::UnfinishedDelimitedTripleDot(
                            UnfinishedDelimitedTripleDot::Empty,
                        ),
                        FinishedStackItem::Token(token),
                    )
                }

                other_item => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::MatchCaseParam(UnfinishedMatchCaseParam::Empty),
                    other_item,
                ),
            },
            UnfinishedMatchCase::AwaitingOutput {
                dot_token,
                variant_name,
                params,
                triple_dot,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::FatArrow => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Comma | TokenKind::RCurly => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::MatchCase(
                                dot_token.clone(),
                                MatchCase {
                                    span: span_range_excluding_end(
                                        file_id,
                                        &dot_token,
                                        end_delimiter.raw(),
                                    ),
                                    variant_name: variant_name.clone(),
                                    params: params.clone(),
                                    triple_dot: triple_dot.clone(),
                                    output: expression,
                                },
                                end_delimiter,
                            ))
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::unexpected_token(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
