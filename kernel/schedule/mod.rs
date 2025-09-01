//! Rynux schedule module

pub mod task;
pub mod preempt;

mod wait_list;

pub use task::CurrentTask;
pub use wait_list::{WaitTaskList, WaitTaskNode, WaitQueue};


/// Get current task
pub fn current() -> CurrentTask {
    CurrentTask::get()
}
