use std::sync::{Arc, Mutex};
use std::thread;

trait Task {
    fn execute(&self, value: i32);
}

struct Engine {
    data: Arc<Mutex<Vec<i32>>>,
}

impl Engine {
    fn new() -> Self {
        Engine {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn perform_update(&self, value: i32) -> Result<(), String> {
        if value < 0 {
            return Err("Negative value not allowed".to_string());
        } else {
            return Ok(());
        }
    }

    fn run_tasks(&self, value: i32) {
        let mut handles = Vec::new();
        for _ in 0..4 {
            let data_clone = Arc::clone(&self.data);
            let eng = self.clone();
            handles.push(thread::spawn(move || {
                match eng.perform_update(value) {
                    Ok(()) => {
                        let mut vec_lock = data_clone.lock().unwrap();
                        vec_lock.push(value);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }

    fn get_data(&self) -> Vec<i32> {
        self.data.lock().unwrap().clone()
    }
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Engine {
            data: Arc::clone(&self.data),
        }
    }
}

impl Task for Engine {
    fn execute(&self, value: i32) {
        self.run_tasks(value);
    }
}

fn main() {
    let engine = Engine::new();
    engine.execute(-1);
    let result = engine.get_data();
    println!("Data: {:?}", result);
}

fn oracle() {
    // Test Setup:
    // For a negative input (-1) the corrected code should not update the vector.
    // In contrast, the vulnerable code erroneously updates the vector.
    let engine = Engine::new();
    engine.execute(-1);
    let data = engine.get_data();
    // The invariant: With proper error handling the vector should remain empty.
    assert!(data.is_empty(), "Test failed: Data was updated with a negative value");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_negative_input() {
        oracle();
    }
}
