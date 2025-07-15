//! ARM64 system registers

use crate::bitflags::bitflags;

bitflags! {
    /// Current Exception Level
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct CurrentEL: u64 {
        /// EL1 bit
        const EL1 = 1<<2;
        /// EL2 bit
        const EL2 = 2<<2;
    }
}

impl CurrentEL {
    /// Read register.
    #[inline(always)]
    pub fn read() -> Self {
        let mut current_el: u64;
        sys_coproc_read_raw!(u64, "CurrentEL", "x", current_el);
        Self::from_bits_truncate(current_el)
    }
}
