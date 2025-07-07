//! Arm64 Page table PGDIR

use super::{PGTABLE_LEVELS, hard::arm64_hw_pgtable_levels_shift};
use crate::arch::arm64::va_layout::VA_BITS;

/// Pgd value
pub type PgdVal = u64;

/// Pgdir
pub struct Pgdir(pub PgdVal);

impl Pgdir {
    /// determines the size a top-level page table entry can map
    pub const SHIFT: usize = arm64_hw_pgtable_levels_shift(4 - PGTABLE_LEVELS);
    /// Size of a PGDIR entry map in bytes.
    pub const SIZE: usize = 1 << Self::SHIFT;
    /// Mask for PGDIR entry
    pub const MASK: usize = !(Self::SIZE - 1);
    /// Number of entries per Pgd
    pub const PTRS: usize = 1 << (VA_BITS - Self::SHIFT);


    /// Create a new Pgd
    pub const fn new(val: PgdVal) -> Self {
        Self(val)
    }

    /// Get the value of the Pgd
    pub const fn pgd_value(&self) -> PgdVal {
        self.0
    }
}
