use super::*;

pub struct NonEmptyListId<T> {
    pub len: NonZeroUsize,
    pub start: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> NonEmptyListId<T> {
    pub fn new(start: usize, len: NonZeroUsize) -> Self {
        Self {
            start,
            len,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Clone for NonEmptyListId<T> {
    fn clone(&self) -> NonEmptyListId<T> {
        Self::new(self.start, self.len)
    }
}

impl<T> Copy for NonEmptyListId<T> {}

impl<T> std::hash::Hash for NonEmptyListId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.len.hash(state);
    }
}

impl<T> std::fmt::Debug for NonEmptyListId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ListId {{ start: {}, len: {} }}", self.start, self.len)
    }
}

impl<T> PartialEq<NonEmptyListId<T>> for NonEmptyListId<T> {
    fn eq(&self, other: &NonEmptyListId<T>) -> bool {
        self.start == other.start && self.len == other.len
    }
}

impl<T> Eq for NonEmptyListId<T> {}
