//! Arch Virtual Address Layout
use crate::cfg_if;

/// Trait for architecture-specific virtual address layout.
pub trait ArchVaLayout {
    /// The virtual address of the start of the linear map
    fn kernel_va_start() -> usize;
    /// Kernel image virtual memory and physical memory offset 
    fn kimg_va_offset() -> usize;
    /// liner map end
    fn linear_map_end() -> usize;
}

cfg_if! {
    if #[cfg(test)] {
        pub use super::dummy::va_layout::DummyVaLayout as VaLayout;
    } else if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::va_layout::Arm64VaLayout as VaLayout;
    }
}
