//! Control  compiler behavior

/// Compiler barrier
#[inline(always)]
pub fn barrier() {
    unsafe {
        core::arch::asm!("");
    }
}
