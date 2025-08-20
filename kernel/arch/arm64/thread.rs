//! Arm64 thread info
//! TODO: 
//!   - not support CONFIG_ARM64_SW_TTBR0_PAN
//!   - not support CONFIG_SHADOW_CALL_STACK
//!   - not support CONFIG_CPU_BIG_ENDIAN
//!

use crate::schedule::preempt::INIT_TASK_PREEMPT_COUNT;
use crate::bitflags::bitflags;
use crate::schedule::task::Task;
use crate::arch::arm64::sysregs::sp_el0::SpEl0;

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
#[derive(Copy, Clone)]
pub struct PreemptCount {
    /// Need resched
    pub need_resched: u32,
    /// Preempt count
    pub count: u32,
}

/// Preempt info
#[repr(C)]
#[derive(Copy, Clone)]
pub union PreemptInfo {
    /// Preempt count
    pub preempt_count: u64,
    /// Preempt info
    pub preempt: PreemptCount,
}

/// Thread info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Arm64ThreadInfo {
    /// Flags
    pub flags: ThreadInfoFlags,
    /// Preempt info
    pub preempt: PreemptInfo,
    /// Cpu id
    pub cpu: u32,
}

impl Arm64ThreadInfo {
    /// Default thread info
    pub const fn default() -> Self {
        Self {
            flags: ThreadInfoFlags::FOREIGN_FPSTATE,
            preempt: PreemptInfo {
                preempt_count: INIT_TASK_PREEMPT_COUNT,
            },
            cpu: 0,
        }
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
