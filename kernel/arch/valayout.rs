//! Arch Virtual Address Layout

/// Trait for architecture-specific virtual address layout.
pub trait ArchVaLayout {
    /// The virtual address of the start of the linear map
    fn kernel_va_start() -> usize;
    /// Kernel image virtual memory and physical memory offset
    fn kimg_va_offset() -> usize;
    /// liner map end
    fn linear_map_end() -> usize;
}

cfg_if::cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::mm::Arm64VaLayout as VaLayout;
    } else {
        pub use super::dummy::va_layout::DummyVaLayout as VaLayout;
    }
}
