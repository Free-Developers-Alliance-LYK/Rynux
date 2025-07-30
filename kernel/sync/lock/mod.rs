//! Synchronisation primitives.

pub mod base;
pub mod spinlock;

pub use base::{Backend, Lock, BaseLockGuard};
pub use spinlock::{RawSpinLockIrq, RawSpinLockIrqGuard};
