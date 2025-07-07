//! Arm64 Page table hardware definition
use crate::klib::math::div_round_up;
use crate::mm::page::PAGE_SHIFT;

const PTDESC_ORDER: usize = 3;

/// Number of VA bits resolved by a single translation table level
pub const PTDESC_TABLE_SHIFT: usize = PAGE_SHIFT - PTDESC_ORDER;

/// Number of page-table levels required to address 'va_bits' wide
/// address, without section mapping.
///
/// levels = DIV_ROUND_UP((va_bits - PAGE_SHIFT), PTDESC_TABLE_SHIFT)
/// where DIV_ROUND_UP(n, d) => (((n) + (d) - 1) / (d))
///
pub const fn arm64_hw_pgtable_levels(va_bits: usize) -> usize {
    div_round_up(va_bits - PAGE_SHIFT, PTDESC_TABLE_SHIFT)
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
pub const fn arm64_hw_pgtable_levels_shift(n: usize) -> usize {
    (4 - n) * PTDESC_TABLE_SHIFT + PTDESC_ORDER
}
