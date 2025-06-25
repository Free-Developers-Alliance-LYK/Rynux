//! Arm64 Page table P4D
//!
use super::hard::{arm64_hw_pgtable_levels_shift, PTDESC_TABLE_SHIFT};

pub(crate) struct P4d {
}

impl P4d {
    // determines the size a pte page table entry can map
    const SHIFT: usize = arm64_hw_pgtable_levels_shift(0);
    // Size of a PUD entry in bytes.
    const SIZE: usize = 1 << Self::SHIFT;
    // Number of entries per PUD
    const PTRS: usize = 1 <<  PTDESC_TABLE_SHIFT;
    // Mask for PUD entry
    const MASK: usize = !(Self::SIZE - 1);
}
