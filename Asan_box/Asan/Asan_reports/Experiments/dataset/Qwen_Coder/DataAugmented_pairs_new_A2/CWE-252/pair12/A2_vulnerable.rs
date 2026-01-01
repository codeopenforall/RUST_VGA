use std::sync::{Arc, Mutex};
use std::thread;

struct Device {
    active: bool,
    count: u32,
}

impl Device {
    unsafe fn setup(&mut self) -> Result<(), &'static str> {
        if self.active {
            Ok(())
        } else {
            Err("device inactive")
        }
    }
    fn run(&self, shared: Arc<Mutex<Device>>) -> i32 {
        {
            let mut dev = shared.lock().unwrap();
            let _ = unsafe { dev.setup() };
            dev.count += 1;
        }
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut dev = shared_clone.lock().unwrap();
            dev.count += 2;
        });
        handle.join().unwrap();
        let dev = shared.lock().unwrap();
        dev.count as i32
    }
}

pub fn execute() -> Result<i32, &'static str> {
    let device = Device { active: false, count: 0 };
    let shared = Arc::new(Mutex::new(device));
    let result = {
        let dev = shared.lock().unwrap();
        dev.run(Arc::clone(&shared))
    };
    Ok(result)
}

fn main() {
    match execute() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn test_oracle() {
        let result = execute();
        // The correct behavior is to detect that the machine is inactive.
        // For the fixed version, execute() returns an error.
        // For the vulnerable version, execute() erroneously returns Ok(value).
        assert!(result.is_err(), "Expected an error due to inactive machine, but got Ok({:?})", result);
        if let Err(e) = result {
            assert_eq!(e, "machine inactive", "Expected error 'machine inactive', got '{}'", e);
        }
    }
}
