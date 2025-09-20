//! Bit manipulation

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h.
#[inline(always)]
pub const fn genmask32(h: u32, l: u32) -> u32 {
    if l > 31 || h > 31 || l > h {
        0
    } else {
        let width = h - l + 1;
        let mask = if width == 32 {
            u32::MAX
        } else {
            ((1u32 << width) - 1) << l
        };
        mask
    }
}

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h.
pub const fn genmask64(h: u32, l: u32) -> u64 {
    if l > 63 || h > 63 || l > h {
        0
    } else {
        let width = h - l + 1;
        let mask = if width == 64 {
            u64::MAX
        } else {
            ((1u64 << width) - 1) << l
        };
        mask
    }
}

/// Create a contiguous bitmask starting at bit position @l and ending at
/// position @h.
pub const fn genmask128(h: u32, l: u32) -> u128 {
    if l > 127 || h > 127 || l > h {
        0
    } else {
        let width = h - l + 1;
        let mask = if width == 128 {
            u128::MAX
        } else {
            ((1u128 << width) - 1) << l
        };
        mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genmask_basic() {
        assert_eq!(genmask32(32 - 1, 0), !0u32);
        assert_eq!(genmask32(15, 8), 0xFF00);
        assert_eq!(genmask32(7, 0), 0xFF);
        assert_eq!(genmask32(0, 0), 1);
        assert_eq!(genmask32(3, 1), 0b1110);

        assert_eq!(genmask64(64 - 1, 0), !0u64);
        assert_eq!(genmask64(31, 0), 0xFFFF_FFFF);
        assert_eq!(genmask64(63, 32), 0xFFFF_FFFF_0000_0000);
        assert_eq!(genmask64(15, 8), 0xFF00);
        assert_eq!(genmask64(0, 0), 1);
    }

    #[test]
    fn test_l_greater_than_h() {
        assert_eq!(genmask32(3, 5), 0);
        assert_eq!(genmask64(10, 20), 0);
        assert_eq!(genmask128(7, 8), 0);
    }
}
