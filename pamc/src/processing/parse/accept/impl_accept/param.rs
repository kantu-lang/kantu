use super::*;

impl Accept for UnfinishedParam {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParam::NoExplicitLabel {
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
                            label: if *is_tilded {
                                Some(ParamLabel::Implicit)
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
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Standard(token.content.clone()),
                        };
                        *self = UnfinishedParam::ExplicitLabelAndName {
                            first_token: first_token.clone(),
                            is_dashed: *is_dashed,
                            label: label.clone(),
                            name,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Underscore => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::Reserved(ReservedIdentifierName::Underscore),
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
                            label: Some(ParamLabel::Explicit(label.clone())),
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
