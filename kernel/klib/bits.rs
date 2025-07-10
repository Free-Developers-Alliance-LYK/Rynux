//! Bit manipulation

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_64BIT)] {
        /// BITS_PER_LONG is always 64
        pub const BITS_PER_LONG: usize = 64;
    } else {
        /// BITS_PER_LONG is always 32
        pub const BITS_PER_LONG: usize = 32;
    }
}

/// BITS_PER_LONG_LONG is always 64
pub const BITS_PER_LONG_LONG: usize = 64;

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h. For example
/// genmask(39, 21) gives us the 64bit vector 0x000000ffffe00000.
pub const fn genmask(h: u32, l: u32) -> u64 {
    genmask_n(h, l, BITS_PER_LONG as u32)
}

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h. For example
/// genmask_n(39, 21, 64) gives us the 64bit vector 0x000000ffffe00000.
pub const fn genmask_n(h: u32, l: u32, bits: u32) -> u64 {
    ((!0u64) << l) & ((!0u64) >> (bits - 1 - h))
}

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h. For example
/// genmask_ull(39, 21) gives us the 64bit vector 0x000000ffffe00000.
pub const fn genmask_ull(h: u32, l: u32) -> u128 {
    genmask_ull_n(h, l, BITS_PER_LONG_LONG as u32)
}

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h. For example
/// genmask_ull_n(39, 21, 64) gives us the 64bit vector 0x000000ffffe00000.
pub const fn genmask_ull_n(h: u32, l: u32, bits: u32) -> u128 {
    ((!0u128) << l) & ((!0u128) >> (bits - 1 - h))
}

#[cfg(test)]
mod tests {

     #[test]
    fn test_genmask_basic() {
        let bits = BITS_PER_LONG as u32;
        if bits == 32 {
            assert_eq!(genmask(bits - 1, 0), !0u32 as u64);
            assert_eq!(genmask(15, 8), 0xFF00);
            assert_eq!(genmask(7, 0), 0xFF);
            assert_eq!(genmask(0, 0), 1);
            assert_eq!(genmask(3, 1), 0b1110);
        } else if bits == 64 {
            assert_eq!(genmask(bits - 1, 0), !0u64);
            assert_eq!(genmask(31, 0), 0xFFFF_FFFF);
            assert_eq!(genmask(63, 32), 0xFFFF_FFFF_0000_0000);
            assert_eq!(genmask(15, 8), 0xFF00);
            assert_eq!(genmask(0, 0), 1);
        }
    }

    #[test]
    fn test_genmask_n_manual() {
        assert_eq!(genmask_n(31, 0, 32), !0u32 as u64);
        assert_eq!(genmask_n(15, 8, 32), 0xFF00);
        assert_eq!(genmask_n(0, 0, 32), 1);

        assert_eq!(genmask_n(63, 0, 64), !0u64);
        assert_eq!(genmask_n(63, 32, 64), 0xFFFF_FFFF_0000_0000);
        assert_eq!(genmask_n(0, 0, 64), 1);
    }

    #[test]
    fn test_genmask_ull() {
        assert_eq!(genmask_ull(127, 64), (!0u128 << 64) & (!0u128));
        assert_eq!(genmask_ull(63, 0), !0u64 as u128);
        assert_eq!(genmask_ull(0, 0), 1);
    }

    #[test]
    fn test_l_greater_than_h() {
        assert_eq!(genmask(3, 5), 0);
        assert_eq!(genmask_n(10, 20, 32), 0);
        assert_eq!(genmask_ull(7, 8), 0);
    }
}


