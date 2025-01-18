/*
    OnceCell<T>

    A cell which can nominally be written to only once.

    This allows obtaining a shared &T reference to its inner value without copying or replacing it unlike Cell
    and without runtime borrow checks unlike RefCell.

    However only immutable references can be obtained unless one has a mutable reference to the cell itself.
    In the same vein, the cell can only be re-initialized with such a mutable reference.

    For thread-safe version of this struct see OnceLock
*/

use std::mem;

use crate::unsafecell::UnsafeCell;

pub struct OnceCell<T> {
    // Invariant: written to at most once.
    inner: UnsafeCell<Option<T>>,
}

impl<T> !Sync for OnceCell<T> {}

impl<T> OnceCell<T> {
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: UnsafeCell::new(None),
        }
    }

    pub fn get(&self) -> Option<&T> {
        // SAFETY: safe due to inner's invariant
        unsafe { &*self.inner.get() }.as_ref()
    }

    // Gets the mutable reference to the underlying value.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        self.inner.get_mut().as_mut()
    }

    // Sets the contents of the cell to 'value'

    /// # Errors
    /// This method return `Ok(()) if the cell was empty and `Err(value)` if
    /// it was full
    ///
    pub fn set(&self, value: T) -> Result<(), T> {
        match self.try_insert(value) {
            Ok(_) => Ok(()),
            Err((_, value)) => Err(value),
        }
    }

    /// sets the contents of the cell `value` if the cell was empty, then
    /// returns a reference to it.
    pub fn try_insert(&self, value: T) -> Result<&T, (&T, T)> {
        // If there is an existing value then return an Error with the Old Value.
        if let Some(old) = self.get() {
            return Err((old, value));
        }

        // SAFETY: this is the only place where we set the slot. no races
        // due to reentrancy/concurrancy are possible, and we've
        // checked that slot is currently `None`, so this write
        // maintains the inner's invarient

        // let slot = unsafe { &mut *self.inner.get() };
        // slot.insert(value);
        let _ = unsafe { *self.inner.get() = Some(value) }; // update the value.
        let v = unsafe { (*self.inner.get()).as_mut().unwrap_unchecked() }; //get the updated value.
        Ok(v)
    }

    /// Consumes the cell, returning the wrapped value.
    /// Returns `None` if the cell was empty
    pub fn into_inner(self) -> Option<T> {
        self.inner.into_inner()
    }

    /// Takes the value out of this `OnceCell`, moving it back to an uninitialized state.
    /// Has no effect and returns `None` if the `OnceCell` hasn't been initialized.
    ///
    /// SAFETY is guaranteed by requiring a mutable reference.
    ///
    pub fn take(&mut self) -> Option<T> {
        mem::take(self).into_inner()
    }
}

impl<T> Default for OnceCell<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for OnceCell<T> {
    fn clone(&self) -> Self {
        // create new cell
        let new_cell = OnceCell::new();
        if let Some(curr) = self.get() {
            match new_cell.set(curr.clone()) {
                Ok(_) => (),
                Err(_) => unreachable!(),
            }
        }
        new_cell
    }
}

impl<T> From<T> for OnceCell<T> {
    fn from(value: T) -> Self {
        OnceCell {
            inner: UnsafeCell::new(Some(value)),
        }
    }
}

impl<T: PartialEq> PartialEq for OnceCell<T> {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T: Eq> Eq for OnceCell<T> {}
