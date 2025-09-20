//! Synchronisation primitives.

pub mod base;
pub mod mutex;
pub mod qspinlock;
pub mod spinlock;

pub use base::{Backend, BaseLockGuard, Lock};
pub use mutex::{Mutex, MutexGuard};
pub use spinlock::{RawSpinLockNoIrq, RawSpinLockNoIrqGuard};
