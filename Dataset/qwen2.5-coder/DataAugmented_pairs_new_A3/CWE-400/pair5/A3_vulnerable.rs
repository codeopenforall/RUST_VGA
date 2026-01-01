use std::cell::UnsafeCell;
use std::sync::Arc;

pub struct WorkQueue {
    inner: UnsafeCell<Vec<u32>>,
}

unsafe impl Sync for WorkQueue {}

impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            inner: UnsafeCell::new(Vec::new()),
        })
    }

    pub fn add_task(&self, task: u32) {
        unsafe {
            (*self.inner.get()).push(task);
        }
    }

    pub fn len(&self) -> usize {
        unsafe {
            (*self.inner.get()).len()
        }
    }

    pub fn clear(&self) {
        unsafe {
            let tasks = &mut *self.inner.get();
            tasks.clear();
        }
    }
}

fn main() {
    let queue = WorkQueue::new();
    queue.add_task(42);
    println!("Total tasks accumulated: {}", queue.len());
}