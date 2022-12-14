use super::*;

#[derive(Clone, Debug)]
pub enum UnfinishedStackItem {
    File(Box<UnfinishedFile>),
    Type(UnfinishedTypeStatement),
    Let(UnfinishedLetStatement),
    Params(UnfinishedParams),
    Param(UnfinishedParam),
    Variant(UnfinishedVariant),
    UnfinishedDelimitedExpression(UnfinishedDelimitedExpression),
    UnfinishedDelimitedGoalKwOrExpression(UnfinishedDelimitedGoalKwOrExpression),
    UnfinishedDelimitedQuestionMarkOrExpression(UnfinishedDelimitedQuestionMarkOrExpression),
    Fun(UnfinishedFun),
    Match(UnfinishedMatch),
    Forall(UnfinishedForall),
    Check(UnfinishedCheck),
    CheckAssertions(UnfinishedCheckAssertions),
    CheckAssertion(UnfinishedCheckAssertion),
    Dot(UnfinishedDot),
    Call(UnfinishedCall),
    MatchCase(UnfinishedMatchCase),
}

#[derive(Clone, Debug)]
pub struct UnfinishedFile {
    pub first_token: Token,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug)]
pub enum UnfinishedTypeStatement {
    Keyword(Token),
    Name(Token, Identifier),
    Params(Token, Identifier, Vec<Param>),
    Variants(Token, Identifier, Vec<Param>, Vec<Variant>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedLetStatement {
    Keyword(Token),
    Name(Token, Identifier),
}

#[derive(Clone, Debug)]
pub struct UnfinishedParams {
    pub first_token: Token,
    pub maximum_dashed_params_allowed: usize,
    pub pending_dash: Option<Token>,
    pub params: Vec<Param>,
}

#[derive(Clone, Debug)]
pub enum UnfinishedParam {
    Name {
        first_token: Token,
        is_dashed: bool,
        name: Identifier,
    },
}

#[derive(Clone, Debug)]
pub enum UnfinishedVariant {
    Dot(Token),
    Name(Token, Identifier),
    Params(Token, Identifier, Vec<Param>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedExpression {
    Empty,
    WaitingForEndDelimiter(Token, Expression),
}

#[derive(Clone, Debug)]
pub enum UnfinishedFun {
    Keyword(Token),
    Name(Token, Identifier),
    Params(Token, Identifier, Vec<Param>),
    ReturnType(Token, Identifier, Vec<Param>, Expression),
}

#[derive(Clone, Debug)]
pub enum UnfinishedMatch {
    Keyword(Token),
    Cases(Token, Expression, Vec<MatchCase>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedForall {
    Keyword(Token),
    Params(Token, Vec<Param>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedCheck {
    Keyword(Token),
    Assertions(Token, Vec<CheckAssertion>),
}

#[derive(Clone, Debug)]
pub struct UnfinishedCheckAssertions {
    pub first_token: Token,
    pub assertions: Vec<CheckAssertion>,
}

#[derive(Clone, Debug)]
pub struct UnfinishedCheckAssertion {
    pub first_token: Token,
    pub left: GoalKwOrExpression,
    pub kind: CheckAssertionKind,
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedGoalKwOrExpression {
    Empty,
    WaitingForEndDelimiter { goal_kw: Token },
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedQuestionMarkOrExpression {
    Empty,
    WaitingForEndDelimiter { question_mark: Token },
}

#[derive(Clone, Debug)]
pub struct UnfinishedDot {
    pub first_token: Token,
    pub left: Expression,
}

#[derive(Clone, Debug)]
pub struct UnfinishedCall {
    pub first_token: Token,
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug)]
pub enum UnfinishedMatchCase {
    Dot(Token),
    VariantName(Token, Identifier),
    ParamsInProgress(Token, Identifier, Vec<Identifier>, CurrentlyHasEndingComma),
    AwaitingOutput(Token, Identifier, Vec<Identifier>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurrentlyHasEndingComma(pub bool);
