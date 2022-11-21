use super::*;

pub struct SemanticId<T> {
    pub raw: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SemanticId<T> {
    pub fn new(raw: usize) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for SemanticId<T> {
    fn clone(&self) -> SemanticId<T> {
        SemanticId {
            raw: self.raw,
            _phantom: self._phantom,
        }
    }
}

impl<T> Copy for SemanticId<T> {}

impl<T> std::hash::Hash for SemanticId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> std::fmt::Debug for SemanticId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", self.raw)
    }
}

impl<T> PartialEq<SemanticId<T>> for SemanticId<T> {
    fn eq(&self, other: &SemanticId<T>) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for SemanticId<T> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExpressionSemanticId {
    Name(SemanticId<stripped::NameExpression>),
    Call(SemanticId<stripped::Call>),
    Fun(SemanticId<stripped::Fun>),
    Match(SemanticId<stripped::Match>),
    Forall(SemanticId<stripped::Forall>),
}
