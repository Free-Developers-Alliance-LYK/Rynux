//! Arm64 Page table PMD
//!
use super::hard::{arm64_hw_pgtable_levels_shift, PTDESC_TABLE_SHIFT};
use crate::mm::page::PAGE_SHIFT;

/// Pmd value
pub type PmdVal = u64;

/// Pmd
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Pmd(u64);

#[allow(dead_code)]
impl Pmd {
    // Returns the ARM64 continue pmd shift for a given page shift (log2 of page size).
    //
    // # Arguments
    // * `page_shift` - The log2 of the page size in bytes. (e.g., 12 for 4KB, 14 for 16KB, 16 for 64KB)
    //
    // # Returns
    // * The CONT_PTE_SHIFT value, i.e., log2 of the number of pages that can be merged into a contiguous PTE.
    //
    // # ARM64 logic:
    // - For 4KB pages (page_shift = 12):  CONT_PMD_SHIFT = 4  (16 pages, 16*2MB = 32MB block)
    // - For 16KB pages (page_shift = 14): CONT_PTE_SHIFT = 5  (32 pages, 32*32MB = 1GB block)
    // - For 64KB pages (page_shift = 16): CONT_PTE_SHIFT = 5  (32 pages, 32*512MB= 16GB block)
    // - Otherwise, defaults to 4 (for safety; can be adjusted for other configurations).
    const fn pmd_cont_shift(page_shift: usize) -> usize {
        match page_shift {
            12 => 4,  // 4KB pages
            14 => 5,  // 16KB pages
            16 => 5,  // 64KB pages
            _  => 4,  // Default/fallback value
        }
    }

    // determines the size a pmd page table entry can map
    pub(crate) const SHIFT: usize = arm64_hw_pgtable_levels_shift(2);
    // Size of a PMD entry in bytes.
    pub(crate) const SIZE: usize = 1 << Self::SHIFT;
    // Number of entries per PMD
    const PTRS: usize = 1 <<  PTDESC_TABLE_SHIFT;
    // Mask for PMD entry
    const MASK: usize = !(Self::SIZE - 1);
    // determines the continue Pmd size map
    const CONT_SHIFT: usize = Self::pmd_cont_shift(PAGE_SHIFT) + Self::SHIFT;
    // Size of a continue PMD entry in bytes.
    const CONT_SIZE: usize = 1 << Self::CONT_SHIFT;
    // Number of entries per continue PMD
    const CONT_PTRS: usize = 1 << (Self::CONT_SHIFT - Self::SHIFT);
    // Mask for continue PMD entry
    const CONT_MASK: usize = !(Self::CONT_SIZE - 1);

    /// Create a new Pmd
    pub const fn new(val: PmdVal) -> Self {
        Self(val)
    }

    /// Get the value of the Pmd
    pub const fn pmd_value(&self) -> PmdVal {
        self.0
    }
}
