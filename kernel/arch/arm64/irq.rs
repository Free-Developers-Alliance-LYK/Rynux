//! ARM64-specific IRQ handling code.

use core::arch::asm;
use crate::arch::irq::ArchIrq;

/// Arm64 IRQ
pub struct Arm64Irq;

impl ArchIrq for Arm64Irq {
    /// ARM64 IRQ state
    type IrqState = usize;

    #[inline(always)]
    fn local_save_and_disable() -> Self::IrqState {
        let flags: usize;
        // save `DAIF` flags, mask `I` bit (disable IRQs)
        unsafe { asm!("mrs {}, daif; msr daifset, #2", out(reg) flags) };
        flags
    }

    #[inline(always)]
    fn local_restore(flags: Self::IrqState) {
        // restore `DAIF` flags
        unsafe { asm!("msr daif, {}", in(reg) flags) };
    }
}
