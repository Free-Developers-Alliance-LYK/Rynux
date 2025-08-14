//! Arm64 Page table idmap
use crate::{
    cfg_if,
    arch::arm64::{
        mm::SEGMENT_ALIGN,
        pgtable::Arm64PgtableConfig,
        kernel::image::KERNEL_IMAGE_SIZE,
        va_layout::Arm64VaLayout,
    },
    mm::page::PageConfig,
    macros::need_export,
    size::SZ_2M,
};

const fn early_entries(lvl: usize, vstart: usize, vend: usize) -> usize {
    ((vend - 1) >> (Arm64PgtableConfig::SWAPPER_BLOCK_SHIFT + lvl * Arm64PgtableConfig::PTDESC_TABLE_SHIFT))
         - (vstart >> (Arm64PgtableConfig::SWAPPER_BLOCK_SHIFT + lvl * Arm64PgtableConfig::PTDESC_TABLE_SHIFT)) + 1
}

const fn early_level(lvl: usize, lvls: usize, vstart: usize, vend: usize, add: usize) -> usize {
    if lvls > lvl {
        early_entries(lvl, vstart, vend) + add
    } else {
        0
    }
}

const fn early_pages(lvls: usize, vstart: usize, vend: usize, add: usize) -> usize {
    1  // PGDIR PAGE
    + early_level(3, lvls, vstart, vend, add) // each entry needs a next level page table
    + early_level(2, lvls, vstart, vend, add) // each entry needs a next level page table
    + early_level(1, lvls, vstart, vend, add) // each entry needs a next level page table
}


// kernel end vaddr
const _END: usize = Arm64VaLayout::KIMAGE_VADDR + KERNEL_IMAGE_SIZE;

/// Idmap
pub struct InitIdmap;

impl InitIdmap {
    /// The virtual address space size for the initial identity mapping.
    pub const VA_BITS: usize = 48;

    // The initial ID map consists of the kernel image, mapped as two separate
    // segments, and may appear misaligned wrt the swapper block size. This means
    // we need 3 additional pages. The DT could straddle a swapper block boundary,
    // so it may need 2.
    const fn early_idmap_extra_pages() -> usize {
        if Arm64PgtableConfig::SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN {
            3
        } else {
            0
        }
    }

    const fn early_idmap_extra_fdt_pages() -> usize {
        if Arm64PgtableConfig::SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN {
            2
        } else {
            0
        }
    }

    // Number of page-table levels required to address IDMAP_VA_BITS wide
    // address, without section mapping.
    const LEVELS: usize = Arm64PgtableConfig::hw_pgtable_levels(Self::VA_BITS);
    /// The root level of the initial identity mapping.
    pub const ROOT_LEVEL: usize = 4 - Self::LEVELS;
    /// Number of page-table levels required to address IDMAP_VA_BITS wide
    /// address,but it may use section mapping.
    pub const PGTABLE_LEVELS: usize = Self::LEVELS - Arm64PgtableConfig::SWAPPER_SKIP_LEVEL;
    /// The number of page tables needed for the initial identity mapping.
    pub const DIR_PAGES: usize = early_pages(Self::PGTABLE_LEVELS, Arm64VaLayout::KIMAGE_VADDR, _END, 1);
    /// The size of the initial identity mapping.
    pub const DIR_SIZE: usize = (Self::DIR_PAGES + Self::early_idmap_extra_pages()) * PageConfig::PAGE_SIZE;

    /// The maximum size of the FDT.
    pub const MAX_FDT_SIZE: usize = SZ_2M;

    // The number of page tables needed for the initial FDT mapping.
    const EARLY_FDT_PAGES: usize = early_pages(Self::PGTABLE_LEVELS, 0, Self::MAX_FDT_SIZE, 1) - 1;
    /// Page table memory size required for early FDT mapping
    pub const EARLY_FDT_PAGE_SIZE: usize = (Self::EARLY_FDT_PAGES + Self::early_idmap_extra_fdt_pages()) * PageConfig::PAGE_SIZE;
}

/// The size of the initial identity mapping.
#[need_export]
pub static INIT_IDMAP_DIR_SIZE: usize = InitIdmap::DIR_SIZE;

/// InitMap
pub struct InitMap;

impl InitMap {
    // A relocatable kernel may execute from an address that differs from the one at
    // which it was linked. In the worst case, its runtime placement may intersect
    // with two adjacent PGDIR entries, which means that an additional page table
    // may be needed at each subordinate level.
    cfg_if! {
        if #[cfg(CONFIG_RELOCATABLE)] {
            const EXTRA_PAGE: usize = 1;
        } else {
            const EXTRA_PAGE: usize = 0;
        }
    }

    const fn early_segment_extra_pages() -> usize {
         /* The number of segments in the kernel image (text, rodata, inittext, initdata, data+bss) */
        const KERNEL_SEGMENT_COUNT: usize = 5;
        if Arm64PgtableConfig::SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN {
            KERNEL_SEGMENT_COUNT + 1
        } else {
            0
        }
    }

    /// The number of page tables needed for the initial mapping.
    pub const DIR_PAGES: usize = early_pages(Arm64PgtableConfig::SWAPPER_PGTABLE_LEVELS,
        Arm64VaLayout::KIMAGE_VADDR, _END, Self::EXTRA_PAGE);

    /// The size of the initial page tables.
    pub const DIR_SIZE: usize = (Self::DIR_PAGES + Self::early_segment_extra_pages()) * PageConfig::PAGE_SIZE;
}

/// The size of the initial page tables.
#[need_export]
pub static INIT_DIR_SIZE: usize = InitMap::DIR_SIZE;
