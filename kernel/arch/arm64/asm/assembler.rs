//! ARM64-specific assembler code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_CPU_BIG_ENDIAN)] {
        /// Execute instruction only on big endian systems.
        #[macro_export]
        macro_rules! cpu_be {
            ($instr:expr) => {
                $instr
            };
        }

        /// Execute instruction only on little endian systems.
        #[macro_export]
        macro_rules! cpu_le {
            ($instr:expr) => { "" };
        }
    } else {
        /// Execute instruction only on big endian systems.
        #[macro_export]
        macro_rules! cpu_be {
            ($instr:expr) => { "" };
        }

        /// Execute instruction only on little endian systems.
        #[macro_export]
        macro_rules! cpu_le {
            ($instr:expr) => {
                $instr
            };
        }
    }
}

/// Generate an absolute address instruction.
/// @dst: destination register (64 bit wide)
/// @sym: name of the symbol
#[macro_export]
macro_rules! adr_l {
    ($dst:expr, $sym:expr) => {
        concat!("adrp\t", $dst, ", ", $sym, "\n",
                "add\t", $dst, ", ", $dst, ", :lo12:", $sym, "\n")
    };
}

/// Generate a load instruction.
/// @dst: destination register (32 or 64 bit wide)
/// @sym: name of the symbol
/// @tmp: optional 64-bit scratch register to be used if <dst> is a
///       32-bit wide register, in which case it cannot be used to hold
///       the address
#[macro_export]
macro_rules! str_l {
    ($src:expr, $sym:expr, $tmp:expr) => {
        concat!("adrp\t", $tmp, ", ", $sym, "\n",
                "str\t", $src, ", [", $tmp, ", :lo12:", $sym, "]\n")
    };
}


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

