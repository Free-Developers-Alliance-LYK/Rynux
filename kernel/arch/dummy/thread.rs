//! dummy thread info

/// Thread info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DummyThreadInfo {
    /// Cpu id
    pub cpu: u32,
}

impl DummyThreadInfo {
    /// Default thread info
    pub const fn default() -> Self {
        Self {
            cpu: 0,
        }
    }
}
