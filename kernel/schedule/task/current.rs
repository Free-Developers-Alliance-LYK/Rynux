//! Rynux task current

use core::mem::ManuallyDrop;
use core::ops::Deref;

use crate::sync::arc::Arc;
use crate::arch::thread::{ArchCurrentTrait, ArchCurrent};
use crate::schedule::task::{Task, TaskRef};

/// Current task Wrapper
pub struct CurrentTask(ManuallyDrop<TaskRef>);

impl CurrentTask {
    fn try_get() -> Option<Self> {
        let ptr: *const Task = ArchCurrent::read();
        if !ptr.is_null() {
            Some(Self(ManuallyDrop::new(unsafe { Arc::from_raw(ptr) })))
        } else {
            None
        }
    }

    /// Get current task.
    #[inline(always)]
    pub fn get() -> Self {
        Self::try_get().expect("current task is uninitialized")
    }

    /// Converts [`CurrentTask`] to [`TaskRef`].
    pub fn as_task_ref(&self) -> &TaskRef {
        &self.0
    }

    /// as ptr
    #[inline(always)]
    pub fn as_ptr(&self) -> *const Task {
        Arc::<Task>::as_ptr(&self.0)
    }

    /// Clone current task ref.
    #[inline(always)]
    pub fn clone(&self) -> TaskRef {
        self.0.deref().clone()
    }

    /// Compare whether two [`CurrentTask`] pointers reference the same underlying object.
    #[inline(always)]
    pub fn ptr_eq(&self, other: &TaskRef) -> bool {
        Arc::ptr_eq(&self.0, other)
    }

    /// Set current task.
    #[inline(always)]
    pub fn set_current(task: TaskRef) {
        let ptr = Arc::into_raw(task);
        ArchCurrent::write(ptr);
    }

    /// Switch current task.
    #[inline(always)]
    pub fn switch_current(prev: Self, next: TaskRef) {
        let Self(arc) = prev;
        ManuallyDrop::into_inner(arc); // `call Arc::drop()` to decrease prev task reference count.
        //SAFETY: next is a valid task ref.
        Self::set_current(next);
    }
}

impl Deref for CurrentTask {
    type Target = Task;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
