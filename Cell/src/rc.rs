use std::{marker::PhantomData, ops::Deref, ptr::NonNull};

use crate::cell::Cell;

/// Single threaded reference counting pointers. `Rc` stands for Reference Counted.
/// The Type Rc<T> provides shared ownership of a value of type `T` allocated in the heap
/// Invoking `Clone` on `Rc` produces a new pointer to the same allocation in the heap
/// When the last `Rc` pointer to a given allocation is destroyed, the value stored in that allocation is also droped
///
/// Shared references in rust disallow mutation by default and `Rc` is no exception
/// you cannot generally obtain a mutable reference to something inside an `Rc`.
/// if you need mutability, put a `Cell` or `RefCell` inside the `Rc`
///
/// `Rc` uses non-atomic reference counting. This means that overhead is very low but an `Rc` cannot be sent
/// between threads and consequently `Rc` does not implement `Send`. As a result the Rust compiler will check
/// at compile time that you are not sending `Rc`s between threads. If you need multi-threaded atomic
/// reference counting use sync::Arc

struct RcInner<T: ?Sized> {
    refcount: Cell<usize>,
    value: T,
}

pub struct Rc<T: ?Sized> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>, // PhantomData tells the compiler that when we drop Rc, check the Inner T if is dropped.
}

impl<T> !Sync for Rc<T> {}
impl<T> !Send for Rc<T> {}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        Rc {
            // SAFETY: Box does not give us a Null pointer.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: self._marker,
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T: ?Sized> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            drop(inner);
            // SAFETY: we are the only reference left, and we are being dropped.
            // therefore, after us, there will be no Rc's and no reference to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rc's so don't drop the Box!.
            inner.refcount.set(c - 1);
        }
    }
}

// *mut , *const -> Raw pointers
// &(Shared reference)
// &mut Exclusive reference , no shared reference.

// *mut is like you might be able to mutate, something like you might have an exclusive reference to

// *const is like you never can be able to mutate
// from *mut you can go exclusive reference. using unsafe, but have to document why its safe.

// !Sized , not Sized at all
// ?Sized, it does not have to be Sized

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rc_new() {
        let rc = Rc::new(5);
        assert_eq!(*rc, 5);
    }

    #[test]
    fn test_rc_clone() {
        let rc1 = Rc::new(5);
        let rc2 = rc1.clone();
        assert_eq!(*rc1, 5);
        assert_eq!(*rc2, 5);
    }

    #[test]
    fn test_rc_refcount() {
        let rc1 = Rc::new(5);
        let rc2 = rc1.clone();
        let rc3 = rc2.clone();
        assert_eq!(*rc1, 5);
        assert_eq!(*rc2, 5);
        assert_eq!(*rc3, 5);
    }

    #[test]
    fn test_rc_drop() {
        struct DropTest {
            dropped: Rc<Cell<bool>>,
        }

        impl Drop for DropTest {
            fn drop(&mut self) {
                self.dropped.set(true);
            }
        }

        let dropped = Rc::new(Cell::new(false));
        {
            let _rc = Rc::new(DropTest {
                dropped: dropped.clone(),
            });
        }
        assert!(dropped.get());
    }
}
