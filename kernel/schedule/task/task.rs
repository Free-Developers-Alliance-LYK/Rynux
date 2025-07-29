//! Rynux task struct

use core::ptr::NonNull;

use super::{
    TaskState,
    TaskStack,
};

use crate::macros::cache_aligned;
use crate::arch::thread::ArchThreadInfo;

/// Task struct
#[allow(dead_code)]
#[cache_aligned]
pub struct Task {
    thread_info: ArchThreadInfo,
    // state 
    state: TaskState,
    // stack
    stack: TaskStack,
}

impl Task {
    /// Create a new task
    pub const fn new(state: TaskState, stack: TaskStack) -> Self {
        Self {
            thread_info: ArchThreadInfo::default(),
            state,
            stack,
        }
    }

    /// task stack top
    #[inline(always)]
    pub fn top_of_stack(&self) -> NonNull<u8> {
        self.stack.top()
    }

    /// end stack
    #[inline(always)]
    pub fn end_of_stack(&self) -> NonNull<u8> {
        self.stack.end()
    }

    /// set task stack to magic
    #[inline(always)]
    pub fn set_stack_end_magic(&self) {
        self.stack.set_stack_end_magic();
    }

    /// Zero stack
    #[inline(always)]
    pub fn zero_stack(&self) {
        self.stack.zeroed();
    }

    /// thread info
    #[inline(always)]
    pub fn thread_info(&self) -> &ArchThreadInfo {
        &self.thread_info
    }
}


unsafe impl Send for Task {}
unsafe impl Sync for Task {}
