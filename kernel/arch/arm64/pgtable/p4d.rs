//! Arm64 Page table P4D
//!
use super::hard::{arm64_hw_pgtable_levels_shift, PTDESC_TABLE_SHIFT};

/// P4d value
pub type P4dVal = u64;




/// P4d
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct P4d (u64);

impl P4d {
    // determines the size a pte page table entry can map
    const SHIFT: usize = arm64_hw_pgtable_levels_shift(0);
    // Size of a PUD entry in bytes.
    const SIZE: usize = 1 << Self::SHIFT;
    // Number of entries per PUD
    const PTRS: usize = 1 <<  PTDESC_TABLE_SHIFT;
    // Mask for PUD entry
    const MASK: usize = !(Self::SIZE - 1);

    /// Create a new P4d
    pub const fn new(val: P4dVal) -> Self {
        Self(val)
    }

    /// Get the value of the P4d
    pub const fn p4d_value(&self) -> P4dVal {
        self.0
    }
}
