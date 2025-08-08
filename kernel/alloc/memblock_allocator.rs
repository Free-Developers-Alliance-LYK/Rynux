// SPDX-License-Identifier: GPL-2.0

//!  Mmeblock Allocator support.

use core::{
    alloc::Layout,
    ptr::NonNull,
    ptr,
};

use super::{AllocError, Allocator, AllocFlags};
use crate::mm::memblock::MemBlock;

/// Early kernel memory allocator
pub struct MemblockAllocator;

/// Returns a proper size to alloc a new object aligned to `new_layout`'s alignment.
fn aligned_size(new_layout: Layout) -> usize {
    // Customized layouts from `Layout::from_size_align()` can have size < align, so pad first.
    let layout = new_layout.pad_to_align();
    layout.size()
}

// SAFETY: 
// `memblock_alloc` is a function that allocates memory from the kernel's memblock allocator.
// It is safe to use this function as long as the caller ensures that the size and alignment
unsafe impl Allocator for MemblockAllocator {
    #[inline]
    unsafe fn alloc(
        new_layout: Layout,
        flags: AllocFlags,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let size = aligned_size(new_layout);
        let ptr = if size == 0 {
            // Zero-sized allocations are always valid
            // return a dangling pointer that is aligned to `new_layout`.
            super::dangling_from_layout(new_layout)
        } else {
            // try to allocate memory from the kernel's memblock allocator
            MemBlock::alloc(size, new_layout.align(), flags)?
        };

        Ok(NonNull::slice_from_raw_parts(ptr, size))
    }

    #[inline]
    unsafe fn realloc(
        old_ptr: NonNull<u8>,
        new_layout: Layout,
        old_layout: Layout,
        flags: AllocFlags,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let new_size = aligned_size(new_layout);
        let old_size = aligned_size(old_layout);
        // first try to alloc a new memory block with the new layout
        // if alloc fails, not free old memory block
        //
        // SAFETY: new_layout is a valid layout for allocation.
        let new_ptr = unsafe {
            Self::alloc(new_layout, flags)?
        };
        // then copy the old memory block to the new memory block
        if old_size > 0 {
            // SAFETY: `ptr` is a valid pointer to an allocation of size `old_size`
            // and `new` is a valid pointer to an allocation of size `new_layout.size()`.
            unsafe {
                ptr::copy_nonoverlapping(old_ptr.as_ptr(), new_ptr.as_ptr() as *mut u8, old_size.min(new_size));
            }
        }

        // free the old memory block
        // SAFETY: `ptr` is a valid pointer to an allocation of size `old_layout.size()`.
        unsafe {Self::free(old_ptr, old_layout);}
        // return the new memory block
        Ok(new_ptr)
    }

    #[inline]
    unsafe fn free(ptr: NonNull<u8>, layout: Layout) {
        let size = aligned_size(layout);
        if size == 0 {
            // Zero-sized allocations are always valid, no need to free
            return;
        }
        MemBlock::free(ptr, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memblock_setup() {
        // This function should set up the memblock allocator.
        // It is usually called during kernel initialization.
        // For testing purposes, we nedd manually set up the memblock allocator.

    }

    #[test]
    fn test_memblock_allocator() {
    }
}
