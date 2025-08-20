//! kernel alloc module


pub mod kbox;
mod allocator;

pub use allocator::{AllocFlags, AllocError, Allocator, dangling_from_layout};

use crate::cfg_if;
cfg_if! {
    if #[cfg(test)] {
        mod allocator_test;
        pub use allocator_test::Cmalloc;
        pub type MemblockAllocator = Cmalloc;
    } else {
        mod memblock_allocator;
        pub use memblock_allocator::MemblockAllocator;
    }
}
