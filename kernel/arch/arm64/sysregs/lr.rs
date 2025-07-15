//! ARM64 lr


/// LR
pub struct Lr;

impl Lr {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let lr: u64;
        read_raw!(u64, "lr", "x", lr);
        lr
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(lr: u64) {
        write_raw!(u64, "lr", "x", lr);
    }
}
