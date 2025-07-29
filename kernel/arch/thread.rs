//! Rynux thread


use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::thread_info::Arm64ThreadInfo as ArchThreadInfo;
    }
}
