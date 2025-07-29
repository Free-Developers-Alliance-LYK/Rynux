//! Rynux arm64 setup

use crate::{
    macros::{section_cache_aligned, section_init_data, cache_aligned, section_init_text},
    mm::addr::PhysAddr,
    types::OnceCell,
    arch::setup::ArchProcessorInit,
    arch::arm64::sysregs::MpidrEl1,
    arch::cpu::MAX_CPUS,
};

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
pub static MMU_ENABLED_AT_BOOT2: usize = 0;


/// BOOT CPU MODE from EL1
pub const BOOT_CPU_MODE_EL1: usize = 0xe11;
/// BOOT CPU MODE from EL2
pub const BOOT_CPU_MODE_EL2: usize = 0xe12;


/// FDT phys addr
#[section_init_data]
static FDT_POINTER : OnceCell<PhysAddr> = OnceCell::new();

/// Set FDT pointer
pub fn set_fdt_pointer(pa: PhysAddr) {
    FDT_POINTER.set(pa);
}

/// Arm64 processor init
pub struct Arm64ProcessorInit;

impl ArchProcessorInit for Arm64ProcessorInit {
    #[section_init_text]
    fn smp_setup_processor_id() {
        let aff = MpidrEl1::read().affinity();
        set_cpu_logical_map(0, aff);
    }
}

static __CPU_LOGICAL_MAP: [u64; MAX_CPUS] = [MpidrEl1::INVALID_HWID; MAX_CPUS];


