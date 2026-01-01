use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    threshold: i32,
}

impl Processor {
    fn new(threshold: i32) -> Self {
        Processor { threshold }
    }

    fn validate(&self, input: i32) -> bool {
        input >= self.threshold
    }

    fn process(&self, input: i32) -> Result<String, &'static str> {
        if self.validate(input) {
            Ok(format!("Processed value: {}", input))
        } else {
            Err("Access Denied")
        }
    }
}

pub fn main() {
    let proc = Arc::new(Mutex::new(Processor::new(50)));
    let proc_clone = proc.clone();
    let handle = thread::spawn(move || {
        let p = proc_clone.lock().unwrap();
        p.process(20)
    });
    let result = handle.join().unwrap();
    match result {
        Ok(msg) => println!("{}", msg),
        Err(err) => println!("Error: {}", err),
    }
}