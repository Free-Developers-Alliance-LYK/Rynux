//! Arm64 mmu

use crate::types::OnceCell;
use crate::macros::section_rodata_after_init;

#[link_section = ".mmuoff.data.write"]
/// Save Cpu boot status
pub static EARLY_CPU_BOOT_STATUS: usize = 0;

#[allow(dead_code)]
/// Cpu boot status
pub enum CpuStatus {
    /// Default is success
    CpuBootSuccess = 0,
    /// Killed by other
    CpuKillMe = 1,
    /// Stuck in kernel
    CpuStuckInKernel = 2,
    /// Panic in kernel
    CpuPanicKernel = 3,
}


#[section_rodata_after_init]
static KIMAGE_VOFFSET : OnceCell<usize> = OnceCell::new();

/// Set kimage voffset
#[inline(always)]
pub fn set_kimage_va_offset(voffset: usize) {
    KIMAGE_VOFFSET.set(voffset);
}
