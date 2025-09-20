//! Rynux arm64 setup

use crate::{
    arch::arm64::{mm::fixmap::FixMap, pgtable::pgprot::PtePgProt},
    arch::setup::ArchBootSetupTrait,
    macros::{cache_aligned, section_cache_aligned, section_init_data},
    mm::PhysAddr,
    types::OnceCell,
};

#[cfg(not(test))]
use crate::mm::memblock::GLOBAL_MEMBLOCK;

/// Have to define this struct with repr align
#[allow(dead_code)]
#[cache_aligned]
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
#[section_init_data]
pub static MMU_ENABLED_AT_BOOT2: usize = 0;

/// BOOT CPU MODE from EL1
pub const BOOT_CPU_MODE_EL1: usize = 0xe11;
/// BOOT CPU MODE from EL2
pub const BOOT_CPU_MODE_EL2: usize = 0xe12;

/// FDT phys addr
#[section_init_data]
static FDT_POINTER: OnceCell<PhysAddr> = OnceCell::new();

/// Set FDT pointer
pub fn set_fdt_pointer(pa: PhysAddr) {
    FDT_POINTER.set(pa);
}

/// Archip-specific boot setup trait.
pub struct Arm64BootSetup;

impl Arm64BootSetup {
    #[cfg(not(test))]
    fn setup_machine_fdt() {
        let (dt_virt, size) =
            FixMap::remap_fdt(*FDT_POINTER.get().unwrap(), PtePgProt::PAGE_KERNEL_RO);
        crate::drivers::fdt::LinuxFdtWrapper::setup(dt_virt);
        // init bootcoomand line from fdt bootargs
        crate::init::command_line::setup_from_fdt();
        // reserve fdt mem
        GLOBAL_MEMBLOCK
            .lock()
            .add_reserved(*FDT_POINTER.get().unwrap(), size);
    }
}

impl ArchBootSetupTrait for Arm64BootSetup {
    #[cfg(not(test))]
    fn setup_arch() {
        FixMap::early_fixmap_init();
        Self::setup_machine_fdt();
        crate::arch::arm64::mm::init::memblock_init();
        crate::arch::arm64::mm::mmu::paging_init();
        crate::init::GLOBAL_COMMAND_LINE
            .lock()
            .parse_early_options();
        // init arm64 memblock
        //crate::arch::arm64::mm::init::paging_init();
    }

    #[cfg(test)]
    fn setup_arch() {}
}
