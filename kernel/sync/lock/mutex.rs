// SPDX-License-Identifier: GPL-2.0

//! Rynux mutex implement
//!
//!

use crate::schedule::{current, task::TaskState, WaitQueue};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicUsize, Ordering};

/// A mutual exclusion primitive.
///
/// ```
/// use kernel::sync::Mutex;
///
/// struct Example {
///     a: u32,
///     b: u32,
/// }
///
/// fn example(m: &RawSpinLock<Example>) {
///     let mut guard = m.lock();
///     guard.a += 10;
///     guard.b += 20;
/// }
/// ```
///
pub type Mutex<T> = super::Lock<T, MutexBackend>;

/// A Guard acquired from locking a [`Mutex`].
///
/// [`BaseLockGuard`]: super::BaseLockGuard
pub type MutexGuard<'a, T> = super::BaseLockGuard<'a, T, MutexBackend>;

/// Mutex backend.
pub struct MutexBackend;

/// Mutex inner.
pub struct MutexLockInner {
    /// The atomic
    owner: AtomicUsize,
    wait_queue: WaitQueue,
}

impl MutexLockInner {
    const fn new() -> Self {
        Self {
            owner: AtomicUsize::new(0),
            wait_queue: WaitQueue::new(),
        }
    }
}

impl<T> Mutex<T> {
    /// Constructs a new mutex.
    pub const fn new(t: T, name: Option<&'static str>) -> Self {
        Self {
            inner: UnsafeCell::new(MutexLockInner::new()),
            data: UnsafeCell::new(t),
            _name: name,
        }
    }
}

impl super::Backend for MutexBackend {
    type Inner = MutexLockInner;
    type GuardState = ();

    fn lock(inner: &mut Self::Inner) -> Self::GuardState {
        // fast path TODO: use optimistic spinning
        for _ in 0..100 {
            if let Some(guard) = Self::try_lock(inner) {
                return guard;
            }
        }
        loop {
            if let Some(guard) = Self::try_lock(inner) {
                return guard;
            }
            inner.wait_queue.wait_until(TaskState::UNINTERRUPTIBLE, || {
                inner.owner.load(Ordering::Acquire) == 0
            });
        }
    }

    fn unlock(inner: &mut Self::Inner, _guard_state: &Self::GuardState) {
        let owner = inner.owner.load(Ordering::Relaxed);
        assert_eq!(owner, current().as_ptr() as usize);
        inner.owner.store(0, Ordering::Release);
        inner.wait_queue.notify_one();
    }

    fn try_lock(inner: &mut Self::Inner) -> Option<Self::GuardState> {
        let curr_ptr = current().as_ptr() as usize;

        if inner
            .owner
            .compare_exchange(0, curr_ptr, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            return Some(());
        }
        None
    }

    fn assert_is_held(inner: &Self::Inner) {
        assert_eq!(
            inner.owner.load(Ordering::Relaxed),
            current().as_ptr() as usize
        );
    }
}
