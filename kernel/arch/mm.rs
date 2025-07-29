//! Memory management code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        /// ARM64-specific thread memory layout.
        pub use super::arm64::mm::thread_layout::ArchThreadMemLayout;
    }
}
