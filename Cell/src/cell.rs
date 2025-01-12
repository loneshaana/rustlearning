#![feature(auto_traits, negative_impls)]

use std::cell::UnsafeCell;

struct Cell<T: ?Sized> {
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let c = Cell {
//             value: UnsafeCell::new(42),
//         };
//         let d = Cell::new(42);
//         assert_eq!(c.value, 42);
//         assert_eq!(c.value, d.value);
//     }
// }
