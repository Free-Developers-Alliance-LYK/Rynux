//! Arm64 mm init

use crate::{
    arch::arm64::mm::{Arm64PhysConfig, Arm64VaLayout},
    drivers::fdt::GLOBAL_FDT,
    global_sym::{_end, _stext},
    mm::memblock::GLOBAL_MEMBLOCK,
    mm::{PhysAddr, VirtAddr},
};

/// Arm64 memblock init
pub fn memblock_init() {
    // scan mem from fdt
    GLOBAL_MEMBLOCK.lock().scan_mem_from_fdt(&GLOBAL_FDT);
    // remove memory above our supported physical address size
    GLOBAL_MEMBLOCK.lock().remove_memory(
        PhysAddr::from(1 << Arm64PhysConfig::PHYS_MASK_SHIFT),
        usize::MAX,
    );

    let linear_region_size = Arm64VaLayout::KERNEL_VA_LINE_SIZE;

    // aling down to a supported address
    let mut memstart_addr = GLOBAL_MEMBLOCK
        .lock()
        .start_of_dram()
        .align_down(Arm64PhysConfig::memstart_align());

    // The linear virtual address space cannot cover the physical memory.
    // To trim the physical memory, trim it from the high address first,
    // and be careful not to trim the kernel.
    let pa_end = VirtAddr::from(_end as usize).symbol_to_phys();
    let remove_start = pa_end.max(memstart_addr + linear_region_size);
    GLOBAL_MEMBLOCK
        .lock()
        .remove_memory(remove_start, usize::MAX);

    // The linear virtual address space still cannot cover the physical
    // memory, indicating that the kernel is outside the high address
    // coverage range and is clipped from the low address
    if (GLOBAL_MEMBLOCK.lock().end_of_dram() - memstart_addr) > linear_region_size {
        memstart_addr = (GLOBAL_MEMBLOCK.lock().end_of_dram() - linear_region_size)
            .align_up(Arm64PhysConfig::memstart_align());
        GLOBAL_MEMBLOCK
            .lock()
            .remove_memory(PhysAddr::from(0), memstart_addr.as_usize());
    }

    // memblock set kernel image as reserved
    GLOBAL_MEMBLOCK.lock().add_reserved(
        VirtAddr::from(_stext as usize).symbol_to_phys(),
        _end as usize - _stext as usize,
    );

    GLOBAL_MEMBLOCK.lock().reserve_mem_from_fdt(&GLOBAL_FDT);
}
