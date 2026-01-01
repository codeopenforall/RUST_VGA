use std::sync::{Arc, Mutex};
use std::thread;

struct Coordinator {
    state: Arc<Mutex<u32>>,
}

impl Coordinator {
    fn new() -> Self {
        Coordinator {
            state: Arc::new(Mutex::new(0)),
        }
    }

    fn update_state(&self, code: u32) -> Result<(), &'static str> {
        let state_clone = Arc::clone(&self.state);
        let handle = thread::spawn(move || {
            let mut data = state_clone.lock().unwrap();
            match code {
                42 => {
                    *data = 99;
                    Ok(())
                }
                _ => {
                    *data = 0;
                    Err("operation failed")
                }
            }
        });
        handle.join().unwrap()
    }

    fn read_state(&self) -> u32 {
        *self.state.lock().unwrap()
    }
}

fn main() {
    let app = Coordinator::new();
    let result = app.update_state(42);
    match result {
        Ok(()) => println!("Success, state is {}", app.read_state()),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::Coordinator;

    #[test]
    fn test_update_trigger() {
        let app = Coordinator::new();
        // For input 42 the expected behavior is to update state to 99 and return Ok.
        let res = app.update_state(42);
        assert!(res.is_ok(), "Expected Ok result for input code 42");
        assert_eq!(app.read_state(), 99, "State should be updated to 99 for input code 42");
    }
}
