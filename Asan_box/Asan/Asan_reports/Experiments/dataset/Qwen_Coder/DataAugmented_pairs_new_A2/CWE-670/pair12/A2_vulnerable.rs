use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    fn new() -> Self {
        Engine { flag: Arc::new(Mutex::new(0)) }
    }

    fn process(&self, val: u32) -> Result<u32, &'static str> {
        match val {
            2 => {
                let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
                *cnt += 10;
                Ok(*cnt)
            },
            1 | _ => {
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    let bad_ptr = base_ptr.offset(1);
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt + 5) // Introducing a defect by adding 5 instead of setting it to 5
                }
            },
        }
    }
}

fn main() {
    let core = Engine::new();
    match core.process(1) {
        Ok(res) => println!("Outcome: {}", res),
        Err(err) => eprintln!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::Engine;

    // The oracle function: this test is designed to pass with the corrected control flow and fail with the flawed one.
    #[test]
    fn test_engine_safe_update() {
        let system = Engine::new();
        // With the fixed version, input 1 correctly updates the flag from 0 to 5.
        let outcome = system.process(1).expect("Expected a valid result");
        assert_eq!(outcome, 5, "For valid input, the state should be incremented by 5.");
    }
}
