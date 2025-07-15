//! ARM64 elr_el1


/// ELR_EL1
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ElrEl1(u64);

impl ElrEl1 {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let lr: u64;
        sys_coproc_read_raw!(u64, "ELR_EL1", "x", lr);
        lr
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(lr: u64) {
        sys_coproc_write_raw!(u64, "ELR_EL1", "x", lr);
    }
}
