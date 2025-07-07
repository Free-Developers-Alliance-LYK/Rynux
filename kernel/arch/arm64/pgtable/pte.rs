//! Arm64 Page table PTE

use crate::mm::page::PAGE_SHIFT;
use crate::arch::arm64::pgtable::hard::PTDESC_TABLE_SHIFT;

/// Pte value
pub type PteVal = u64;

/// Pte
pub struct Pte(pub PteVal);


#[allow(dead_code)]
impl Pte {
    // Returns the ARM64 CONT_PTE_SHIFT for a given page shift (log2 of page size).
    //
    // # Arguments
    // * `page_shift` - The log2 of the page size in bytes. (e.g., 12 for 4KB, 14 for 16KB, 16 for 64KB)
    //
    // # Returns
    // * The CONT_PTE_SHIFT value, i.e., log2 of the number of pages that can be merged into a contiguous PTE.
    //
    // # ARM64 logic:
    // - For 4KB pages (page_shift = 12):  CONT_PTE_SHIFT = 4  (16 pages, 64KB block)
    // - For 16KB pages (page_shift = 14): CONT_PTE_SHIFT = 7  (128 pages, 2MB block)
    // - For 64KB pages (page_shift = 16): CONT_PTE_SHIFT = 5  (32 pages, 2MB block)
    // - Otherwise, defaults to 4 (for safety; can be adjusted for other configurations).
    const fn pte_cont_shift(page_shift: usize) -> usize {
        match page_shift {
            12 => 4,  // 4KB pages
            14 => 7,  // 16KB pages
            16 => 5,  // 64KB pages
            _  => 4,  // Default/fallback value
        }
    }

    // determines the size a pte page table entry can map
    const SHIFT: usize = PAGE_SHIFT;
    // Size of a PTE entry in bytes.
    const SIZE: usize = 1 << Self::SHIFT;
    // Mask for aligning to a PTE entry
    const MASK: usize = !(Self::SIZE - 1);
    // Number of entries per PTE
    const PTRS: usize = 1 <<  PTDESC_TABLE_SHIFT;
    // determines the continue PTE size map
    const CONT_SHIFT: usize =  Self::pte_cont_shift(PAGE_SHIFT) + PAGE_SHIFT;
    // Size of a contiguous PTE entry in bytes.
    const CONT_SIZE: usize = 1 << Self::CONT_SHIFT;
    // Number of entries per contiguous PTE
    const CONT_PTRS: usize = 1 << Self::CONT_SHIFT - PAGE_SHIFT;
    // Mask for aligning to a contiguous PTE entry
    const CONT_MASK: usize = !(Self::CONT_SIZE - 1);

    /// Create a new Pte
    pub const fn new(val: PteVal) -> Self {
        Self(val)
    }

    /// Get the value of the Pte
    pub const fn pte_value(&self) -> PteVal {
        self.0
    }
}
