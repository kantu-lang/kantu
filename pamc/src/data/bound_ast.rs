use crate::data::{FileId, TextPosition};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeStatement {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
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
    Name(NameExpression),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameExpression {
    pub components: Vec<Identifier>,
    /// De Bruijn index (zero-based).
    pub db_index: DbIndex,
}

/// De Bruijn index (zero-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DbIndex(pub usize);

/// De Bruijn level (zero-based).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DbLevel(pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub start: Option<TextPosition>,
    pub name: IdentifierName,
}

impl From<crate::data::simplified_ast::Identifier> for Identifier {
    fn from(id: crate::data::simplified_ast::Identifier) -> Self {
        Self {
            start: Some(id.start),
            name: id.name,
        }
    }
}

pub use crate::data::simplified_ast::IdentifierName;

pub use crate::data::simplified_ast::ReservedIdentifierName;

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
    pub body: Expression,
    /// This is used by the type checker to
    /// determine whether it can skip type-checking
    /// the function's body.
    /// This is necessary to prevent infinite recursion
    /// when checking recursive functions.
    pub skip_type_checking_body: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub variant_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}
