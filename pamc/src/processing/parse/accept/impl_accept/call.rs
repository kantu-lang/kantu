use super::*;

impl Accept for UnfinishedCall {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::DelimitedExpression(_, arg, end_delimiter) => {
                let args = NonEmptyVec::from_pushed(self.args.clone(), arg);
                match end_delimiter.raw().kind {
                    TokenKind::Comma => AcceptResult::ContinueToNextToken,
                    TokenKind::RParen => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::UndelimitedExpression(
                            self.first_token.clone(),
                            Expression::Call(Box::new(Call {
                                span: span_range_including_end(
                                    file_id,
                                    &self.first_token,
                                    end_delimiter.raw(),
                                ),
                                callee: self.callee.clone(),
                                args,
                            })),
                        ),
                    ),
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::UnexpectedToken(end_delimiter.into_raw()))
                    }
                }
            }
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::StandardIdentifier
                | TokenKind::Underscore
                | TokenKind::TypeTitleCase
                | TokenKind::Fun
                | TokenKind::Match
                | TokenKind::Forall
                | TokenKind::Check => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::UnfinishedDelimitedExpression(
                        UnfinishedDelimitedExpression::Empty,
                    ),
                    FinishedStackItem::Token(token),
                ),
                TokenKind::RParen => match NonEmptyVec::try_from(self.args.clone()) {
                    Ok(args) => AcceptResult::PopAndContinueReducing(
                        FinishedStackItem::UndelimitedExpression(
                            self.first_token.clone(),
                            Expression::Call(Box::new(Call {
                                span: span_range_including_end(file_id, &self.first_token, &token),
                                callee: self.callee.clone(),
                                args,
                            })),
                        ),
                    ),
                    Err(_) => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
            },
            other_item => unexpected_finished_item(&other_item),
        }
    }
}