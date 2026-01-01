use std::sync::{Arc, Mutex};

pub struct DataHolder {
    data: Mutex<Vec<u32>>,
}

impl DataHolder {
    pub fn new(size: usize) -> Self {
        DataHolder { data: Mutex::new(vec![0; size]) }
    }

    pub fn get(&self, idx: usize) -> u32 {
        let guard = self.data.lock().expect("Lock poisoned");
        guard.get(idx).copied().unwrap_or(0)
    }

    pub fn set(&self, idx: usize, value: u32) {
        let mut guard = self.data.lock().expect("Lock poisoned");
        guard[idx] = value;
    }
}

pub fn critical_run() {
    let holder = Arc::new(DataHolder::new(10));
    let holder_clone = Arc::clone(&holder);

    // Simulate some operations
    holder.set(5, 42);

    // Check for invariant violation
    let value = holder.get(5);
    if value == 42 {
        eprintln!("Invariant violated: trigger value encountered; handling error gracefully.");
        return;
    }

    // Continue with other operations
    // ...
}