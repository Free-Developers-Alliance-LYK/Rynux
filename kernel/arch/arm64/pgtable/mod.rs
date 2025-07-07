//! Arm64 Page table

use crate::cfg_if;

pub mod hard;
pub mod pgdir;
pub mod pte;
pub use pgdir::Pgdir;
pub use pte::Pte;

pub mod idmap;
pub mod pmd;

cfg_if! {
    if #[cfg(all(CONFIG_ARM64_16K_PAGES, CONFIG_ARM64_VA_BITS_36)) ] {
        /// Number of page-table levels
        pub(crate) const HAS_PMD: bool = false;
        pub const PGTABLE_LEVELS: usize = 2;
    } else if #[cfg(all(CONFIG_ARM64_64K_PAGES, CONFIG_ARM64_VA_BITS_42))] {
        /// Number of page-table levels
        pub(crate) const HAS_PMD: bool = false;
        pub const PGTABLE_LEVELS: usize = 2;
    } else if #[cfg(all(CONFIG_ARM64_4K_PAGES, CONFIG_ARM64_VA_BITS_39))] {
        /// Number of page-table levels
        pub const PGTABLE_LEVELS: usize = 3;
        #[allow(dead_code)]
        pub(crate) const HAS_PMD: bool = true;
    } else if #[cfg(all(CONFIG_ARM64_16K_PAGES, CONFIG_ARM64_VA_BITS_47))] {
        /// Number of page-table levels
        pub const PGTABLE_LEVELS: usize = 3;
        pub(crate) const HAS_PMD: bool = true;
    } else if #[cfg(all(CONFIG_ARM64_64K_PAGES,any(CONFIG_ARM64_VA_BITS_48, CONFIG_ARM64_VA_BITS_52)))] {
        /// Number of page-table levels
        pub const PGTABLE_LEVELS: usize = 3;
        pub(crate) const HAS_PMD: bool = true;
    } else if #[cfg(all(CONFIG_ARM64_16K_PAGES,any(CONFIG_ARM64_VA_BITS_48, CONFIG_ARM64_VA_BITS_52)))] {
        /// Number of page-table levels
        pub const PGTABLE_LEVELS: usize = 4;
        pub(crate) const HAS_PMD: bool = true;
        pub mod pud;
    } else if #[cfg(all(not(CONFIG_ARM64_64K_PAGES),CONFIG_ARM64_VA_BITS_48))] {
        /// Number of page-table levels
        pub const PGTABLE_LEVELS: usize = 4;
        pub(crate) const HAS_PMD: bool = true;
        pub mod pud;
    } else if #[cfg(all(CONFIG_ARM64_4K_PAGES, CONFIG_ARM64_VA_BITS_52))] {
        /// Number of page-table levels
        pub const PGTABLE_LEVELS: usize = 5;
        pub(crate) const HAS_PMD: bool = true;
        pub mod pud;
        pub mod p4d;
    } else {
        compile_error!("Unknown page-table levels");
    }
}
