//! Kernel architecture-specific code.

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub mod arm64;
    } else {
        pub mod dummy;
    }
}

pub mod cache;
pub mod cpu;
pub mod irq;
pub mod mm;
pub mod ptrace;
pub mod setup;
pub mod symbols;
pub mod thread;
pub mod valayout;
pub mod vmrynux;
