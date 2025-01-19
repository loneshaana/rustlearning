use crate::unsafecell::UnsafeCell;

pub struct SyncUnsafeCell<T: ?Sized> {
    value: UnsafeCell<T>,
}

// Allow Sync if T is Sync
unsafe impl<T: ?Sized + Sync> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
        }
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}

impl<T: ?Sized> SyncUnsafeCell<T> {
    pub const fn get(&self) -> *mut T {
        self.value.get()
    }

    pub const fn get_mut(&mut self) -> &mut T {
        self.value.get_mut()
    }
}

impl<T: Default> Default for SyncUnsafeCell<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for SyncUnsafeCell<T> {
    fn from(t: T) -> SyncUnsafeCell<T> {
        SyncUnsafeCell::new(t)
    }
}
#[cfg(test)]
mod tests {
    use super::SyncUnsafeCell;

    #[test]
    fn test_new() {
        let cell = SyncUnsafeCell::new(42);
        assert_eq!(unsafe { *cell.get() }, 42);
    }

    #[test]
    fn test_into_inner() {
        let cell = SyncUnsafeCell::new(42);
        assert_eq!(cell.into_inner(), 42);
    }

    #[test]
    fn test_get() {
        let cell = SyncUnsafeCell::new(42);
        assert_eq!(unsafe { *cell.get() }, 42);
    }

    #[test]
    fn test_get_mut() {
        let mut cell = SyncUnsafeCell::new(42);
        *cell.get_mut() = 43;
        assert_eq!(unsafe { *cell.get() }, 43);
    }

    #[test]
    fn test_default() {
        let cell: SyncUnsafeCell<i32> = Default::default();
        assert_eq!(unsafe { *cell.get() }, 0);
    }

    #[test]
    fn test_from() {
        let cell: SyncUnsafeCell<i32> = SyncUnsafeCell::from(42);
        assert_eq!(unsafe { *cell.get() }, 42);
    }
}
