//! ARM64 general purpose registers

/// SP
pub struct Sp;

impl Sp {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let sp: u64;
        read_raw!(u64, "sp", "x", sp);
        sp
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(sp: u64) {
        write_raw!(u64, "sp", "x", sp);
    }
}


///x0
pub struct X0;
impl X0 {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let x0: u64;
        read_raw!(u64, "x0", "x", x0);
        x0
    }

    /// Write register.
    #[inline(always)]
    pub fn write_raw(x0: u64) {
        write_raw!(u64, "x0", "x", x0);
    }
}

/// X29
pub struct X29;

impl X29 {
    /// Read register.
    #[inline(always)]
    pub fn read_raw() -> u64 {
        let x29: u64;
        read_raw!(u64, "x29", "x", x29);
        x29
    }
    /// Write register.
    #[inline(always)]
    pub fn write_raw(x29: u64) {
        write_raw!(u64, "x29", "x", x29);
    }
}

