//! ID_AA64MMFR3_EL1

use crate::bitflags::bitflags;

bitflags! {
    /// ID_AA64MMFR3_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct IdAa64mmfr3El1: u64 {
        /// SPACC
        const SPACC = 0b1111 << 60;
        /// ADERR
        const ADERR = 0b1111 << 56;
        /// SDERR
        const SDERR = 0b1111 << 52;
        /// RPZ
        const RPZ = 0b1111 << 48;
        /// ANERR
        const ANERR = 0b1111 << 44;
        /// SNERR
        const SNERR = 0b1111 << 40;
        /// D128_2
        const D128_2 = 0b1111 << 36;
        /// D128
        const D128 = 0b1111 << 32;
        /// MEC
        const MEC = 0b1111 << 28;
        /// AIE
        const AIE = 0b1111 << 24;
        /// S2POE
        const S2POE = 0b1111 << 20;
        /// S1POE
        const S1POE = 0b1111 << 16;
        /// S2PIE
        const S2PIE = 0b1111 << 12;
        /// S1PIE
        const S1PIE = 0b1111 << 8;
        /// SCTLRX
        const SCTLRX = 0b1111 << 4;
        /// TCRX
        const TCRX = 0b1111 << 0;
    }
}

impl IdAa64mmfr3El1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let id_aa64mmfr3_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64MMFR3_EL1", "x", id_aa64mmfr3_el1);
        Self::from_bits_truncate(id_aa64mmfr3_el1)
    }

    /// Read register raw.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let id_aa64mmfr3_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64MMFR3_EL1", "x", id_aa64mmfr3_el1);
        id_aa64mmfr3_el1
    }

    const S1PIE_OFFSET: u64 = 8;
    /// s1pie
    #[inline(always)]
    pub fn s1pie(&self) -> u64 {
        (self.bits() & Self::S1PIE.bits()) >> Self::S1PIE_OFFSET
    }

    /// s1pie
    #[inline(always)]
    pub fn s1pie_support(&self) -> bool {
        self.s1pie() != 0
    }

    const TCRX_OFFSET: u64 = 0;
    /// tcrx
    #[inline(always)]
    pub fn tcrx(&self) -> u64 {
        (self.bits() & Self::TCRX.bits()) >> Self::TCRX_OFFSET
    }

    /// s2pie
    #[inline(always)]
    pub fn tcrx_support(&self) -> bool {
        self.tcrx() != 0
    }
}
