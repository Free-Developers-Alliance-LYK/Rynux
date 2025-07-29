//! Per cpu

use crate::arch::cpu::MAX_CPUS;
use crate::macros::section_read_mostly;

/// Per cpu offset
#[section_read_mostly]
static __PER_CPU_OFFSET: [usize; MAX_CPUS] = [0; MAX_CPUS];

/// From cpu id Get per cpu offset
#[inline(always)]
pub fn get_per_cpu_offset(cpu: u32) -> usize {
    __PER_CPU_OFFSET[cpu as usize]
}
