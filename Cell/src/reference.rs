use crate::cell::Cell;
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

pub type BorrowFlag = isize;
const UNUSED: BorrowFlag = 0;

#[inline(always)]
fn is_reading(b: BorrowFlag) -> bool {
    b > UNUSED
}
// BorrowRef takes some lifetype and attachs this lifetime with Cell<BorrowFlag>
pub struct BorrowRef<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

// A Ref contains the NonNull value of T with a borrow ptr.
pub struct Ref<'b, T: ?Sized + 'b> {
    pub value: NonNull<T>,
    pub borrow: BorrowRef<'b>,
}

impl<T: ?Sized> Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() } // return the reference
    }
}

pub struct BorrowRefMut<'b> {
    borrow: &'b Cell<BorrowFlag>,
}

pub struct RefMut<'b, T: ?Sized + 'b> {
    // NB: we use a pointer instead of `&'b mut T` to avoid `noalias` violations, because a
    // `RefMut` argument doesn't hold exclusivity for its whole scope, only until it drops.
    pub value: NonNull<T>,
    pub borrow: BorrowRefMut<'b>,
    pub marker: PhantomData<&'b mut T>,
}

impl<T: ?Sized> Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.value.as_mut() } // return the mutable reference.
    }
}

impl<'b> BorrowRef<'b> {
    pub fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRef<'b>> {
        let b = borrow.get().wrapping_add(1);
        if !is_reading(b) {
            // is someone is not reading it , means there is an immutable reference. so we can not send a mutable reference.
            None
        } else {
            borrow.set(b);
            Some(BorrowRef { borrow })
        }
    }
}

impl<'b> BorrowRefMut<'b> {
    pub fn new(borrow: &'b Cell<BorrowFlag>) -> Option<BorrowRefMut<'b>> {
        match borrow.get() {
            UNUSED => {
                // no mutable and immutable reference exists
                borrow.set(UNUSED - 1); // decrement to mark and immutable refenrece.
                Some(BorrowRefMut { borrow })
            }
            _ => None,
        }
    }
}
