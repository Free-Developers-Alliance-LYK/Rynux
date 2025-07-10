//! Arm64 Page table PTE

use crate::{
    mm::page::PAGE_SHIFT,
    arch::arm64::pgtable::hard::PTDESC_TABLE_SHIFT,
    cfg_if,
};


/// Pte
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Pte(u64);

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
    /// Number of entries per PTE
    pub const PTRS: usize = 1 <<  PTDESC_TABLE_SHIFT;
    // determines the continue PTE size map
    const CONT_SHIFT: usize =  Self::pte_cont_shift(PAGE_SHIFT) + PAGE_SHIFT;
    /// Size of a contiguous PTE entry in bytes.
    pub const CONT_SIZE: usize = 1 << Self::CONT_SHIFT;
    // Number of entries per contiguous PTE
    const CONT_PTRS: usize = 1 << Self::CONT_SHIFT - PAGE_SHIFT;
    // Mask for aligning to a contiguous PTE entry
    const CONT_MASK: usize = !(Self::CONT_SIZE - 1);

    // Address mask
    const PTE_ADDR_LOW_MASK: u64 = ((1 << (50 - PAGE_SHIFT)) - 1) << PAGE_SHIFT;

    cfg_if! {
        if #[cfg(CONFIG_ARM64_PA_BITS_52)] {
            if #[cfg(CONFIG_ARM64_64K_PAGES)] {
                use klib::bits::genmask_ull;
                const PTE_ADDR_HIGH_MASK: u64 = 0xf << 12;
                const PTE_ADDR_HIGH_SHIFT: u64 = 36;
                const PHYS_TO_PTE_ADDR_MASK: u64 = Self::PTE_ADDR_LOW_MASK | Self::PTE_ADDR_HIGH_MASK;
            } else {
                /*
                 * PA is 52bit, lower 12 bit is not used(Page Aligned),
                 * still need to store 40bit(12-52)
                 * PTE only (12-50 bit) can store PA, still need to store 51-52bit
                 * in PTE(9,10)
                 */
                const PTE_ADDR_HIGH_MASK: u64 = 0x3 << 8;
                const PTE_ADDR_HIGH_SHIFT: u64 = 42;
                const PHYS_TO_PTE_ADDR_MASK: u64 = genmask_ull(49, 8);
            }
        }
    }

    /// Check if the pte is none
    pub fn is_none(&self) -> bool {
        self.0 == 0
    }

    /// Create a new Pte
    pub fn new(val: u64) -> Self {
        Self(val)
    }

    /// Get the value of the Pte
    pub fn bits(&self) -> u64 {
        self.0
    }

    cfg_if! {
        if #[cfg(CONFIG_ARM64_PA_BITS_52)] {
            /// Consume the pte and convert it to a physical address
            #[inline(always)]
            pub fn to_phys(self) -> u64 {
                // remove the maybe shared bit
                let val = self.0 & !PgProt::pte_maybe_shared().bits();
                let low_addr = val & Self::PTE_ADDR_LOW_MASK;
                let high_addr = (val & Self::PTE_ADDR_HIGH_MASK) << Self::PTE_ADDR_HIGH_SHIFT;
                low_addr | high_addr
            }

            /// Convert a physical address to a pte
            #[inline(always)]
            pub fn from_phys(pa: usize) -> Self {
                let pa = pa as u64;
                Self((pa | (pa >> Self::PTE_ADDR_HIGH_SHIFT)) & Self::PHYS_TO_PTE_ADDR_MASK)
            }

        } else {
            /// Consume the pte and convert it to a physical address
            #[inline(always)]
            pub fn to_phys(self) -> usize {
                (self.0 & Self::PTE_ADDR_LOW_MASK) as usize
            }

            /// Convert a physical address to a pte
            #[inline(always)]
            pub fn from_phys(pa: usize) -> Self {
                let pa = pa as u64;
                Self(pa)
            }
        }
    }
}

