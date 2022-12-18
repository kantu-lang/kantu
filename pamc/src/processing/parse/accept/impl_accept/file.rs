use super::*;

impl Accept for UnfinishedFile {
    fn accept(&mut self, item: FinishedStackItem, file_id: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::TypeLowerCase => AcceptResult::Push(UnfinishedStackItem::Type(
                    UnfinishedTypeStatement::Keyword(token),
                )),
                TokenKind::Let => AcceptResult::Push(UnfinishedStackItem::Let(
                    UnfinishedLetStatement::Keyword(token),
                )),
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
