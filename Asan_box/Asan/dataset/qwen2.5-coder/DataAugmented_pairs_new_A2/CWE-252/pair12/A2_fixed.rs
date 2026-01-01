use std::sync::{Arc, Mutex};
use std::thread;

struct Machine {
    active: bool,
    counter: u32,
}

impl Machine {
    unsafe fn init(&mut self) -> Result<(), &'static str> {
        if self.active {
            Ok(())
        } else {
            Err("machine inactive")
        }
    }

    fn operate(&self, shared: Arc<Mutex<Machine>>) -> i32 {
        {
            let mut mach = shared.lock().unwrap();
            let result = unsafe { mach.init() };
            if result.is_err() {
                return -1; // Return an error indicator
            }
            mach.counter += 1;
        }
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut mach = shared_clone.lock().unwrap();
            mach.counter += 2;
        });
        handle.join().unwrap();
        let mach = shared.lock().unwrap();
        mach.counter as i32
    }
}

pub fn execute() -> Result<i32, &'static str> {
    let machine = Machine { active: false, counter: 0 };
    let shared = Arc::new(Mutex::new(machine));
    let result = {
        let mach = shared.lock().unwrap();
        let op_result = mach.operate(Arc::clone(&shared));
        if op_result == -1 {
            return Err("machine inactive");
        }
        op_result
    };
    Ok(result)
}

fn main() {
    match execute() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}