///! Rynux task state
use crate::bitflags::bitflags;

bitflags! {
    /// Task state
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub struct TaskState: u32 {
        // Used inn tsk->state
        /// Task is running
        const RUNNING = 1 << 0;
        /// Task is interruptible
        const INTERRUPTIBLE = 1 << 1;
        /// Task is uninterruptible
        const UNINTERRUPTIBLE = 1 << 2;
        /// Task is STOPPED
        const STOPPED = 1 << 3;
        /// Task is TRACED
        const TRACED = 1 << 4;

        // Used in tsk->exit_state
        /// Task is EXITING Dead
        const EXITING_DEAD = 1 << 5;
        /// Task is EXITING Zombie
        const EXITING_ZOMBIE = 1 << 6;

        // Used in tsk->state again
        /// Task is PARKED
        const PARKED = 1 << 7;
        /// Task is DEAD
        const DEAD = 1 << 8;
        /// Task is WAKEKILL
        const WAKEKILL = 1 << 9;
        /// Task is WAKING
        const WAKING = 1 << 10;
        /// Task is NOLOAD
        const NOLOAD = 1 << 11;
        /// Task is NEW
        const NEW = 1 << 12;
    }
}
