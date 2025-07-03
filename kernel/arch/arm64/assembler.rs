//! ARM64-specific assembler code.

use klib::cfg_if;

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
