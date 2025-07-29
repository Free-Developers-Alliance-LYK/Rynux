//! Arm64 specific ptrace code

#[repr(u64)]
#[derive(Copy, Clone, Debug, Default)]
/// Stack frame meta type
pub enum StackFrameMetaType {
    /// This value is reserved.
    #[default] None = 0,
    /// The record is the last entry on the stack.
    Final = 1,
    /// The record is embedded within a struct pt_regs, recording the registers at
    /// an arbitrary point in time.
    PtRegs = 2,
}

/// A standard AAPCS64 frame record.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct FrameRecord {
    /// Frame pointer
    pub fp: u64,
    /// Link register
    pub lr: u64,
}

/// A metadata frame record indicating a special unwind.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct StackFrameMeta {
    /// Type of the frame
    pub ty: StackFrameMetaType,
    /// Address of the frame
    pub record: FrameRecord,
}

impl StackFrameMeta {
    /// Get raw ptr
    #[inline(always)]
    pub fn as_ptr(&self) -> *const Self {
        self as *const Self
    }
}

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

    /// Original x0 value
    pub orig_x0: u64,
    /// System call number
    pub syscallno: i32,
    /// Priority mask register
    pub pmr: u32,

    /// Software Delegated Exception TTBR1
    pub sdei_ttbr1: u64,

    /// Stack frame record
    pub stackframe: StackFrameMeta,
}

impl PtRegs {
    /// From raw ptr
    #[inline(always)]
    pub unsafe fn from_raw(raw: *const Self) -> Self {
        unsafe {*raw}
    }

    #[inline(always)]
    /// Init stack frame
    pub fn init_stackframe(&mut self) {
        self.stackframe.record.fp = 0;
        self.stackframe.record.lr = 0;
        self.stackframe.ty = StackFrameMetaType::Final;
    }
}
