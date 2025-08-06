//! Rynux thread

use crate::cfg_if;
/*
use crate::schedule::task::Task;
/// Arch current task
pub trait ArchCurrent {
    /// Read current task
    fn read() -> *const Task;
    fn write(task: *const Task);
}
*/
cfg_if! {
    if #[cfg(test)] {
        pub use super::dummy::thread::DummyThreadInfo as ArchThreadInfo; 
    } else if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::thread::Arm64ThreadInfo as ArchThreadInfo;
        //pub use super::arm64::thread::Arm64Current as Current;
    }
}
