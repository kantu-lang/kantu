use crate::{
    lex::{Token, TokenKind},
    unbound_ast::*,
};

#[derive(Clone, Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
}

pub fn parse_file(tokens: Vec<Token>) -> Result<File, ParseError> {
    let first_token = if let Some(t) = tokens.iter().find(is_not_whitespace_ref) {
        t.clone()
    } else {
        return Ok(File(vec![]));
    };
    let mut stack: Vec<StackItem> = vec![StackItem::File(Box::new(StackFile {
        first_token,
        items: vec![],
    }))];

    for token in tokens.into_iter().filter(is_not_whitespace) {
        stack.push(StackItem::Token(token));
        while stack.len() >= 2 {
            let popped = stack.pop().unwrap();
            let top = stack.last_mut().unwrap();
            let accept_result = top.accept(popped);
            match accept_result {
                AcceptResult::ContinueToNextToken => break,
                AcceptResult::ContinueReducing => continue,
                AcceptResult::Push(item) => {
                    stack.push(item);
                    break;
                }
                AcceptResult::Error(err) => return Err(err),
            }
        }
    }

    Ok(File(vec![]))
}

fn is_not_whitespace(token: &Token) -> bool {
    token.kind != TokenKind::Whitespace
}

fn is_not_whitespace_ref(token: &&Token) -> bool {
    token.kind != TokenKind::Whitespace
}

#[derive(Clone, Debug)]
enum StackItem {
    File(Box<StackFile>),
    Token(Token),
    Type(StackTypeStatement),
    Let(StackLetStatement),
}

#[derive(Clone, Debug)]
struct StackFile {
    first_token: Token,
    items: Vec<FileItem>,
}

#[derive(Clone, Debug)]
enum StackTypeStatement {
    Keyword(Token),
    Name(Token, Identifier),
    Params(Token, Identifier, Vec<Param>),
    Constructors(Token, Identifier, Vec<Constructor>),
}

#[derive(Clone, Debug)]
enum StackLetStatement {
    Keyword(Token),
    Name(Token, Identifier),
}

#[derive(Clone, Debug)]
enum AcceptResult {
    ContinueToNextToken,
    ContinueReducing,
    Push(StackItem),
    Error(ParseError),
}

mod accept {
    use super::*;

    impl StackItem {
        pub fn accept(&mut self, item: StackItem) -> AcceptResult {
            match self {
                StackItem::File(file) => file.accept(item),
                StackItem::Token(token) => {
                    AcceptResult::Error(ParseError::UnexpectedToken(token.clone()))
                }
            }
        }
    }
    impl StackFile {
        fn accept(&mut self, item: StackItem) -> AcceptResult {
            match item {
                StackItem::File(file) => {
                    AcceptResult::Error(ParseError::UnexpectedToken(file.first_token))
                }
                StackItem::Token(token) => match token.kind {
                    TokenKind::TypeLowerCase => {
                        AcceptResult::Push(StackItem::Type(StackTypeStatement::Keyword(token)))
                    }
                    TokenKind::Let => {
                        AcceptResult::Push(StackItem::Let(StackLetStatement::Keyword(token)))
                    }
                    _ => AcceptResult::Error(ParseError::UnexpectedToken(token)),
                },
                StackItem::Type(type_) => {
                    self.items.push(type_.into());
                    AcceptResult::ContinueReducing
                }
            }
        }
    }
}

mod into_unbound_ast_node {
    use super::*;

    impl From<StackTypeStatement> for FileItem {
        fn from(let_: StackTypeStatement) -> FileItem {
            FileItem::Type(TypeStatement {
                // TODO
            })
        }
    }
}
