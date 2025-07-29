//! Rynux arch setup

use crate::cfg_if;

pub trait ArchProcessorInit {
    /// Setup processor id
    fn smp_setup_processor_id();
}

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::kernel::setup::Arm64ProcessorInit as ProcessorInit;
    }
}
