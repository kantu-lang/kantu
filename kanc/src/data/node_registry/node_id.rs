pub struct NodeId<T> {
    pub raw: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NodeId<T> {
    pub fn new(raw: usize) -> Self {
        Self {
            raw,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for NodeId<T> {
    fn clone(&self) -> NodeId<T> {
        NodeId {
            raw: self.raw,
            _phantom: self._phantom,
        }
    }
}

impl<T> Copy for NodeId<T> {}

impl<T> std::hash::Hash for NodeId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T> std::fmt::Debug for NodeId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NodeId({})", self.raw)
    }
}

impl<T> PartialEq<NodeId<T>> for NodeId<T> {
    fn eq(&self, other: &NodeId<T>) -> bool {
        self.raw == other.raw
    }
}

impl<T> Eq for NodeId<T> {}
