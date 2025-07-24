//! Thread memory layout for ARM64 architecture

use crate::mm::page::PageConfig;
use crate::cfg_if;

/// ARM64-specific thread memory layout
pub struct ArchThreadMemLayout();

impl ArchThreadMemLayout {
    /// Minimum thread shift
    pub const MIN_THREAD_SHIFT: usize = 14;
    /// Thread size
    pub const THREAD_SIZE: usize = 1 << Self::THREAD_SHIFT;
    cfg_if! {
        if #[cfg(CONFIG_VMAP_STACK)] {
            const fn thread_shift() -> usize {
                if Self::MIN_THREAD_SHIFT < PageConfig::PAGE_SHIFT {
                    PageConfig::PAGE_SHIFT
                } else {
                    Self::MIN_THREAD_SHIFT
                }
            }
            /// Thread shift
            pub const THREAD_SHIFT: usize = Self::thread_shift();
            /// Thread align
            pub const THREAD_ALIGN: usize = 2 * Self::THREAD_SIZE;
        } else {
            /// Thread shift
            pub const THREAD_SHIFT: usize = Self::MIN_THREAD_SHIFT;
            /// Thread align
            pub const THREAD_ALIGN: usize = Self::THREAD_SIZE;
        }
    }
}
