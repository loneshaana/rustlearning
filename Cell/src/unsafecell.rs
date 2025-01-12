#![feature(auto_traits, negative_impls)]

pub struct UnsafeCell<T: ?Sized> {
    value: T,
}

impl<T: ?Sized> !Sync for UnsafeCell<T> {}

impl<T> UnsafeCell<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    // unwraps the value, consuming the cell.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: ?Sized> UnsafeCell<T> {
    // Gets a mutable pointer to the wrapped value.

    pub fn get(&self) -> *mut T {
        // the case converts self into a raw, immutable pointer of an UnsafeCell<T>

        // cast changes the pointer from *const UnsafeCell<T> to *const T
        // Since UnsafeCell<T> has the same in-memory representation as its inner type T, this case is valid.

        // *const T as *mut T
        // This final cast converts the immutable pointer *const T to a mutable pointer *mut T.
        // while casting from *const T to *mut T is allowed, it doen't inherently make the data mutable
        // to safely mutate the data, you must ensure that no other referneces(mutable or immutable)
        // to the data exist.
        self as *const UnsafeCell<T> as *const T as *mut T
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    pub fn from_mut(value: &mut T) -> &mut Self<T> {
        // SAFETY: UnsafeCell<T> has the same memory layout as T
        unsafe { &mut *(value as *mut T as *mut Self<T>) }
    }
}

impl<T: Default> Default for UnsafeCell<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> FromT<T> for UnsafeCell<T> {
    fn from(t: T) -> Self {
        UnsafeCell::new(t)
    }
}
