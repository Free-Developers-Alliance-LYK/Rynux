//! Architecture-specific IRQ code.

use crate::cfg_if;

/// Architecture-specific IRQ code.
pub trait ArchIrq {
    /// The type used to store the IRQ state.
    type IrqState;
    /// Disable IRQs and return the previous IRQ state.
    fn local_save_and_disable() -> Self::IrqState;
    /// Restore the IRQ state.
    fn local_restore(state: Self::IrqState);
}

cfg_if! {
    if #[cfg(CONFIG_ARM64)] {
        pub use super::arm64::irq::Arm64Irq as IRQ;
    }
}
