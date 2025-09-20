//! kernel alloc module

mod allocator;
pub mod kbox;

pub use allocator::{dangling_from_layout, AllocError, AllocFlags, Allocator};

cfg_if::cfg_if! {
    if #[cfg(test)] {
        mod allocator_test;
        pub use allocator_test::Cmalloc;
        pub type MemblockAllocator = Cmalloc;
    } else {
        mod memblock_allocator;
        pub use memblock_allocator::MemblockAllocator;
    }
}
