//! Wait queue 
//! 
//! allow waiter use stack mem

use crate::list::{GetLinks, Links, RawList};
use crate::sync::lock::spinlock::RawSpinLockNoIrq;
use crate::schedule::task::{TaskRef, TaskState ,set_current_state};

/// Wait queue Node
#[allow(dead_code)]
pub struct WaitTaskNode {
    links: Links<Self>,
    task: TaskRef,
}

impl WaitTaskNode {
    /// Create a new wait node
    pub const fn new(task: TaskRef) -> Self {
        Self {
            links: Links::new(),
            task,
        }
    }
}

impl GetLinks for WaitTaskNode { 
    type EntryType = Self;
    fn get_links(data: &Self) -> &Links<Self> {
        &data.links
    }
}

/// Wait queue 
///
/// This wait queue support user push and pop with reference of node
/// and won't own the node ownership.
///
/// # Safety
/// On a schdule wait context, a `WaitTaskNode` should always declare as a
/// stack variable and constructed with current task, and once it has been added
/// to a wait queue, it must be keep alive until it is removed from the
/// wait queue. 
///
/// We design this wait queue use a stack variable but not a heap allocated
/// variable to avoid heap allocation.  
/// No need to worry about the circular dependencies between memory allocation
/// and synchronization mechanisms(mutex, spinlock, etc.)
/// 
pub struct WaitTaskList {
    list: RawList::<WaitTaskNode>,
}

impl WaitTaskList {
    /// Create a new wait queue
    pub const fn new() -> Self {
        Self {
            list: RawList::new(),
        }
    }

    /// Is wait queue empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    /// push back wait node
    ///
    /// Safety: caller must make sure node lifetime is longer than the list
    #[inline]
    pub unsafe fn push_back(&mut self, node: &WaitTaskNode) {
        unsafe {
            self.list.push_back(node);
        }
    }

    /// Remove a node to the wait queue
    ///
    /// Safety: caller must make sure node lifetime is longer than the list
    /// and node is on the list
    #[inline]
    pub unsafe fn remove(&mut self, node: &WaitTaskNode) -> bool {
        unsafe {self.list.remove(node)}
    }

    /// Pop front wait node
    #[inline]
    pub fn pop_front(&mut self) -> Option<&WaitTaskNode> {
        self.list.pop_front().map(|node| unsafe { &*node.as_ptr() })
    }

    /// First entry
    #[inline]
    pub fn front(&self) -> Option<&WaitTaskNode> {
        self.list.front().map(|node| unsafe { &*node.as_ptr() })
    }

    /// Front node equal
    #[inline]
    pub fn front_eq(&self, node: &WaitTaskNode) -> bool {
        self.front().map(|n| core::ptr::eq(n, node)).unwrap_or(false)
    }

}

/// Declare a waiter with current task
///
/// ```rust
/// declare_waiter!(waiter);
/// ```
///
#[macro_export]
macro_rules! declare_waiter {
    ($name: ident) => {
        let $name: $crate::schedule::wait_list::WaitTaskNode = $crate::schedule::WaitTaskNode::new($crate::schedule::current().as_task_ref().clone());

    };
}

/// A queue to store tasks that are waiting for some conditions.
pub struct WaitQueue {
    queue: RawSpinLockNoIrq<WaitTaskList>,
}

impl WaitQueue {
    /// Create a new wait queue
    pub const fn new() -> Self {
        Self {
            queue: RawSpinLockNoIrq::new(WaitTaskList::new(), None),
        }
    }

    /// Wait until notified
    pub fn wait(&self, state: TaskState) {
        declare_waiter!(waiter);
        let mut queue = self.queue.lock();

        set_current_state(state);
        // Safety: Once waiter is created, it will not be dropped
        // before we return from this function(the waiter is removed
        // from the list)
        unsafe {queue.push_back(&waiter);}
        drop(queue);

        // TODO: call schedule
        todo!();

        /*
        // Maybe wakeup by signal or other reasons, so need to remove it from the list
        let mut queue = self.queue.lock();
        set_current_state(TaskState::RUNNING);
        // Safety: Once waiter is created, it will not be dropped,
        // we active remove it from list make sure it is safe
        unsafe {
            queue.remove(&waiter);
        }
        drop(queue);
        */
    }

    
    /// Wait until condition is met
    pub fn wait_until<F>(&self, state: TaskState, condition: F)
    where
        F: Fn() -> bool,
    {
        declare_waiter!(waiter);
        loop {
            if condition() {
                break;
            }
            let mut queue = self.queue.lock();
            set_current_state(state);
            // Safety: Once waiter is created, it will not be dropped
            // before we return from this function(the waiter is removed
            // from the list)
            unsafe {queue.push_back(&waiter);}
            drop(queue);
            // TODO: call schedule
            todo!();
        }

        let mut queue = self.queue.lock();
        set_current_state(TaskState::RUNNING);
        unsafe {queue.remove(&waiter);}
        drop(queue);
    }


    /// Notify one waiter
    ///
    /// This may cause concurrency with other wake-up processes, such as
    /// passive timeout (wake-up from timerlist) and active notify
    ///
    /// task
    /// wait.lock.push()
    /// state = UN/INTERRUPTIBLE
    /// schedule next   
    ///                            timeout                        notify one 
    ///                            timer_list.lock.get()        wait.lock.get()
    ///                            try_wakeup()                     try_wakeup()          
    ///
    ///
    ///
    ///
    pub fn notify_one(&self) -> bool {
        let mut queue = self.queue.lock();
        if let Some(_node) = queue.pop_front() {
            // TODO: call wakeup
            todo!();
            //return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr::NonNull;
    use std::alloc::Layout;
    use crate::sync::arc::Arc;
    use crate::schedule::task::{Task, TaskStack, TaskState};

    fn new_task() -> Task {
        Task::new(TaskState::RUNNING, TaskStack::new(NonNull::new(0xf as *mut u8).unwrap(), Layout::new::<u8>(), false))
    }
    #[test]
    fn test_wait_list() {
        let task = new_task();
        let mut list = WaitTaskList::new();
        assert!(list.is_empty());

        let waiter = WaitTaskNode::new(Arc::new(task));
        // Safety: waiter life time is longer than the list
        unsafe {
            list.push_back(&waiter);
            assert!(!list.is_empty());
            list.remove(&waiter);
            assert!(list.is_empty());

            list.push_back(&waiter);
            assert!(!list.is_empty());
            let node = list.pop_front();
            assert!(core::ptr::eq(node.unwrap(), &waiter));
            assert!(list.is_empty());
    
            list.push_back(&waiter);
            assert!(!list.is_empty());
            assert!(list.front_eq(&waiter));
        }
    }

    #[test]
    fn test_double_remove_is_ok() {
        let task = new_task();
        let mut list = WaitTaskList::new();
        assert!(list.is_empty());
        let waiter = WaitTaskNode::new(Arc::new(task));
        unsafe {
            list.push_back(&waiter);
            assert!(list.remove(&waiter));
            assert!(list.is_empty());
            assert!(!list.remove(&waiter));
        }
    }
}
