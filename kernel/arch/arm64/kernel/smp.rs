//! Arm64 smp 

use crate::arch::arm64::sysregs::MpidrEl1;
use crate::arch::cpu::MAX_CPUS;
use crate::sync::lock::RawSpinLockIrq;

/// Cpu logical map
pub struct CpuLogicalMap {
    map: [u64; MAX_CPUS],
    main_cpu_hwid: u64,
    cpu_count: usize,
}

#[allow(dead_code)]
impl CpuLogicalMap {
    const fn new() -> Self {
        Self {
            map: [MpidrEl1::INVALID_HWID; MAX_CPUS],
            main_cpu_hwid: MpidrEl1::INVALID_HWID,
            cpu_count: 1,
        }
    }

    /// Duplicate MPIDRs are a recipe for disaster. Scan all initialized
    /// entries and check for duplicates. If any is found just ignore the
    /// cpu. cpu_logical_map was initialized to INVALID_HWID to avoid
    /// matching valid MPIDR values.
    fn check_duplicate(&self, cpu: usize, hwid: u64) -> bool {
        for i in 0..cpu {
            if self.map[i] == hwid {
                return true;
            }
        }
        false
    }

    /// Get cpu logical map
    pub fn get(&self, cpu: usize) -> u64 {
        self.map[cpu]
    }

    /// Set cpu logical map
    pub fn set(&mut self, cpu: usize, hwid: u64) {
        assert!(!self.check_duplicate(cpu, hwid));
        if hwid == self.main_cpu_hwid {
            self.cpu_count += 1;
        }
        self.map[cpu] = hwid;
    }

    /// Set main cpu hwid
    pub fn set_main_cpu_hwid(&mut self, hwid: u64) {
        self.main_cpu_hwid = hwid;
    }

}

/// This variable does not actually have concurrency and is only processed once
/// in the boot process, but using RawSpinLockIrq ensures that it can be compiled.
pub static __CPU_LOGICAL_MAP: RawSpinLockIrq<CpuLogicalMap> = RawSpinLockIrq::new(CpuLogicalMap::new(), None);

#[allow(dead_code)]
/// write CPU map
fn set_logical_map(cpu: usize, hwid: u64) {
    let mut map = __CPU_LOGICAL_MAP.lock();
    map.set(cpu, hwid);
}

/// Write main cpu hwid
pub fn set_main_cpu_hwid(hwid: u64) {
    let mut map = __CPU_LOGICAL_MAP.lock();
    map.set_main_cpu_hwid(hwid);
}
