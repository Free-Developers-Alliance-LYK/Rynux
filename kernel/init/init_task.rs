//! Rynux init task

use core::ptr::NonNull;
use core::alloc::Layout;

use crate::{
    schedule::task::{
        Task,
        TaskStack,
        TaskRef,
    },
    arch::mm::ArchThreadMemLayout,
    sync::arc::{Arc,ArcInner},
    global_sym::init_stack,
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

/// SAFETY: we know what we are doing here. 
/// we use a satic mem to init Arc, if this init Arc refcont to 0, it will panic. 
static INIT_TASK_ARC: ArcInner<Task> = ArcInner::new_static(Task::new_boot(INIT_TASK_STACK));
/// SAFETY: we know what we are doing here. 
/// we use a satic mem to init Arc, if this init Arc refcont to 0, it will panic. 
pub static INIT_TASK_REF: TaskRef = unsafe {Arc::from_static(&INIT_TASK_ARC)}; 
