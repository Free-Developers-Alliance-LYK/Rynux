//! ARM64-specific assembler code.

/// Generate an absolute address instruction.
/// @dst: destination register (64 bit wide)
/// @sym: name of the symbol
#[macro_export]
macro_rules! adr_l {
    ($dst:expr, $sym:expr) => {
        concat!("adrp\t", $dst, ", ", $sym, "\n", "add\t", $dst, ", ", $dst, ", :lo12:", $sym, "\n")
    };
}

pub use adr_l;

/// Generate a load instruction.
#[macro_export]
macro_rules! str_l {
    ($src:expr, $sym:expr, $tmp:expr) => {
        concat!(
            "adrp\t", $tmp, ", ", $sym, "\n", "str\t", $src, ", [", $tmp, ", :lo12:", $sym, "]\n"
        )
    };
}

pub use str_l;

/// Exception return
///
/// Will jump to wherever the corresponding link register points to, and therefore never return.
#[inline(always)]
pub fn eret() -> ! {
    unsafe {
        core::arch::asm!("eret", options(nomem, nostack));
        core::hint::unreachable_unchecked()
    }
}
