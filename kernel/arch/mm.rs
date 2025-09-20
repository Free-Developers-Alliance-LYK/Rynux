//! Memory management code.

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        /// ARM64-specific thread memory layout.
        pub use super::arm64::mm::thread_layout::Arm64ThreadMemLayout as ArchThreadMemLayout;
    } else {
        pub use super::dummy::mm::DummyThreadMemLayout as ArchThreadMemLayout;
    }
}
