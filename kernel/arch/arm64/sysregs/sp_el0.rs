//! ARM64 sp_el0

/// SP_EL0
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SpEl0(u64);

impl SpEl0 {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let sp: u64;
        sys_coproc_read_raw!(u64, "SP_EL0", "x", sp);
        sp
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(sp: u64) {
        sys_coproc_write_raw!(u64, "SP_EL0", "x", sp);
    }
}
