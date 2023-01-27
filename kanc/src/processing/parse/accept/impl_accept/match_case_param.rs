use super::*;

impl Accept for UnfinishedMatchCaseParam {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedMatchCaseParam::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        *self = UnfinishedMatchCaseParam::Colon(token);
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::StandardIdentifier | TokenKind::Underscore => {
                        let identifier = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedMatchCaseParam::Identifier {
                            first_token: token,
                            identifier,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _ => AcceptResult::Error(ParseError::unexpected_token(token)),
                },

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedMatchCaseParam::Colon(colon) => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::StandardIdentifier => {
                    let label = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::new(token.content),
                    };
                    *self = UnfinishedMatchCaseParam::ColonIdentifier(colon.clone(), label);
                    AcceptResult::ContinueToNextToken
                }

                // This is semantically illegal, but we choose to define it as a _grammatically_ legal
                // term. Thus, we must handle it as a success case (even though the AST simplifier will
                // ultimately reject it down the line).
                FinishedStackItem::Token(token) if token.kind == TokenKind::Underscore => {
                    let label = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
                    };
                    *self = UnfinishedMatchCaseParam::ColonIdentifier(colon.clone(), label);
                    AcceptResult::ContinueToNextToken
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedMatchCaseParam::ColonIdentifier(colon, label_and_value) => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Ok(end_delimiter) => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::MatchCaseParam(
                                colon.clone(),
                                MatchCaseParam {
                                    span: span_single(file_id, colon)
                                        .inclusive_merge(label_and_value.span),
                                    label_clause: Some(ParamLabelClause::Implicit),
                                    name: label_and_value.clone(),
                                },
                                end_delimiter,
                            ))
                        }
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::unexpected_token(original_token))
                        }
                    }
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedMatchCaseParam::Identifier {
                first_token,
                identifier,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        *self = UnfinishedMatchCaseParam::IdentifierColon {
                            first_token: first_token.clone(),
                            label: identifier.clone(),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _ => {
                        let token = ExpressionEndDelimiter::try_new(token);
                        match token {
                            Ok(end_delimiter) => AcceptResult::PopAndContinueReducing(
                                FinishedStackItem::MatchCaseParam(
                                    first_token.clone(),
                                    MatchCaseParam {
                                        span: identifier.span,
                                        label_clause: None,
                                        name: identifier.clone(),
                                    },
                                    end_delimiter,
                                ),
                            ),
                            Err(original_token) => {
                                AcceptResult::Error(ParseError::unexpected_token(original_token))
                            }
                        }
                    }
                },

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedMatchCaseParam::IdentifierColon { first_token, label } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier | TokenKind::Underscore => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content),
                        };
                        *self = UnfinishedMatchCaseParam::IdentifierColonIdentifier {
                            first_token: first_token.clone(),
                            label: label.clone(),
                            name,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _ => AcceptResult::Error(ParseError::unexpected_token(token)),
                },

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedMatchCaseParam::IdentifierColonIdentifier {
                first_token,
                label,
                name,
            } => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Ok(end_delimiter) => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::MatchCaseParam(
                                first_token.clone(),
                                MatchCaseParam {
                                    span: label.span.inclusive_merge(name.span),
                                    label_clause: Some(ParamLabelClause::Explicit(label.clone())),
                                    name: name.clone(),
                                },
                                end_delimiter,
                            ))
                        }
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::unexpected_token(original_token))
                        }
                    }
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
