//! Arm64 Page table PUD
//!

use crate::arch::arm64::pgtable::config::Arm64PgtableConfig;

/// Pud value
pub type PudVal = u64;

/// Pud
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Pud(u64);

#[allow(dead_code)]
impl Pud {
    // determines the size a pte page table entry can map
    const SHIFT: usize = Arm64PgtableConfig::hw_pgtable_levels_shift(1);
    // Size of a PUD entry in bytes.
    const SIZE: usize = 1 << Self::SHIFT;
    // Number of entries per PUD
    const PTRS: usize = 1 <<  Arm64PgtableConfig::PTDESC_TABLE_SHIFT;
    // Mask for PUD entry
    const MASK: usize = !(Self::SIZE - 1);

    /// Create a new Pud
    pub const fn new(val: PudVal) -> Self {
        Self(val)
    }

    /// Get the value of the Pud
    pub const fn pud_value(&self) -> PudVal {
        self.0
    }


}

