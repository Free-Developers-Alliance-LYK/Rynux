//! ARM64 amuserenr_el0

use crate::bitflags::bitflags;
use super::id_aa64pfr0_el1::{IdAa64pfr0El1, Amu};

bitflags! {
    /// AMUSERENR_EL0
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct AmuserenrEl0: u64 {
        /// EN bit
        const EN = 1 << 0;
        /// SW bit
        const SW = 1 << 1;
        /// CR bit
        const CR = 1 << 2;
        /// ER bit
        /// Event counter can be read at EL0
        const ER = 1 << 3;
    }
}

impl AmuserenrEl0 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let amuserenr: u64;
        sys_coproc_read_raw!(u64, "S3_3_C13_C2_3", "x", amuserenr);
        Self::from_bits_truncate(amuserenr)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let amuserenr = self.bits();
        sys_coproc_write_raw!(u64, "S3_3_C13_C2_3", "x", amuserenr);
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(amuserenr: u64) {
        sys_coproc_write_raw!(u64, "S3_3_C13_C2_3", "x", amuserenr);
    }

    /// reset AMUSERENR_EL0 if AMUv1 present
    #[inline(always)]
    pub fn reset() {
        let amu = IdAa64pfr0El1::read_amu();
        if amu != Amu::NI {
            Self::write_raw(0);
        }
    }

}
