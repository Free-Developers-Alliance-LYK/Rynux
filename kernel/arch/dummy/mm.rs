//! Thread memory layout for ARM64 architecture

/// Dummy thread memory layout
pub struct DummyThreadMemLayout();

impl DummyThreadMemLayout {
    /// Minimum thread shift
    pub const MIN_THREAD_SHIFT: usize = 14;
    /// Thread size
    pub const THREAD_SIZE: usize = 1 << Self::THREAD_SHIFT;
    /// Thread shift
    pub const THREAD_SHIFT: usize = Self::MIN_THREAD_SHIFT;
    /// Thread align
    pub const THREAD_ALIGN: usize = Self::THREAD_SIZE;
}
