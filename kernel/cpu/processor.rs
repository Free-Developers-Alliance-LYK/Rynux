//! Processor manager

use super::CpuStateManager;
use crate::arch::cpu::{ArchProcessor, ArchProcessorTrait};

/// Processor manager
#[allow(dead_code)] // TODO: Remove this when the CpuMask is fully integrated.
pub struct ProcessorManager {
    arch_processor: ArchProcessor,
    cpu_state_manager: CpuStateManager,
}

#[allow(dead_code)] // TODO: Remove this when the CpuMask is fully integrated.
impl ProcessorManager {
    /// Initialize the boot processor
    pub fn boot_init(&self) {
        self.arch_processor.boot_init();
        self.cpu_state_manager.boot_init(self.boot_processor_id());
    }

    /// Get the ID of the boot processor
    pub fn boot_processor_id(&self) -> usize {
        self.arch_processor.boot_processor_id()
    }
}

/// Global instance of the processor manager
static PROCESSOR_MANAGER: ProcessorManager = ProcessorManager {
    arch_processor: ArchProcessor::new(),
    cpu_state_manager: CpuStateManager::new(),
};

/// Boot initialization for the processor manager
pub fn processor_boot_init() {
    PROCESSOR_MANAGER.boot_init();
}
