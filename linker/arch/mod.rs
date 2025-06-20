//! Defining variables for different architectures

use klib::cfg_if;

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub(crate) mod arm64;
        pub use arm64::DISCARDS as DISCARDS;
    }
}
