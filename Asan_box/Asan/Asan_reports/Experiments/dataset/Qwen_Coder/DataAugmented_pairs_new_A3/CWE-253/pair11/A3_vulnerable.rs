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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    // The oracle function is intended to fail for the vulnerable version and pass for the corrected version.
    // It invokes perform with a negative delta. In a correct implementation, perform should return false.
    // In the vulnerable version, the inverted check would result in true.
    #[test]
    fn test_oracle() {
        let counter = Arc::new(SharedCounter::new(5));
        let result = perform(&counter, -10);
        // For the fixed code, the update returns Err and perform should correctly return false.
        assert_eq!(result, false, "Expected the operation to fail on negative update, but it succeeded");
    }
}
