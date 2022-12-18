use super::*;

impl Accept for UnfinishedDot {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::StandardIdentifier => {
                    let right = Identifier {
                        span: span_single(file_id, &token),
                        name: IdentifierName::Standard(token.content.clone()),
                    };
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::UndelimitedExpression(
                        self.first_token.clone(),
                        Expression::Dot(Box::new(Dot {
                            span: self.left.span().inclusive_merge(right.span),
                            left: self.left.clone(),
                            right,
                        })),
                    ))
                }
                _other_token_kind => AcceptResult::Error(ParseError::unexpected_token(token)),
            },
            other_item => wrapped_unexpected_finished_item_err(&other_item),
        }
    }
}
