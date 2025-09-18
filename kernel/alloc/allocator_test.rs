// SPDX-License-Identifier: GPL-2.0

//! So far the kernel's `Box` and `Vec` types can't be used by userspace test cases, since all users
//! of those types (e.g. `CString`) use kernel allocators for instantiation.
//!
//! In order to allow userspace test cases to make use of such types as well, implement the
//! `Cmalloc` allocator within the `allocator_test` module and type alias all kernel allocators to
//! `Cmalloc`. The `Cmalloc` allocator uses libc's `realloc()` function as allocator backend.

#![allow(missing_docs)]

use super::{AllocError, Allocator, AllocFlags};
use core::alloc::Layout;
use core::cmp;
use core::ptr;
use core::ptr::NonNull;

/// The userspace allocator based on libc.
pub struct Cmalloc;

pub type Kmalloc = Cmalloc;
pub type Vmalloc = Kmalloc;
pub type KVmalloc = Kmalloc;

unsafe extern "C" {
    #[link_name = "aligned_alloc"]
    fn libc_aligned_alloc(align: usize, size: usize) -> *mut std::ffi::c_void;

    #[link_name = "free"]
    fn libc_free(ptr: *mut std::ffi::c_void);
}

/// Returns a proper size to alloc a new object aligned to `new_layout`'s alignment.
fn aligned_layout(layout: Layout) -> Result<Layout, AllocError> {
     // ISO C (ISO/IEC 9899:2011) defines `aligned_alloc`:
     //
     // > The value of alignment shall be a valid alignment supported by the implementation
     // [...].
     //
     // As an example of the "supported by the implementation" requirement, POSIX.1-2001 (IEEE
     // 1003.1-2001) defines `posix_memalign`:
     //
     // > The value of alignment shall be a power of two multiple of sizeof (void *).
     //
     // and POSIX-based implementations of `aligned_alloc` inherit this requirement. At the time
     // of writing, this is known to be the case on macOS (but not in glibc).
     //
     // Satisfy the stricter requirement to avoid spurious test failures on some platforms.
     let min_align = core::mem::size_of::<*const std::ffi::c_void>();
     let layout = layout.align_to(min_align).map_err(|_| AllocError::InvalidAlign)?;
     let layout = layout.pad_to_align();
     Ok(layout)
}


// SAFETY:
// - memory remains valid until it is explicitly freed,
// - passing a pointer to a valid memory allocation created by this `Allocator` is always OK,
// - `realloc` provides the guarantees as provided in the `# Guarantees` section.
unsafe impl Allocator for Cmalloc {
    fn alloc(layout: Layout, flags: AllocFlags) -> Result<NonNull<[u8]>, AllocError> {
        let layout = aligned_layout(layout)?;
        let size = layout.size();
        let ptr = if size == 0 {
            // Zero-sized allocations are always valid
            // return a dangling pointer that is aligned to `new_layout`.
            super::dangling_from_layout(layout)
        } else {
            // exceeds the given size and alignment requirements.
            let dst = unsafe { libc_aligned_alloc(layout.align(), layout.size()) } as *mut u8;
            let dst = NonNull::new(dst).ok_or(AllocError::NoMemory)?;
            if flags.contains(AllocFlags::ZERO) {
                // SAFETY: The preceding calls to `libc_aligned_alloc` and `NonNull::new`
                // guarantee that `dst` points to memory of at least `layout.size()` bytes.
                unsafe { dst.as_ptr().write_bytes(0, layout.size()) };
            }
            dst
        };
        Ok(NonNull::slice_from_raw_parts(ptr, size))

    }

    unsafe fn free(ptr: NonNull<u8>, _layout: Layout) {
        unsafe { libc_free(ptr.as_ptr().cast())};
    }

    unsafe fn realloc(
        ptr: NonNull<u8>,
        layout: Layout,
        old_layout: Layout,
        flags: AllocFlags,
    ) -> Result<NonNull<[u8]>, AllocError> {
        let old_layout = aligned_layout(old_layout)?;
        let new_layout = aligned_layout(layout)?;
        // first try to alloc a new memory block with the new layout
        // if alloc fails, not free old memory block
        //
        // SAFETY: new_layout is a valid layout for allocation.
        let new_ptr = Self::alloc(new_layout, flags)?;
        // then copy the old memory block to the new memory block
        if old_layout.size() > 0 {
            // SAFETY: `ptr` is a valid pointer to an allocation of size `old_size`
            // and `new` is a valid pointer to an allocation of size `new_layout.size()`.
            unsafe {
                ptr::copy_nonoverlapping(ptr.as_ptr(), new_ptr.as_ptr() as *mut u8, old_layout.size().min(new_layout.size()));
            }
        }

        // free the old memory block
        // SAFETY: `ptr` is a valid pointer to an allocation of size `old_layout.size()`.
        unsafe {Self::free(ptr, old_layout);}
        // return the new memory block
        Ok(new_ptr)
    }
}
