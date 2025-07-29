//! Cpu

use crate::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        /// Max cpus
        pub const MAX_CPUS: usize = 32;
    }
}

