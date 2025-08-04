//! ARM64 daif

use crate::bitflags::bitflags;

bitflags! {
    /// DAIF
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Daif: u64 {
        /// Debug
        const D = 1 << 9;
        /// SERR
        const A = 1 << 8;
        /// FIRQ
        const I = 1 << 7;
        /// IRQ
        const F = 1 << 6;
    }
}


impl Daif {
    /// Read raw register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let daif: u64;
        sys_coproc_read_raw!(u64, "DAIF", "x", daif);
        daif
    }

    /// Write raw register.
    #[inline(always)]
    pub fn write_raw(daif: u64) {
        sys_coproc_write_raw!(u64, "DAIF", "x", daif);
    }

    /// Disable irq
    #[inline(always)]
    pub fn disable_irq() {
        unsafe { core::arch::asm!("msr daifset, #3") };
    }

    /// Enable irq
    #[inline(always)]
    pub fn enable_irq() {
        unsafe { core::arch::asm!("msr daifclr, #3") };
    }
}
