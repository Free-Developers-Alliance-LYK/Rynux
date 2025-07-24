//! Rynux init task

use core::ptr::NonNull;
use core::alloc::Layout;

use crate::{
    schedule::task::{
        Task,
        TaskState,
        TaskStack,
    },
    arch::mm::ArchThreadMemLayout,
};

static INIT_TASK_STACK: TaskStack = TaskStack::new(
    NonNull::new(init_stack as *mut u8).unwrap(),
    unsafe {
        // SAFETY:
        // init_stack size is defined in vmrynux.rs, size is THREAD_SIZE
        // and aligned with THREAD_ALIGN
        Layout::from_size_align_unchecked(ArchThreadMemLayout::THREAD_SIZE,
        ArchThreadMemLayout::THREAD_ALIGN)
    },
    true
);

/// Init First task
pub static INIT_TASK: Task = Task::new(
    TaskState::RUNNING,
    INIT_TASK_STACK,
);

extern "C" {
    /// init_stack define in vmrynux.rs
    pub fn init_stack();
}
