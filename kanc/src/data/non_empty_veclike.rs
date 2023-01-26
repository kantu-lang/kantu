use std::{
    convert::TryFrom,
    iter::IntoIterator,
    marker::PhantomData,
    num::NonZeroUsize,
    ops::{Deref, DerefMut, Index, IndexMut, Range},
};

use bumpalo::collections::Vec as BumpVec;

pub type NonEmptyVec<T> = NonEmptyVeclike<T, Vec<T>>;

pub type NonEmptyBumpVec<'bump, T> = NonEmptyVeclike<T, BumpVec<'bump, T>>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct NonEmptyVeclike<T, V: Veclike<T>> {
    _phantom: PhantomData<T>,
    raw: V,
}

pub trait Veclike<T>:
    Deref<Target = [T]> + DerefMut + FromIterator<T> + IntoIterator<Item = T> + Extend<T> + Default
{
    fn singleton(t: T) -> Self;

    fn cloned_from_slice(slice: &[T]) -> Self
    where
        T: Clone;

    fn push(&mut self, t: T);

    fn append(&mut self, other: &mut Self);

    fn pop(&mut self) -> Option<T>;
}

impl<T> Veclike<T> for Vec<T> {
    fn singleton(t: T) -> Self {
        vec![t]
    }

    fn cloned_from_slice(slice: &[T]) -> Self
    where
        T: Clone,
    {
        slice.to_vec()
    }

    fn push(&mut self, t: T) {
        Vec::push(self, t);
    }

    fn append(&mut self, other: &mut Self) {
        Vec::append(self, other);
    }

    fn pop(&mut self) -> Option<T> {
        Vec::pop(self)
    }
}

impl<T, V: Veclike<T>> NonEmptyVeclike<T, V> {
    pub fn singleton(value: T) -> Self {
        Self {
            _phantom: PhantomData,
            raw: V::singleton(value),
        }
    }

    pub fn from_pushed(mut original: V, end: T) -> NonEmptyVeclike<T, V> {
        original.push(end);
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: original,
        }
    }
}

impl<T, VT: Veclike<T>> NonEmptyVeclike<T, VT> {
    pub fn into_raw(self) -> VT {
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

    pub fn append(&mut self, other: &mut VT) {
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

    pub fn into_popped(mut self) -> (VT, T) {
        let last = self.raw.pop().unwrap();
        (self.raw, last)
    }

    pub fn into_mapped<U, V: Veclike<U>>(self, f: impl FnMut(T) -> U) -> NonEmptyVeclike<U, V> {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.into_iter().map(f).collect::<V>(),
        }
    }

    pub fn enumerate_into_mapped<U, V: Veclike<U>>(
        self,
        f: impl FnMut((usize, T)) -> U,
    ) -> NonEmptyVeclike<U, V> {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.into_iter().enumerate().map(f).collect(),
        }
    }

    pub fn try_enumerate_into_mapped<U, V: Veclike<U>, E>(
        self,
        f: impl FnMut((usize, T)) -> Result<U, E>,
    ) -> Result<NonEmptyVeclike<U, V>, E> {
        Ok(NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self
                .raw
                .into_iter()
                .enumerate()
                .map(f)
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn try_into_mapped<U, V: Veclike<U>, E>(
        self,
        f: impl FnMut(T) -> Result<U, E>,
    ) -> Result<NonEmptyVeclike<U, V>, E> {
        Ok(NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.into_iter().map(f).collect::<Result<_, _>>()?,
        })
    }
}

impl<T, V: Veclike<T>> IntoIterator for NonEmptyVeclike<T, V> {
    type Item = T;
    type IntoIter = V::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

impl<'a, T, V: Veclike<T>> IntoIterator for &'a NonEmptyVeclike<T, V> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter()
    }
}

impl<'a, T, V: Veclike<T>> IntoIterator for &'a mut NonEmptyVeclike<T, V> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.iter_mut()
    }
}

impl<T, V: Veclike<T>> Index<usize> for NonEmptyVeclike<T, V> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.raw[index]
    }
}

impl<T, V: Veclike<T>> IndexMut<usize> for NonEmptyVeclike<T, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.raw[index]
    }
}

impl<T, V: Veclike<T>> Index<Range<usize>> for NonEmptyVeclike<T, V> {
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.raw[index]
    }
}

impl<T, V: Veclike<T>> IndexMut<Range<usize>> for NonEmptyVeclike<T, V> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.raw[index]
    }
}

impl<T, V: Veclike<T>> Deref for NonEmptyVeclike<T, V> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl<T, V: Veclike<T>> DerefMut for NonEmptyVeclike<T, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.raw
    }
}

pub trait OptionalNonEmptyToPossiblyEmpty {
    type PossiblyEmptyOwned;
    type PossiblyEmptyRef<'a>
    where
        Self: 'a;
    type PossiblyEmptyMut<'a>
    where
        Self: 'a;

    fn into_possibly_empty(self) -> Self::PossiblyEmptyOwned;

    fn to_possibly_empty<'b>(&'b self) -> Self::PossiblyEmptyRef<'b>;

    fn to_possibly_empty_mut<'b>(&'b mut self) -> Self::PossiblyEmptyMut<'b>;
}

impl<T> OptionalNonEmptyToPossiblyEmpty for Option<NonEmptyVeclike<T, Vec<T>>> {
    type PossiblyEmptyOwned = Vec<T>;
    type PossiblyEmptyRef<'a> = &'a [T] where
    Self: 'a;
    type PossiblyEmptyMut<'a> = &'a mut [T] where
    Self: 'a;

