//! Dummy virtual address layout.

use crate::arch::valayout::ArchVaLayout;

/// Virtual Address Layout
pub struct DummyVaLayout();

impl ArchVaLayout for DummyVaLayout {
    #[inline(always)]
    /// The virtual address of the start of the linear map
    fn kernel_va_start() -> usize {
        0
    }

    #[inline(always)]
    /// The end of the linear map, where all other kernel mappings begin.
    fn linear_map_end() -> usize {
        0
    }

    #[inline(always)]
    /// Kernel image virtual memory and physical memory offset
    fn kimg_va_offset() -> usize {
        0
    }
}
