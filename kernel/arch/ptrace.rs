//! Architecture-specific ptrace code.

use crate::macros::need_export;

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::ptrace::PtRegs;
    } else {
        pub use super::dummy::ptrace::PtRegs;
    }
}

/// Export the size of the pt_regs struct
#[need_export]
pub static EXPORT_PT_REG_SIZE: usize = core::mem::size_of::<PtRegs>();
