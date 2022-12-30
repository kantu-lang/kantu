use super::*;

impl Accept for UnfinishedUseStatement {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedUseStatement::Empty => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Pub => {
                        *self = UnfinishedUseStatement::ExplicitVisibility {
                            first_token: token.clone(),
                            visibility: PendingPubClause::PubKw(token),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Use => {
                        *self = UnfinishedUseStatement::Keyword {
                            first_token: token,
                            visibility: None,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedUseStatement::ExplicitVisibility {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => {
                        if let PendingPubClause::PubKw(_) = visibility {
                            AcceptResult::PushAndContinueReducingWithNewTop(
                                UnfinishedStackItem::ParenthesizedWeakAncestor(
                                    UnfinishedParenthesizedWeakAncestor::Empty,
                                ),
                                FinishedStackItem::Token(token),
                            )
                        } else {
                            AcceptResult::Error(ParseError::unexpected_token(token))
                        }
                    }
                    TokenKind::Use => {
                        *self = UnfinishedUseStatement::Keyword {
                            first_token: first_token.clone(),
                            visibility: Some(visibility.clone().finalize(file_id)),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                FinishedStackItem::ParenthesizedWeakAncestor(
                    weak_ancestor_first_token,
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
                            &FinishedStackItem::ParenthesizedWeakAncestor(
                                weak_ancestor_first_token,
                                ancestor,
                            ),
                        )
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedUseStatement::Keyword {
                first_token,
                visibility,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Mod => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Mod,
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    TokenKind::Super => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(1).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super2 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(2).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super3 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(3).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super4 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(4).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super5 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(5).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super6 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(6).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super7 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(7).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Super8 => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Super(
                                NonZeroUsize::new(8).unwrap(),
                            ),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    TokenKind::StandardIdentifier => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Identifier(IdentifierName::new(
                                token.content,
                            )),
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    TokenKind::Pack => {
                        let first_component = UseStatementFirstComponent {
                            span: span_single(file_id, &token),
                            kind: UseStatementFirstComponentKind::Pack,
                        };
                        *self = UnfinishedUseStatement::AtLeastOneComponent {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component,
                            other_components: vec![],
                            has_trailing_dot: false,
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedUseStatement::AtLeastOneComponent {
                first_token,
                visibility,
                first_component,
                other_components,
                has_trailing_dot,
            } => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Dot if !*has_trailing_dot => {
                        *has_trailing_dot = true;
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::As if !*has_trailing_dot => {
                        *self = UnfinishedUseStatement::As {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component: first_component.clone(),
                            other_components: other_components.clone(),
                        };
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Semicolon if !*has_trailing_dot => {
                        AcceptResult::PopAndContinueReducing(FinishedStackItem::Use(
                            first_token.clone(),
                            UseStatement {
                                span: span_range_including_end(file_id, &first_token, &token),
                                visibility: visibility.clone(),
                                first_component: first_component.clone(),
                                other_components: other_components.clone(),
                                import_modifier: None,
                            },
                        ))
                    }

                    TokenKind::StandardIdentifier if *has_trailing_dot => {
                        *has_trailing_dot = false;
                        let component = Identifier {
                            span: span_single(file_id, &token),
                            name: IdentifierName::new(token.content),
                        };
                        other_components.push(component);
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::Star if *has_trailing_dot => {
                        *self = UnfinishedUseStatement::FinishedImportModifier {
                            first_token: first_token.clone(),
                            visibility: visibility.clone(),
                            first_component: first_component.clone(),
                            other_components: other_components.clone(),
                            import_modifier: WildcardOrAlternateName {
                                span: span_single(file_id, &token),
                                kind: WildcardOrAlternateNameKind::Wildcard,
                            },
                        };
                        AcceptResult::ContinueToNextToken
                    }

                    _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedUseStatement::As {
                first_token,
                visibility,
                first_component,
                other_components,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::StandardIdentifier => {
                    *self = UnfinishedUseStatement::FinishedImportModifier {
                        first_token: first_token.clone(),
                        visibility: visibility.clone(),
                        first_component: first_component.clone(),
                        other_components: other_components.clone(),
                        import_modifier: WildcardOrAlternateName {
                            span: span_single(file_id, &token),
                            kind: WildcardOrAlternateNameKind::AlternateName(IdentifierName::new(
                                token.content,
                            )),
                        },
                    };
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedUseStatement::FinishedImportModifier {
                first_token,
                visibility,
                first_component,
                other_components,
                import_modifier,
            } => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::Semicolon => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Use(
                        first_token.clone(),
                        UseStatement {
                            span: span_range_including_end(file_id, &first_token, &token),
                            visibility: visibility.clone(),
                            first_component: first_component.clone(),
                            other_components: other_components.clone(),
                            import_modifier: Some(import_modifier.clone()),
                        },
                    ))
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
