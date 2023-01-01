use super::*;

impl Accept for UnfinishedFile {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::Pub => {
                    if self.pending_visibility.is_some() {
                        AcceptResult::Error(ParseError::unexpected_token(token))
                    } else {
                        self.pending_visibility = Some(PendingPubClause::PubKw(token));
                        AcceptResult::ContinueToNextToken
                    }
                }
                TokenKind::LParen => {
                    if let Some(PendingPubClause::PubKw(_)) = &self.pending_visibility {
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
                TokenKind::Use => {
                    let visibility = self
                        .pending_visibility
                        .take()
                        .map(|visibility| visibility.finalize(file_id));
                    let first_token = visibility.as_ref().map(get_pub_kw_token).unwrap_or(token);
                    AcceptResult::Push(UnfinishedStackItem::Use(UnfinishedUseStatement::Keyword {
                        first_token,
                        visibility,
                    }))
                }
                TokenKind::Mod => {
                    let visibility = self
                        .pending_visibility
                        .take()
                        .map(|visibility| visibility.finalize(file_id));
                    let first_token = visibility.as_ref().map(get_pub_kw_token).unwrap_or(token);
                    AcceptResult::Push(UnfinishedStackItem::Mod(UnfinishedModStatement::Keyword {
                        first_token,
                        visibility,
                    }))
                }
                TokenKind::TypeLowerCase => {
                    let visibility = self
                        .pending_visibility
                        .take()
                        .map(|visibility| visibility.finalize(file_id));
                    let first_token = visibility.as_ref().map(get_pub_kw_token).unwrap_or(token);
                    AcceptResult::Push(UnfinishedStackItem::Type(
                        UnfinishedTypeStatement::Keyword {
                            first_token,
                            visibility,
                        },
                    ))
                }
                TokenKind::Let => {
                    let visibility = self
                        .pending_visibility
                        .take()
                        .map(|visibility| visibility.finalize(file_id));
                    let first_token = visibility.as_ref().map(get_pub_kw_token).unwrap_or(token);
                    AcceptResult::Push(UnfinishedStackItem::Let(UnfinishedLetStatement::Keyword {
                        first_token,
                        visibility,
                    }))
                }
                TokenKind::Eoi => {
                    let span = {
                        let first_span = self.items.first().map(|item| item.span());
                        let last_span = self.items.last().map(|item| item.span());
                        match (first_span, last_span) {
                            (Some(first_span), Some(last_span)) => {
                                first_span.inclusive_merge(last_span)
                            }
                            _ => TextSpan {
                                file_id,
                                start: 0,
                                end: 0,
                            },
                        }
                    };
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::File(
                        self.first_token.clone(),
                        File {
                            span,
                            id: file_id,
                            items: self.items.clone(),
                        },
                    ))
                }
                _ => AcceptResult::Error(ParseError::unexpected_token(token)),
            },
            FinishedStackItem::ParenthesizedModScopeModifier(
                parenthesized_mod_scope_modifier_first_token,
                modifier,
            ) => {
                if let Some(PendingPubClause::PubKw(pub_kw_token)) = self.pending_visibility.take()
                {
                    let visibility = PubClause {
                        span: span_single(file_id, &pub_kw_token).inclusive_merge(modifier.span),
                        scope_modifier: Some(modifier),
                    };
                    self.pending_visibility = Some(PendingPubClause::Finished(visibility));
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
            FinishedStackItem::Use(_, use_) => {
                self.items.push(FileItem::Use(use_));
                AcceptResult::ContinueToNextToken
            }
            FinishedStackItem::Mod(_, mod_) => {
                self.items.push(FileItem::Mod(mod_));
                AcceptResult::ContinueToNextToken
            }
            FinishedStackItem::Type(_, type_) => {
                self.items.push(FileItem::Type(type_));
                AcceptResult::ContinueToNextToken
            }
            FinishedStackItem::Let(_, let_) => {
                self.items.push(FileItem::Let(let_));
                AcceptResult::ContinueToNextToken
            }
            other_item => wrapped_unexpected_finished_item_err(&other_item),
        }
    }
}

fn get_pub_kw_token(clause: &PubClause) -> Token {
    Token {
        kind: TokenKind::Pub,
        start_index: clause.span.start,
        content: "pub".to_string(),
    }
}
