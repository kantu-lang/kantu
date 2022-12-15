use std::{
    convert::TryFrom,
    iter::IntoIterator,
    num::NonZeroUsize,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NonEmptyVec<T> {
    raw: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn singleton(value: T) -> Self {
        Self { raw: vec![value] }
    }

    pub fn from_pushed(mut original: Vec<T>, end: T) -> NonEmptyVec<T> {
        original.push(end);
        NonEmptyVec { raw: original }
    }
}

impl<T> NonEmptyVec<T> {
    pub fn into_raw(self) -> Vec<T> {
        self.raw
    }

    pub fn raw(&self) -> &[T] {
        &self.raw
    }

    pub fn raw_mut(&mut self) -> &mut [T] {
        &mut self.raw
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    pub fn non_zero_len(&self) -> NonZeroUsize {
        NonZeroUsize::new(self.len()).unwrap()
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn push(&mut self, value: T) {
        self.raw.push(value);
    }

    pub fn append(&mut self, other: &mut Vec<T>) {
        self.raw.append(other);
    }

    pub fn extend(&mut self, other: impl IntoIterator<Item = T>) {
        self.raw.extend(other);
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.raw.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.raw.iter_mut()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.raw.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.raw.get_mut(index)
    }

    pub fn first(&self) -> &T {
        &self.raw[0]
    }

    pub fn first_mut(&mut self) -> &mut T {
        &mut self.raw[0]
    }

    pub fn last(&self) -> &T {
        self.raw.last().unwrap()
    }

    pub fn last_mut(&mut self) -> &mut T {
        self.raw.last_mut().unwrap()
    }

    pub fn split_first(&self) -> (&T, &[T]) {
        self.raw.split_first().unwrap()
    }

    pub fn split_first_mut(&mut self) -> (&mut T, &mut [T]) {
        self.raw.split_first_mut().unwrap()
    }

    pub fn split_last(&self) -> (&T, &[T]) {
        self.raw.split_last().unwrap()
    }

    pub fn split_last_mut(&mut self) -> (&mut T, &mut [T]) {
        self.raw.split_last_mut().unwrap()
    }

    pub fn split_at(&self, index: usize) -> (&[T], &[T]) {
        self.raw.split_at(index)
    }

    pub fn split_at_mut(&mut self, index: usize) -> (&mut [T], &mut [T]) {
        self.raw.split_at_mut(index)
    }

    pub fn into_popped(mut self) -> (T, Vec<T>) {
        let last = self.raw.pop().unwrap();
        (last, self.raw)
    }

    pub fn into_mapped<U>(self, f: impl FnMut(T) -> U) -> NonEmptyVec<U> {
        NonEmptyVec {
            raw: self.raw.into_iter().map(f).collect(),
        }
    }

    pub fn try_into_mapped<U, E>(
        self,
        f: impl FnMut(T) -> Result<U, E>,
    ) -> Result<NonEmptyVec<U>, E> {
        Ok(NonEmptyVec {
            raw: self.raw.into_iter().map(f).collect::<Result<_, _>>()?,
        })
    }
}

impl<T> IntoIterator for NonEmptyVec<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

impl<T> Index<usize> for NonEmptyVec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw[index]
    }
}

impl<T> IndexMut<usize> for NonEmptyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.raw[index]
    }
}

impl<T> Index<Range<usize>> for NonEmptyVec<T> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.raw[index]
    }
}

impl<T> IndexMut<Range<usize>> for NonEmptyVec<T> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.raw[index]
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<T> DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

pub trait OptionalNonEmptyToVec {
    type Output;
    fn option_to_vec(self) -> Vec<Self::Output>;
}

impl<T> OptionalNonEmptyToVec for Option<NonEmptyVec<T>> {
    type Output = T;
    fn option_to_vec(self) -> Vec<T> {
        self.map_or_else(Vec::new, Vec::from)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EmptyVecError;

impl<T> TryFrom<Vec<T>> for NonEmptyVec<T> {
    type Error = EmptyVecError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptyVecError)
        } else {
            Ok(Self { raw: value })
        }
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(value: NonEmptyVec<T>) -> Self {
        value.raw
    }
}

macro_rules! impl_from_array {
    ($n:expr) => {
        impl<T> From<[T; $n]> for NonEmptyVec<T> {
            fn from(value: [T; $n]) -> Self {
                Self { raw: value.into() }
            }
        }
    };
}

impl_from_array!(1);
impl_from_array!(2);
impl_from_array!(3);
impl_from_array!(4);
impl_from_array!(5);
impl_from_array!(6);
impl_from_array!(7);
impl_from_array!(8);
