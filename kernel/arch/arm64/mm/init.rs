//! Arm64 mm init

use crate::{
    mm::memblock::GLOBAL_MEMBLOCK,
    mm::{PhysAddr,VirtAddr},
    arch::arm64::mm::{Arm64PhysConfig, Arm64VaLayout},
    global_sym::_end,
};

/// Arm64 memblock init
pub fn memblock_init() {
    // scan mem from fdt
    crate::mm::memblock::setup_from_fdt();
    // remove memory above our supported physical address size
    GLOBAL_MEMBLOCK.lock().remove_memory(PhysAddr::from(1 << Arm64PhysConfig::PHYS_MASK_SHIFT), usize::MAX);

    let linear_region_size = Arm64VaLayout::KERNEL_VA_LINE_SIZE;

    // aling down to a supported address
    let mut memstart_addr = GLOBAL_MEMBLOCK.lock().start_of_dram().align_down(Arm64PhysConfig::memstart_align());

    // Start cropping from high address memory first, need to preserve kernel
    let pa_end = VirtAddr::from(_end as usize).symbol_to_phys();
    let remove_start = pa_end.max(memstart_addr + linear_region_size);
    GLOBAL_MEMBLOCK.lock().remove_memory(remove_start, usize::MAX);
        
    // Liner mem still not cover all phy mem?
    // we can only crop from low address memory
    if (GLOBAL_MEMBLOCK.lock().end_of_dram() - memstart_addr) > linear_region_size {
        memstart_addr = (GLOBAL_MEMBLOCK.lock().end_of_dram() - linear_region_size).align_up(Arm64PhysConfig::memstart_align());
        GLOBAL_MEMBLOCK.lock().remove_memory(PhysAddr::from(0), memstart_addr.as_usize());
    }





}




