//! Memory management code.

mod addr;
pub mod memblock;
pub mod page;
pub mod percpu;

pub use addr::{PhysAddr, VirtAddr};
