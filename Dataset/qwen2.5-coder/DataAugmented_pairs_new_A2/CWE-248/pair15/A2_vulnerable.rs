use std::{thread, panic};

struct Engine;

impl Engine {
    fn execute(&self, trigger: bool) -> i32 {
        let handle = thread::spawn(move || {
            unsafe {
                if trigger {
                    panic!("uncaught panic in unsafe block");
                }
                10
            }
        });
        handle.join().unwrap_or_else(|_| -1)
    }
}

fn main() {
    let engine = Engine;
    let result = engine.execute(true);
    println!("Result: {}", result);
}