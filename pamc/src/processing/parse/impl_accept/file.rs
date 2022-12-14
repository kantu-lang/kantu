use super::*;

impl Accept for UnfinishedFile {
    fn accept(&mut self, item: FinishedStackItem, _: FileId) -> AcceptResult {
        match item {
            FinishedStackItem::Token(token) => match token.kind {
                TokenKind::TypeLowerCase => AcceptResult::Push(UnfinishedStackItem::Type(
                    UnfinishedTypeStatement::Keyword(token),
                )),
                TokenKind::Let => AcceptResult::Push(UnfinishedStackItem::Let(
                    UnfinishedLetStatement::Keyword(token),
                )),
                _ => AcceptResult::Error(ParseError::UnexpectedToken(token)),
            },
            FinishedStackItem::Type(_, type_) => {
                self.items.push(FileItem::Type(type_));
                AcceptResult::ContinueToNextToken
            }
            FinishedStackItem::Let(_, let_) => {
                self.items.push(FileItem::Let(let_));
                AcceptResult::ContinueToNextToken
            }
            other_item => unexpected_finished_item(&other_item),
        }
    }
}
