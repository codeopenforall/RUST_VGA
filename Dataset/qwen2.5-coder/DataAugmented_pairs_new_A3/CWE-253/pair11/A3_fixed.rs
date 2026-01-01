use std::sync::{Arc, Mutex};

struct SharedCounter {
    value: Mutex<i32>,
}

impl SharedCounter {
    fn new(initial_value: i32) -> Self {
        SharedCounter {
            value: Mutex::new(initial_value),
        }
    }

    fn update(&self, delta: i32) -> Result<(), String> {
        if delta < 0 {
            return Err("Negative update not allowed".to_string());
        }
        let mut value = self.value.lock().unwrap();
        *value += delta;
        Ok(())
    }
}

fn perform(counter: &Arc<SharedCounter>, delta: i32) -> bool {
    let res = counter.update(delta);
    if res.is_ok() {
        true
    } else {
        false
    }
}