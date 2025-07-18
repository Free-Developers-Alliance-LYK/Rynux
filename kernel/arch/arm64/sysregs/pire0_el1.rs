//! ARM64 pire0_el1

use crate::bitflags::bitflags;

bitflags! {
    /// PIRE0_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct Pire0El1: u64 {
        /// Perm0
        const Perm0 = 0b1111 << 0;
        /// Perm1
        const Perm1 = 0b1111 << 4;
        /// Perm2
        const Perm2 = 0b1111 << 8;
        /// Perm3
        const Perm3 = 0b1111 << 12;
        /// Perm4
        const Perm4 = 0b1111 << 16;
        /// Perm5
        const Perm5 = 0b1111 << 20;
        /// Perm6
        const Perm6 = 0b1111 << 24;
        /// Perm7
        const Perm7 = 0b1111 << 28;
        /// Perm8
        const Perm8 = 0b1111 << 32;
        /// Perm9
        const Perm9 = 0b1111 << 36;
        /// Perm10
        const Perm10 = 0b1111 << 40;
        /// Perm11
        const Perm11 = 0b1111 << 44;
        /// Perm12
        const Perm12 = 0b1111 << 48;
        /// Perm13
        const Perm13 = 0b1111 << 52;
        /// Perm14
        const Perm14 = 0b1111 << 56;
        /// Perm15
        const Perm15 = 0b1111 << 60;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum Perm {
    None = 0b0000,
    Read = 0b0001,
    Exec = 0b0010,
    ReadExec = 0b0011,
    ReadWrite = 0b0101,
    ReadWriteExec = 0b0111,
}

impl From<Perm> for u64 {
    #[inline(always)]
    fn from(value: Perm) -> Self {
        value as u64
    }
}

impl Pire0El1 {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let pire0_el1: u64;
        sys_coproc_read_raw!(u64, "PIRE0_EL1", "x", pire0_el1);
        Self::from_bits_truncate(pire0_el1)
    }

    /// Write register.
    #[inline(always)]
    pub fn write(&self) {
        let pire0_el1 = self.bits();
        sys_coproc_write_raw!(u64, "PIRE0_EL1", "x", pire0_el1);
    }
}
