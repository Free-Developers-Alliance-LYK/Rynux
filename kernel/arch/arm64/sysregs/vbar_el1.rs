//! ARM64 vbar_el1

/// VbarEl1
pub struct VbarEl1;

impl VbarEl1 {
    /// Write register.
    #[inline(always)]
    pub fn write_raw(vbar: u64) {
        sys_coproc_write_raw!(u64, "VBAR_EL1", "x", vbar);
    }
}
