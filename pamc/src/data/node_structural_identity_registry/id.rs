use super::*;

// TODO: Implement Debug, PartialEq, Eq for StructuralId<T>,
// since #[derive] only works if T implements the respective traits.
#[derive(Debug, PartialEq, Eq)]
pub struct StructuralId<T> {
    pub raw: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> StructuralId<T> {
    pub fn new(raw: usize) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for StructuralId<T> {
    fn clone(&self) -> StructuralId<T> {
        StructuralId {
            raw: self.raw,
            _phantom: self._phantom,
        }
    }
}

impl<T> Copy for StructuralId<T> {}

impl<T> std::hash::Hash for StructuralId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionStructuralId {
    Name(NodeId<NameExpression>),
    Call(NodeId<Call>),
    Fun(NodeId<Fun>),
    Match(NodeId<Match>),
    Forall(NodeId<Forall>),
}
