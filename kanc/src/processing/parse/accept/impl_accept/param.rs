use super::*;

impl Accept for UnfinishedParam {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParam::NoIdentifier {
                pending_tilde,
                pending_dash,
                is_dash_allowed,
            } => {
                match item {
                    FinishedStackItem::Token(token) => match token.kind {
                        TokenKind::Tilde => {
                            if pending_dash.is_some() {
                                // A tilde can never come after a dash.
                                AcceptResult::Error(ParseError::unexpected_token(token))
                            } else if pending_tilde.is_some() {
                                // Double tildes are forbidden.
                                AcceptResult::Error(ParseError::unexpected_token(token))
                            } else {
                                *pending_tilde = Some(token);
                                AcceptResult::ContinueToNextToken
                            }
                        }
                        TokenKind::Dash => {
                            if *is_dash_allowed && pending_dash.is_none() {
                                *pending_dash = Some(token);
                                AcceptResult::ContinueToNextToken
                            } else {
                                AcceptResult::Error(ParseError::unexpected_token(token))
                            }
                        }
                        TokenKind::StandardIdentifier | TokenKind::Underscore => {
                            let name_or_label = Identifier {
                                span: span_single(file_id, &token),
                                name: IdentifierName::new(token.content.clone()),
                            };

                            let pending_tilde = pending_tilde.take();
                            let pending_dash = pending_dash.take();
                            let is_tilded = pending_tilde.is_some();
                            let is_dashed = pending_dash.is_some();
                            *self = UnfinishedParam::FirstIdentifier {
                                first_token: pending_tilde
                                    .unwrap_or_else(|| pending_dash.unwrap_or_else(|| token)),
                                is_tilded,
                                is_dashed,
                                is_dash_allowed: *is_dash_allowed,
                                name_or_label,
                            };
                            AcceptResult::ContinueToNextToken
                        }
                        _ => AcceptResult::Error(ParseError::unexpected_token(token)),
                    },

                    other_item => wrapped_unexpected_finished_item_err(&other_item),
                }
            }
            UnfinishedParam::FirstIdentifier {
                first_token,
                is_tilded,
                is_dashed,
                is_dash_allowed,
                name_or_label,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    TokenKind::Tilde => {
                        if !*is_tilded && !*is_dashed {
                            *self = UnfinishedParam::ExplicitLabel {
                                first_token: first_token.clone(),
                                is_dashed: false,
                                is_dash_allowed: *is_dash_allowed,
                                label: name_or_label.clone(),
                            };
                            AcceptResult::ContinueToNextToken
                        } else {
                            AcceptResult::Error(ParseError::unexpected_token(token))
                        }
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Param(
                        first_token.clone(),
                        Param {
                            span: span_range_excluding_end(
                                file_id,
                                &first_token,
                                end_delimiter.raw(),
                            ),
                            label_clause: if *is_tilded {
                                Some(ParamLabelClause::Implicit)
                            } else {
                                None
                            },
                            is_dashed: *is_dashed,
                            name: name_or_label.clone(),
                            type_: expression,
                        },
                        end_delimiter,
                    ))
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParam::ExplicitLabel {
                first_token,
                is_dashed,
                is_dash_allowed,
                label,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Dash => {
                        let is_dash_forbidden = !*is_dash_allowed;
                        if *is_dashed || is_dash_forbidden {
                            AcceptResult::Error(ParseError::unexpected_token(token))
                        } else {
                            *is_dashed = true;
                            AcceptResult::ContinueToNextToken
                        }
                    }
                    TokenKind::StandardIdentifier | TokenKind::Underscore => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedParam::ExplicitLabelAndName {
                            first_token: first_token.clone(),
                            is_dashed: *is_dashed,
                            label: label.clone(),
                            name,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParam::ExplicitLabelAndName {
                first_token,
                is_dashed,
                label,
                name,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Param(
                        first_token.clone(),
                        Param {
                            span: span_range_excluding_end(
                                file_id,
                                &first_token,
                                end_delimiter.raw(),
                            ),
                            label_clause: Some(ParamLabelClause::Explicit(label.clone())),
                            is_dashed: *is_dashed,
                            name: name.clone(),
                            type_: expression,
                        },
                        end_delimiter,
                    ))
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
