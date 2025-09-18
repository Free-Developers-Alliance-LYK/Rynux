//! Rynux fdt driver

use core::ops::Deref;

use crate::types::OnceCell;
use crate::mm::VirtAddr;
use crate::fdtree_rs::LinuxFdt;

pub use crate::fdtree_rs::FdtNode;

//use crate::arch::arm64::early_debug::early_uart_put_u64_hex;

/// A wrapper for LinuxFdt to impl Deref
pub struct LinuxFdtWrapper <'a> {
    fdt: LinuxFdt<'a>,
}

/// A static instance of the FDT. only init once
pub static GLOBAL_FDT: OnceCell<LinuxFdtWrapper<'static>> = 
    OnceCell::new();

impl <'a> LinuxFdtWrapper <'a> {
    pub(crate) fn setup(fdt_va: VirtAddr)  {
    // SAFETY: fdt from_ptr would check header and magic number
    GLOBAL_FDT.set(
        unsafe { 
            LinuxFdtWrapper { fdt: LinuxFdt::from_ptr(fdt_va.as_usize() as *const u8).expect("Invalid fdt")}
        });
    }

    // scan fdt reserved memory and reserve it in memblock
    pub(crate) fn early_scan_reserved_memory() {
        let fdt = GLOBAL_FDT.deref();
        // first reserved system memory
        for r in fdt.sys_memory_reservations() {
            GLOBAL_MEMBLOCK.lock().add_reserved(PhysAddr::from(r.address() as usize), r.size());
        }

    }
}

impl Deref for LinuxFdtWrapper<'static> {
    type Target = LinuxFdt<'static>;

    fn deref(&self) -> &Self::Target {
        &self.fdt
    }
}
