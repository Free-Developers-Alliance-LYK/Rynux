//! ARM64 system registers

use crate::bitflags::bitflags;
use crate::cfg_if;

bitflags! {
    /// SCRCTLR_ELx
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct SctlrEl1: u64 {
        /// Endianness bit
        const ELX_EE = 1 << 25;
        /// SA bit
        const ELX_SA = 1 << 3;
        /// Data cache BIT
        const ELX_C = 1 << 2;
        /// Alignment check
        const ELX_A = 1 << 1;
        /// mmu enable BIT
        const ELX_M = 1 << 0;


        /// EL1 LSMAOE bit
        const EL1_LSMAOE = 1 << 29;
        /// EL1 nTLSMD bit
        const EL1_nTLSMD = 1 << 28;
        /// EL1 E0E bit
        const EL1_E0E = 1 << 24;
        /// EL1 EIS bit
        const EL1_EIS = 1 << 22;
        /// EL1 IESB bit
        const EL1_IESB = 1 << 21;
        /// EL1 TSCXT bit
        const EL1_TSCXT = 1 << 20;
        /// EL1 EOS bit
        const EL1_EOS = 1 << 11;

    }
}

impl SctlrEl1 {
    cfg_if!{
        if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
            const ENDIAN_SET: Self = Self::from_bits_truncate(Self::ELX_EE.bits() | Self::EL1_E0E.bits());
        } else {
            const ENDIAN_SET: Self = Self::from_bits_truncate(0);
        }
    }


    /// INIT_SCTLR_EL1_MMU_OFF
    pub const INIT_SCTLR_EL1_MMU_OFF: Self = Self::from_bits_truncate(Self::ENDIAN_SET.bits() | Self::EL1_LSMAOE.bits() | Self::EL1_nTLSMD.bits() | Self::EL1_EIS.bits() | Self::EL1_TSCXT.bits() | Self::EL1_EOS.bits());

    /// EE bit shift
    pub const ELX_EE_SHIFT: u64 = 25;


    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let sctlr: u64;
        sys_coproc_read_raw!(u64, "SCTLR_EL1", "x", sctlr);
        Self::from_bits_truncate(sctlr)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let sctlr = self.bits();
        sys_coproc_write_raw!(u64, "SCTLR_EL1", "x", sctlr);
    }
}
