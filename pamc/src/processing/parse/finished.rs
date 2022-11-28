use super::*;

#[derive(Clone, Debug)]
pub enum FinishedStackItem {
    Token(Token),
    Type(
        /// First token ("type")
        Token,
        TypeStatement,
    ),
    Let(
        /// First token ("let")
        Token,
        LetStatement,
    ),
    Params(
        /// First token ("(")
        Token,
        Vec<Param>,
    ),
    Param(
        /// First token
        Token,
        Param,
        ExpressionEndDelimiter,
    ),
    Variant(
        /// First token (".")
        Token,
        Variant,
        ExpressionEndDelimiter,
    ),
    DelimitedExpression(
        /// First token (".")
        Token,
        Expression,
        ExpressionEndDelimiter,
    ),
    UndelimitedExpression(
        /// First token (".")
        Token,
        Expression,
    ),
    MatchCase(
        /// First token (".")
        Token,
        MatchCase,
        ExpressionEndDelimiter,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Can be `,;{})`
pub struct ExpressionEndDelimiter(pub Token);

impl FinishedStackItem {
    pub fn first_token(&self) -> &Token {
        match self {
            FinishedStackItem::Token(token) => &token,
            FinishedStackItem::Type(token, _) => &token,
            FinishedStackItem::Let(token, _) => &token,
            FinishedStackItem::Params(token, _) => &token,
            FinishedStackItem::Param(token, _, _) => &token,
            FinishedStackItem::Variant(token, _, _) => &token,
            FinishedStackItem::DelimitedExpression(token, _, _) => &token,
            FinishedStackItem::UndelimitedExpression(token, _) => &token,
            FinishedStackItem::MatchCase(token, _, _) => &token,
        }
    }
}
