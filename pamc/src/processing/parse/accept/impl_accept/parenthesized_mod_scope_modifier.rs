use super::*;

impl Accept for UnfinishedParenthesizedModScopeModifier {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParenthesizedModScopeModifier::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LParen => {
                    *self = UnfinishedParenthesizedModScopeModifier::LParen(token);
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParenthesizedModScopeModifier::LParen(l_paren_token) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Star => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            modifier: ParenthesizedModScopeModifier {
                                span,
                                kind: ModScopeModifierKind::Global,
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Mod => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            modifier: ParenthesizedModScopeModifier {
                                span,
                                kind: ModScopeModifierKind::Mod,
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
                        *self = UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            modifier: ParenthesizedModScopeModifier {
                                span,
                                kind: ModScopeModifierKind::PackRelative {
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

            UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
                l_paren_token,
                modifier,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::RParen => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::ParenthesizedModScopeModifier(
                            l_paren_token.clone(),
                            ParenthesizedModScopeModifier {
                                span: modifier.span.inclusive_merge(span_single(file_id, &token)),
                                kind: modifier.kind.clone(),
                            },
                        ),
                    )
                }
                FinishedStackItem::Token(token) if token.kind == TokenKind::Dot => {
                    match &mut modifier.kind {
                        ModScopeModifierKind::PackRelative { path_after_pack_kw } => {
                            *self = UnfinishedParenthesizedModScopeModifier::PackRelativeAwaitingIdentifier {
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

            UnfinishedParenthesizedModScopeModifier::PackRelativeAwaitingIdentifier {
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
                    *self = UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
                        l_paren_token: l_paren_token.clone(),
                        modifier: ParenthesizedModScopeModifier {
                            span: span_single(file_id, &l_paren_token)
                                .inclusive_merge(component_span),
                            kind: ModScopeModifierKind::PackRelative {
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

impl UnfinishedParenthesizedModScopeModifier {
    fn set_to_super_n(
        &mut self,
        file_id: FileId,
        l_paren_token: Token,
        super_n_token: &Token,
        n: NonZeroUsize,
    ) -> AcceptResult {
        let span = span_range_including_end(file_id, &l_paren_token, super_n_token);
        *self = UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
            l_paren_token: l_paren_token,
            modifier: ParenthesizedModScopeModifier {
                span,
                kind: ModScopeModifierKind::Super(n),
            },
        };
        AcceptResult::ContinueToNextToken
    }
}
