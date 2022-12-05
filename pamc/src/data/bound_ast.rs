use crate::data::{
    bind_error::BindError, fun_recursion_validation_result::IllegalFunRecursionError,
    simplified_ast as unbound, FileId, TextPosition,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct File {
    pub id: FileId,
    pub items: Vec<FileItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FileItem {
    Type(TypeStatement),
    Let(LetStatement),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeStatement {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Param {
    pub is_dashed: bool,
    pub name: Identifier,
    pub type_: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: Identifier,
    pub params: Vec<Param>,
    pub return_type: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Expression {
    Name(NameExpression),
    Call(Box<Call>),
    Fun(Box<Fun>),
    Match(Box<Match>),
    Forall(Box<Forall>),
    Check(Box<Check>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee: Expression,
    pub args: Vec<Expression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee: Expression,
    pub cases: Vec<MatchCase>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub variant_name: Identifier,
    pub params: Vec<Identifier>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub params: Vec<Param>,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Check {
    pub checkee_annotation: CheckeeAnnotation,
    pub output: Expression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CheckeeAnnotation {
    Goal(GoalCheckeeAnnotation),
    Expression(ExpressionCheckeeAnnotation),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GoalCheckeeAnnotation {
    pub goal_kw_position: TextPosition,
    pub checkee_type: QuestionMarkOrPossiblyInvalidExpression,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExpressionCheckeeAnnotation {
    pub checkee: Expression,
    pub checkee_type: QuestionMarkOrPossiblyInvalidExpression,
    pub checkee_value: Option<QuestionMarkOrPossiblyInvalidExpression>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum QuestionMarkOrPossiblyInvalidExpression {
    QuestionMark { start: TextPosition },
    Expression(PossiblyInvalidExpression),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PossiblyInvalidExpression {
    Valid(Expression),
    Invalid(InvalidExpression),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum InvalidExpression {
    SymbolicallyInvalid(SymbolicallyInvalidExpression),
    IllegalFunRecursion(IllegalFunRecursionExpression),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SymbolicallyInvalidExpression {
    pub expression: unbound::Expression,
    pub error: BindError,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IllegalFunRecursionExpression {
    pub expression: Expression,
    pub error: IllegalFunRecursionError,
}
