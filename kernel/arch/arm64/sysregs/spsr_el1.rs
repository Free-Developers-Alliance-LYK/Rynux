//! ARM64 spsr_el1

use crate::bitflags::bitflags;

bitflags! {
    /// SPSR_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct SpsrEl1: u64 {
        /// Debug
        const D = 1 << 9;
        /// SERR
        const A = 1 << 8;
        /// FIRQ
        const I = 1 << 7;
        /// IRQ
        const F = 1 << 6;
        /// T32 Instruction set state. Set to the value of PSTATE.T on taking an exception to EL1, and copied to PSTATE.T on executing an exception return operation in EL1.
        const T = 1 << 5;
        /// Aarch32  CPSR BIT
        const M = 1 << 4;
        /// Mode
        const MODE =  0b1111 << 0;
    }
}

#[allow(non_upper_case_globals)]
impl SpsrEl1 {
    /// Modes EL0 
    pub const MODE_EL0t: Self = Self::from_bits_truncate(0b0000);
    /// EL1 with SP_EL0 (EL1t)
    pub const MODE_EL1t: Self = Self::from_bits_truncate(0b0100);
    /// EL1 with SP_EL1 (EL1h)
    pub const MODE_EL1h: Self = Self::from_bits_truncate(0b0101);
    /// EL2 with SP_EL0 (EL2t)
    pub const MODE_EL2t: Self = Self::from_bits_truncate(0b1000);
    /// EL2 with SP_EL1 (EL2h)
    pub const MODE_EL2h: Self = Self::from_bits_truncate(0b1001);
    /// EL3 with SP_EL0 (EL3t)
    pub const MODE_EL3t: Self = Self::from_bits_truncate(0b1100);
    /// EL3 with SP_EL1 (EL3h)
    pub const MODE_EL3h: Self = Self::from_bits_truncate(0b1101);

    /// INIT_PSTATE_EL1
    pub const INIT_PSTATE_EL1: Self = Self::from_bits_truncate(Self::D.bits() | Self::A.bits() | Self::I.bits() | Self::F.bits() | Self::MODE_EL1h.bits());

    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let sctlr: u64;
        sys_coproc_read_raw!(u64, "SPSR_EL1", "x", sctlr);
        Self::from_bits_truncate(sctlr)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let sctlr = self.bits();
        sys_coproc_write_raw!(u64, "SPSR_EL1", "x", sctlr);
    }
}
