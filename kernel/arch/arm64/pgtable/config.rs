//! Arm64 Pgtable Config

use crate::{
    arch::arm64::{kernel::image::MIN_KIMG_ALIGN, pgtable::pmd::PmdTable},
    cfg_if,
    klib::math::div_round_up,
    mm::page::PageConfig,
};

/// Arm64 Pgtable Config
pub struct Arm64PgtableConfig();

#[allow(dead_code)]
impl Arm64PgtableConfig {
    cfg_if! {
        if #[cfg(all(CONFIG_ARM64_16K_PAGES, CONFIG_ARM64_VA_BITS_36)) ] {
            pub(crate) const HAS_PMD: bool = false;
            pub(crate) const HAS_PUD: bool = false;
            /// Number of page-table levels
            pub const PGTABLE_LEVELS: usize = 2;
            compile_error!("Current Not support PGTABLE_LEVELS 2");
        } else if #[cfg(all(CONFIG_ARM64_64K_PAGES, CONFIG_ARM64_VA_BITS_42))] {
            pub(crate) const HAS_PMD: bool = false;
            pub(crate) const HAS_PUD: bool = false;
            /// Number of page-table levels
            pub const PGTABLE_LEVELS: usize = 2;
            compile_error!("Current Not support PGTABLE_LEVELS 2");
        } else if #[cfg(all(CONFIG_ARM64_4K_PAGES, CONFIG_ARM64_VA_BITS_39))] {
            /// Number of page-table levels
            pub const PGTABLE_LEVELS: usize = 3;
            #[allow(dead_code)]
            pub(crate) const HAS_PMD: bool = true;
            pub(crate) const HAS_PUD: bool = false;
        } else if #[cfg(all(CONFIG_ARM64_16K_PAGES, CONFIG_ARM64_VA_BITS_47))] {
            /// Number of page-table levels
            pub const PGTABLE_LEVELS: usize = 3;
            pub(crate) const HAS_PMD: bool = true;
            pub(crate) const HAS_PUD: bool = false;
        } else if #[cfg(all(CONFIG_ARM64_64K_PAGES,CONFIG_ARM64_VA_BITS_48))] {
            /// Number of page-table levels
            pub const PGTABLE_LEVELS: usize = 3;
            pub(crate) const HAS_PMD: bool = true;
            pub(crate) const HAS_PUD: bool = false;
        } else if #[cfg(all(not(CONFIG_ARM64_64K_PAGES),CONFIG_ARM64_VA_BITS_48))] {
            /// Number of page-table levels
            pub const PGTABLE_LEVELS: usize = 4;
            pub(crate) const HAS_PMD: bool = true;
            pub(crate) const HAS_PUD: bool = true;
        } else {
            compile_error!("Unknown page-table levels");
        }
    }

    pub(crate) const PTDESC_ORDER: usize = 3;

    /// Number of VA bits resolved by a single translation table level
    pub const PTDESC_TABLE_SHIFT: usize = PageConfig::PAGE_SHIFT - Self::PTDESC_ORDER;

    /// Number of page-table levels required to address 'va_bits' wide
    /// address, without section mapping.
    ///
    /// levels = DIV_ROUND_UP((va_bits - PAGE_SHIFT), PTDESC_TABLE_SHIFT)
    /// where DIV_ROUND_UP(n, d) => (((n) + (d) - 1) / (d))
    ///
    pub const fn hw_pgtable_levels(va_bits: usize) -> usize {
        div_round_up(va_bits - PageConfig::PAGE_SHIFT, Self::PTDESC_TABLE_SHIFT)
        //((va_bits - PTDESC_ORDER - 1) / PTDESC_TABLE_SHIFT)
    }

    /// Size mapped by an entry at level n ( -1 <= n <= 3)
    /// We map PTDESC_TABLE_SHIFT at all translation levels and PAGE_SHIFT bits
    /// in the final page. The maximum number of translation levels supported by
    /// the architecture is 5. Hence, starting at level n, we have further
    /// ((4 - n) - 1) levels of translation excluding the offset within the page.
    /// So, the total number of bits mapped by an entry at level n is :
    ///
    ///  ((4 - n) - 1) * PTDESC_TABLE_SHIFT + PAGE_SHIFT
    ///
    /// Rearranging it a bit we get :
    ///   (4 - n) * PTDESC_TABLE_SHIFT + PTDESC_ORDER
    pub const fn hw_pgtable_levels_shift(n: usize) -> usize {
        (4 - n) * Self::PTDESC_TABLE_SHIFT + Self::PTDESC_ORDER
    }

    // The physical and virtual addresses of the start of the kernel image are
    // equal modulo 2 MiB (per the arm64 booting.txt requirements). Hence we can
    // use section mapping with 4K (section size = 2M) but not with 16K (section
    // size = 32M) or 64K (section size = 512M).
    const fn swapper_skip_level_shift() -> (usize, usize) {
        if Arm64PgtableConfig::HAS_PMD {
            if PmdTable::ENTRY_SIZE <= MIN_KIMG_ALIGN {
                (1, PmdTable::SHIFT)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        }
    }

    pub(crate) const SWAPPER_SKIP_LEVEL: usize = Self::swapper_skip_level_shift().0;
    pub(crate) const SWAPPER_BLOCK_SHIFT: usize = Self::swapper_skip_level_shift().1;
    pub(crate) const SWAPPER_BLOCK_SIZE: usize = 1 << Self::SWAPPER_BLOCK_SHIFT;
    pub(crate) const SWAPPER_PGTABLE_LEVELS: usize =
        Self::PGTABLE_LEVELS - Self::SWAPPER_SKIP_LEVEL;

    /// Calculate the number of entries in a span of virtual addresses
    pub const fn spann_nr_entries(vstart: usize, vend: usize, shift: usize) -> usize {
        ((vend - 1) >> shift) - (vstart >> shift) + 1
    }
}
