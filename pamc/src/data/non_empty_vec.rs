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

    pub fn into_popped(mut self) -> (Vec<T>, T) {
        let last = self.raw.pop().unwrap();
        (self.raw, last)
    }

    pub fn into_mapped<U>(self, f: impl FnMut(T) -> U) -> NonEmptyVec<U> {
        NonEmptyVec {
            raw: self.raw.into_iter().map(f).collect(),
        }
    }

    pub fn enumerate_into_mapped<U>(self, f: impl FnMut((usize, T)) -> U) -> NonEmptyVec<U> {
        NonEmptyVec {
            raw: self.raw.into_iter().enumerate().map(f).collect(),
        }
    }

    pub fn try_enumerate_into_mapped<U, E>(
        self,
        f: impl FnMut((usize, T)) -> Result<U, E>,
    ) -> Result<NonEmptyVec<U>, E> {
        Ok(NonEmptyVec {
            raw: self
                .raw
                .into_iter()
                .enumerate()
                .map(f)
                .collect::<Result<_, _>>()?,
        })
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

impl<'a, T> IntoIterator for &'a NonEmptyVec<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut NonEmptyVec<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter_mut()
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

pub trait OptionalNonEmptyVecToVec {
    type Output;
    fn into_vec(self) -> Vec<Self::Output>;
}

impl<T> OptionalNonEmptyVecToVec for Option<NonEmptyVec<T>> {
    type Output = T;
    fn into_vec(self) -> Vec<T> {
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

pub trait OptionalNonEmptyVecLen {
    fn len(&self) -> usize;
}

impl<T> OptionalNonEmptyVecLen for Option<NonEmptyVec<T>> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.len()).unwrap_or(0)
    }
}

impl<T> NonEmptyVec<T> {
    pub fn as_non_empty_slice(&self) -> NonEmptySlice<'_, T> {
        NonEmptySlice { raw: &self.raw }
    }

    pub fn as_non_empty_mut(&mut self) -> NonEmptySliceMut<'_, T> {
        NonEmptySliceMut { raw: &mut self.raw }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NonEmptySlice<'a, T> {
    raw: &'a [T],
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NonEmptySliceMut<'a, T> {
    raw: &'a mut [T],
}

impl<'a, T> Deref for NonEmptySlice<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.raw
    }
}

impl<'a, T> Deref for NonEmptySliceMut<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.raw
    }
}

impl<'a, T> DerefMut for NonEmptySliceMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.raw
    }
}

impl<'a, T> NonEmptySlice<'a, T> {
    pub fn new(slice: &'a [T], start: usize, len: NonZeroUsize) -> Self {
        Self {
            raw: &slice[start..start + len.get()],
        }
    }
}

impl<'a, T> NonEmptySliceMut<'a, T> {
    pub fn new(slice: &'a mut [T], start: usize, len: NonZeroUsize) -> Self {
        Self {
            raw: &mut slice[start..start + len.get()],
        }
    }
}

impl<'a, T> From<NonEmptySlice<'a, T>> for &'a [T] {
    fn from(value: NonEmptySlice<'a, T>) -> Self {
        value.raw
    }
}

impl<'a, T> From<NonEmptySliceMut<'a, T>> for &'a mut [T] {
    fn from(value: NonEmptySliceMut<'a, T>) -> Self {
        value.raw
    }
}

impl<'a, T> NonEmptySlice<'a, T> {
    pub fn to_non_empty_vec(&self) -> NonEmptyVec<T>
    where
        T: Clone,
    {
        NonEmptyVec {
            raw: self.raw.to_vec(),
        }
    }

    pub fn to_mapped<U>(&self, f: impl FnMut(&T) -> U) -> NonEmptyVec<U>
    where
        T: Clone,
    {
        NonEmptyVec {
            raw: self.raw.iter().map(f).collect(),
        }
    }

    pub fn try_to_mapped<U, E>(
        &self,
        f: impl FnMut(&T) -> Result<U, E>,
    ) -> Result<NonEmptyVec<U>, E>
    where
        T: Clone,
    {
        Ok(NonEmptyVec {
            raw: self.raw.iter().map(f).collect::<Result<_, _>>()?,
        })
    }

    pub fn enumerate_to_mapped<U>(&self, f: impl FnMut((usize, &T)) -> U) -> NonEmptyVec<U> {
        NonEmptyVec {
            raw: self.raw.iter().enumerate().map(f).collect(),
        }
    }

    pub fn try_enumerate_to_mapped<U, E>(
        &self,
        f: impl FnMut((usize, &T)) -> Result<U, E>,
    ) -> Result<NonEmptyVec<U>, E> {
        Ok(NonEmptyVec {
            raw: self
                .raw
                .iter()
                .enumerate()
                .map(f)
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn map_to_unzipped<U, V>(
        &self,
        f: impl FnMut(&T) -> (U, V),
    ) -> (NonEmptyVec<U>, NonEmptyVec<V>) {
        let (u, v) = self.raw.iter().map(f).unzip();
        (NonEmptyVec { raw: u }, NonEmptyVec { raw: v })
    }

    pub fn to_popped(&self) -> (&[T], &T) {
        let last_index = self.len() - 1;
        (&self[..last_index], &self[last_index])
    }

    pub fn to_cons(&self) -> (&T, &[T]) {
        (&self[0], &self[1..])
    }
}

impl<'a, T> NonEmptySliceMut<'a, T> {
    pub fn to_non_empty_vec(&self) -> NonEmptyVec<T>
    where
        T: Clone,
    {
        NonEmptyVec {
            raw: self.raw.to_vec(),
        }
    }

    pub fn to_mapped<U>(&self, f: impl FnMut(&T) -> U) -> NonEmptyVec<U>
    where
        T: Clone,
    {
        NonEmptyVec {
            raw: self.raw.iter().map(f).collect(),
        }
    }

    pub fn try_to_mapped<U, E>(
        &self,
        f: impl FnMut(&T) -> Result<U, E>,
    ) -> Result<NonEmptyVec<U>, E>
    where
        T: Clone,
    {
        Ok(NonEmptyVec {
            raw: self.raw.iter().map(f).collect::<Result<_, _>>()?,
        })
    }

    pub fn to_popped(&self) -> (&[T], &T) {
        let last_index = self.len() - 1;
        (&self[..last_index], &self[last_index])
    }

    pub fn to_popped_mut(&mut self) -> (&mut [T], &mut T) {
        let last_index = self.len() - 1;
        let (remaining, singleton_last) = self.split_at_mut(last_index);
        (remaining, &mut singleton_last[0])
    }

    pub fn to_cons(&self) -> (&T, &[T]) {
        (&self[0], &self[1..])
    }

    pub fn to_cons_mut(&mut self) -> (&mut T, &mut [T]) {
        let (singleton_car, cdr) = self.split_at_mut(1);
        (&mut singleton_car[0], cdr)
    }

    pub fn enumerate_to_mapped<U>(&self, f: impl FnMut((usize, &T)) -> U) -> NonEmptyVec<U> {
        NonEmptyVec {
            raw: self.raw.iter().enumerate().map(f).collect(),
        }
    }

    pub fn try_enumerate_to_mapped<U, E>(
        &self,
        f: impl FnMut((usize, &T)) -> Result<U, E>,
    ) -> Result<NonEmptyVec<U>, E> {
        Ok(NonEmptyVec {
            raw: self
                .raw
                .iter()
                .enumerate()
                .map(f)
                .collect::<Result<_, _>>()?,
        })
    }
}
