use std::{thread, panic};

struct Engine;

impl Engine {
    fn run(&self, trigger: bool) -> i32 {
        let handle = thread::spawn(move || {
            if trigger {
                panic!("uncaught panic in unsafe block");
            }
            10
        });

        match handle.join() {
            Ok(value) => value,
            Err(_) => -1,
        }
    }
}

fn main() {
    let engine = Engine;
    let result = engine.run(true);
    println!("Result: {}", result);
}