//! Rynux thread

use crate::schedule::task::Task;

/// Arch thread info
pub trait ArchThreadInfoTrait {
    /// Preempt count
    fn preempt_count(&self) -> u32;
    /// Add preempt count
    fn preempt_count_add(&self, val: u32);
    /// Sub preempt count
    fn preempt_count_sub(&self, val: u32);
}

/// Arch current task
pub trait ArchCurrentTrait {
    /// Read current task
    fn read() -> *const Task;
    /// write current task
    fn write(task: *const Task);
}

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::thread::Arm64ThreadInfo as ArchThreadInfo;
        pub use super::arm64::thread::Arm64Current as ArchCurrent;
    } else {
        pub use super::dummy::thread::DummyThreadInfo as ArchThreadInfo;
        pub use super::dummy::thread::DummyCurrent as ArchCurrent;
    }
}
