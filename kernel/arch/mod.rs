//! Kernel architecture-specific code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub mod arm64;

        /// ARM64-specific mem module code.
        pub mod mm {
            pub use super::arm64::mm::thread_layout::ArchThreadMemLayout;
        }
    }
}

pub mod cache;
