//! Dummy specific ptrace code

/// This struct defines the way the registers are stored on the stack during an
/// exception.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct PtRegs {
    /// General purpose registers
    pub regs: [u64; 31],
    /// Stack pointer
    pub sp: u64,
    /// Program counter
    pub pc: u64,
    /// Processor state
    pub pstate: u64,
}

impl PtRegs {
    /// From raw ptr
    #[inline(always)]
    pub unsafe fn from_raw(raw: *const Self) -> Self {
        unsafe { *raw }
    }
}
