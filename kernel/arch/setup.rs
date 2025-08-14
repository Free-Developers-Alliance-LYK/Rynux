//! Rynux arch setup

use crate::cfg_if;

/// Arch processor init
pub trait ArchBootSetupTrait {
    /// Setup the architecture-specific boot processor.
    fn setup_arch();
}

cfg_if! {
    if #[cfg(test)] {
        // Dummy boot setup for testing
    } else if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::kernel::setup::Arm64BootSetup as ArchBootSetup;
    }
}
