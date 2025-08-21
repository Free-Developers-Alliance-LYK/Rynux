//! Arm64 thread info
//! TODO: 
//!   - not support CONFIG_ARM64_SW_TTBR0_PAN
//!   - not support CONFIG_SHADOW_CALL_STACK
//!   - not support CONFIG_CPU_BIG_ENDIAN
//!

use core::sync::atomic::{AtomicU64, Ordering};

use crate::schedule::preempt::INIT_TASK_PREEMPT_COUNT;
use crate::bitflags::bitflags;
use crate::schedule::task::Task;
use crate::arch::arm64::sysregs::sp_el0::SpEl0;
use crate::arch::thread::ArchThreadInfoTrait;

bitflags! {
    /// Flags
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct ThreadInfoFlags: u64 {
        /// CPU's FP state is not current's
        const FOREIGN_FPSTATE = 0b1;
    }
}

/// Preempt info
#[repr(C)]
struct PreemptInfo {
    raw: AtomicU64,  // [need_resched: u32 | count: u32] 
}

#[allow(dead_code)]
impl PreemptInfo {
    #[cfg(target_endian = "little")]
    const NEED_SHIFT: u32 = 0;
    #[cfg(target_endian = "little")]
    const COUNT_SHIFT: u32 = 32;

    #[cfg(target_endian = "big")]
    const NEED_SHIFT: u32 = 32;
    #[cfg(target_endian = "big")]
    const COUNT_SHIFT: u32 = 0;

    const FIELD_MASK: u64 = 0xFFFF_FFFF;

    #[inline(always)]
    /// Default preempt info
    const fn default() -> Self {
        Self {
            raw: AtomicU64::new(INIT_TASK_PREEMPT_COUNT),
        }
    }

    #[inline(always)]
    /// Get raw value
    fn raw(&self) -> u64 {
        self.raw.load(Ordering::Relaxed)
    }

    #[inline(always)]
    /// Get count
    fn count(&self) -> u32 {
        ((self.raw() >> Self::COUNT_SHIFT) & Self::FIELD_MASK) as u32
    }

    #[inline(always)]
    /// Get need resched
    fn need_resched(&self) -> u32 {
        ((self.raw() >> Self::NEED_SHIFT) & Self::FIELD_MASK) as u32
    }

    #[inline(always)]
    /// Add count
    fn add_count(&self, val: u32) {
        self.update_count(|c| c.wrapping_add(val));
    }

    #[inline(always)]
    /// Sub count
    fn sub_count(&self, val: u32) {
        self.update_count(|c| c.wrapping_sub(val));
    }

    #[inline(always)]
    /// Update count
    fn update_count<F: Fn(u32) -> u32>(&self, f: F) {
        let mut cur = self.raw.load(Ordering::Relaxed);
        loop {
            let c = ((cur >> Self::COUNT_SHIFT) & Self::FIELD_MASK) as u32;
            let new_c = f(c);
            let new = (cur & !(Self::FIELD_MASK << Self::COUNT_SHIFT)) | ((new_c as u64) << Self::COUNT_SHIFT);
            match self.raw.compare_exchange_weak(
                cur,
                new,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(next) => cur = next,
            }
        }
    }

}

/// Thread info
#[repr(C)]
pub struct Arm64ThreadInfo {
    /// Flags
    pub flags: ThreadInfoFlags,
    /// Preempt info
    preempt: PreemptInfo,
    /// Cpu id
    pub cpu: u32,
}

impl Arm64ThreadInfo {
    /// Default thread info
    pub const fn default() -> Self {
        Self {
            flags: ThreadInfoFlags::FOREIGN_FPSTATE,
            preempt: PreemptInfo::default(),
            cpu: 0,
        }
    }
}

impl ArchThreadInfoTrait for Arm64ThreadInfo {
    #[inline(always)]
    fn preempt_count(&self) -> u32 {
        self.preempt.count()
    }

    #[inline(always)]
    fn preempt_count_add(&self, val: u32) {
        self.preempt.add_count(val);
    }

    #[inline(always)]
    fn preempt_count_sub(&self, val: u32) {
        self.preempt.sub_count(val);
    }
}

/// Arm64 Current
pub struct Arm64Current;

use crate::arch::thread::ArchCurrentTrait;
impl ArchCurrentTrait for Arm64Current {
    #[inline(always)]
    fn read() -> *const Task {
        SpEl0::read_raw() as *const Task
    }

    #[inline(always)]
    fn write(task: *const Task) {
        SpEl0::write_raw(task as u64);
    }
}
