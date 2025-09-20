//! Dummy irq
use crate::arch::irq::ArchIrq;

/// Arm64 IRQ
pub struct DummyIrq;

impl ArchIrq for DummyIrq {
    /// ARM64 IRQ state
    type IrqState = ();

    #[inline(always)]
    fn local_disable() {}

    #[inline(always)]
    fn local_enable() {}

    #[inline(always)]
    fn local_save_and_disable() -> Self::IrqState {
        ()
    }

    #[inline(always)]
    fn local_restore(_flags: Self::IrqState) {}
}
