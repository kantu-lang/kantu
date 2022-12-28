use super::*;

#[derive(Clone, Debug)]
pub enum FinishedStackItem {
    Token(Token),
    File(
        /// First token
        Token,
        File,
    ),
    ParenthesizedWeakAncestor(
        /// First token
        Token,
        ParenthesizedWeakAncestor,
    ),
    Mod(
        /// First token
        Token,
        ModStatement,
    ),
    Type(
        /// First token
        Token,
        TypeStatement,
    ),
    Let(
        /// First token
        Token,
        LetStatement,
    ),
    Params(
        /// First token ("(")
        Token,
        NonEmptyVec<Param>,
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
        /// First token
        Token,
        Expression,
        ExpressionEndDelimiter,
    ),
    UndelimitedExpression(
        /// First token
        Token,
        Expression,
    ),
    DelimitedCallArg(
        /// First token
        Token,
        CallArg,
        ExpressionEndDelimiter,
    ),
    MatchCase(
        /// First token (".")
        Token,
        MatchCase,
        ExpressionEndDelimiter,
    ),
    MatchCaseParam(
        /// First token (".")
        Token,
        MatchCaseParam,
        ExpressionEndDelimiter,
    ),
    DelimitedTripleDot(
        /// First (and only) token ("...")
        Token,
        ExpressionEndDelimiter,
    ),
    CheckAssertions(
        /// First token ("(")
        Token,
        NonEmptyVec<CheckAssertion>,
    ),
    CheckAssertion(
        /// First token
        Token,
        CheckAssertion,
        ExpressionEndDelimiter,
    ),
    DelimitedGoalKwOrExpression(
        /// First token
        Token,
        GoalKwOrExpression,
        ExpressionEndDelimiter,
    ),
    DelimitedQuestionMarkOrExpression(
        /// First token
        Token,
        QuestionMarkOrExpression,
        ExpressionEndDelimiter,
    ),
    DelimitedImpossibleKwOrExpression(
        /// First token
        Token,
        MatchCaseOutput,
        ExpressionEndDelimiter,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpressionEndDelimiter(Token);

impl ExpressionEndDelimiter {
    pub fn is_end_delimiter(kind: TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Comma
                | TokenKind::Semicolon
                | TokenKind::Colon
                | TokenKind::Equal
                | TokenKind::LCurly
                | TokenKind::RCurly
                | TokenKind::RParen
                | TokenKind::Eoi
        )
    }

    pub fn try_new(token: Token) -> Result<Self, Token> {
        if ExpressionEndDelimiter::is_end_delimiter(token.kind) {
            Ok(Self(token))
        } else {
            Err(token)
        }
    }
}

impl ExpressionEndDelimiter {
    pub fn raw(&self) -> &Token {
        &self.0
    }

    pub fn into_raw(self) -> Token {
        self.0
    }
}

impl FinishedStackItem {
    pub fn first_token(&self) -> &Token {
        match self {
            FinishedStackItem::Token(token) => &token,
            FinishedStackItem::File(token, _) => &token,
            FinishedStackItem::ParenthesizedWeakAncestor(token, _) => &token,
            FinishedStackItem::Mod(token, _) => &token,
            FinishedStackItem::Type(token, _) => &token,
            FinishedStackItem::Let(token, _) => &token,
            FinishedStackItem::Params(token, _) => &token,
            FinishedStackItem::Param(token, _, _) => &token,
            FinishedStackItem::Variant(token, _, _) => &token,
            FinishedStackItem::DelimitedExpression(token, _, _) => &token,
            FinishedStackItem::UndelimitedExpression(token, _) => &token,
            FinishedStackItem::DelimitedCallArg(token, _, _) => &token,
            FinishedStackItem::MatchCase(token, _, _) => &token,
            FinishedStackItem::MatchCaseParam(token, _, _) => &token,
            FinishedStackItem::DelimitedTripleDot(token, _) => &token,
            FinishedStackItem::CheckAssertions(token, _) => &token,
            FinishedStackItem::CheckAssertion(token, _, _) => &token,
            FinishedStackItem::DelimitedGoalKwOrExpression(token, _, _) => &token,
            FinishedStackItem::DelimitedQuestionMarkOrExpression(token, _, _) => &token,
            FinishedStackItem::DelimitedImpossibleKwOrExpression(token, _, _) => &token,
        }
    }
}
