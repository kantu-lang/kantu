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
                        let n = get_n_from_super_n_token(&token).expect("super_n_token.content should be of the form \"superN\" where N is a positive integer");
                        let span = span_range_including_end(file_id, &l_paren_token, &token);
                        *self = UnfinishedParenthesizedModScopeModifier::ReadyForRParen {
                            l_paren_token: l_paren_token.clone(),
                            modifier: ParenthesizedModScopeModifier {
                                span,
                                kind: ModScopeModifierKind::Super(n),
                            },
                        };
                        AcceptResult::ContinueToNextToken
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