    fn into_possibly_empty(self) -> Vec<T> {
        self.map_or_else(Vec::new, Vec::from)
    }

    fn to_possibly_empty<'b>(&'b self) -> &'b [T] {
        match self.as_ref() {
            Some(v) => v.as_ref(),
            None => &[],
        }
    }

    fn to_possibly_empty_mut(&mut self) -> &mut [T] {
        match self.as_mut() {
            Some(v) => v.as_mut(),
            None => &mut [],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EmptyVecError;

impl<T> TryFrom<Vec<T>> for NonEmptyVeclike<T, Vec<T>> {
    type Error = EmptyVecError;

    fn try_from(value: Vec<T>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(EmptyVecError)
        } else {
            Ok(Self {
                _phantom: PhantomData,
                raw: value,
            })
        }
    }
}

impl<T> From<NonEmptyVeclike<T, Vec<T>>> for Vec<T> {
    fn from(value: NonEmptyVeclike<T, Vec<T>>) -> Self {
        value.raw
    }
}

macro_rules! impl_from_array {
    ($n:expr) => {
        impl<T> From<[T; $n]> for NonEmptyVeclike<T, Vec<T>> {
            fn from(value: [T; $n]) -> Self {
                Self {
                    _phantom: PhantomData,
                    raw: value.into(),
                }
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

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, V: Veclike<T>> OptionalNonEmptyVecLen for Option<NonEmptyVeclike<T, V>> {
    fn len(&self) -> usize {
        self.as_ref().map(|v| v.len()).unwrap_or(0)
    }
}

impl<T, V: Veclike<T>> NonEmptyVeclike<T, V> {
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
    pub fn to_non_empty_veclike<V: Veclike<T>>(&self) -> NonEmptyVeclike<T, V>
    where
        T: Clone,
    {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: V::cloned_from_slice(self.raw),
        }
    }

    pub fn to_non_empty_vec(&self) -> NonEmptyVeclike<T, Vec<T>>
    where
        T: Clone,
    {
        self.to_non_empty_veclike()
    }

    pub fn to_mapped<U, V: Veclike<U>>(&self, f: impl FnMut(&T) -> U) -> NonEmptyVeclike<U, V>
    where
        T: Clone,
    {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.iter().map(f).collect(),
        }
    }

    pub fn try_to_mapped<U, V: Veclike<U>, E>(
        &self,
        f: impl FnMut(&T) -> Result<U, E>,
    ) -> Result<NonEmptyVeclike<U, V>, E>
    where
        T: Clone,
    {
        Ok(NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.iter().map(f).collect::<Result<_, _>>()?,
        })
    }

    pub fn enumerate_to_mapped<U, V: Veclike<U>>(
        &self,
        f: impl FnMut((usize, &T)) -> U,
    ) -> NonEmptyVeclike<U, V> {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.iter().enumerate().map(f).collect(),
        }
    }

    pub fn try_enumerate_to_mapped<U, V: Veclike<U>, E>(
        &self,
        f: impl FnMut((usize, &T)) -> Result<U, E>,
    ) -> Result<NonEmptyVeclike<U, V>, E> {
        Ok(NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self
                .raw
                .iter()
                .enumerate()
                .map(f)
                .collect::<Result<_, _>>()?,
        })
    }

    pub fn map_to_unzipped<U1, U2, V1: Veclike<U1>, V2: Veclike<U2>>(
        &self,
        f: impl FnMut(&T) -> (U1, U2),
    ) -> (NonEmptyVeclike<U1, V1>, NonEmptyVeclike<U2, V2>) {
        let (u, v) = self.raw.iter().map(f).unzip();
        (
            NonEmptyVeclike {
                _phantom: PhantomData,
                raw: u,
            },
            NonEmptyVeclike {
                _phantom: PhantomData,
                raw: v,
            },
        )
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
    pub fn to_non_empty_veclike<V: Veclike<T>>(&self) -> NonEmptyVeclike<T, V>
    where
        T: Clone,
    {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: V::cloned_from_slice(self.raw),
        }
    }

    pub fn to_non_empty_vec(&self) -> NonEmptyVeclike<T, Vec<T>>
    where
        T: Clone,
    {
        self.to_non_empty_veclike()
    }

    pub fn to_mapped<U, V: Veclike<U>>(&self, f: impl FnMut(&T) -> U) -> NonEmptyVeclike<U, V>
    where
        T: Clone,
    {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.iter().map(f).collect(),
        }
    }

    pub fn try_to_mapped<U, V: Veclike<U>, E>(
        &self,
        f: impl FnMut(&T) -> Result<U, E>,
    ) -> Result<NonEmptyVeclike<U, V>, E>
    where
        T: Clone,
    {
        Ok(NonEmptyVeclike {
            _phantom: PhantomData,
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

    pub fn enumerate_to_mapped<U, V: Veclike<U>>(
        &self,
        f: impl FnMut((usize, &T)) -> U,
    ) -> NonEmptyVeclike<U, V> {
        NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self.raw.iter().enumerate().map(f).collect(),
        }
    }

    pub fn try_enumerate_to_mapped<U, V: Veclike<U>, E>(
        &self,
        f: impl FnMut((usize, &T)) -> Result<U, E>,
    ) -> Result<NonEmptyVeclike<U, V>, E> {
        Ok(NonEmptyVeclike {
            _phantom: PhantomData,
            raw: self
                .raw
                .iter()
                .enumerate()
                .map(f)
                .collect::<Result<_, _>>()?,
        })
    }
}
