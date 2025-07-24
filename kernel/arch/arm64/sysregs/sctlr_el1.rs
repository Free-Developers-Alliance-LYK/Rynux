//! ARM64 system registers

use crate::bitflags::bitflags;
use crate::cfg_if;

bitflags! {
    /// SCRCTLR_ELx
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct SctlrEl1: u64 {
        /// T1DCP
        const T1DCP = 1 << 63;
        /// SPINTMASK
        const SPINTMASK = 1 << 62;
        /// NMI
        const NMI = 1 << 61;
        /// EnTP2
        const EnTP2 = 1 << 60;
        /// TCSO
        const TCSO = 1 << 59;
        /// TCSO0
        const TCSO0 = 1 << 58;
        /// EPAN
        const EPAN = 1 << 57;
        /// EnALS
        const EnALS = 1 << 56;
        /// EnAS0
        const EnAS0 = 1 << 55;
        /// EnASR
        const EnASR = 1 << 54;
        /// TME
        const TME = 1 << 53;
        /// TME0
        const TME0 = 1 << 52;
        /// TMT
        const TMT = 1 << 51;
        /// TMT0
        const TMT0 = 1 << 50;
        /// TWEDEL
        const TWEDEL = 0b1111 << 46;
        /// TWEDEn
        const TWEDEn = 1 << 45;
        /// DSSBS
        const DSSBS = 1 << 44;
        /// ATA
        const ATA = 1 << 43;
        /// ATA0
        const ATA0 = 1 << 42;
        /// TCF
        const TCF = 0b11 << 40;
        /// TCF0
        const TCF0 = 0b11 << 38;
        /// ITFSB
        const ITFSB = 1 << 37;
        /// BT1
        const BT1 = 1 << 36;
        /// BT0
        const BT0 = 1 << 35;
        /// EnFPM
        const EnFPM = 1 << 34;
        /// MSCEn
        const MSCEn = 1 << 33;
        /// CMOW
        const CMOW = 1 << 32;
        /// EnIA
        const EnIA = 1 << 31;
        /// EnIB
        const EnIB = 1 << 30;
        /// LSMAOE
        const LSMAOE = 1 << 29;
        /// nTLSMD
        const nTLSMD = 1 << 28;
        /// EnDA
        const EnDA = 1 << 27;
        /// UCI
        const UCI = 1 << 26;
        /// Endianness bit
        const EE = 1 << 25;
        /// E0E
        const E0E = 1 << 24;
        /// SPAN
        const SPAN = 1 << 23;
        /// EIS
        const EIS = 1 << 22;
        /// IESB
        const IESB = 1 << 21;
        /// TSCXT
        const TSCXT = 1 << 20;
        /// WXN
        const WXN = 1 << 19;
        /// nTWE
        const nTWE = 1 << 18;
        /// nTWI
        const nTWI = 1 << 16;
        /// UCT
        const UCT = 1 << 15;
        /// DZE
        const DZE = 1 << 14;
        /// EnDB
        const EnDB = 1 << 13;
        /// I bit
        const I = 1 << 12;
        /// EOS
        const EOS = 1 << 11;
        /// EnRCTX
        const EnRCTX = 1 << 10;
        /// UMA
        const UMA = 1 << 9;
        /// SED
        const SED = 1 << 8;
        /// ITD
        const ITD = 1 << 7;
        /// nAA
        const nAA = 1 << 6;
        /// CP15BEN
        const CP15BEN = 1 << 5;
        /// SA0
        const SA0 = 1 << 4;
        /// SA bit
        const SA = 1 << 3;
        /// Data cache BIT
        const C = 1 << 2;
        /// Alignment check
        const A = 1 << 1;
        /// mmu enable BIT
        const M = 1 << 0;
    }
}

impl SctlrEl1 {
    cfg_if!{
        if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
            const ENDIAN_SET: Self = Self::from_bits_truncate(Self::EE.bits() | Self::E0E.bits());
        } else {
            const ENDIAN_SET: Self = Self::from_bits_truncate(0);
        }
    }


    /// INIT_SCTLR_EL1_MMU_OFF
    pub const INIT_SCTLR_EL1_MMU_OFF: Self = Self::from_bits_truncate(Self::ENDIAN_SET.bits() | Self::LSMAOE.bits() | Self::nTLSMD.bits() | Self::EIS.bits() | Self::TSCXT.bits() | Self::EOS.bits());

    /// INIT_SCTLR_EL1_MMU_ON
    pub const INIT_SCTLR_EL1_MMU_ON: Self = Self::from_bits_truncate(
        Self::M.bits() | Self::C.bits() | Self::SA.bits() |
        Self::SA0.bits() | Self::SED.bits() | Self::I.bits() |
        Self::DZE.bits() | Self::UCT.bits() | Self::nTWE.bits() |
        Self::IESB.bits() | Self::SPAN.bits() | Self::ITFSB.bits() |
        Self::ENDIAN_SET.bits() | Self::UCI.bits() | Self::EPAN.bits() |
        Self::LSMAOE.bits() | Self::nTLSMD.bits() | Self::EIS.bits() |
        Self::TSCXT.bits() | Self::EOS.bits()
    );


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
        unsafe {
            core::arch::asm!(
            "isb",
            "ic iallu",
            "dsb nsh",
            "isb",)
        }
    }
}
