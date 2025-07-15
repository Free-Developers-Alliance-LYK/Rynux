//! Cache

/// Arm64 L1 Cache line size
pub const L1_CACHE_SHIFT: usize = 6;
/// Arm64 L1 Cache line size
pub const L1_CACHE_BYTES: usize = 1 << L1_CACHE_SHIFT;

macro_rules! dcache_line_size {
    ($reg:expr, $tmp:expr) => {
        concat!(
            "mrs ", $tmp, ", ctr_el0\n",
            "ubfm ", $tmp, ", ", $tmp, ", #16, #19\n",
            "mov ", $reg, ", #4\n",
            "lsl ", $reg, ", ", $reg, ", ", $tmp, "\n"
        )
    }
}

// TODO: support ARM64_WORKAROUND_CLEAN_CACHE
macro_rules! __dcache_op_workaround_clean_cache {
    ($op:expr, $addr:expr) => {
        concat!(
            "dc ", $op, ", ", $addr, "\n",
        )
    }
}

macro_rules! dcache_op {
    ("cvau", $start:expr) => {
        __dcache_op_workaround_clean_cache!("cvau", $start)
    };
    ("cvac", $start:expr) => {
        __dcache_op_workaround_clean_cache!("cvac", $start)
    };
    ("cvap", $start:expr) => {
        concat!("sys 3, c7, c12, 1, ", $start, "\n")
    };
    ("cvadp", $start:expr) => {
        concat!("sys 3, c7, c13, 1, ", $start, "\n")
    };
    ($op:expr, $start:expr) => {
        concat!("dc ", $op, ", ", $start, "\n")
    };
}

/// Macro to perform a data cache maintenance for the interval
/// [start, end) with dcache line size explicitly provided.
///
/// - op      - operation passed to dc instruction
/// - domain  - domain used in dsb instruction
/// - start   - starting virtual address of the region
/// - end     - end virtual address of the region
/// - linesz  - dcache line size
/// - tmp     - temporary register
/// - fixup   - optional label to branch to on user fault
/// - Corrupts - start, end, tmp
macro_rules! dcache_by_myline_op {
    ($op:expr, $domain:expr, $start:expr, $end:expr, $linesz:expr, $tmp:expr) => {
        concat!(
            "sub ", $tmp, ", ", $linesz, ", #1\n",
            "bic ", $start, ", ", $start, ", ", $tmp, "\n",
            "0:\n",
            dcache_op!($op, $start),
            "add ", $start, ", ", $start, ", ", $linesz, "\n",
            "cmp ", $start, ", ", $end, "\n",
            "b.lo 0b\n",
            "dsb ", $domain, "\n"

            //TODO: support cond_uaccess_extable
        )
    };
}

/// Macro to perform a data cache maintenance for the interval
/// [start, end)
///
/// - op      - operation passed to dc instruction
/// - domain  - domain used in dsb instruction
/// - start   - starting virtual address of the region
/// - end     - end virtual address of the region
/// - fixup   - optional label to branch to on user fault
/// - Corrupts - start, end, tmp1, tmp2
macro_rules! dcache_by_line_op {
    ($op:expr, $domain:expr, $start:expr, $end:expr, $tmp1:expr, $tmp2:expr) => {
        concat!(
            // tmp1 store cache line size
            dcache_line_size!($tmp1, $tmp2),
            dcache_by_myline_op!($op, $domain, $start, $end, $tmp1, $tmp2)
        )
    };

    ($op:expr, $domain:expr, $start:expr, $end:expr, $tmp1:expr, $tmp2:expr, $fixup:expr) => {
        compile_error!("Not implemented")
    };
}

/// Ensure that any D-cache lines for the interval [start, end)
/// are invalidated. Any partial lines at the ends of the interval are
/// also cleaned to PoC to prevent data loss.
///
/// - start   - start address of region
/// - end     - end address of region
///
#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text"]
pub unsafe extern "C" fn dcache_inval_poc(start: usize, end: usize) {
        core::arch::naked_asm!(
            "bti c",
            // x0 = start, x1 = end
            // cache line size: x2

            dcache_line_size!("x2", "x3"),
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

/// Ensure that any D-cache lines for the interval [start, end)
/// are cleaned to the PoC.
///
/// - start   - start address of region
/// - end     - end address of region
///
#[unsafe(naked)]
#[no_mangle]
#[link_section = ".text"]
pub unsafe extern "C" fn dcache_clean_poc(start: usize, end: usize) {
    core::arch::naked_asm!(
        "bti c",
        // x0 = start, x1 = end
        // cache line size: x2  x3:tmp register
        dcache_by_line_op!("cvac", "sy", "x0", "x1", "x2", "x3"),
    )
}
