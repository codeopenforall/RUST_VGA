use std::sync::{Arc, Mutex};

struct Machine {
    counter: i32,
    active: bool,
}

impl Machine {
    unsafe fn init(&mut self) -> Result<(), &'static str> {
        if !self.active {
            return Err("machine inactive");
        }
        Ok(())
    }

    fn operate(&self, shared: Arc<Mutex<Machine>>) -> Result<i32, &'static str> {
        let mut mach = shared.lock().unwrap();
        unsafe { mach.init()? };
        mach.counter += 1;
        Ok(mach.counter as i32)
    }
}

fn execute() -> Result<i32, &'static str> {
    let machine = Arc::new(Mutex::new(Machine { counter: 0, active: false }));
    let result = machine.lock().unwrap().operate(Arc::clone(&machine));
    result
}

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
