//! dummy thread info

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
