//! ARM64-specific IRQ handling code.

use crate::arch::arm64::sysregs::Daif;
use crate::arch::irq::ArchIrq;
use crate::compiler::barrier;

/// Arm64 IRQ
pub struct Arm64Irq;

impl ArchIrq for Arm64Irq {
    /// ARM64 IRQ state
    type IrqState = u64;

    #[inline(always)]
    fn local_disable() {
        barrier();
        Daif::disable_irq();
        barrier();
    }

    #[inline(always)]
    fn local_enable() {
        barrier();
        Daif::enable_irq();
        barrier();
    }

    #[inline(always)]
    fn local_save_and_disable() -> Self::IrqState {
        let flags = Daif::read_raw();
        Self::local_disable();
        flags
    }

    #[inline(always)]
    fn local_restore(flags: Self::IrqState) {
        barrier();
        Daif::write_raw(flags);
        barrier();
    }
}
