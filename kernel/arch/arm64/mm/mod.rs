//! ARM64-specific mem module code.

use crate::size::*;
use crate::mm::page::PAGE_SHIFT;
use klib::cfg_if;

/// Alignment of kernel segments (e.g. .text, .data).
/// 4 KB granule:  16 level 3 entries, with contiguous bit
/// 16 KB granule:   4 level 3 entries, without contiguous bit
/// 64 KB granule:   1 level 3 entry
#[no_mangle]
pub static SEGMENT_ALIGN: usize = SZ_64K;


/// ARM64-specific thread memory layout
pub struct ArchThreadMemLayout {
}

impl ArchThreadMemLayout {
    /// Minimum thread shift
    pub const MIN_THREAD_SHIFT: usize = 14;
    /// Thread size
    pub const THREAD_SIZE: usize = 1 << Self::THREAD_SHIFT;
    cfg_if! {
        if #[cfg(CONFIG_VMAP_STACK)] {
            const fn thread_shift() -> usize {
                if Self::MIN_THREAD_SHIFT < PAGE_SHIFT {
                    PAGE_SHIFT
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

