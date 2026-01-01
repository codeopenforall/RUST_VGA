#![allow(unused_unsafe)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine;

impl Engine {
    pub fn run_input(&self, load: usize) -> Result<(), String> {
        if load > 100 {
            return Err(String::from("Input load too high"));
        }
        let mut result: usize = 0;
        for i in 0..load {
            result = result.wrapping_add(i);
        }
        let shared = Arc::new(Mutex::new(result));
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let mut val = thread_shared.lock().unwrap();
            *val = val.wrapping_mul(2);
        });
        handle.join().unwrap();
        Ok(())
    }
}

fn main() {
    let eng = Engine;
    match eng.run_input(10) {
        Ok(()) => println!("Processing completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}