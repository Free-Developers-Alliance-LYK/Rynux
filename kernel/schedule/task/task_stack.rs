//! Rynux task stack

use core::ptr::NonNull;
use core::alloc::Layout;

/// Task stack
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub struct TaskStack {
    ptr: NonNull<u8>,
    layout: Layout,
    is_static: bool,
}

impl TaskStack {
    /// Create a new task stack
    pub const fn new(ptr: NonNull<u8>, layout: Layout, is_static: bool) -> Self {
        Self {
            ptr,
            layout,
            is_static,
        }
    }

    /// Get top stack
    #[inline(always)]
    pub const fn top(&self) -> NonNull<u8> {
         // SAFETY: stack size is include in layout
         unsafe {
             core::mem::transmute(self.ptr.as_ptr().add(self.layout.size()))
         }
    }
}

unsafe impl Sync for TaskStack {}
unsafe impl Send for TaskStack {}

/*
impl Drop for TaskStack {
    fn drop(&mut self) {
        if !self.is_static {
        }
    }
}
*/
