//! An implementation of a static list structure
//! with traits and generics
#![deny(missing_docs)]

use std::marker::PhantomData;
use std::mem;

mod index;
mod iter;
mod macros;

pub use index::{Indexable, ToIndexable};
pub use iter::{ListIter, ListIterMut, ListIterOwned};

/// Trait representing a "linked list"
///
/// These must be implemented by types that consist the list.
///
/// Functions accepting a list should either take a trait object,
/// or a generic parameter satisfying [`LinkedList`]
pub trait LinkedList: Default
where
    Self: Sized,
{
    /// Type of contained value
    type Item: Default;
    /// Type of the list if you remove the top
    /// element or recurse to the next element
    type Next: LinkedList;
    /// Type of the list if you add an element
    type Pushed: LinkedList;

    /// Length from this element to the tail
    fn len(&self) -> usize;

    /// If the list is not empty, return a reference
    /// to first element
    fn head(&self) -> Option<&Self::Item>;
    /// Return a mutable reference to first element,
    /// if the list isn't empty
    fn head_mut(&mut self) -> Option<&mut Self::Item>;

    /// Remove the first element of the list, returning
    /// it along with the remainder of the list
    fn pop(self) -> (Option<Self::Item>, Self::Next);
    /// Add a new element to the beginnning of the list
    fn push(self, elem: Self::Item) -> Self::Pushed;
    /// Remove an element from the list at given index,
    /// items after the element have their index shifted by one
    /// to prevent holes in the list
    fn remove(self, idx: usize) -> (Self::Item, Self::Next);
    /// Insert a new element to the list at given index,
    /// where idx at most equals `len` before insertion
    fn insert(self, idx: usize, elem: Self::Item) -> Self::Pushed;

    /// Get an immutable reference to element at index
    fn get(&self, idx: usize) -> Option<&Self::Item>;
    /// Iterator over references
    fn iter<'a>(&'a self) -> ListIter<'a, Self> {
        let len = self.len();
        ListIter(self, 0, len)
    }

    /// Get a mutable reference for element at index
    fn get_mut(&mut self, _idx: usize) -> Option<&mut Self::Item>;
    /// Iterate over mutable references
    fn iter_mut<'a>(&'a mut self) -> ListIterMut<'a, Self> {
        let len = self.len();
        ListIterMut(self, 0, len)
    }

    /// Convert into an iterator over owned values, consuming
    /// the list in the process
    fn into_iter(self) -> ListIterOwned<Self> {
        let len = self.len();
        ListIterOwned(Box::new(self), 0, len)
    }
}

/// Contains an element and the rest of the list
#[derive(Debug, Default)]
pub struct Cons<T: Default, N: LinkedList>(pub T, pub N);
/// Tail of the list
#[derive(Debug, Default)]
pub struct Nil<T>(pub PhantomData<T>);

#[rustfmt::skip]
impl<T> LinkedList for Nil<T>
where
    T: Default,
{
    type Item = T;
    type Next = Nil<T>;
    type Pushed = Cons<Self::Item, Self>;

    fn head(&self) -> Option<&Self::Item> { None }
    fn head_mut(&mut self) -> Option<&mut Self::Item> { None }
    fn get(&self, _idx: usize) -> Option<&Self::Item> { None }
    fn get_mut(&mut self, _idx: usize) -> Option<&mut Self::Item> { None }
    fn len(&self) -> usize { 0 }
    fn pop(self) -> (Option<Self::Item>, Self::Next) { (None, self) }
    fn push(self, elem: Self::Item) -> Self::Pushed { Cons(elem, self) }
    fn remove(self, _idx: usize) -> (Self::Item, Self::Next) {
        panic!("Can't remove from an empty list")
    }
    fn insert(self, idx: usize, elem: Self::Item) -> Self::Pushed {
        if idx != 0 {
            panic!("Can't insert to an index other than 0");
        }
        self.push(elem)
    }
}

impl<T, N> LinkedList for Cons<T, N>
where
    N: LinkedList<Item = T>,
    T: Default,
{
    type Item = T;
    type Next = N;
    type Pushed = Cons<Self::Item, Self>;

    fn head(&self) -> Option<&Self::Item> {
        Some(&self.0)
    }

    fn head_mut(&mut self) -> Option<&mut Self::Item> {
        Some(&mut self.0)
    }

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        if idx == 0 {
            self.head()
        } else {
            self.1.get(idx - 1)
        }
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        if idx == 0 {
            self.head_mut()
        } else {
            self.1.get_mut(idx - 1)
        }
    }
    fn len(&self) -> usize {
        1 + self.1.len()
    }
    fn pop(self) -> (Option<Self::Item>, Self::Next) {
        (Some(self.0), self.1)
    }

    fn push(self, elem: Self::Item) -> Self::Pushed {
        Cons(elem, self)
    }
    fn remove(mut self, idx: usize) -> (Self::Item, Self::Next) {
        if idx == 0 {
            if let (Some(head), tail) = self.pop() {
                (head, tail)
            } else {
                unreachable!("Presence of element is certain")
            }
        } else if idx >= self.len() {
            panic!("Index out of bounds")
        } else {
            let mut res: Self::Item = Self::Item::default();
            for (i, item) in self.iter_mut().enumerate() {
                if idx == i {
                    res = mem::take(item);
                }
            }
            let mut next: Self::Item = mem::take(self.head_mut().unwrap());
            for (i, item) in self.iter_mut().enumerate().skip(1) {
                if i <= idx {
                    let tmp = mem::take(item);
                    *item = next;
                    next = tmp;
                }
            }
            (res, self.pop().1)
        }
    }
    fn insert(self, idx: usize, elem: Self::Item) -> Self::Pushed {
        if idx == 0 {
            self.push(elem)
        } else if idx > self.len() {
            panic!("Index out of bounds")
        } else {
            let mut tmp = self.push(Default::default());

            for i in 0..idx {
                let val = mem::take(tmp.get_mut(i + 1).unwrap());
                (&mut tmp).indexable()[i] = val;
            }

            (&mut tmp).indexable()[idx] = elem;

            tmp
        }
    }
}
