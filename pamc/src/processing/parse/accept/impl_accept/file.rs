use super::*;

impl Accept for UnfinishedFile {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::Pub => {
                    if self.pending_visibility.is_some() {
                        AcceptResult::Error(ParseError::unexpected_token(token))
                    } else {
                        self.pending_visibility = Some(PendingVisibilityClause::PubKw(token));
                        AcceptResult::ContinueToNextToken
                    }
                }
                TokenKind::LParen => {
                    if let Some(PendingVisibilityClause::PubKw(_)) = &self.pending_visibility {
                        AcceptResult::PushAndContinueReducingWithNewTop(
                            UnfinishedStackItem::WeakAncestor(UnfinishedWeakAncestor::Empty),
                            FinishedStackItem::Token(token),
                        )
                    } else {
                        AcceptResult::Error(ParseError::unexpected_token(token))
                    }
                }
                TokenKind::TypeLowerCase => {
                    let visibility = self
                        .pending_visibility
                        .take()
                        .map(|visibility| visibility.finalize(file_id));
                    AcceptResult::Push(UnfinishedStackItem::Type(
                        UnfinishedTypeStatement::Keyword {
                            first_token: token,
                            visibility,
                        },
                    ))
                }
                TokenKind::Let => {
                    let visibility = self
                        .pending_visibility
                        .take()
                        .map(|visibility| visibility.finalize(file_id));
                    AcceptResult::Push(UnfinishedStackItem::Let(UnfinishedLetStatement::Keyword {
                        first_token: token,
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
            FinishedStackItem::WeakAncestor(weak_ancestor_first_token, ancestor) => {
                if let Some(PendingVisibilityClause::PubKw(pub_kw_token)) =
                    self.pending_visibility.take()
                {
                    let visibility = VisibilityClause {
                        span: span_single(file_id, &pub_kw_token).inclusive_merge(ancestor.span),
                        ancestor: Some(ancestor),
                    };
                    self.pending_visibility = Some(PendingVisibilityClause::Finished(visibility));
                    AcceptResult::ContinueToNextToken
                } else {
                    wrapped_unexpected_finished_item_err(&FinishedStackItem::WeakAncestor(
                        weak_ancestor_first_token,
                        ancestor,
                    ))
                }
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
