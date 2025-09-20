//! Cpu state  manager

use super::CpuMask;
use core::sync::atomic::{AtomicU64, Ordering};

struct CpuOnline {
    online_cnt: AtomicU64,
    online_mask: CpuMask,
}

/// A manager for CPU state, tracking online, active, present, and possible CPUs.
#[allow(dead_code)] // TODO: Remove this when the CpuMask is fully integrated.
pub(super) struct CpuStateManager {
    boot_cpu_id: AtomicU64,
    online: CpuOnline,
    active: CpuMask,
    present: CpuMask,
    possible: CpuMask,
}

#[allow(dead_code)] // TODO: Remove this when the CpuMask is fully integrated.
impl CpuStateManager {
    /// Creates a new instance of the CPU state manager.
    pub(super) const fn new() -> Self {
        CpuStateManager {
            boot_cpu_id: AtomicU64::new(0),
            online: CpuOnline {
                online_cnt: AtomicU64::new(0),
                online_mask: CpuMask::new(),
            },
            active: CpuMask::new(),
            present: CpuMask::new(),
            possible: CpuMask::new(),
        }
    }
    /// Initializes the CPU state manager with the given boot CPU ID.
    pub(super) fn boot_cpu_id(&self) -> usize {
        self.boot_cpu_id.load(Ordering::Relaxed) as usize
    }

    /// Creates a new CPU state manager with the specified boot CPU ID.
    pub(super) fn online_count(&self) -> u64 {
        self.online.online_cnt.load(Ordering::Relaxed)
    }

    pub(super) fn boot_init(&self, boot_cpu_id: usize) {
        // Initialize the boot CPU ID.
        self.boot_cpu_id
            .store(boot_cpu_id as u64, Ordering::Relaxed);

        // Set the boot CPU as online.
        self.online.online_mask.set(boot_cpu_id);
        self.online.online_cnt.fetch_add(1, Ordering::Relaxed);

        // Mark the boot CPU as active, present, and possible.
        self.active.set(boot_cpu_id);
        self.present.set(boot_cpu_id);
        self.possible.set(boot_cpu_id);
    }
}
