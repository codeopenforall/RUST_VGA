use std::sync::{Arc, Mutex};
use std::thread;

struct Controller {
    counter: Arc<Mutex<i32>>,
}

impl Controller {
    fn new(initial: i32) -> Self {
        Controller {
            counter: Arc::new(Mutex::new(initial)),
        }
    }

    fn update(&self, adjustment: i32) -> Result<i32, &'static str> {
        let shared = Arc::clone(&self.counter);
        let handle = thread::spawn(move || {
            let mut num = shared.lock().unwrap();
            *num += adjustment;
        });
        handle.join().unwrap();
        let current = *self.counter.lock().unwrap();
        if current < 0 {
            Err("Counter went negative")
        } else {
            Ok(current)
        }
    }
}

fn main() {
    let controller = Controller::new(1);
    match controller.update(10) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}