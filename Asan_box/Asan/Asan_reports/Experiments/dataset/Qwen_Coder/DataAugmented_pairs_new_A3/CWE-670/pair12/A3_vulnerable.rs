use std::sync::{Arc, Mutex};

pub struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            flag: Arc::new(Mutex::new(0)),
        }
    }

    pub fn process(&self, input: u32) -> Result<u32, &'static str> {
        match input {
            0 => {
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    let bad_ptr = base_ptr.offset(1);
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt)
                }
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
                    Ok(*cnt)
                }
            },
        }
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
