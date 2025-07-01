//! ARM64-specific cache module code.
//!

/// Arm64 L1 Cache line size
pub const L1_CACHE_SHIFT: usize = 6;
/// Arm64 L1 Cache line size
pub const L1_CACHE_BYTES: usize = 1 << L1_CACHE_SHIFT;
