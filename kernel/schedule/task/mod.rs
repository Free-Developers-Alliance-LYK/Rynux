//! Rynux task module

//pub(crate) mod current;
pub(crate) mod task;
pub(crate) mod task_stack;
pub(crate) mod task_state;
mod current;

pub use task::Task;
pub use task_stack::TaskStack;
pub use task_state::TaskState;
pub use current::CurrentTask;

use crate::sync::arc::Arc;
/// Task ref
/// In most cases, we should not use tasks directly, but references with
/// reference counting.
pub type TaskRef = Arc<Task>;
