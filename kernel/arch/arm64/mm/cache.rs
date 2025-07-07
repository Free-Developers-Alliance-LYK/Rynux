//! Cache

/// Arm64 L1 Cache line size
pub const L1_CACHE_SHIFT: usize = 6;
/// Arm64 L1 Cache line size
pub const L1_CACHE_BYTES: usize = 1 << L1_CACHE_SHIFT;

///
/// Ensure that any D-cache lines for the interval [start, end)
/// are invalidated. Any partial lines at the ends of the interval are
/// also cleaned to PoC to prevent data loss.
///
/// - start   - start address of region
/// - end     - end address of region
///
#[naked]
#[no_mangle]
#[link_section = ".text"]
pub unsafe extern "C" fn dcache_inval_poc(start: usize, end: usize) {
    unsafe {
        core::arch::naked_asm!(
            // x0 = start, x1 = end
            // cache line size: x2
            "mrs x3, ctr_el0",
            "ubfm x3, x3, #16, #19", // cache line size encoding
            "mov x2, #4", // bytes per word
            "lsl x2, x2, x3", // actual cache line size

            "sub x3, x2, #1",
            "tst x1, x3",  // end cache line aligned?
            "bic x1, x1, x3",
            "b.eq 1f",
            "dc civac, x1", // clean & invalidate D / U line
            "1: tst x0, x3", // start cache line aligned?
            "bic x0, x0, x3",
            "b.eq 2f",
            "dc civac, x0", // clean & invalidate D / U line
            "b 3f",
            "2: dc ivac, x0", // invalidate D / U line
            "3: add x0, x0, x2",
            "cmp x0, x1",
            "b.lo 2b",
            "dsb sy",
            "ret"
        );
    }
}
