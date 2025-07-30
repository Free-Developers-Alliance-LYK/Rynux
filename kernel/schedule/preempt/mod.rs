//! Rynux preempt module

const PREEMPT_SHIFT: u64 = 0;
const PREEMPT_OFFSET: u64 = 1 << PREEMPT_SHIFT;
/// Task init Disable preemption until the scheduler is running.
pub const INIT_TASK_PREEMPT_COUNT: u64 = PREEMPT_OFFSET;



/// Disable preemption.
pub fn preempt_disable() {
    //todo
}

/// Enable preemption.
pub fn preempt_enable() {
    //todo
}
