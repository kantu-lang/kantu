use super::*;

impl Accept for UnfinishedDelimitedTripleDot {
    fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
        match self {
            UnfinishedDelimitedTripleDot::Empty => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::TripleDot => {
                    *self = UnfinishedDelimitedTripleDot::WaitingForEndDelimiter(token);
                    AcceptResult::ContinueToNextToken
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },

            UnfinishedDelimitedTripleDot::WaitingForEndDelimiter(triple_dot) => match item {
                FinishedStackItem::Token(token) => {
                    let token = ExpressionEndDelimiter::try_new(token);
                    match token {
                        Err(original_token) => {
                            AcceptResult::Error(ParseError::unexpected_token(original_token))
                        }
                        Ok(end_delimiter) => AcceptResult::PopAndContinueReducing(
                            FinishedStackItem::DelimitedTripleDot(
                                triple_dot.clone(),
                                end_delimiter,
                            ),
                        ),
                    }
                }
                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
