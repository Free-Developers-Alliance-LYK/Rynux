//! Arm64 pi map

use kernel::macros::section_init_text;
//use kernel::arch::arm64::pgtable::{Pgdir, Pte};
//use kernel::mm::page::PAGE_SIZE;

/// Create initial ID map
#[no_mangle]
#[section_init_text]
pub unsafe extern "C" fn create_init_idmap(_pg_dir: u64, _clrmask: u64) -> u64 {
    0
}

