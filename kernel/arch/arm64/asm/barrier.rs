//! ARM64 barrier


/// Instruction Synchronization Barrier
#[inline(always)]
pub fn isb()
{
    unsafe { core::arch::asm!("isb", options(nostack)) }
}
