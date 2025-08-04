//! Rynux type

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::marker::PhantomData;

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



/// Zero-sized type to mark types not [`Send`].
///
/// Refer to linux: ruser/kernel/types.rs
///
/// Add this type as a field to your struct if your type should not be sent to a different task.
/// Since [`Send`] is an auto trait, adding a single field that is `!Send` will ensure that the
/// whole type is `!Send`.
///
/// If a type is `!Send` it is impossible to give control over an instance of the type to another
/// task. This is useful to include in types that store or reference task-local information. A file
/// descriptor is an example of such task-local information.
///
/// This type also makes the type `!Sync`, which prevents immutable access to the value from
/// several threads in parallel.
pub type NotThreadSafe = PhantomData<*mut ()>;


/// Used to construct instances of type [`NotThreadSafe`] similar to how `PhantomData` is
/// constructed.
///
/// [`NotThreadSafe`]: type@NotThreadSafe
#[allow(non_upper_case_globals)]
pub const NotThreadSafe: NotThreadSafe = PhantomData;
