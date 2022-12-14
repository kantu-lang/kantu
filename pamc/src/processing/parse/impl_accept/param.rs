use super::*;

impl Accept for UnfinishedParam {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedParam::Name(first_token, is_dashed, name) => match item {
                FinishedStackItem::Token(token) => match token.kind {
                    TokenKind::Colon => {
                        AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                            UnfinishedDelimitedExpression::Empty,
                        ))
                    }
                    _other_token_kind => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                FinishedStackItem::DelimitedExpression(_, expression, end_delimiter) => {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::Param(
                        first_token.clone(),
                        Param {
                            span: span_range_excluding_end(
                                file_id,
                                &first_token,
                                end_delimiter.raw(),
                            ),
                            is_dashed: *is_dashed,
                            name: name.clone(),
                            type_: expression,
                        },
                        end_delimiter,
                    ))
                }
                other_item => unexpected_finished_item(&other_item),
            },
        }
    }
}
