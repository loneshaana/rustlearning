use std::fmt::Debug;

use crate::unsafecell::UnsafeCell;

#[derive(Debug)]
pub struct Cell<T: ?Sized> {
    value: UnsafeCell<T>,
}

unsafe impl<T: ?Sized> Send for Cell<T> where T: Send {}

// Note that this negative impl isn't strictly necessary for correctness,
// as `Cell` wraps `UnsafeCell`, which is itself !Sync
// However, given how important `Cell`'s `!Sync`-ness is,
// having an explicit negative impl is nice for documentation purposes
// and results in nicer error messages.
impl<T: ?Sized> !Sync for Cell<T> {}

impl<T: Copy> Clone for Cell<T> {
    #[inline]
    fn clone(&self) -> Self {
        Cell::new(self.get())
    }
}

impl<T: Default> Default for Cell<T> {
    fn default() -> Self {
        Cell::new(Default::default())
    }
}

impl<T: PartialEq + Copy> PartialEq for Cell<T> {
    fn eq(&self, other: &Cell<T>) -> bool {
        self.get() == other.get()
    }
}

impl<T> From<T> for Cell<T> {
    fn from(t: T) -> Self {
        Cell::new(t)
    }
}

impl<T> Cell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, val: T) {
        self.replace(val);
    }

    // SAFETY: This can cause data races if called from a separate thread
    // but `Cell` is not `Sync` so this is safe
    pub fn replace(&self, val: T) {
        unsafe {
            let old = self.value.get();
            *old = val;
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

// Add functionality to Cell if T has a Copy trait.
impl<T: Copy> Cell<T> {
    // Returns Copy of the contained value

    pub fn get(&self) -> T {
        // SAFETY: This can cause data races if called from a separate thread
        // but `Cell` is not `Sync` so this is safe
        unsafe { *self.value.get() }
    }
}

impl<T: ?Sized> Cell<T> {
    // Returns a raw pointer to the underlying data in this cell.

    pub fn as_ptr(&self) -> *mut T {
        self.value.get()
    }

    // Returns a mutable reference to the underlying data.
    // This call borrows `Cell` mutably(at-compile time) which guarantees that we possess the only reference.

    pub fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }

    // Returns a `&Cell<T>` from a `&mut T`

    pub fn from_mut(t: &mut T) -> &Cell<T> {
        unsafe { &*(t as *mut T as *const Cell<T>) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let c = Cell {
            value: UnsafeCell::new(42),
        };
        assert_eq!(c.get(), 42);
    }

    #[test]
    fn test_set() {
        let c = Cell::new(10);
        c.set(20);
        assert_eq!(c.get(), 20);
    }

    #[test]
    fn test_replace() {
        let c = Cell::new(30);
        c.replace(40);
        assert_eq!(c.get(), 40);
    }

    #[test]
    fn test_into_inner() {
        let c = Cell::new(50);
        assert_eq!(c.into_inner(), 50);
    }

    #[test]
    fn test_clone() {
        let c1 = Cell::new(60);
        let c2 = c1.clone();
        assert_eq!(c1.get(), c2.get());
    }

    #[test]
    fn test_default() {
        let c: Cell<i32> = Cell::default();
        assert_eq!(c.get(), 0);
    }

    #[test]
    fn test_partial_eq() {
        let c1 = Cell::new(70);
        let c2 = Cell::new(70);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_from() {
        let c = Cell::from(80);
        assert_eq!(c.get(), 80);
    }

    #[test]
    fn test_as_ptr() {
        let c = Cell::new(90);
        let ptr = c.as_ptr();
        unsafe {
            assert_eq!(*ptr, 90);
        }
    }

    #[test]
    fn test_get_mut() {
        let mut c = Cell::new(100);
        *c.get_mut() = 110;
        assert_eq!(c.get(), 110);
    }

    #[test]
    fn test_from_mut() {
        let mut value = 120;
        let c = Cell::from_mut(&mut value);
        c.set(130);
        assert_eq!(value, 130);
    }
}
