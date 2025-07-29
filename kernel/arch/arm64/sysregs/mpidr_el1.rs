//! ARM64 mpidr_el1


use crate::bitflags::bitflags;

bitflags! {
    /// MPIDR_EL1
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct MpidrEl1: u64 {
        /// Aff0
        const AFF0 = 0xff << 0;
        /// Aff1
        const AFF1 = 0xff << 8;
        /// Aff2
        const AFF2 = 0xff << 16;
        /// MT
        const MT = 1 << 24;
        /// Indicates a Uniprocessor system, as distinct from PE 0 in a
        /// multiprocessor system.
        const U = 1 << 30;
        /// Aff3
        const AFF3 = 0xff << 32;
    }
}


impl MpidrEl1 {
    
    /// Invalid HWID
    pub const INVALID_HWID: u64 = u64::MAX;

    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let mpidr: u64;
        sys_coproc_read_raw!(u64, "MPIDR_EL1", "x", mpidr);
        Self::from_bits_truncate(mpidr)
    }
    
    /// Read register raw.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let mpidr: u64;
        sys_coproc_read_raw!(u64, "MPIDR_EL1", "x", mpidr);
        mpidr
    }

    #[inline(always)]
    /// Affinity
    pub fn affinity(&self) -> u64 {
        self.bits() & (Self::AFF3.bits() |
            Self::AFF2.bits() |
            Self::AFF1.bits() | 
            Self::AFF0.bits())
    }

}
