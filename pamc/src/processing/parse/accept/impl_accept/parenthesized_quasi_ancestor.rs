use super::*;

impl Accept for UnfinishedParenthesizedQuasiAncestor {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParenthesizedQuasiAncestor::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LParen => {
                    *self = UnfinishedParenthesizedQuasiAncestor::LParen(token);
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParenthesizedQuasiAncestor::LParen(l_paren_token) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Star => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedQuasiAncestor::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedQuasiAncestor {
                                span,
                                kind: QuasiAncestorKind::Global,
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Mod => {
                        let span = span_range_including_end(file_id, l_paren_token, &token);
                        *self = UnfinishedParenthesizedQuasiAncestor::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedQuasiAncestor {
                                span,
                                kind: QuasiAncestorKind::Mod,
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
                        *self = UnfinishedParenthesizedQuasiAncestor::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedQuasiAncestor {
                                span,
                                kind: QuasiAncestorKind::PackRelative {
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

            UnfinishedParenthesizedQuasiAncestor::ReadyForRParen {
                l_paren_token,
                ancestor,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::RParen => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::ParenthesizedQuasiAncestor(
                            l_paren_token.clone(),
                            ParenthesizedQuasiAncestor {
                                span: ancestor.span.inclusive_merge(span_single(file_id, &token)),
                                kind: ancestor.kind.clone(),
                            },
                        ),
                    )
                }
                FinishedStackItem::Token(token) if token.kind == TokenKind::Dot => {
                    match &mut ancestor.kind {
                        QuasiAncestorKind::PackRelative { path_after_pack_kw } => {
                            *self = UnfinishedParenthesizedQuasiAncestor::PackRelativeAwaitingIdentifier {
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

            UnfinishedParenthesizedQuasiAncestor::PackRelativeAwaitingIdentifier {
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
                    *self = UnfinishedParenthesizedQuasiAncestor::ReadyForRParen {
                        l_paren_token: l_paren_token.clone(),
                        ancestor: ParenthesizedQuasiAncestor {
                            span: span_single(file_id, &l_paren_token)
                                .inclusive_merge(component_span),
                            kind: QuasiAncestorKind::PackRelative {
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

impl UnfinishedParenthesizedQuasiAncestor {
    fn set_to_super_n(
        &mut self,
        file_id: FileId,
        l_paren_token: Token,
        super_n_token: &Token,
        n: NonZeroUsize,
    ) -> AcceptResult {
        let span = span_range_including_end(file_id, &l_paren_token, super_n_token);
        *self = UnfinishedParenthesizedQuasiAncestor::ReadyForRParen {
            l_paren_token: l_paren_token,
            ancestor: ParenthesizedQuasiAncestor {
                span,
                kind: QuasiAncestorKind::Super(n),
            },
        };
        AcceptResult::ContinueToNextToken
    }
}
