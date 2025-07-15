//! ARM64 cpacr_el1

use crate::bitflags::bitflags;

bitflags! {
    /// CPACR_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct CpacrEl1: u64 {
        /// FPEN bit
        const FPEN = 1 << 20;
        /// ZEN bit
        const ZEN = 1 << 16;
    }
}


impl CpacrEl1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let cpacr: u64;
        sys_coproc_read_raw!(u64, "CPACR_EL1", "x", cpacr);
        Self::from_bits_truncate(cpacr)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let cpacr = self.bits();
        sys_coproc_write_raw!(u64, "CPACR_EL1", "x", cpacr);
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(cpacr: u64) {
        sys_coproc_write_raw!(u64, "CPACR_EL1", "x", cpacr);
    }
}

