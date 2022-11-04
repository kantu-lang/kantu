use crate::data::node_equality_checker::{ExpressionSemanticId, SemanticId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub db_index: DbIndex,
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee_id: ExpressionSemanticId,
    pub arg_list_id: SemanticId<Vec<ExpressionSemanticId>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub param_type_list_id: SemanticId<Vec<ExpressionSemanticId>>,
    pub dash_index: Option<usize>,
    pub return_type_id: ExpressionSemanticId,
    pub body_id: ExpressionSemanticId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee_id: ExpressionSemanticId,
    /// We use `Set` rather than `Vec` to denote that
    /// the order of the arms is insignificant.
    /// For example, `match x { .A => y, .B => y }` is the same as
    /// `match x { .B => y, .A => y }`.
    pub case_list_id: SemanticId<Set<SemanticId<MatchCase>>>,
}

/// An unconstructable dummy type we created just to pass to `SemanticId`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Set<T> {
    _phantom: std::marker::PhantomData<T>,
    _cannot_construct: std::convert::Infallible,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub variant_name_id: SemanticId<IdentifierName>,
    pub output_id: ExpressionSemanticId,
}

pub use crate::data::light_ast::IdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub param_type_list_id: SemanticId<Vec<ExpressionSemanticId>>,
    pub output_id: ExpressionSemanticId,
}
