use crate::LinkedList;

/// Double-ended, exact-size iterator over list references
pub struct ListIter<'a, L: LinkedList + ?Sized>(
    pub(crate) &'a L,
    pub(crate) usize,
    pub(crate) usize,
);

/// Double-ended, exact-size iterator over mutable list references
pub struct ListIterMut<'a, L: LinkedList + ?Sized>(
    pub(crate) &'a mut L,
    pub(crate) usize,
    pub(crate) usize,
);

/// Double-ended, exact-size iterator over list values
pub struct ListIterOwned<L: LinkedList + ?Sized>(
    pub(crate) Box<L>,
    pub(crate) usize,
    pub(crate) usize,
);

impl<'a, L> Iterator for ListIter<'a, L>
where
    L: LinkedList,
{
    type Item = &'a L::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == self.2 {
            return None;
        }
        let res = self.0.get(self.1);
        self.1 += 1;
        res
    }
}

impl<'a, L> DoubleEndedIterator for ListIter<'a, L>
where
    L: LinkedList,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.1 == self.2 {
            return None;
        }
        self.2 -= 1;
        self.0.get(self.2)
    }
}

impl<'a, L> ExactSizeIterator for ListIter<'a, L>
where
    L: LinkedList,
{
    fn len(&self) -> usize {
        self.2 - self.1
    }
}

impl<'a, L> Iterator for ListIterMut<'a, L>
where
    L: LinkedList,
{
    type Item = &'a mut L::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == self.2 {
            return None;
        }
        self.1 += 1;
        unsafe { std::mem::transmute(self.0.get_mut(self.1 - 1)) }
    }
}

impl<'a, L> DoubleEndedIterator for ListIterMut<'a, L>
where
    L: LinkedList,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.1 == self.2 {
            return None;
        }
        self.2 -= 1;
        unsafe { std::mem::transmute(self.0.get_mut(self.2)) }
    }
}

impl<'a, L> ExactSizeIterator for ListIterMut<'a, L>
where
    L: LinkedList,
{
    fn len(&self) -> usize {
        self.2 - self.1
    }
}

impl<L> Iterator for ListIterOwned<L>
where
    L: LinkedList,
{
    type Item = L::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == self.2 {
            return None;
        }
        self.1 += 1;
        Some(std::mem::take(self.0.get_mut(self.1 - 1)?))
    }
}

impl<L> DoubleEndedIterator for ListIterOwned<L>
where
    L: LinkedList,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.1 == self.2 {
            return None;
        }
        self.2 -= 1;
        Some(std::mem::take(self.0.get_mut(self.2)?))
    }
}

impl<L> ExactSizeIterator for ListIterOwned<L>
where
    L: LinkedList,
{
    fn len(&self) -> usize {
        self.2 - self.1
    }
}
