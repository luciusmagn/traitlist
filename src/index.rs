//! Helper structs and traits to facilitate
//! indexing a [`LinkedList`]
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

use crate::LinkedList;

/// An arbitrary structure to prevent orphan instances when
/// implementing index for [`LinkedList`]
pub struct Indexable<L, H>(pub(crate) L, pub(crate) PhantomData<H>);

/// Trait for conversion into an object that satisfies
/// [`std::ops::Index`] and [`std::ops::IndexMut`]
pub trait ToIndexable<L, H> {
    /// Return an indexable object
    fn indexable(self) -> Indexable<L, H>;
}

impl<'a, L> ToIndexable<&'a L, L> for &'a L
where
    L: LinkedList,
{
    fn indexable(self) -> Indexable<&'a L, L> {
        Indexable(self, PhantomData)
    }
}

impl<'a, L> ToIndexable<&'a mut L, L> for &'a mut L
where
    L: LinkedList,
{
    fn indexable(self) -> Indexable<&'a mut L, L> {
        Indexable(self, PhantomData)
    }
}

impl<'a, L> Index<usize> for Indexable<&'a L, L>
where
    L: LinkedList,
{
    type Output = L::Item;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get(index).unwrap()
    }
}

impl<'a, L> Index<usize> for Indexable<&'a mut L, L>
where
    L: LinkedList,
{
    type Output = L::Item;
    fn index(&self, _: usize) -> &Self::Output {
        unreachable!("The mutable borrow should always kick in")
    }
}

impl<'a, L> IndexMut<usize> for Indexable<&'a mut L, L>
where
    L: LinkedList,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}
