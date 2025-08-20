//! Rynux thread

use crate::cfg_if;

use crate::schedule::task::Task;

/// Arch current task
pub trait ArchCurrentTrait {
    /// Read current task
    fn read() -> *const Task;
    /// write current task
    fn write(task: *const Task);
}

cfg_if! {
    if #[cfg(test)] {
        pub use super::dummy::thread::DummyThreadInfo as ArchThreadInfo; 
        pub use super::dummy::thread::DummyCurrent as ArchCurrent;
    } else if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::thread::Arm64ThreadInfo as ArchThreadInfo;
        pub use super::arm64::thread::Arm64Current as ArchCurrent;
    }
}
