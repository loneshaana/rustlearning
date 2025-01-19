use std::fmt::Error;
use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::cell::Cell;
use crate::reference::{BorrowRef, BorrowRefMut, Ref, RefMut};
use crate::unsafecell::UnsafeCell;

type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;

#[inline(always)]
fn is_reading(x: BorrowFlag) -> bool {
    x > UNUSED
}

#[inline(always)]
fn is_writing(x: BorrowFlag) -> bool {
    x < UNUSED
}

/*
RefCell uses Rust's lifetimes to implement "dynamic borrowing" a process whereby one can claim temporary,
exclusive, mutable access to the inner value. Borrows for RefCell are tracked at runtime, unlike Rust's native reference types
which are entirely tracked at compile time.


An immutable reference to RefCell's inner value &T can be obtained with borrow and a mutable borrow &mut T can be
obtained with borrow_mut. When these functions are called, they first verify that Rust's borrow rules will be satistied.
any number of immutable borrows are allowed or a single mutable borrow is allowed, but never both.
If a borrow is attempted that would voilate these rules, the thread will panic.

The corresponding Sync version of RefCell is RwLock
*/

// A mutable memory location with dynamically checked borrow rules.
pub struct RefCell<T: ?Sized> {
    borrow: Cell<BorrowFlag>,
    value: UnsafeCell<T>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            borrow: Cell::new(UNUSED),
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }

    // Replaces the wrapper value with a new one, returning the old value
    // without deinitalizing either one.

    pub fn replace(&self, t: T) -> T {
        std::mem::replace(&mut *self.borrow_mut(), t)
    }

    pub fn swap(&self, other: &Self) {
        std::mem::swap(&mut *self.borrow_mut(), &mut *other.borrow_mut());
    }
}

impl<T: ?Sized> RefCell<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        match self.try_borrow() {
            Ok(val) => val,
            Err(_) => panic!("An immutable reference exists, so can't create a mutable reference"),
        }
    }

    pub fn try_borrow(&self) -> Result<Ref<'_, T>, Error> {
        match BorrowRef::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.value.get()) };
                Ok(Ref { value, borrow: b })
            }
            None => Err(Error),
        }
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<'_, T>, Error> {
        match BorrowRefMut::new(&self.borrow) {
            Some(b) => {
                let value = unsafe { NonNull::new_unchecked(self.value.get()) };
                Ok(RefMut {
                    value,
                    borrow: b,
                    marker: PhantomData,
                })
            }
            None => Err(Error),
        }
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        match self.try_borrow_mut() {
            Ok(b) => b,
            Err(_) => panic!("RefCell<T> already borrowed"),
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_new() {
        let cell = RefCell::new(5);
        assert_eq!(*cell.borrow(), 5);
    }

    #[test]
    fn test_replace() {
        let cell = RefCell::new(5);
        let old_value = cell.replace(10);
        assert_eq!(old_value, 5);
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    fn test_swap() {
        let cell1 = RefCell::new(5);
        let cell2 = RefCell::new(10);
        cell1.swap(&cell2);
        assert_eq!(*cell1.borrow(), 10);
        assert_eq!(*cell2.borrow(), 5);
    }

    #[test]
    fn test_borrow() {
        let cell = RefCell::new(5);
        let borrow = cell.borrow();
        assert_eq!(*borrow, 5);
    }

    #[test]
    fn test_borrow_mut() {
        let cell = RefCell::new(5);
        {
            let mut borrow_mut = cell.borrow_mut();
            *borrow_mut = 10;
        }
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    #[should_panic(expected = "RefCell<T> already borrowed")]
    fn test_borrow_mut_panic() {
        let cell = RefCell::new(5);
        let _borrow1 = cell.borrow_mut();
        let _borrow2 = cell.borrow_mut(); // This should panic
    }

    #[test]
    #[should_panic(expected = "RefCell<T> already borrowed")]
    fn test_borrow_panic() {
        let cell = RefCell::new(5);
        let _borrow1 = cell.borrow();
        let _borrow2 = cell.borrow_mut(); // This should panic
    }
}
