//! Rynux arch setup

/// Arch processor init
pub trait ArchBootSetupTrait {
    /// Setup the architecture-specific boot processor.
    fn setup_arch();
}

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::kernel::setup::Arm64BootSetup as ArchBootSetup;
    }
}
