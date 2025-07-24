//! Rynux task struct

use core::ptr::NonNull;

use super::{
    TaskState,
    TaskStack,
};

use crate::macros::cache_aligned;

/// Task struct
#[allow(dead_code)]
#[cache_aligned]
pub struct Task {
    // state 
    state: TaskState,
    // stack
    stack: TaskStack,
}

impl Task {
    /// Create a new task
    pub const fn new(state: TaskState, stack: TaskStack) -> Self {
        Self {
            state,
            stack,
        }
    }

    /// Get task stack top
    #[inline(always)]
    pub fn stack_top(&self) -> NonNull<u8> {
        self.stack.top()
    }
}


unsafe impl Send for Task {}
unsafe impl Sync for Task {}
