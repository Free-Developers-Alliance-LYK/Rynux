//! This module contains the architecture-specific processor initialization code for Arm64.

use crate::arch::arm64::kernel::smp;
use crate::arch::arm64::sysregs::MpidrEl1;
use crate::arch::cpu::ArchProcessorTrait;
use crate::macros::section_init_text;

/// Arm64 processor manager
pub struct Arm64Processor;

impl ArchProcessorTrait for Arm64Processor {
    #[section_init_text]
    fn boot_init(&self) {
        let aff = MpidrEl1::read().affinity();
        smp::set_main_cpu_hwid(aff);
    }

    fn boot_processor_id(&self) -> usize {
        // Always return 0 for the boot processor ID in Arm64.
        0
    }
}

impl Arm64Processor {
    /// Create a new instance of the Arm64 processor manager.
    pub const fn new() -> Self {
        Self
    }
}
