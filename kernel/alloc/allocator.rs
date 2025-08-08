//! Allocator support

use core::alloc::Layout;
use core::ptr::NonNull;
use crate::bitflags::bitflags;

bitflags! {
    /// Flags for memory allocation in the kernel.
    ///
    /// These flags are used to control the behavior of memory allocation functions.
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub struct AllocFlags: u32 {
        /// Movable BIT: indicates that the page can be moved by page migration
        /// during memory compaction or can be reclaimed. 
        const MOVABLE = 0b1 << 0;
        /// Reclaimable BIT: indicates that the page can be reclaimed
        const RECLAIMABLE = 0b1 << 1;
    }
}

/// Error type for memory allocation failures.
pub enum AllocError {
    /// No enough memory available for allocation.
    NoMemory,
    /// Invalid Align
    InvalidAlign,
    /// Invalid Size
    InvalidSize,
    /// Indicates that the allocation failed for some other reason.
    Other,
}

impl AllocError {
    /// Returns a string representation of the error.
    pub fn to_string(&self) -> &'static str {
        match self {
            AllocError::NoMemory => "No memory available for allocation",
            AllocError::InvalidSize => "Invalid size for allocation",
            AllocError::InvalidAlign => "Invalid alignment",
            AllocError::Other => "Other allocation error",
        }
    }
}

/// The kernel's [`Allocator`] trait.
///
/// An implementation of [`Allocator`] can allocate, re-allocate and free memory buffers described
/// via [`Layout`].
///
/// [`Allocator`] is designed to be implemented as a ZST; [`Allocator`] functions do not operate on
/// an object instance.
///
/// In order to be able to support `#[derive(CoercePointee)]` later on, we need to avoid a design
/// that requires an `Allocator` to be instantiated, hence its functions must not contain any kind
/// of `self` parameter.
///
/// # Safety
///
/// - A memory allocation returned from an allocator must remain valid until it is explicitly freed.
///
/// - Any pointer to a valid memory allocation must be valid to be passed to any other [`Allocator`]
///   function of the same type.
///
/// - Implementers must ensure that all trait functions abide by the guarantees documented in the
///   `# Guarantees` sections.
pub unsafe trait Allocator {
    /// Allocate memory based on `layout` and `flags`.
    ///
    /// On success, returns a buffer represented as `NonNull<[u8]>` that satisfies the layout
    /// constraints (i.e. minimum size and alignment as specified by `layout`).
    ///
    /// # Guarantees
    ///
    /// When the return value is `Ok(ptr)`, then `ptr` is
    /// - valid for reads and writes for `layout.size()` bytes, until it is passed to
    ///   [`Allocator::free`] or [`Allocator::realloc`],
    /// - aligned to `layout.align()`,
    ///
    unsafe fn alloc(new_layout: Layout, flags: AllocFlags) -> Result<NonNull<[u8]>, AllocError>; 

    /// For kernel allocations that should not stall for direct reclaim, start physical IO or
    /// use any filesystem callback.  It is very likely to fail to allocate memory, even for very
    /// to `realloc` guarantees that the new or grown buffer has at least `Layout::size` bytes, but
    /// may also be larger.                                                
    ///                                                                    
    /// If the requested size is smaller than the size of the existing allocation, `realloc` may or
    /// may not shrink the buffer; this is implementation specific to the allocator.
    ///                                                                    
    /// On allocation failure, the existing buffer, if any, remains valid. 
    ///                                                                    
    /// The buffer is represented as `NonNull<[u8]>`.                      
    ///                                                                    
    unsafe fn realloc(
        ptr: NonNull<u8>,
        layout: Layout,
        old_layout: Layout,
        flags: AllocFlags,
      ) -> Result<NonNull<[u8]>, AllocError>;

    /// Free an existing memory allocation.
    ///
    /// # Safety
    ///
    /// - `ptr` must point to a valid memory allocation created by this [`Allocator`].
    ///   if `old_layout` is zero-sized, `ptr` does not need to be a pointer returned by this
    ///   [`Allocator`].
    /// - `layout` must match the `Layout` the allocation has been created with.
    /// - The memory pointed to by `ptr` must not be accessed after this function returns.
    unsafe fn free(ptr: NonNull<u8>, layout: Layout);
}

/// Returns a properly aligned dangling pointer from the given `layout`.
pub fn dangling_from_layout(layout: Layout) -> NonNull<u8> {
    let ptr = layout.align() as *mut u8;
    // SAFETY: `layout.align()` (and hence `ptr`) is guaranteed to be non-zero.
    unsafe { NonNull::new_unchecked(ptr) }
}
