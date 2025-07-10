//! ARM64 system registers

use core::arch::asm;

use crate::bitflags::bitflags;

bitflags! {
    /// Current Exception Level
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct CurrentELFlags: u64 {
        /// EL1 bit
        const EL1 = 1<<2;
        /// EL2 bit
        const EL2 = 2<<2;
    }
}

/// SCRCTLR_ELx
pub struct SctlrElx {}

impl SctlrElx {
    /// EE bit shift
    pub const EE_SHIFT: u64 = 25;

    /// EE bit
    pub const EE: u64 = 1 << Self::EE_SHIFT;

    /// SA bit
    pub const SA: u64 = 1 << 3;
    /// Data cache BIT
    pub const C: u64 = 1 << 2;
    /// Alignment check
    pub const A: u64 = 1 << 1;
    /// mmu enable BIT
    pub const M: u64 = 1 << 0;
}

bitflags!{
    /// TCR flags
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct TcrFlags: u64 {
        /// Tcr DS bit
        const TCR_DS = 1 << 59;
    }
}

#[allow(dead_code)]
impl TcrFlags {
    const T0SZ_OFFSET: u64 = 0;
    const T1SZ_OFFSET: u64 = 16;
    const TXSZ_WIDTH: u64 = 6;
    const T0SZ_MASK: u64 = ((1 << Self::TXSZ_WIDTH) - 1) << Self::T0SZ_OFFSET;
    const T1SZ_MASK: u64 = ((1 << Self::TXSZ_WIDTH) - 1) << Self::T1SZ_OFFSET;

    /// Read TCR_EL1 register.
    #[inline(always)]
    pub fn read_tcr() -> Self {
        let tcr: u64;
        unsafe {
            asm!(
                "mrs {out}, tcr_el1",
                out = out(reg) tcr
            );
        }
        Self::from_bits_truncate(tcr)
    }

    /// T0SZ
    #[inline(always)]
    pub const fn t0sz(x: u64) -> u64 {
        (64 - x) << Self::T0SZ_OFFSET
    }

    /// T1SZ
    #[inline(always)]
    pub const fn t1sz(x: u64) -> u64 {
        (64 - x) << Self::T1SZ_OFFSET
    }

    /// TxSZ
    #[inline(always)]
    pub const fn txsz(x: u64) -> u64 {
        Self::t0sz(x) | Self::t1sz(x)
    }
}
