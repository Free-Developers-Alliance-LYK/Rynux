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
}
