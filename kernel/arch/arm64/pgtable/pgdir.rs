//! Arm64 Page table PGDIR

use super::{PGTABLE_LEVELS, hard::arm64_hw_pgtable_levels_shift};
use crate::arch::arm64::va_layout::VA_BITS;

/// Pgdir
pub struct Pgdir {
}

impl Pgdir {
    /// determines the size a top-level page table entry can map
    pub const SHIFT: usize = arm64_hw_pgtable_levels_shift(4 - PGTABLE_LEVELS);
    /// Size of a PGDIR entry map in bytes.
    pub const SIZE: usize = 1 << Self::SHIFT;
    /// Mask for PGDIR entry
    pub const MASK: usize = !(Self::SIZE - 1);
    /// Number of entries per Pgd
    pub const PTRS: usize = 1 << (VA_BITS - Self::SHIFT);
}
