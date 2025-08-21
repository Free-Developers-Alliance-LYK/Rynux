//! dummy thread info


use crate::arch::thread::ArchThreadInfoTrait;

/// Thread info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DummyThreadInfo {
    /// Cpu id
    pub cpu: u32,
}

impl DummyThreadInfo {
    /// Default thread info
    pub const fn default() -> Self {
        Self {
            cpu: 0,
        }
    }
}

impl ArchThreadInfoTrait for DummyThreadInfo {
    fn preempt_count(&self) -> u32 {
        0
    }

    fn preempt_count_add(&self, _val: u32) {
    }

    fn preempt_count_sub(&self, _val: u32) {
    }
}


use crate::arch::thread::ArchCurrentTrait;
use crate::schedule::task::Task;

/// Dummy Current
pub struct DummyCurrent;
impl ArchCurrentTrait for DummyCurrent {
    #[inline(always)]
    fn read() -> *const Task {
        0 as *const Task
    }

    #[inline(always)]
    fn write(_task: *const Task) {
    }
}
