//! TTBR0_EL1

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64_PA_BITS_52)] {
        use crate::klib::bits::genmask_ull;
        const TTBR_BADDR_MASK_52: u64 = genmask_ull(47, 2);
        #[inline(always)]
        fn phys_to_ttbr(phys: u64) -> u64 {
            (phys | (phys >> 46)) & Self::TTBR_BADDR_MASK_52
        }
    } else {
        #[inline(always)]
        fn phys_to_ttbr(phys: u64) -> u64 {
            phys
        }
    }
}

/// Ttbr0El1
pub struct Ttbr0El1();

impl Ttbr0El1 {
    /// write pg_dir phys
    #[inline(always)]
    pub fn write_pg_dir(pg_dir: u64) {
        let ttbr0 = phys_to_ttbr(pg_dir);
        sys_coproc_write_raw!(u64, "TTBR0_EL1", "x", ttbr0);
    }
}

/// Ttbr1El1
pub struct Ttbr1El1();

impl Ttbr1El1 {
    /// write pg_dir phys
    #[inline(always)]
    pub fn write_pg_dir(pg_dir: u64) {
        let ttbr1 = phys_to_ttbr(pg_dir);
        sys_coproc_write_raw!(u64, "TTBR1_EL1", "x", ttbr1);
    }
}
