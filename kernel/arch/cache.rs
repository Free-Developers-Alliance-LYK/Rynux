//! Cache
use crate::cfg_if;

cfg_if! {
    if #[cfg(test)] {
        /// fake cache align
        pub const SMP_CACHE_BYTES: usize = 0;

    } else if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::mm::cache::L1_CACHE_BYTES;
        /// Cache line size
        pub const SMP_CACHE_BYTES: usize = L1_CACHE_BYTES;
    }
}
