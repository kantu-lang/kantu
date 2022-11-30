pub struct ListId<T> {
    pub start: usize,
    pub len: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> ListId<T> {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            len,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for ListId<T> {
    fn clone(&self) -> ListId<T> {
        Self::new(self.start, self.len)
    }
}

impl<T> Copy for ListId<T> {}

impl<T> std::hash::Hash for ListId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.len.hash(state);
    }
}

impl<T> std::fmt::Debug for ListId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListId {{ start: {}, len: {} }}", self.start, self.len)
    }
}

impl<T> PartialEq<ListId<T>> for ListId<T> {
    fn eq(&self, other: &ListId<T>) -> bool {
        self.start == other.start && self.len == other.len
    }
}

impl<T> Eq for ListId<T> {}
