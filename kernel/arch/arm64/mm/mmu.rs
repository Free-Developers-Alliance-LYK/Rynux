//! Arm64 mmu

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
