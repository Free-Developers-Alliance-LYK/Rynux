//! Kernel architecture-specific code.

use klib::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub mod arm64;
    }
}
