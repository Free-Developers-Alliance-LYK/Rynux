//! ID_AA64PFR0_EL1

use crate::bitflags::bitflags;

bitflags! {
    /// ID_AA64PFR0_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct IdAa64pfr0El1: u64 {
        /// CSV3
        const CSV3 = 0b1111 << 60;
        /// CSV2
        const CSV2 = 0b1111 << 56;
        /// RME
        const RME = 0b1111 << 52;
        /// DIT
        const DIT = 0b1111 << 48;
        /// AMU
        const AMU = 0b1111 << 44;
        /// MPAM
        const MPAM = 0b1111 << 40;
        /// SEL2 
        const SEL2 = 0b1111 << 36;
        /// SVE
        const SVE = 0b1111 << 32;
        /// RAS
        const RAS = 0b1111 << 28;
        /// GIC 
        const GIC = 0b1111 << 24;
        /// AdvSIMD
        const AdvSIMD = 0b1111 << 20;
        /// FP
        const FP = 0b1111 << 16;
        /// EL3 
        const EL3 = 0b1111 << 12;
        /// EL2 
        const EL2 = 0b1111 << 8;
        /// EL1 
        const EL1 = 0b1111 << 4;
        /// EL0 
        const EL0 = 0b1111 << 0;
    }
}

/// AMU
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Amu {
    /// Not implemented
    NI = 0b0000,
    /// Implemented
    V1 = 0b0001,
    /// V1p1
    V1P1 = 0b0010,
}

impl From<u64> for Amu {
    #[inline(always)]
    fn from(value: u64) -> Self {
        match value {
            0b0000 => Amu::NI,
            0b0001 => Amu::V1,
            0b0010 => Amu::V1P1,
            _ => panic!("Invalid AMU value"),
        }
    }
}


impl IdAa64pfr0El1 {
    const AMU_SHIFT: u64 = 16;

    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let id_aa64pfr0_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64PFR0_EL1", "x", id_aa64pfr0_el1);
        Self::from_bits_truncate(id_aa64pfr0_el1)
    }

    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let id_aa64pfr0_el1: u64;
        sys_coproc_read_raw!(u64, "ID_AA64PFR0_EL1", "x", id_aa64pfr0_el1);
        id_aa64pfr0_el1
    }
    
    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let id_aa64pfr0_el1 = self.bits();
        sys_coproc_write_raw!(u64, "ID_AA64PFR0_EL1", "x", id_aa64pfr0_el1);
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(id_aa64pfr0_el1: u64) {
        sys_coproc_write_raw!(u64, "ID_AA64PFR0_EL1", "x", id_aa64pfr0_el1);
    }


    /// Read PMU
    #[inline(always)]
    pub fn read_amu() -> Amu {
        let id_aa64pfr0_el1 = Self::read();
        Amu::from((id_aa64pfr0_el1.bits() & Self::AMU.bits()) >> Self::AMU_SHIFT)
    }

}


