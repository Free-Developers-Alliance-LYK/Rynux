//! Kernel architecture-specific code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(test)] {
        pub mod dummy;
    } else if #[cfg(CONFIG_ARM64)] {
        pub mod arm64;
    }
}

pub mod cache;
pub mod mm;
pub mod ptrace;
pub mod symbols;
pub mod thread;
pub mod cpu;
pub mod setup;
pub mod irq;
pub mod vmrynux;
