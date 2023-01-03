use super::*;

impl Accept for UnfinishedCheck {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match self {
            UnfinishedCheck::Keyword(check_kw) => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LParen => {
                    AcceptResult::Push(UnfinishedStackItem::CheckAssertions(
                        UnfinishedCheckAssertions {
                            first_token: token,
                            assertions: vec![],
                        },
                    ))
                }

                FinishedStackItem::CheckAssertions(_, assertions) => {
                    *self = UnfinishedCheck::Assertions(check_kw.clone(), assertions);
                    AcceptResult::ContinueToNextToken
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
            UnfinishedCheck::Assertions(check_kw, assertions) => match item {
                FinishedStackItem::Token(token) if token.kind == TokenKind::LCurly => {
                    AcceptResult::Push(UnfinishedStackItem::UnfinishedDelimitedExpression(
                        UnfinishedDelimitedExpression::Empty,
                    ))
                }

                FinishedStackItem::DelimitedExpression(_, output, end_delimiter)
                    if end_delimiter.raw().kind == TokenKind::RCurly =>
                {
                    AcceptResult::PopAndContinueReducing(FinishedStackItem::UndelimitedExpression(
                        check_kw.clone(),
                        Expression::Check(Box::new(Check {
                            span: span_range_including_end(file_id, &check_kw, end_delimiter.raw()),
                            assertions: assertions.clone(),
                            output,
                        })),
                    ))
                }

                other_item => wrapped_unexpected_finished_item_err(&other_item),
            },
        }
    }
}
