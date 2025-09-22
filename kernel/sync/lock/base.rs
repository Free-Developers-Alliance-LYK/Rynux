//! Common lock base types.

use crate::types::NotThreadSafe;
use core::cell::UnsafeCell;

/// The "backend" of a lock.
///
/// It is the actual implementation of the lock, without the need to repeat patterns used in all
/// locks.
///
/// # Safety
///
/// - Implementers must ensure that only one thread/CPU may access the protected data once the lock
///   is owned, that is, between calls to [`lock`] and [`unlock`].
///
/// [`lock`]: Backend::lock
/// [`unlock`]: Backend::unlock
pub trait Backend {
    /// The backend private data;
    type Inner;

    /// The state required to be kept between [`lock`] and [`unlock`].
    ///
    /// [`lock`]: Backend::lock
    /// [`unlock`]: Backend::unlock
    type GuardState;

    /// Acquires the lock, making the caller its owner.
    ///
    /// # Safety
    ///
    fn lock(state: &mut Self::Inner) -> Self::GuardState;

    /// Tries to acquire the lock.
    fn try_lock(inner: &mut Self::Inner) -> Option<Self::GuardState>;

    /// Releases the lock, giving up its ownership.
    ///
    fn unlock(inner: &mut Self::Inner, guard_state: &Self::GuardState);

    /// Asserts that the lock is held using lockdep.
    ///
    fn assert_is_held(inner: &Self::Inner);
}

/// A generial lock type.
pub struct Lock<T: ?Sized, B: Backend> {
    // The lock backend private data.
    pub(crate) inner: UnsafeCell<B::Inner>,
    // Name
    pub(crate) _name: Option<&'static str>,
    // The data protected by the lock.
    pub(crate) data: UnsafeCell<T>,
}

// SAFETY: `Lock` can be transferred across thread boundaries iff the data it protects can.
unsafe impl<T: ?Sized + Send, B: Backend> Send for Lock<T, B> {}

// SAFETY: `Lock` serialises the interior mutability it provides, so it is `Sync` as long as the
// data it protects is `Send`.
unsafe impl<T: ?Sized + Send, B: Backend> Sync for Lock<T, B> {}

impl<T: ?Sized, B: Backend> Lock<T, B> {
    /// Acquires the lock, blocking the current thread until it is able to do so.
    pub fn lock(&self) -> BaseLockGuard<'_, T, B> {
        // SAFETY: inner already be initialised.
        unsafe { BaseLockGuard::new(self, B::lock(&mut *self.inner.get())) }
    }

    /// Tries to acquire the lock.
    pub fn try_lock(&self) -> Option<BaseLockGuard<'_, T, B>> {
        // SAFETY: inner already be initialised.
        unsafe { B::try_lock(&mut *self.inner.get()).map(|state| BaseLockGuard::new(self, state)) }
    }
}

/// A lock guard.
///
/// Allows mutual exclusion primitives that implement the [`Backend`] trait to automatically unlock
/// when a guard goes out of scope. It also provides a safe and convenient way to access the data
/// protected by the lock.
pub struct BaseLockGuard<'a, T: ?Sized, B: Backend> {
    pub(crate) lock: &'a Lock<T, B>,
    pub(crate) state: B::GuardState,
    _not_send: NotThreadSafe,
}

// SAFETY: `Guard` is sync when the data protected by the lock is also sync.
unsafe impl<T: Sync + ?Sized, B: Backend> Sync for BaseLockGuard<'_, T, B> {}

impl<'a, T: ?Sized, B: Backend> BaseLockGuard<'a, T, B> {
    /// Constructs a new immutable lock guard.
    ///
    /// # Safety
    ///
    /// The caller must ensure that it owns the lock.
    pub unsafe fn new(lock: &'a Lock<T, B>, state: B::GuardState) -> Self {
        // SAFETY: The caller can only hold the lock if `Backend::init` has already been called.
        unsafe { B::assert_is_held(&*lock.inner.get()) };
        Self {
            lock,
            state,
            _not_send: NotThreadSafe,
        }
    }
    /// Returns the lock that this guard originates from.
    ///
    /// The following example shows how to assert the corresponding
    /// lock is held.
    ///
    /// ```
    /// # use kernel::sync::lock::{Backend, BaseLockGuard, Lock};
    ///
    /// fn assert_held<T, B: Backend>(guard: &BaseLockGuard<'_, T, B>, lock: &Lock<T, B>) {
    ///     // Address-equal means the same lock.
    ///     assert!(core::ptr::eq(guard.lock_ref(), lock));
    /// }
    ///
    /// ```
    pub fn lock_ref(&self) -> &'a Lock<T, B> {
        self.lock
    }

    #[allow(dead_code)]
    pub(crate) fn force_unlock(&mut self) {
        // SAFETY: The caller owns the lock, so it is safe to unlock it.
        unsafe {
            B::unlock(&mut *self.lock.inner.get(), &self.state);
        }
    }
}

impl<T: ?Sized, B: Backend> core::ops::Deref for BaseLockGuard<'_, T, B> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: The caller owns the lock, so it is safe to deref the protected data.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized, B: Backend> core::ops::DerefMut for BaseLockGuard<'_, T, B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The caller owns the lock, so it is safe to deref the protected data.
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized, B: Backend> Drop for BaseLockGuard<'_, T, B> {
    fn drop(&mut self) {
        // SAFETY: The caller owns the lock, so it is safe to unlock it.
        unsafe {
            B::unlock(&mut *self.lock.inner.get(), &self.state);
        }
    }
}
