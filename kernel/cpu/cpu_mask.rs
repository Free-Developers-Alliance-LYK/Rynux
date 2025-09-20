//! Cpus mask

use crate::arch::cpu::MAX_CPUS;
use core::sync::atomic::{AtomicU64, Ordering};

const WORD_SHIFT: usize = 6; // 64 = 1 << 6
const WORD_MASK: usize = (1 << WORD_SHIFT) - 1;
const WORDS: usize = (MAX_CPUS + WORD_MASK) >> WORD_SHIFT;

/// A mask for managing CPU availability.
pub struct CpuMask {
    mask: [AtomicU64; WORDS],
}

#[allow(dead_code)] // TODO: Remove this when the CpuMask is fully integrated.
impl CpuMask {
    /// Creates a new CpuMask with all CPUs initially set to 0.
    pub const fn new() -> Self {
        CpuMask {
            mask: [AtomicU64::new(0); WORDS],
        }
    }

    #[inline]
    /// Sets the CPU at the given index in the mask.
    pub fn set(&self, cpu: usize) {
        debug_assert!(cpu < MAX_CPUS);
        let index = cpu >> WORD_SHIFT;
        let bit = cpu & WORD_MASK;
        self.mask[index].fetch_or(1u64 << bit, Ordering::Relaxed);
    }

    #[inline]
    /// Clears the CPU at the given index in the mask.
    pub fn clear(&self, cpu: usize) {
        debug_assert!(cpu < MAX_CPUS);
        let index = cpu >> WORD_SHIFT;
        let bit = cpu & WORD_MASK;
        self.mask[index].fetch_and(!(1u64 << bit), Ordering::Relaxed);
    }

    #[inline]
    /// Checks if the CPU at the given index in the mask is set.
    pub fn is_set(&self, cpu: usize) -> bool {
        debug_assert!(cpu < MAX_CPUS);
        let index = cpu >> WORD_SHIFT;
        let bit = cpu & WORD_MASK;
        (self.mask[index].load(Ordering::Relaxed) & (1u64 << bit)) != 0
    }

    #[inline]
    /// Tests and sets the CPU at the given index in the mask.
    pub fn test_and_set(&self, cpu: usize) -> bool {
        debug_assert!(cpu < MAX_CPUS);
        let index = cpu >> WORD_SHIFT;
        let bit = cpu & WORD_MASK;
        let mask = 1u64 << bit;
        self.mask[index].fetch_or(mask, Ordering::Relaxed) & mask != 0
    }

    #[inline]
    /// Tests and clears the CPU at the given index in the mask.
    pub fn test_and_clear(&self, cpu: usize) -> bool {
        debug_assert!(cpu < MAX_CPUS);
        let index = cpu >> WORD_SHIFT;
        let bit = cpu & WORD_MASK;
        let mask = 1u64 << bit;
        self.mask[index].fetch_and(!mask, Ordering::Relaxed) & mask != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_mask() {
        let mask = CpuMask::new();
        assert!(!mask.is_set(0));
        mask.set(0);
        assert!(mask.is_set(0));
        mask.clear(0);
        assert!(!mask.is_set(0));

        for i in 0..MAX_CPUS {
            mask.set(i);
            assert!(mask.is_set(i));
            mask.clear(i);
            assert!(!mask.is_set(i));
        }
    }

    #[test]
    fn test_cpumask_test_and_set() {
        let mask = CpuMask::new();
        for i in 0..MAX_CPUS {
            assert!(
                !mask.test_and_set(i),
                "CPU {} should not be set initially",
                i
            );
            assert!(mask.is_set(i), "CPU {} should be set after test_and_set", i);
        }

        for i in 0..MAX_CPUS {
            assert!(
                mask.test_and_clear(i),
                "CPU {} should be set before test_and_clear",
                i
            );
            assert!(
                !mask.is_set(i),
                "CPU {} should not be set after test_and_clear",
                i
            );
        }
    }
}
