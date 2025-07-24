//! Arm64 specific ptrace code


/// This struct defines the way the registers are stored on the stack during an
/// exception.
pub struct PtRegs {
    /// General purpose registers
    pub regs: [u64; 31],
    /// Stack pointer
    pub sp: u64,
    /// Program counter
    pub pc: u64,
    /// Processor state
    pub pstate: u64,

    /// Original x0 value
    pub orig_x0: u64,
    /// System call number
    pub syscallno: i32,
    /// Priority mask register
    pub pmr: u32,

    /// Software Delegated Exception TTBR1
    pub sdei_ttbr1: u64,
}
