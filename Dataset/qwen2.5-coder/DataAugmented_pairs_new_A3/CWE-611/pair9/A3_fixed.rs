use std::sync::{Arc, Mutex};

struct Handler {
    shared: Arc<Mutex<i32>>,
}

impl Handler {
    pub fn operate(&self) -> Result<i32, &'static str> {
        let result = self.shared.lock().map_err(|_| "Lock failed")?;
        if let Some(val) = result.checked_add(0) {
            Ok(val)
        } else {
            Err("Operation failed")
        }
    }

    pub fn execute(&self) -> Result<i32, &'static str> {
        let res = self.shared.lock().map_err(|_| "Lock failed")?;
        if let Some(val) = res.checked_add(0) {
            Ok(val)
        } else {
            Err("Thread operation failed")
        }
    }
}