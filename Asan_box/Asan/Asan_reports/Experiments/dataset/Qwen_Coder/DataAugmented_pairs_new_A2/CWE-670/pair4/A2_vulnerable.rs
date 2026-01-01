#![allow(dead_code)]
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    counter: Arc<Mutex<u32>>,
}

impl Service {
    fn new() -> Self {
        Service {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    fn process(&self, command: &str) -> Result<u32, &'static str> {
        let mut result: u32 = 0;
        match command {
            "normal" => {
                let mut cnt = self.counter.lock().unwrap();
                *cnt += 1;
                result = *cnt;
            }
            "danger" => {
                unsafe {
                    let ptr: *mut u32 = &mut result as *mut u32;
                    *ptr = 100;
                }
                let counter_clone = Arc::clone(&self.counter);
                let handle = thread::spawn(move || {
                    let mut lock = counter_clone.lock().unwrap();
                    if *lock >= 0 {
                        *lock += 10;
                    }
                });
                handle.join().unwrap();
                result = 100;
            }
            _ => return Err("unsupported command"),
        }
        Ok(result)
    }
}

fn main() {
    let service = Service::new();
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "normal" };
    match service.process(input) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

// This test oracle function is separate from the main application code.
// It asserts that when the "danger" command is processed, an error is returned.
// For the vulnerable code, the test will fail (since it erroneously returns Ok(100)),
// whereas for the fixed code the test passes.
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle() {
        let service = Service::new();
        // When passing a dangerous command, we expect an error.
        // The fixed version returns Err("Danger command not allowed")
        match service.process("danger") {
            Ok(val) => panic!("Test failed: expected error, got Ok({})", val),
            Err(e) => assert_eq!(e, "Danger command not allowed"),
        }
    }
}
