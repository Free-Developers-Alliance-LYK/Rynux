//! Rynux type

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicU8, Ordering};

const UNINIT: u8 = 0;
const WRITING: u8 = 1;
const INIT: u8 = 2;

/// A cell which can be written to only once.
pub struct OnceCell<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T: Sync> Sync for OnceCell<T> {}
unsafe impl<T: Send> Send for OnceCell<T> {}

impl<T> OnceCell<T> {
    /// Create a new OnceCell.
    pub const fn new() -> Self {
        Self {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(UNINIT),
        }
    }

    /// Set the value of the cell.
    pub fn set(&self, value: T) {
        match self
            .state
            .compare_exchange(UNINIT, WRITING, Ordering::Acquire, Ordering::Acquire)
        {
            Ok(_) => {
                unsafe {
                    (*self.value.get()).write(value);
                }
                self.state.store(INIT, Ordering::Release);
            }
            Err(INIT) => panic!("Already set"),
            Err(WRITING) => panic!("Concurrent set"),
            _ => unreachable!(),
        }
    }

    /// Get the value of the cell.
    pub fn get(&self) -> Option<&T> {
        if self.state.load(Ordering::Acquire) == INIT {
            Some(unsafe { (*self.value.get()).assume_init_ref() })
        } else {
            None
        }
    }

    /// Get the mutable reference of the cell.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.state.load(Ordering::Acquire) == INIT {
            Some(unsafe { (*self.value.get()).assume_init_mut() })
        } else {
            None
        }
    }
}

impl<T> Deref for OnceCell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get().expect("OnceCell is not initialized")
    }
}

impl<T> DerefMut for OnceCell<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut().expect("OnceCell is not initialized")
    }
}

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

/// for loop closure return value
pub enum ForStepResult {
    /// continue
    Next,
    /// jump to
    JumpTo(usize),
    /// break
    Break,
}
