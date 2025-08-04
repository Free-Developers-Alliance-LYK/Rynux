//! Rynux task current

use alloc::sync::Arc;
use core::mem::ManuallyDrop;

use crate::arch::thread::{ArchCurrent, Current};
use crate::schedule::task::TaskRef;

static _NO_BOOT_STATIC_TASK: OnceCell<bool> = OnceCell::new();
/// Once boot is finished, should update boot static init task to Arc
pub fn no_boot_static_task() {
    _NO_BOOT_STATIC_TASK.set(true);
}

enum CurrentTaskInner {
    Boot(&'static Task),
    Normal(TaskRef),
}

/// Current task Wrapper
pub struct CurrentTask(CurrentTaskInner);

impl CurrentTask {
    pub(crate) fn try_get() -> Option<Self> {
        let ptr: *const super::Task = Current::read();
        if !ptr.is_null() {
            if ptr == &INIT_TASK && _NO_BOOT_STATIC_TASK.get().is_none() {
                return Some(Self(CurrentTaskInner::Boot(&INIT_TASK)));
            }

            Some(Self(unsafe { ManuallyDrop::new(TaskRef::from_raw(ptr)) }))
        } else {
            None
        }
    }

    pub(crate) fn get() -> Self {
        Self::try_get().expect("current task is uninitialized")
    }

    /// Converts [`CurrentTask`] to [`TaskRef`].
    pub fn as_task_ref(&self) -> &TaskRef {
        // Boot not support
        match &self.0 {
            CurrentTaskInner::Boot(_) => panic!("boot task not support"),
            CurrentTaskInner::Normal(task) => task,
        }
    }

    pub(crate) fn clone(&self) -> TaskRef {
        // Boot static task not support clone
        match &self.0 {
            CurrentTaskInner::Boot(_) => panic!("boot task not support"),
            CurrentTaskInner::Normal(task) => task.clone(),
        }
    }

    pub(crate) fn ptr_eq(&self, other: &TaskRef) -> bool {
        match &self.0 {
            CurrentTaskInner::Boot(_) => panic!("boot task not support"),
            CurrentTaskInner::Normal(task) => Arc::ptr_eq(task, other),
        }
    }

    pub(crate) unsafe fn set_current(init_task: TaskRef) {
        let ptr = Arc::into_raw(init_task);
        axhal::cpu::set_current_task_ptr(ptr);
    }

    pub(crate) unsafe fn switch_current(prev: Self, next: TaskRef) {
        let Self(arc) = prev;
        ManuallyDrop::into_inner(arc); // `call Arc::drop()` to decrease prev task reference count.
        Self::set_current(next);
    }
}

impl Deref for CurrentTask {
    type Target = Task;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            CurrentTaskInner::Boot(task) => task,
            CurrentTaskInner::Normal(task) => &task,
        }
    }

}
