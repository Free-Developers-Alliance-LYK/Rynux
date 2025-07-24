//! Rynux type
use core::cell::UnsafeCell;
use core::mem::MaybeUninit;

/// A cell which can be written to only once.
pub struct OnceCell<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    is_set: UnsafeCell<bool>,
}


impl<T> OnceCell<T> {
    /// Create a new OnceCell.
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            is_set: UnsafeCell::new(false),
        }
    }

    /// Set the value of the cell.
    pub fn set(&self, value: T) {
        if unsafe { *self.is_set.get() } {
            panic!("Already set");
        }
        unsafe {
            (*self.value.get()).write(value);
            *self.is_set.get() = true;
        }
    }

    /// Get the value of the cell.
    pub fn get(&self) -> Option<&T> {
        if unsafe { *self.is_set.get() } {
            Some(unsafe { (*self.value.get()).assume_init_ref() })
        } else {
            None
        }
    }
}

unsafe impl<T> Sync for OnceCell<T> {}

