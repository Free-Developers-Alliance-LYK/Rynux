//! Arm64 Phys Config

use crate::arch::arm64::pgtable;
use crate::arch::arm64::mm::sparse_mem::SECTION_SIZE_BITS;
use crate::cfg_if;

/// Arm64 Phys Config
pub struct Arm64PhysConfig();

impl Arm64PhysConfig {
    // To make optimal use of block mappings when laying out the linear
    // mapping, round down the base of physical memory to a size that can
    // be mapped efficiently, i.e., either PUD_SIZE (4k granule) or PMD_SIZE
    // (64k granule), or a multiple that can be mapped using contiguous bits
    // in the page tables: 32 * PMD_SIZE (16k granule)
    cfg_if! {
        if #[cfg(CONFIG_ARM64_4K_PAGES)] {
            // Phys mem start shift
            const MEM_START_SHIFT: usize = pgtable::PudTable::SHIFT;
        } else if #[cfg(CONFIG_ARM64_16K_PAGES)] {
            // Phys mem start shift
            const MEM_START_SHIFT: usize = pgtable::PmdTable::CONT_SHIFT;
        } else if #[cfg(CONFIG_ARM64_64K_PAGES)] {
            // Phys mem start shift
            const MEM_START_SHIFT: usize = pgtable::PmdTable::SHIFT;
        } else {
            compile_error!("Unknown page size");
        }
    }

    cfg_if! {
        if #[cfg(CONFIG_ARM64_PA_BITS_52)] {
            /// Physical address mask shift
            pub const PHYS_MASK_SHIFT: usize = 52;
            /// Physical address mask
            pub const PHYS_MASK: usize = (1 << Self::PHYS_MASK_SHIFT) - 1;
        } else {
            /// Physical address mask shift
            pub const PHYS_MASK_SHIFT: usize = 48;
            /// Physical address mask
            pub const PHYS_MASK: usize = (1 << Self::PHYS_MASK_SHIFT) - 1;
        }
    }


    /// sparsemem vmemmap imposes an additional requirement on the alignment of
    /// memstart_addr, due to the fact that the base of the vmemmap region
    /// has a direct correspondence, and needs to appear sufficiently aligned
    /// in the virtual address space.
    pub const  fn memstart_align() -> usize {
        if Self::MEM_START_SHIFT < SECTION_SIZE_BITS {
            1 << SECTION_SIZE_BITS
        } else {
            1 << Self::MEM_START_SHIFT
        }
    }
}

