//! Rynux fdt driver

use crate::types::OnceCell;
use crate::mm::VirtAddr;
use crate::fdtree_rs::LinuxFdt;

//use crate::arch::arm64::early_debug::early_uart_put_u64_hex;

/// A static instance of the FDT. only init once
pub static GLOBAL_FDT: OnceCell<LinuxFdt<'static>> = OnceCell::new();

/// Setup FDT
pub fn setup_fdt(fdt_va: VirtAddr) {
    // SAFETY: CALLER must ensure fdt_va is valid
    GLOBAL_FDT.set(
        unsafe { LinuxFdt::from_ptr(fdt_va.as_usize() as *const u8).expect("Invalid fdt")}
    );
}
