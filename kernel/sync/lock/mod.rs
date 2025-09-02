//! Synchronisation primitives.

pub mod base;
pub mod spinlock;
pub mod mutex;
pub mod qspinlock;

pub use base::{Backend, Lock, BaseLockGuard};
pub use spinlock::{RawSpinLockNoIrq, RawSpinLockNoIrqGuard};
pub use mutex::{Mutex, MutexGuard};
