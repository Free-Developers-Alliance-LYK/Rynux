//! Arm64 Page table idmap
use super::{
    HAS_PMD,
    hard::{arm64_hw_pgtable_levels,PTDESC_TABLE_SHIFT},
    pmd::Pmd,
};

use super::PGTABLE_LEVELS;
use crate::mm::page::PAGE_SIZE;
use crate::arch::arm64::va_layout::{KIMAGE_VADDR, KERNEL_IMAGE_SIZE, MIN_KIMG_ALIGN};

const fn early_entries(lvl: usize, vstart: usize, vend: usize) -> usize {
    ((vend - 1) >> (SWAPPER_BLOCK_SHIFT + lvl * PTDESC_TABLE_SHIFT)) - (vstart >> (SWAPPER_BLOCK_SHIFT + lvl * PTDESC_TABLE_SHIFT)) + 1
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

// The physical and virtual addresses of the start of the kernel image are
// equal modulo 2 MiB (per the arm64 booting.txt requirements). Hence we can
// use section mapping with 4K (section size = 2M) but not with 16K (section
// size = 32M) or 64K (section size = 512M).
const fn swapper_skip_level_shift() -> (usize, usize) {
    if HAS_PMD {
        if Pmd::SIZE <= MIN_KIMG_ALIGN {
            (1, Pmd::SHIFT)
        } else {
            (0, 0)
        }
    } else {
        (0, 0)
    }
}

const SWAPPER_SKIP_LEVEL: usize = swapper_skip_level_shift().0;
const SWAPPER_BLOCK_SHIFT: usize = swapper_skip_level_shift().1;
const SWAPPER_BLOCK_SIZE: usize = 1 << SWAPPER_BLOCK_SHIFT;
const SWAPPER_PGTABLE_LEVELS: usize = PGTABLE_LEVELS - SWAPPER_SKIP_LEVEL;

// The initial ID map consists of the kernel image, mapped as two separate
// segments, and may appear misaligned wrt the swapper block size. This means
// we need 3 additional pages. The DT could straddle a swapper block boundary,
// so it may need 2.
const fn early_idmap_extra_pages() -> usize {
    use crate::arch::arm64::mm::SEGMENT_ALIGN;
    if SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN {
        3
    } else {
        0
    }
}

const fn early_segment_extra_pages() -> usize {
    use crate::arch::arm64::mm::SEGMENT_ALIGN;
     /* The number of segments in the kernel image (text, rodata, inittext, initdata, data+bss) */
    const KERNEL_SEGMENT_COUNT: usize = 5;
    if SWAPPER_BLOCK_SIZE > SEGMENT_ALIGN {
        KERNEL_SEGMENT_COUNT + 1
    } else {
        0
    }
}

// The virtual address space size for the initial identity mapping.
const IDMAP_VA_BITS: usize = 48;
// Number of page-table levels required to address IDMAP_VA_BITS wide
// address, without section mapping.
const IDMAP_LEVELS: usize = arm64_hw_pgtable_levels(IDMAP_VA_BITS);
// The root level of the initial identity mapping.
#[allow(dead_code)]
const IDMAP_ROOT_LEVEL: usize = 4 - IDMAP_LEVELS;
// Number of page-table levels required to address IDMAP_VA_BITS wide
// address,but it may use section mapping.
const INIT_IDMAP_PGTABLE_LEVELS: usize = IDMAP_LEVELS - SWAPPER_SKIP_LEVEL;
// kernel end vaddr
const _END: usize = KIMAGE_VADDR + KERNEL_IMAGE_SIZE;

/// The number of page tables needed for the initial identity mapping.
#[no_mangle]
pub static INIT_IDMAP_DIR_PAGES: usize = early_pages(INIT_IDMAP_PGTABLE_LEVELS, KIMAGE_VADDR, _END, 1);

/// The size of the initial identity mapping.
#[no_mangle]
pub static INIT_IDMAP_DIR_SIZE: usize = (INIT_IDMAP_DIR_PAGES + early_idmap_extra_pages()) * PAGE_SIZE;

static INIT_DIR_PAGES: usize = early_pages(SWAPPER_PGTABLE_LEVELS, KIMAGE_VADDR, _END, 1);
/// The size of the initial page tables.
#[no_mangle]
pub static INIT_DIR_SIZE: usize = (INIT_DIR_PAGES + early_segment_extra_pages()) * PAGE_SIZE;

