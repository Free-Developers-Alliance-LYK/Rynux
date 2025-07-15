//! TLB asm

/// Flush all TLB
#[inline(always)]
pub fn local_flush_tlb_all() {
    unsafe {
        core::arch::asm!("dsb nshst; tlbi vmalle1; dsb nsh; isb", options(nostack, nomem));
    }
}
