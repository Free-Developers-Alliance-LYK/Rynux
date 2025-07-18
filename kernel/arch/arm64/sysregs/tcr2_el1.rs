//! ARM64 tcr2_el1

use crate::bitflags::bitflags;

bitflags! {
    /// TCR2_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Tcr2El1: u64 {
        /// PnCH
        const PnCH = 1 << 0;
        /// PIE
        const PIE = 1 << 1;
        /// E0POE
        const E0POE = 1 << 2;
        /// POE
        const POE = 1 << 3;
        /// AIE
        const AIE = 1 << 4;
        /// D128
        const D128 = 1 << 5;
        /// PTTWI
        const PTTWI = 1 << 10;
        /// HAFT
        const HAFT = 1 << 11;
        /// DisCH0
        const DisCH0 = 1 << 14;
        /// DisCH1
        const DisCH1 = 1 << 15;
        /// A2
        const A2 = 1 << 16;
        /// FNG0
        const FNG0 = 1 << 17;
        /// FNG1
        const FNG1 = 1 << 18;
        /// FNGNA0
        const FNGNA0 = 1 << 20;
        /// FNGNA1
        const FNGNA1 = 1 << 21;
    }
}

impl Tcr2El1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let tcr2: u64;
        sys_coproc_read_raw!(u64, "TCR2_EL1", "x", tcr2);
        Self::from_bits_truncate(tcr2)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let tcr2 = self.bits();
        sys_coproc_write_raw!(u64, "TCR2_EL1", "x", tcr2);
    }
}
