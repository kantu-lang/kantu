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
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedMatchCase::VariantName(dot_token, variant_name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        *self = UnfinishedMatchCase::ParamsInProgress(
                            dot_token.clone(),
                            variant_name.clone(),
                            vec![],
                            CurrentlyHasEndingComma(false),
                        );
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::FatArrow => {
                        *self = UnfinishedMatchCase::AwaitingOutput(
                            dot_token.clone(),
                            variant_name.clone(),
                            None,
                        );
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedMatchCase::ParamsInProgress(
                dot_token,
                variant_name,
                params,
                currently_has_ending_comma,
            ) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let can_accept_identifier =
                            params.is_empty() || currently_has_ending_comma.0;
                        if can_accept_identifier {
                            let name = Identifier {
                                span: span_single(file_id, &token),
                                name: IdentifierName::Standard(token.content.clone()),
                            };
                            params.push(name);
                            currently_has_ending_comma.0 = false;
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    }
                    TokenKind::Underscore => {
                        let can_accept_identifier =
                            params.is_empty() || currently_has_ending_comma.0;
                        if can_accept_identifier {
                            let name = Identifier {
                                span: span_single(file_id, &token),
                                name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                            };
                            params.push(name);
                            currently_has_ending_comma.0 = false;
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    }
                    TokenKind::Comma => {
                        let can_accept_comma = !currently_has_ending_comma.0 && !params.is_empty();
                        if can_accept_comma {
                            currently_has_ending_comma.0 = true;
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::UnexpectedToken(token))
                        }
                    }
                    TokenKind::RParen => match NonEmptyVec::try_from(params.clone()) {
                        Ok(params) => {
                            *self = UnfinishedMatchCase::AwaitingOutput(
                                dot_token.clone(),
                                variant_name.clone(),
                                Some(params),
                            );
                            AcceptResult::ContinueToNextToken
                        }
                        Err(_) => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                    },
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                other_item => unexpected_finished_item(&other_item),
            },
            UnfinishedMatchCase::AwaitingOutput(dot_token, variant_name, params) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::FatArrow => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
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
                                    output: expression,
                                },
                                end_delimiter,
                            ))
                        }
                        _other_end_delimiter => AcceptResult::Error(ParseError::UnexpectedToken(
                            end_delimiter.into_raw(),
                        )),
                    }
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}
