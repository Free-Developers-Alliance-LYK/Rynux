//! Rynux task module

//pub(crate) mod current;
mod current;
pub(crate) mod task;
pub(crate) mod task_stack;
pub(crate) mod task_state;

pub use current::CurrentTask;
pub use task::Task;
pub use task_stack::TaskStack;
pub use task_state::TaskState;

use crate::sync::arc::Arc;

/// Task ref
/// In most cases, we should not use tasks directly, but references with
/// reference counting.
pub type TaskRef = Arc<Task>;

/// Set current task state
pub fn set_current_state(state: TaskState) {
    CurrentTask::get().set_state(state);
}
