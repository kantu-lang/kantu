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
    pub name: SingletonName,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

/// A singleton name is a name with exactly
/// one component.
/// For example, `Nat` is a singleton name,
/// but `Nat.O` is not (since it has two components).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SingletonName {
    pub component: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Param {
    pub is_dashed: bool,
    pub name: SingletonName,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    pub name: SingletonName,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: SingletonName,
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
    pub db_index: usize,
}

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
    pub name: SingletonName,
    pub params: Vec<Param>,
    pub return_type: Expression,
    pub body: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchCase {
    pub variant_name: UnresolvedSingletonName,
    pub params: Vec<SingletonName>,
    pub output: Expression,
}

/// Sometimes a name cannot be resolved to a symbol
/// during the binding phase.
/// For example, a the symbol that a match case variant name
/// refers to depends on the type of the match expression's matchee.
/// Therefore, the symbol cannot be resolved until after the
/// type checking phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnresolvedSingletonName {
    pub component: Identifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}
