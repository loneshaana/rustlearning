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
            Err(_) => panic!("An immutable refernece exists, so can't create a mutable refernece"),
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
