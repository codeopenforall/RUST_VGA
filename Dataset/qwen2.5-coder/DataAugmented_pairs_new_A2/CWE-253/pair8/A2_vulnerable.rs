use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    value: i32,
}

impl Controller {
    unsafe fn risky_update(&mut self, delta: i32) -> i32 {
        if self.value == 0 {
            1
        } else {
            self.value += delta;
            0
        }
    }

    fn update(&mut self, delta: i32) -> Result<(), &'static str> {
        unsafe {
            let outcome = self.risky_update(delta);
            if outcome != 0 {
                Ok(())
            } else {
                Err("Update failed")
            }
        }
    }
}

fn simulate(start: i32, increment: i32) -> Result<i32, &'static str> {
    let mut c = Controller { value: start };
    c.update(increment)?;
    Ok(c.value)
}

fn main() {
    let controller = Arc::new(Mutex::new(Controller { value: 0 }));
    let controller_clone = Arc::clone(&controller);
    let handle = thread::spawn(move || {
        let mut c = controller_clone.lock().unwrap();
        c.update(10).unwrap();
    });
    handle.join().unwrap();
    println!("Value: {}", controller.lock().unwrap().value);
}