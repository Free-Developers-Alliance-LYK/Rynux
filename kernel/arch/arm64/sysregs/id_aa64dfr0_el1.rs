//! ID_AA64DFR0_EL1

use crate::bitflags::bitflags;

bitflags! {
    /// ID_AA64DFR0_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct IdAa64dfr0El1: u64 {
        /// BRPS 
        const BRPS = 0b1111 << 12;
        /// PMUVer
        const PMUVer = 0b1111 << 8;
        /// TraceVer
        const TraceVer = 0b1111 << 4;
        /// DebugVer
        const DebugVer = 0b1111 << 0;
    }
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PmuVer {
    /// Not implemented
    NI = 0b0000,
    /// Implemented
    IMP = 0b0001,
    /// PMUv3 for Armv8.1
    V3P1 = 0b0100,
    /// PMUv3 for Armv8.4
    V3P4 = 0b0101,
    /// PMUv3 for Armv8.5
    V3P5 = 0b0110,
    /// PMUv3 for Armv8.7
    V3P7 = 0b0111,
    /// PMUv3 for Armv8.8
    V3P8 = 0b1000,
    /// PMUv3 for Armv9
    V3P9 = 0b1001,
    /// IMPLEMENTATION DEFINED form of performance monitors supported, PMUv3 not supported. Arm does not recommend this value for new implementations.
    IMP_DEF = 0b1111,
}

impl From<u64> for PmuVer {
    #[inline(always)]
    fn from(val: u64) -> Self {
        match val {
            0b0000 => PmuVer::NI,
            0b0001 => PmuVer::IMP,
            0b0100 => PmuVer::V3P1,
            0b0101 => PmuVer::V3P4,
            0b0110 => PmuVer::V3P5,
            0b0111 => PmuVer::V3P7,
            0b1000 => PmuVer::V3P8,
            0b1001 => PmuVer::V3P9,
            0b1111 => PmuVer::IMP_DEF,
            _ => panic!("Invalid PMUVer value: {}", val),
        }
    }
}

impl IdAa64dfr0El1 {
    /// PMUVer Shift
    const PMUVER_SHIFT: u64 = 8;

    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let id_aa64dfr0_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64DFR0_EL1", "x", id_aa64dfr0_el1);
        Self::from_bits_truncate(id_aa64dfr0_el1)
    }

    /// Read register raw.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let id_aa64dfr0_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64DFR0_EL1", "x", id_aa64dfr0_el1);
        id_aa64dfr0_el1
    }

    /// Read PMUVer
    #[inline(always)]
    pub fn read_pmu_ver() -> PmuVer {
        let id_aa64dfr0_el1 = Self::read();
        PmuVer::from((id_aa64dfr0_el1.bits() & Self::PMUVer.bits()) >> Self::PMUVER_SHIFT)
    }

}

