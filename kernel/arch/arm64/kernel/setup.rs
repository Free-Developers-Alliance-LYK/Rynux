//! Rynux arm64 setup

use crate::arch::arm64::mm::cache::L1_CACHE_BYTES;
use crate::macros::section_cache_aligned;
use crate::macros::section_init_data;
use crate::static_assertions::const_assert_eq;

const_assert_eq!(L1_CACHE_BYTES, 64);
/// Have to define this struct with repr align
#[allow(dead_code)]
#[repr(align(64))]
pub struct BootArgs {
    x0: usize,
    x1: usize,
    x2: usize,
    x3: usize,
}

/// The recorded values of x0 .. x3 upon kernel entry.
#[section_cache_aligned]
pub static BOOT_ARGS: BootArgs = BootArgs {
    x0: 0,
    x1: 0,
    x2: 0,
    x3: 0,
};

/// Whether the MMU was enabled at boot.
#[section_init_data]
pub static MMU_ENABLED_AT_BOOT: usize = 0;

/// Whether the MMU was enabled at boot.
pub static MMU_ENABLED_AT_BOOT2: usize = 0;
