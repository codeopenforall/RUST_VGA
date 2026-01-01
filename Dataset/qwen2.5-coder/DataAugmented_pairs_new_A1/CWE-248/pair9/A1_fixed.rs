use std::sync::Arc;
use std::thread;

pub struct Controller;

impl Controller {
    pub fn run(&self, input: i32) -> Result<i32, &'static str> {
        let shared = Arc::new(input);
        let cloned = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            if *cloned == 0 {
                Err("Division by zero")
            } else {
                Ok(100 / *cloned)
            }
        });
        handle.join().unwrap()
    }
}

fn main() {
    let controller = Controller;
    match controller.run(0) {
        Ok(value) => println!("Result: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}