use super::*;

impl Accept for UnfinishedModStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedModStatement::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Pub => {
                        *self = UnfinishedModStatement::ExplicitVisibility {
                            first_token: token.clone(),
                            visibility: PendingPubClause::PubKw(token),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Mod => {
                        *self = UnfinishedModStatement::Keyword {
                            first_token: token,
                            visibility: None,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedModStatement::ExplicitVisibility {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        if let PendingPubClause::PubKw(_) = visibility {
                            AcceptResult::PushAndContinueReducingWithNewTop(
                                UnfinishedStackItem::ParenthesizedQuasiAncestor(
                                    UnfinishedParenthesizedQuasiAncestor::Empty,
                                ),
                                FinishedStackItem::Token(token),
                            )
                        } else {
                            AcceptResult::Error(ParseError::unexpected_token(token))
                        }
                    }
                    TokenKind::Mod => {
                        *self = UnfinishedModStatement::Keyword {
                            first_token: first_token.clone(),
                            visibility: Some(visibility.clone().finalize(file_id)),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::ParenthesizedQuasiAncestor(
                    quasi_ancestor_first_token,
                    ancestor,
                ) => {
                    if let PendingPubClause::PubKw(pub_kw_token) = visibility {
                        *visibility = PendingPubClause::Finished(PubClause {
                            span: span_single(file_id, pub_kw_token).inclusive_merge(ancestor.span),
                            ancestor: Some(ancestor),
                        });
                        AcceptResult::ContinueToNextToken
                    } else {
                        wrapped_unexpected_finished_item_err(
                            &FinishedStackItem::ParenthesizedQuasiAncestor(
                                quasi_ancestor_first_token,
                                ancestor,
                            ),
                        )
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedModStatement::Keyword {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedModStatement::Name {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            name,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedModStatement::Name {
                first_token,
                visibility,
                name,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::Semicolon => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Mod(
                        first_token.clone(),
                        ModStatement {
                            span: span_range_including_end(file_id, first_token, &token),
                            visibility: visibility.clone(),
                            name: name.clone(),
                        },
                    ))
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
