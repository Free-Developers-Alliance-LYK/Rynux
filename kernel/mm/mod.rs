//! Memory management code.

pub mod page;
mod addr;
pub mod percpu;
pub mod memblock;

pub use addr::{PhysAddr, VirtAddr};
