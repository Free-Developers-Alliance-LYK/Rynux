//! Dummy 

use crate::arch::cpu::ArchProcessorTrait;

/// dummy processor manager
pub struct DummyProcessor;

impl ArchProcessorTrait for DummyProcessor {
    fn boot_init(&self) {
    }

    fn boot_processor_id(&self) -> usize {
        0
    }
}

impl DummyProcessor {
    /// Create a new instance of the dummy processor manager.
    pub const fn new() -> Self {
        Self
    }
}

