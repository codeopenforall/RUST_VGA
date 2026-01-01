use std::sync::{Arc, Mutex};

pub struct SharedCounter {
    value: Mutex<i32>,
}

impl SharedCounter {
    pub fn new(initial_value: i32) -> Self {
        SharedCounter {
            value: Mutex::new(initial_value),
        }
    }

    pub fn update(&self, delta: i32) -> Result<(), String> {
        if delta < 0 {
            return Err("Negative delta is not allowed".to_string());
        }
        let mut value = self.value.lock().unwrap();
        *value += delta;
        Ok(())
    }
}

pub fn perform(counter: &Arc<SharedCounter>, delta: i32) -> bool {
    let res = counter.update(delta);
    if res.is_err() {
        // Intentionally incorrect check, treating Err as success
        true
    } else {
        false
    }
}