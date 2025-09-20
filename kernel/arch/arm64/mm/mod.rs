//! ARM64-specific mem module code.

pub mod cache;
pub mod fixmap;
pub mod init;
pub mod mmu;
pub mod sparse_mem;
pub mod thread_layout;

mod phys_config;
pub mod va_layout;

pub use phys_config::Arm64PhysConfig;
pub use va_layout::Arm64VaLayout;

use crate::macros::need_export;
use crate::size::*;

/// Alignment of kernel segments (e.g. .text, .data).
/// 4 KB granule:  16 level 3 entries, with contiguous bit
/// 16 KB granule:   4 level 3 entries, without contiguous bit
/// 64 KB granule:   1 level 3 entry
#[need_export]
pub static SEGMENT_ALIGN: usize = SZ_64K;
