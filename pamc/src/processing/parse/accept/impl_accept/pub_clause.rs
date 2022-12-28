use super::*;

impl Accept for UnfinishedPubClause {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedPubClause::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::Pub => {
                    *self = UnfinishedPubClause::PubKw(token);
                    AcceptResult::ContinueToNextToken
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedPubClause::PubKw(pub_kw_token) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::LParen => AcceptResult::PushAndContinueReducingWithNewTop(
                        UnfinishedStackItem::ParenthesizedWeakAncestor(
                            UnfinishedParenthesizedWeakAncestor::Empty,
                        ),
                        FinishedStackItem::Token(token),
                    ),
                    _other_token_kind => AcceptResult::PopAndEnqueueAndContinueReducing(
                        FinishedStackItem::PubClause(
                            pub_kw_token.clone(),
                            PubClause {
                                span: span_single(file_id, pub_kw_token),
                                ancestor: None,
                            },
                        ),
                        FinishedStackItem::Token(token),
                    ),
                },
                FinishedStackItem::ParenthesizedWeakAncestor(_, ancestor) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::PubClause(
                        pub_kw_token.clone(),
                        PubClause {
                            span: span_single(file_id, pub_kw_token).inclusive_merge(ancestor.span),
                            ancestor: Some(ancestor),
                        },
                    ))
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
