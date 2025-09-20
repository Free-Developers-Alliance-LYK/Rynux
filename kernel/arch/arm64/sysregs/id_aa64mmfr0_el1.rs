//! ID_AA64MMFR0_EL1

use crate::{bitflags::bitflags, cfg_if};

bitflags! {
    /// ID_AA64MMFR0_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct IdAa64mmfr0El1: u64 {
        /// PARange
        const PARange = 0b1111 << 0;
        /// ASIDBits
        const ASIDBits = 0b1111 << 4;

        /// BIGEND
        const BIGEND = 0b1111 << 8;
        /// SNSMEM
        const SNSMEM = 0b1111 << 12;
        /// BIGENDEL0
        const BIGENDEL0 = 0b1111 << 16;
        /// TGRAN16
        const TGRAN16 = 0b1111 << 20;
        /// TGRAN64
        const TGRAN64 = 0b1111 << 24;
        /// TGRAN4
        const TGRAN4 = 0b1111 << 28;

        /// TGRAN16_2
        const TGRAN16_2 = 0b1111 << 32;
        /// TGRAN64_2
        const TGRAN64_2 = 0b1111 << 36;
        /// TGRAN4_2
        const TGRAN4_2 = 0b1111 << 40;

        /// EXS
        const EXS = 0b1111 << 44;

        /// RESERVE
        const RESERVE = 0xff << 48;
        /// FGT
        const FGT = 0b1111 << 56;
        /// ECV
        const ECV = 0b1111 << 60;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
/// Parange
pub(crate) enum PaRange {
    /// 32 bits
    Bits32 = 0b0000,
    /// 36 bits
    Bits36 = 0b0001,
    /// 40 bits
    Bits40 = 0b0010,
    /// 42 bits
    Bits42 = 0b0011,
    /// 44 bits
    Bits44 = 0b0100,
    /// 48 bits
    Bits48 = 0b0101,
    /// 52 bits
    Bits52 = 0b0110,
    /// 56 bits
    Bits56 = 0b0111,
}

impl From<u64> for PaRange {
    #[inline(always)]
    fn from(value: u64) -> Self {
        match value {
            0b0000 => PaRange::Bits32,
            0b0001 => PaRange::Bits36,
            0b0010 => PaRange::Bits40,
            0b0011 => PaRange::Bits42,
            0b0100 => PaRange::Bits44,
            0b0101 => PaRange::Bits48,
            0b0110 => PaRange::Bits52,
            _ => panic!("Invalid PARange value"),
        }
    }
}

impl IdAa64mmfr0El1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let id_aa64mmfr0_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64MMFR0_EL1", "x", id_aa64mmfr0_el1);
        Self::from_bits_truncate(id_aa64mmfr0_el1)
    }

    /// Read register raw.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let id_aa64mmfr0_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64MMFR0_EL1", "x", id_aa64mmfr0_el1);
        id_aa64mmfr0_el1
    }

    const PARANGE_SHIFT: u64 = 0;
    cfg_if! {
        if #[cfg(CONFIG_ARM64_PA_BITS_52) ] {
            const ID_AA64MMFR0_EL1_PARANGE_MAX: PaRange = PaRange::Bits52;
        } else {
            const ID_AA64MMFR0_EL1_PARANGE_MAX: PaRange = PaRange::Bits48;
        }
    }

    /// Read PARange
    #[inline(always)]
    pub(crate) fn parange(&self) -> PaRange {
        let sys_pa_size =
            PaRange::from((self.bits() & Self::PARange.bits()) >> Self::PARANGE_SHIFT);
        if sys_pa_size > Self::ID_AA64MMFR0_EL1_PARANGE_MAX {
            Self::ID_AA64MMFR0_EL1_PARANGE_MAX
        } else {
            sys_pa_size
        }
    }

    cfg_if! {
        if #[cfg(CONFIG_ARM64_4K_PAGES) ] {
            const TGRAN_OFFSET: u64 = 28;
            const TGRAN_SUPPORT_MIN: u64 = 0b0000;
            const TGRAN_SUPPORT_MAX: u64 = 0b0111;
        } else if #[cfg(CONFIG_ARM64_16K_PAGES) ] {
            const TGRAN_OFFSET: u64 = 20;
            const TGRAN_SUPPORT_MIN: u64 = 0b0001;
            const TGRAN_SUPPORT_MAX: u64 = 0b1111;
        } else if #[cfg(CONFIG_ARM64_64K_PAGES) ] {
            const TGRAN_OFFSET: u64 = 24;
            const TGRAN_SUPPORT_MIN: u64 = 0b0000;
            const TGRAN_SUPPORT_MAX: u64 = 0b0111;
        }
    }
    /// Read TGRAN
    #[inline(always)]
    pub fn tgran_check(&self) {
        let tg = (self.bits() >> Self::TGRAN_OFFSET) & 0b1111;
        if tg < Self::TGRAN_SUPPORT_MIN || tg > Self::TGRAN_SUPPORT_MAX {
            panic!("tgran check failed");
        }
    }
}
