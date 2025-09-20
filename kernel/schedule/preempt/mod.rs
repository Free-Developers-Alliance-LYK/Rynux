//! Rynux preempt module
use crate::schedule::task::CurrentTask;

const PREEMPT_SHIFT: u64 = 0;
const PREEMPT_OFFSET: u64 = 1 << PREEMPT_SHIFT;

/// Task init Disable preemption until the scheduler is running.
pub const INIT_TASK_PREEMPT_COUNT: u64 = PREEMPT_OFFSET;

/// Disable preemption.
#[inline(never)]
pub fn preempt_disable() {
    let curr = CurrentTask::get();
    curr.preempt_count_add(1);
}

#[inline(never)]
/// Enable preemption.
pub fn preempt_enable() {
    let curr = CurrentTask::get();
    curr.preempt_count_sub(1);
}
