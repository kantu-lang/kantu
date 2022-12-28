use super::*;

#[derive(Clone, Debug)]
pub enum UnfinishedStackItem {
    File(Box<UnfinishedFile>),
    PubClause(UnfinishedPubClause),
    ParenthesizedWeakAncestor(UnfinishedParenthesizedWeakAncestor),
    Type(UnfinishedTypeStatement),
    Let(UnfinishedLetStatement),
    Params(UnfinishedParams),
    Param(UnfinishedParam),
    Variant(UnfinishedVariant),
    UnfinishedDelimitedExpression(UnfinishedDelimitedExpression),
    UnfinishedDelimitedCallArg(UnfinishedDelimitedCallArg),
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
    MatchCaseParam(UnfinishedMatchCaseParam),
    UnfinishedDelimitedTripleDot(UnfinishedDelimitedTripleDot),
    UnfinishedDelimitedImpossibleKwOrExpression(UnfinishedDelimitedImpossibleKwOrExpression),
}

#[derive(Clone, Debug)]
pub struct UnfinishedFile {
    pub first_token: Token,
    pub pending_visibility: Option<PubClause>,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug)]
pub enum UnfinishedPubClause {
    Empty,
    PubKw(Token),
}

#[derive(Clone, Debug)]
pub enum UnfinishedParenthesizedWeakAncestor {
    Empty,
    LParen(Token),
    ReadyForRParen {
        l_paren_token: Token,
        ancestor: ParenthesizedWeakAncestor,
    },
    PackageRelativeAwaitingIdentifier {
        l_paren_token: Token,
        path_after_pack_kw: Vec<Identifier>,
    },
}

#[derive(Clone, Debug)]
pub enum UnfinishedTypeStatement {
    Empty,
    ExplicitVisibility {
        first_token: Token,
        visibility: PubClause,
    },
    Keyword {
        first_token: Token,
        visibility: Option<PubClause>,
    },
    Name {
        first_token: Token,
        visibility: Option<PubClause>,
        name: Identifier,
    },
    Params {
        first_token: Token,
        visibility: Option<PubClause>,
        name: Identifier,
        params: Option<NonEmptyVec<Param>>,
    },
    Variants {
        first_token: Token,
        visibility: Option<PubClause>,
        name: Identifier,
        params: Option<NonEmptyVec<Param>>,
        variants: Vec<Variant>,
    },
}

#[derive(Clone, Debug)]
pub enum UnfinishedLetStatement {
    Empty,
    ExplicitVisibility {
        first_token: Token,
        visibility: PubClause,
    },
    Keyword {
        first_token: Token,
        visibility: Option<PubClause>,
    },
    ExplicitTransparency {
        first_token: Token,
        visibility: Option<PubClause>,
        transparency: ParenthesizedWeakAncestor,
    },
    Name {
        first_token: Token,
        visibility: Option<PubClause>,
        transparency: Option<ParenthesizedWeakAncestor>,
        name: Identifier,
    },
}

#[derive(Clone, Debug)]
pub struct UnfinishedParams {
    pub first_token: Token,
    pub maximum_dashed_params_allowed: usize,
    pub pending_tilde: Option<Token>,
    pub pending_dash: Option<Token>,
    pub params: Vec<Param>,
}

#[derive(Clone, Debug)]
pub enum UnfinishedParam {
    NoIdentifier {
        pending_tilde: Option<Token>,
        pending_dash: Option<Token>,
        is_dash_allowed: bool,
    },
    FirstIdentifier {
        first_token: Token,
        is_tilded: bool,
        is_dashed: bool,
        is_dash_allowed: bool,
        name_or_label: Identifier,
    },
    ExplicitLabel {
        first_token: Token,
        is_dashed: bool,
        is_dash_allowed: bool,
        label: Identifier,
    },
    ExplicitLabelAndName {
        first_token: Token,
        is_dashed: bool,
        label: Identifier,
        name: Identifier,
    },
}

#[derive(Clone, Debug)]
pub enum UnfinishedVariant {
    Empty,
    Dot(Token),
    Name(Token, Identifier),
    Params(Token, Identifier, Option<NonEmptyVec<Param>>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedExpression {
    Empty,
    WaitingForEndDelimiter(Token, Expression),
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedCallArg {
    Empty,
    Colon(Token),
    ColonIdentifier(Token, Identifier),
    Identifier {
        first_token: Token,
        identifier: Identifier,
    },
    IdentifierColon(Identifier),
    Unlabeled,
}

#[derive(Clone, Debug)]
pub enum UnfinishedFun {
    Keyword(Token),
    Name(Token, Identifier),
    Params(Token, Identifier, NonEmptyVec<Param>),
    ReturnType(Token, Identifier, NonEmptyVec<Param>, Expression),
}

#[derive(Clone, Debug)]
pub enum UnfinishedMatch {
    Keyword(Token),
    Cases(Token, Expression, Vec<MatchCase>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedForall {
    Keyword(Token),
    Params(Token, NonEmptyVec<Param>),
}

#[derive(Clone, Debug)]
pub enum UnfinishedCheck {
    Keyword(Token),
    Assertions(Token, NonEmptyVec<CheckAssertion>),
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
    pub args: Vec<CallArg>,
}

#[derive(Clone, Debug)]
pub enum UnfinishedMatchCase {
    Dot(Token),
    VariantName(Token, Identifier),
    ParamsInProgress(Token, Identifier, Vec<MatchCaseParam>),
    AwaitingOutput {
        dot_token: Token,
        variant_name: Identifier,
        params: Option<NonEmptyVec<MatchCaseParam>>,
        triple_dot: Option<TextSpan>,
    },
}

#[derive(Clone, Debug)]
pub enum UnfinishedMatchCaseParam {
    Empty,
    Colon(Token),
    ColonIdentifier(Token, Identifier),
    Identifier {
        first_token: Token,
        identifier: Identifier,
    },
    IdentifierColon {
        first_token: Token,
        label: Identifier,
    },
    IdentifierColonIdentifier {
        first_token: Token,
        label: Identifier,
        name: Identifier,
    },
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedTripleDot {
    Empty,
    WaitingForEndDelimiter(Token),
}

#[derive(Clone, Debug)]
pub enum UnfinishedDelimitedImpossibleKwOrExpression {
    Empty,
    WaitingForEndDelimiter { impossible_kw: Token },
}
