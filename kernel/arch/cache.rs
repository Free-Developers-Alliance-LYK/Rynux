//! Cache

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use crate::arch::arm64::mm::cache::L1_CACHE_BYTES;
        /// Cache line size
        pub const SMP_CACHE_BYTES: usize = L1_CACHE_BYTES;
    }
}

