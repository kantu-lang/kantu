#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File(pub Vec<FileItem>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub constructors: Vec<Constructor>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub start_index: usize,
    pub content: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constructor {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expression {
    TypeTitleCase(TypeTitleCase),
    Identifier(Identifier),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
    Exists(Box<Exists>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeTitleCase {
    pub start_index: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
    pub return_value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub constructor: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exists {
    pub params: Vec<Param>,
    pub output: Expression,
}
