use std::sync::Arc;
use std::thread;

struct Calculator {}

impl Calculator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compute(&self, a: i32, b: i32) -> i32 {
        if b == 0 {
            0
        } else {
            a / b
        }
    }
}

fn main() {
    let calc = Calculator::new();
    let arc_calc = Arc::new(calc);
    let arc_clone = Arc::clone(&arc_calc);
    let handle = thread::spawn(move || {
        arc_clone.compute(10, 0)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}