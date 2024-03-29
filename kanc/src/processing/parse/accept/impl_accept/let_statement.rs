use super::*;

impl Accept for UnfinishedLetStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedLetStatement::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Pub => {
                        *self = UnfinishedLetStatement::ExplicitVisibility {
                            first_token: token.clone(),
                            visibility: PendingPubClause::PubKw(token),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Let => {
                        *self = UnfinishedLetStatement::Keyword {
                            first_token: token,
                            visibility: None,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedLetStatement::ExplicitVisibility {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        if let PendingPubClause::PubKw(_) = visibility {
                            AcceptResult::PushAndContinueReducingWithNewTop(
                                UnfinishedStackItem::ParenthesizedModScopeModifier(
                                    UnfinishedParenthesizedModScopeModifier::Empty,
                                ),
                                FinishedStackItem::Token(token),
                            )
                        } else {
                            AcceptResult::Error(ParseError::unexpected_token(token))
                        }
                    }
                    TokenKind::Let => {
                        *self = UnfinishedLetStatement::Keyword {
                            first_token: first_token.clone(),
                            visibility: Some(visibility.clone().finalize(file_id)),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::ParenthesizedModScopeModifier(
                    parenthesized_mod_scope_modifier_first_token,
                    modifier,
                ) => {
                    if let PendingPubClause::PubKw(pub_kw_token) = visibility {
                        *visibility = PendingPubClause::Finished(PubClause {
                            span: span_single(file_id, pub_kw_token).inclusive_merge(modifier.span),
                            scope_modifier: Some(modifier),
                        });
                        AcceptResult::ContinueToNextToken
                    } else {
                        wrapped_unexpected_finished_item_err(
                            &FinishedStackItem::ParenthesizedModScopeModifier(
                                parenthesized_mod_scope_modifier_first_token,
                                modifier,
                            ),
                        )
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedLetStatement::Keyword {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => AcceptResult::PushAndContinueReducingWithNewTop(
                        UnfinishedStackItem::ParenthesizedModScopeModifier(
                            UnfinishedParenthesizedModScopeModifier::Empty,
                        ),
                        FinishedStackItem::Token(token),
                    ),
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedLetStatement::Name {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            transparency: None,
                            name,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::ParenthesizedModScopeModifier(_, transparency) => {
                    *self = UnfinishedLetStatement::ExplicitTransparency {
                        first_token: first_token.clone(),
                        visibility: visibility.clone(),
                        transparency,
                    };
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedLetStatement::ExplicitTransparency {
                first_token,
                visibility,
                transparency,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedLetStatement::Name {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            transparency: Some(transparency.clone()),
                            name,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedLetStatement::Name {
                first_token,
                visibility,
                transparency,
                name,
            } => match item {
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
                                first_token.clone(),
                                LetStatement {
                                    span: span_range_including_end(
                                        file_id,
                                        &first_token,
                                        end_delimiter.raw(),
                                    ),
                                    visibility: visibility.clone(),
                                    transparency: transparency.clone(),
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
