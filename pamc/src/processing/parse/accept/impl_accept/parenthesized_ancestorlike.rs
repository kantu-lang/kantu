use super::*;

impl Accept for UnfinishedParenthesizedAncestorlike {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParenthesizedAncestorlike::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LParen => {
                    *self = UnfinishedParenthesizedAncestorlike::LParen(token);
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParenthesizedAncestorlike::LParen(l_paren_token) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Star => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedAncestorlike::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedAncestorlike {
                                span,
                                kind: AncestorlikeKind::Global,
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Mod => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedAncestorlike::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedAncestorlike {
                                span,
                                kind: AncestorlikeKind::Mod,
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    TokenKind::Super => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(1).unwrap(),
                        )
                    }
                    TokenKind::Super2 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(2).unwrap(),
                        )
                    }
                    TokenKind::Super3 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(3).unwrap(),
                        )
                    }
                    TokenKind::Super4 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(4).unwrap(),
                        )
                    }
                    TokenKind::Super5 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(5).unwrap(),
                        )
                    }
                    TokenKind::Super6 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(6).unwrap(),
                        )
                    }
                    TokenKind::Super7 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(7).unwrap(),
                        )
                    }
                    TokenKind::Super8 => {
                        let l_paren_token = l_paren_token.clone();
                        self.set_to_super_n(
                            file_id,
                            l_paren_token,
                            &token,
                            NonZeroUsize::new(8).unwrap(),
                        )
                    }

                    TokenKind::Pack => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedAncestorlike::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedAncestorlike {
                                span,
                                kind: AncestorlikeKind::PackRelative {
                                    path_after_pack_kw: vec![],
                                },
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    _ => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParenthesizedAncestorlike::ReadyForRParen {
                l_paren_token,
                ancestor,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::RParen => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::ParenthesizedAncestorlike(
                            l_paren_token.clone(),
                            ParenthesizedAncestorlike {
                                span: ancestor.span.inclusive_merge(span_single(file_id, &token)),
                                kind: ancestor.kind.clone(),
                            },
                        ),
                    )
                }
                FinishedStackItem::Token(token) if token.kind == TokenKind::Dot => {
                    match &mut ancestor.kind {
                        AncestorlikeKind::PackRelative { path_after_pack_kw } => {
                            *self = UnfinishedParenthesizedAncestorlike::PackRelativeAwaitingIdentifier {
                                l_paren_token: l_paren_token.clone(),
                                path_after_pack_kw: path_after_pack_kw.clone(),
                            };
                            AcceptResult::ContinueToNextToken
                        }
                        _ => AcceptResult::Error(ParseError::unexpected_token(token)),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParenthesizedAncestorlike::PackRelativeAwaitingIdentifier {
                l_paren_token,
                path_after_pack_kw,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::StandardIdentifier => {
                    let component_span = span_single(file_id, &token);
                    let component = Identifier {
                        span: component_span,
                        name: IdentifierName::new(token.content),
                    };
                    path_after_pack_kw.push(component);
                    *self = UnfinishedParenthesizedAncestorlike::ReadyForRParen {
                        l_paren_token: l_paren_token.clone(),
                        ancestor: ParenthesizedAncestorlike {
                            span: span_single(file_id, &l_paren_token)
                                .inclusive_merge(component_span),
                            kind: AncestorlikeKind::PackRelative {
                                path_after_pack_kw: path_after_pack_kw.clone(),
                            },
                        },
                    };
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}

impl UnfinishedParenthesizedAncestorlike {
    fn set_to_super_n(
        &mut self,
        file_id: FileId,
        l_paren_token: Token,
        super_n_token: &Token,
        n: NonZeroUsize,
    ) -> AcceptResult {
        let span = span_range_including_end(file_id, &l_paren_token, super_n_token);
        *self = UnfinishedParenthesizedAncestorlike::ReadyForRParen {
            l_paren_token: l_paren_token,
            ancestor: ParenthesizedAncestorlike {
                span,
                kind: AncestorlikeKind::Super(n),
            },
        };
        AcceptResult::ContinueToNextToken
    }
}
