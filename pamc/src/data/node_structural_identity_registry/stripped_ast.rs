use crate::data::node_structural_identity_registry::{ExpressionStructuralId, StructuralId};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameExpression {
    pub db_index: DbIndex,
}

pub use crate::data::bound_ast::{DbIndex, DbLevel};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Call {
    pub callee_id: StructuralId<Call>,
    pub arg_list_id: StructuralId<Vec<ExpressionStructuralId>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fun {
    pub param_type_list_id: StructuralId<Vec<ExpressionStructuralId>>,
    pub dash_index: Option<usize>,
    pub return_type_id: ExpressionStructuralId,
    pub body_id: ExpressionStructuralId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Match {
    pub matchee_id: ExpressionStructuralId,
    /// We use `Set` rather than `Vec` to denote that
    /// the order of the arms is insignificant.
    /// For example, `match x { .A => y, .B => y }` is the same as
    /// `match x { .B => y, .A => y }`.
    pub case_list_id: StructuralId<Set<StructuralId<MatchCase>>>,
}

/// An unconstructable dummy type we created just to pass to `StructuralId`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Set<T> {
    _phantom: std::marker::PhantomData<T>,
    _cannot_construct: std::convert::Infallible,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MatchCase {
    pub variant_name_id: StructuralId<IdentifierName>,
    pub output_id: ExpressionStructuralId,
}

pub use crate::data::light_ast::IdentifierName;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Forall {
    pub param_tytpe_list_id: StructuralId<Vec<ExpressionStructuralId>>,
    pub output_id: ExpressionStructuralId,
}
