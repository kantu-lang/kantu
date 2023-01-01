use super::*;

impl Accept for UnfinishedTypeStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedTypeStatement::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Pub => {
                        *self = UnfinishedTypeStatement::ExplicitVisibility {
                            first_token: token.clone(),
                            visibility: PendingPubClause::PubKw(token),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::TypeLowerCase => {
                        *self = UnfinishedTypeStatement::Keyword {
                            first_token: token,
                            visibility: None,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedTypeStatement::ExplicitVisibility {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        if let PendingPubClause::PubKw(_) = visibility {
                            AcceptResult::PushAndContinueReducingWithNewTop(
                                UnfinishedStackItem::ParenthesizedAncestorlike(
                                    UnfinishedParenthesizedAncestorlike::Empty,
                                ),
                                FinishedStackItem::Token(token),
                            )
                        } else {
                            AcceptResult::Error(ParseError::unexpected_token(token))
                        }
                    }
                    TokenKind::TypeLowerCase => {
                        *self = UnfinishedTypeStatement::Keyword {
                            first_token: first_token.clone(),
                            visibility: Some(visibility.clone().finalize(file_id)),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::ParenthesizedAncestorlike(
                    ancestorlike_first_token,
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
                            &FinishedStackItem::ParenthesizedAncestorlike(
                                ancestorlike_first_token,
                                ancestor,
                            ),
                        )
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedTypeStatement::Keyword {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::StandardIdentifier => {
                        let name = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content.clone()),
                        };
                        *self = UnfinishedTypeStatement::Name {
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

            UnfinishedTypeStatement::Name {
                first_token,
                visibility,
                name,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        AcceptResult::Push(UnfinishedStackItem::Params(UnfinishedParams {
                            first_token: token,
                            maximum_dashed_params_allowed: 0,
                            pending_tilde: None,
                            pending_dash: None,
                            params: vec![],
                        }))
                    }
                    TokenKind::LCurly => {
                        *self = UnfinishedTypeStatement::Variants {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            name: name.clone(),
                            params: None,
                            variants: vec![],
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::Params(_, params) => {
                    *self = UnfinishedTypeStatement::Params {
                        first_token: first_token.clone(),
                        visibility: visibility.clone(),
                        name: name.clone(),
                        params: Some(params),
                    };
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedTypeStatement::Params {
                first_token,
                visibility,
                name,
                params,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LCurly => {
                        *self = UnfinishedTypeStatement::Variants {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            name: name.clone(),
                            params: params.clone(),
                            variants: vec![],
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedTypeStatement::Variants {
                first_token,
                visibility,
                name,
                params,
                variants,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Dot => AcceptResult::Push(UnfinishedStackItem::Variant(
                        UnfinishedVariant::Dot(token),
                    )),
                    TokenKind::RCurly => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                            first_token.clone(),
                            TypeStatement {
                                span: span_range_including_end(file_id, &first_token, &token),
                                visibility: visibility.clone(),
                                name: name.clone(),
                                params: params.clone(),
                                variants: variants.clone(),
                            },
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::Variant(_, variant, end_delimiter) => {
                    variants.push(variant);
                    match end_delimiter.raw().kind {
                        TokenKind::Comma => AcceptResult::ContinueToNextToken,
                        TokenKind::RCurly => {
                            AcceptResult::PopAndContinueReducing(FinishedStackItem::Type(
                                first_token.clone(),
                                TypeStatement {
                                    span: span_range_including_end(
                                        file_id,
                                        &first_token,
                                        end_delimiter.raw(),
                                    ),
                                    visibility: visibility.clone(),
                                    name: name.clone(),
                                    params: params.clone(),
                                    variants: variants.clone(),
                                },
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
