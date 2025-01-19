use crate::cell::Cell;
use core::borrow;
use std::{
    borrow::Borrow,
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

impl Clone for BorrowRef<'_> {
    fn clone(&self) -> Self {
        let borrow = self.borrow.get();
        assert!(borrow != BorrowFlag::MAX);
        self.borrow.set(borrow + 1);
        BorrowRef {
            borrow: self.borrow,
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

// impl Clone for BorrowRefMut<'_> {
//     fn clone(&self) -> Self {
//         let borrow = self.borrow.get();
//         assert!(borrow != BorrowFlag::MIN);
//         self.borrow.set(borrow - 1);
//         BorrowRefMut {
//             borrow: self.borrow,
//         }
//     }
// }

impl Drop for BorrowRef<'_> {
    fn drop(&mut self) {
        let b = self.borrow.get();
        self.borrow.set(b - 1);
    }
}

impl Drop for BorrowRefMut<'_> {
    fn drop(&mut self) {
        let b = self.borrow.get();
        self.borrow.set(b + 1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_ref_new() {
        let cell = Cell::new(UNUSED);
        let borrow_ref = BorrowRef::new(&cell);
        assert!(borrow_ref.is_some());
        assert_eq!(cell.get(), 1);
    }

    #[test]
    fn test_borrow_ref_clone() {
        let cell = Cell::new(UNUSED);
        let borrow_ref = BorrowRef::new(&cell).unwrap();
        let borrow_ref_clone = borrow_ref.clone();
        assert_eq!(cell.get(), 2);
    }

    #[test]
    fn test_borrow_ref_drop() {
        let cell = Cell::new(UNUSED);
        {
            let borrow_ref = BorrowRef::new(&cell).unwrap();
            assert_eq!(cell.get(), 1);
        }
        assert_eq!(cell.get(), UNUSED);
    }

    #[test]
    fn test_borrow_ref_mut_new() {
        let cell = Cell::new(UNUSED);
        let borrow_ref_mut = BorrowRefMut::new(&cell);
        assert!(borrow_ref_mut.is_some());
        assert_eq!(cell.get(), UNUSED - 1);
    }

    #[test]
    fn test_borrow_ref_mut_drop() {
        let cell = Cell::new(UNUSED);
        {
            let borrow_ref_mut = BorrowRefMut::new(&cell).unwrap();
            assert_eq!(cell.get(), UNUSED - 1);
        }
        assert_eq!(cell.get(), UNUSED);
    }

    #[test]
    fn test_ref_deref() {
        let value = 42;
        let cell = Cell::new(UNUSED);
        let borrow_ref = BorrowRef::new(&cell).unwrap();
        let ref_value = Ref {
            value: NonNull::from(&value),
            borrow: borrow_ref,
        };
        assert_eq!(*ref_value, 42);
    }

    #[test]
    fn test_ref_mut_deref() {
        let mut value = 42;
        let cell = Cell::new(UNUSED);
        let borrow_ref_mut = BorrowRefMut::new(&cell).unwrap();
        let mut ref_mut_value = RefMut {
            value: NonNull::from(&mut value),
            borrow: borrow_ref_mut,
            marker: PhantomData,
        };
        assert_eq!(*ref_mut_value, 42);
        *ref_mut_value = 43;
        assert_eq!(*ref_mut_value, 43);
    }
}
