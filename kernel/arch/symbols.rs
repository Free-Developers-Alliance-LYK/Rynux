//! Architecture-specific ptrace code.

use crate::cfg_if;

cfg_if! {
    if #[cfg(test)] {
        // do nothing
    }else if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::symbols::*;
    }
}
