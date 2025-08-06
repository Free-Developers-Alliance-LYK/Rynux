//! Cpu

use crate::cfg_if;

cfg_if! {
    if #[cfg(test)] {
        /// Max cpus
        pub const MAX_CPUS: usize = 1;
    } else if #[cfg(CONFIG_ARM64)] {
        /// Max cpus
        pub const MAX_CPUS: usize = 32;
    }
}

