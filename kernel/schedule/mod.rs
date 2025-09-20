//! Rynux schedule module

pub mod preempt;
pub mod task;

mod wait_list;

pub use task::CurrentTask;
pub use wait_list::{WaitQueue, WaitTaskList, WaitTaskNode};

/// Get current task
pub fn current() -> CurrentTask {
    CurrentTask::get()
}
