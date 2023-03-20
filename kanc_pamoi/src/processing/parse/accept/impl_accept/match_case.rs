use super::*;

impl Accept for UnfinishedMatchCase {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedMatchCase::VariantName(variant_name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        *self = UnfinishedMatchCase::ParamsInProgress(variant_name.clone(), vec![]);
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::FatArrow => {
                        *self = UnfinishedMatchCase::AwaitingOutput {
                            variant_name: variant_name.clone(),
                            params: None,
                            triple_dot: None,
                        };
                        AcceptResult::Push(
                            UnfinishedStackItem::UnfinishedDelimitedImpossibleKwOrExpression(
                                UnfinishedDelimitedImpossibleKwOrExpression::Empty,
                            ),
                        )
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedMatchCase::ParamsInProgress(variant_name, params) => match item {
                FinishedStackItem::MatchCaseParam(_, param, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Comma => {
                            params.push(param);
                            AcceptResult::ContinueToNextToken
                        }
                        TokenKind::RParen => {
                            let params = NonEmptyVec::from_pushed(params.clone(), param);
                            *self = UnfinishedMatchCase::AwaitingOutput {
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
                variant_name,
                params,
                triple_dot,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::FatArrow => AcceptResult::Push(
                        UnfinishedStackItem::UnfinishedDelimitedImpossibleKwOrExpression(
                            UnfinishedDelimitedImpossibleKwOrExpression::Empty,
                        ),
                    ),
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::DelimitedImpossibleKwOrExpression(_, output, end_delimiter) => {
                    match end_delimiter.raw().kind {
                        TokenKind::Comma | TokenKind::RCurly => {
                            let first_token = token_from_standard_identifier(variant_name);
                            let span = span_range_excluding_end(
                                file_id,
                                &first_token,
                                end_delimiter.raw(),
                            );
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::MatchCase(
                                first_token,
                                MatchCase {
                                    span,
                                    variant_name: variant_name.clone(),
                                    params: params.clone(),
                                    triple_dot: triple_dot.clone(),
                                    output,
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
