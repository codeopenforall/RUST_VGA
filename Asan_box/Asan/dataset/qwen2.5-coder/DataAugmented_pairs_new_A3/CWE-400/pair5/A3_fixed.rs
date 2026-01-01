use std::sync::{Arc, Mutex};

pub struct WorkQueue {
    inner: Mutex<Vec<u32>>,
    max_tasks: usize,
}

impl WorkQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(WorkQueue {
            inner: Mutex::new(Vec::new()),
            max_tasks: 10000,
        })
    }

    pub fn add_task(&self, task: u32) {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        if guard.len() < self.max_tasks {
            guard.push(task);
        }
    }

    pub fn len(&self) -> usize {
        let guard = self.inner.lock().expect("Mutex poisoned");
        guard.len()
    }

    pub fn clear(&self) {
        let mut guard = self.inner.lock().expect("Mutex poisoned");
        guard.clear();
    }
}