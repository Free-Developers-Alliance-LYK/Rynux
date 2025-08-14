//! Cpu

use crate::cfg_if;

/// Trait for architecture-specific CPU processor management.
pub trait ArchProcessorTrait {
    /// Initialize the boot processor.
    fn boot_init(&self);
    /// Get the ID of the boot processor.
    fn boot_processor_id(&self) -> usize;
}

cfg_if! {
    if #[cfg(test)] {
        /// Max cpus
        pub const MAX_CPUS: usize = 32;
        pub use super::dummy::cpu::DummyProcessor as ArchProcessor;

    } else if #[cfg(CONFIG_ARM64)] {
        /// Max cpus
        pub const MAX_CPUS: usize = 32;
        pub use super::arm64::cpu::Arm64Processor as ArchProcessor;
    }
}

