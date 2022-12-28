use super::*;

impl Accept for UnfinishedParenthesizedWeakAncestor {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParenthesizedWeakAncestor::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LParen => {
                    *self = UnfinishedParenthesizedWeakAncestor::LParen(token);
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedParenthesizedWeakAncestor::LParen(l_paren_token) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Star => {
                        *self = UnfinishedParenthesizedWeakAncestor::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedWeakAncestor {
                                span: span_single(file_id, &token),
                                kind: WeakAncestorKind::Global,
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Mod => {
                        *self = UnfinishedParenthesizedWeakAncestor::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedWeakAncestor {
                                span: span_single(file_id, &token),
                                kind: WeakAncestorKind::Mod,
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
                        *self = UnfinishedParenthesizedWeakAncestor::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            ancestor: ParenthesizedWeakAncestor {
                                span: span_single(file_id, &token),
                                kind: WeakAncestorKind::PackageRelative {
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

            UnfinishedParenthesizedWeakAncestor::ReadyForRParen {
                l_paren_token,
                ancestor,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::RParen => {
                    AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::ParenthesizedWeakAncestor(
                            l_paren_token.clone(),
                            ParenthesizedWeakAncestor {
                                span: ancestor.span.inclusive_merge(span_single(file_id, &token)),
                                kind: ancestor.kind.clone(),
                            },
                        ),
                    )
                }
                FinishedStackItem::Token(token) if token.kind == TokenKind::Dot => {
                    match &mut ancestor.kind {
                        WeakAncestorKind::PackageRelative { path_after_pack_kw } => {
                            *self = UnfinishedParenthesizedWeakAncestor::PackageRelativeAwaitingIdentifier {
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

            UnfinishedParenthesizedWeakAncestor::PackageRelativeAwaitingIdentifier {
                l_paren_token,
                path_after_pack_kw,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::StandardIdentifier => {
                    let component_span = span_single(file_id, &token);
                    let component = Identifier {
                        span: component_span,
                        name: IdentifierName::Standard(token.content),
                    };
                    path_after_pack_kw.push(component);
                    *self = UnfinishedParenthesizedWeakAncestor::ReadyForRParen {
                        l_paren_token: l_paren_token.clone(),
                        ancestor: ParenthesizedWeakAncestor {
                            span: span_single(file_id, &l_paren_token)
                                .inclusive_merge(component_span),
                            kind: WeakAncestorKind::PackageRelative {
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

impl UnfinishedParenthesizedWeakAncestor {
    fn set_to_super_n(
        &mut self,
        file_id: FileId,
        l_paren_token: Token,
        super_n_token: &Token,
        n: NonZeroUsize,
    ) -> AcceptResult {
        *self = UnfinishedParenthesizedWeakAncestor::ReadyForRParen {
            l_paren_token: l_paren_token,
            ancestor: ParenthesizedWeakAncestor {
                span: span_single(file_id, super_n_token),
                kind: WeakAncestorKind::Super(n),
            },
        };
        AcceptResult::ContinueToNextToken
    }
}
