//! Queue Spinlock implement
//!

use crate::bitflags;
use core::sync::atomic::{AtomicU32, Ordering};

bitflags::bitflags! {
    /// QSpinLock
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    #[repr(C, align(4))]
    pub struct QSpinLock: u32 {
        /// Locked
        const LOCKED = 0xff << 0;
        /// Pending
        const PENDING = 0b1 << 8;
        /// Tail idx
        const TAIL_IDX = 0b11 << 16;
        /// Tail CPU
        const TAIL_CPU   = 0x3FFF << 18;
    }
}

impl QSpinLock {
    const LOCKED_VAL: u32 = 1;
    #[inline(always)]
    /// Translate to `AtomicU32`
    pub fn atomic(&mut self) -> &AtomicU32 {
        unsafe { AtomicU32::from_ptr(self as *mut QSpinLock as *mut u32) }
    }

    /// Try to lock the qspinlock.
    pub fn try_lock(&mut self) -> bool {
        let val = self.atomic().load(Ordering::Relaxed);
        if val != 0 {
            return false;
        }

        self.atomic()
            .compare_exchange(0, Self::LOCKED_VAL, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
    }

    /// Lock
    pub fn lock(&mut self) {
        if self
            .atomic()
            .compare_exchange(0, Self::LOCKED_VAL, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            return;
        }

        self.queued_spin_lock_slowpath();
    }

    fn queued_spin_lock_slowpath(&mut self) {
        todo!()
    }
}
