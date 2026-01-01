use std::sync::{Arc, Mutex};

struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    fn new() -> Self {
        Engine { flag: Arc::new(Mutex::new(0)) }
    }

    fn process(&self, val: u32) -> Result<u32, &'static str> {
        let mut cnt = self.flag.lock().map_err(|_| "mutex poisoned")?;
        match val {
            1 => {
                *cnt += 5;
                Ok(*cnt)
            }
            2 => {
                *cnt += 10;
                Ok(*cnt)
            }
            _ => Err("Invalid input"),
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
