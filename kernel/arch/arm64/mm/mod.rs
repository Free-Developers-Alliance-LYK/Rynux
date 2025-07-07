//! ARM64-specific mem module code.


pub mod cache;
pub mod thread_layout;

use crate::size::*;

/// Alignment of kernel segments (e.g. .text, .data).
/// 4 KB granule:  16 level 3 entries, with contiguous bit
/// 16 KB granule:   4 level 3 entries, without contiguous bit
/// 64 KB granule:   1 level 3 entry
#[no_mangle]
pub static SEGMENT_ALIGN: usize = SZ_64K;
