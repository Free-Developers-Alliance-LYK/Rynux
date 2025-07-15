//! ARM64 pmuserenr_el0

use crate::bitflags::bitflags;
use super::{IdAa64dfr0El1, id_aa64dfr0_el1::PmuVer};

bitflags! {
    /// PMUSERENR_EL0
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct PmuserenrEl0: u64 {
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


impl PmuserenrEl0 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let pmuserenr: u64;
        sys_coproc_read_raw!(u64, "PMUSERENR_EL0", "x", pmuserenr);
        Self::from_bits_truncate(pmuserenr)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let pmuserenr = self.bits();
        sys_coproc_write_raw!(u64, "PMUSERENR_EL0", "x", pmuserenr);
    }

    /// Write raw
    #[inline(always)]
    pub fn write_raw(pmuserenr: u64) {
        sys_coproc_write_raw!(u64, "PMUSERENR_EL0", "x", pmuserenr);
    }

    /// Reset PMUSERENR_EL0 if PMUv3 present 
    #[inline(always)]
    pub fn reset() {
        let pmuver = IdAa64dfr0El1::read_pmu_ver();
        if pmuver == PmuVer::NI || pmuver == PmuVer::IMP_DEF {
            return;
        }
        Self::write_raw(0);
    }
}
