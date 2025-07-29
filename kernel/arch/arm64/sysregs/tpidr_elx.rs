//! ARM64 tpidr_elx


/// TPIDR_EL1
pub struct TpidrEl1;

impl TpidrEl1 {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let tpidr: u64;
        sys_coproc_read_raw!(u64, "TPIDR_EL1", "x", tpidr);
        tpidr
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(tpidr: u64) {
        sys_coproc_write_raw!(u64, "TPIDR_EL1", "x", tpidr);
    }
}
