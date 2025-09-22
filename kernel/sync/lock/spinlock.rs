// SPDX-License-Identifier: GPL-2.0

//! A kernel spinlock.
//! TODO:
//!    - support lock_class_key

use crate::{
    arch::irq::{ArchIrq, IRQ},
    macros::section_spinlock_text,
    schedule::preempt::{preempt_disable, preempt_enable},
};
use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering};

/// A IRQ-Safe and will disable preempt spinlock.
///
/// When multiple CPUs attempt to lock the same spinlock, only one at a time
/// is allowed to progress, the others will block (spinning) until the spinlock is
/// unlocked, at which point another CPU will be allowed to make progress.
/// it must disable preemption to ensure deadlock freedom.
/// since it will disable IRQ, so it is safe to use in IRQ context.
///
/// The following example shows how to use interior mutability to modify the contents of a struct
/// protected by a spinlock despite only having a shared reference:
/// ```
/// use kernel::sync::RawSpinLockNoIrq;
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
pub type RawSpinLockNoIrq<T> = super::Lock<T, RawSpinLockNoIrqBackend>;

/// Raw spinlock backend.
pub struct RawSpinLockNoIrqBackend;

/// A Guard acquired from locking a [`RawSpinLockNoIrq`].
///
/// This is simply a type alias for a Guard returned from locking a [`RawSpinLockNoIrq`].
/// It will unlock the [`RawSpinLockNoIrq`] upon being dropped.
pub type RawSpinLockNoIrqGuard<'a, T> = super::BaseLockGuard<'a, T, RawSpinLockNoIrqBackend>;

/// Raw spinlock inner.
pub struct SpinLockInner {
    /// The atomic
    pub lock: AtomicBool,
}

impl<T> RawSpinLockNoIrq<T> {
    /// Constructs a new raw spinlock.
    pub const fn new(t: T, name: Option<&'static str>) -> Self {
        Self {
            inner: UnsafeCell::new(SpinLockInner {
                lock: AtomicBool::new(false),
            }),
            data: UnsafeCell::new(t),
            _name: name,
        }
    }
}

impl super::Backend for RawSpinLockNoIrqBackend {
    type Inner = SpinLockInner;
    type GuardState = <IRQ as ArchIrq>::IrqState;

    #[section_spinlock_text]
    fn lock(inner: &mut Self::Inner) -> Self::GuardState {
        let irq = IRQ::local_save_and_disable();
        preempt_disable();
        // can fail to lock even if the spinlock is not locked. May be more efficient than `try_lock`
        //  when called in a loop.
        while inner
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Wait until the lock looks unlocked before retrying
            core::hint::spin_loop();
        }
        irq
    }

    #[section_spinlock_text]
    fn unlock(inner: &mut Self::Inner, guard_state: &Self::GuardState) {
        inner.lock.store(false, Ordering::Release);
        IRQ::local_restore(*guard_state);
        preempt_enable();
    }

    #[section_spinlock_text]
    fn try_lock(inner: &mut Self::Inner) -> Option<Self::GuardState> {
        let irq = IRQ::local_save_and_disable();
        preempt_disable();
        let locked = inner
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok();
        if locked {
            Some(irq)
        } else {
            IRQ::local_restore(irq);
            preempt_enable();
            None
        }
    }

    fn assert_is_held(inner: &Self::Inner) {
        assert!(inner.lock.load(Ordering::Relaxed));
    }
}
