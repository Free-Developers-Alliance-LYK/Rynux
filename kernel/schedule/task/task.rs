//! Rynux task struct

use core::mem::ManuallyDrop;
use core::ptr::NonNull;

use super::{TaskStack, TaskState};
use crate::arch::thread::{ArchThreadInfo, ArchThreadInfoTrait};
use crate::macros::cache_aligned;
use crate::sync::lock::spinlock::{RawSpinLockNoIrq, RawSpinLockNoIrqGuard};

/// Task struct
#[allow(dead_code)]
#[cache_aligned]
pub struct Task {
    thread_info: ArchThreadInfo,
    // state
    state: RawSpinLockNoIrq<TaskState>,
    // stack
    stack: TaskStack,
    // boot task?
    is_boot_task: bool,
    /// magic number
    pub magic: u64,
}

impl Task {
    /// Boot task magic
    pub const BOOT_TASK_MAGIC: u64 = 0x12345678;
    /// Create a new task
    pub fn new(state: TaskState, stack: TaskStack) -> Self {
        Self {
            thread_info: ArchThreadInfo::default(),
            state: RawSpinLockNoIrq::new(state, None),
            stack,
            is_boot_task: false,
            magic: 0,
        }
    }

    /// Create boot task
    pub(crate) const fn new_boot(stack: TaskStack) -> Self {
        Self {
            thread_info: ArchThreadInfo::default(),
            state: RawSpinLockNoIrq::new(TaskState::RUNNING, None),
            stack,
            is_boot_task: true,
            magic: Self::BOOT_TASK_MAGIC,
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

    /// add preempt count
    #[inline(always)]
    pub fn preempt_count_add(&self, count: u32) {
        self.thread_info.preempt_count_add(count);
    }

    /// add preempt count
    #[inline(always)]
    pub fn preempt_count_sub(&self, count: u32) {
        self.thread_info.preempt_count_sub(count);
    }

    #[inline(always)]
    /// preempt count
    pub fn preempt_count(&self) -> u32 {
        self.thread_info.preempt_count()
    }

    #[inline(always)]
    /// lock state and return a manually drop guard
    pub fn lock_state_manual(&self) -> ManuallyDrop<RawSpinLockNoIrqGuard<'_, TaskState>> {
        ManuallyDrop::new(self.state.lock())
    }

    #[inline(always)]
    /// set state
    pub fn set_state(&self, state: TaskState) {
        *self.state.lock() = state
    }
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}
