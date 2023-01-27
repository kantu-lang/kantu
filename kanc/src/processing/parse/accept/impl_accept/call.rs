use super::*;

impl Accept for UnfinishedCall {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::DelimitedCallArg(_, arg, end_delimiter) => {
                match end_delimiter.raw().kind {
                    TokenKind::Comma => {
                        self.args.push(arg);
                        AcceptResult::ContinueToNextToken
                    }
                    TokenKind::RParen => {
                        let args = Vec::from_pushed(self.args.clone(), arg);
                        AcceptResult::PopAndContinueReducing(
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
                        )
                    }
                    _other_end_delimiter => {
                        AcceptResult::Error(ParseError::unexpected_token(end_delimiter.into_raw()))
                    }
                }
            }

            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::RParen => match Vec::try_from(self.args.clone()) {
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
                    Err(_) => AcceptResult::Error(ParseError::unexpected_token(token)),
                },
                _other_token_kind => AcceptResult::PushAndContinueReducingWithNewTop(
                    UnfinishedStackItem::UnfinishedDelimitedCallArg(
                        UnfinishedDelimitedCallArg::Empty,
                    ),
                    FinishedStackItem::Token(token),
                ),
            },
            other_item => wrapped_unexpected_finished_item_err(&other_item),
        }
    }
}
