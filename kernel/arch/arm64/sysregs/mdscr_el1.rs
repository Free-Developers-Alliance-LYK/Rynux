//! ARM64 mdscr_el1

use crate::bitflags::bitflags;

bitflags! {
    /// MDSCR_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct MdscrEl1: u64 {
        /// TDCC bit
        const TDCC = 1 << 12;
        /// KDE bit
        const KDE = 1 << 13;
        /// HDE bit
        const HDE = 1 << 14;
        /// MDE bit
        const MDE = 1 << 15;
    }
}

impl MdscrEl1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let mdscr: u64;
        sys_coproc_read_raw!(u64, "MDSCR_EL1", "x", mdscr);
        Self::from_bits_truncate(mdscr)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let mdscr = self.bits();
        sys_coproc_write_raw!(u64, "MDSCR_EL1", "x", mdscr);
    }
}
