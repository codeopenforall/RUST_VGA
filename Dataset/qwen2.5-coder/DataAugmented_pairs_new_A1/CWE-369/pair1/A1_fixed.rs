#![allow(dead_code)]
use std::thread;
trait Arithmetic {
    fn operate(&self, numerator: i32, denominator: i32) -> i32;
}
struct Engine;
impl Arithmetic for Engine {
    fn operate(&self, numerator: i32, denominator: i32) -> i32 {
        if denominator == 0 {
            0
        } else {
            numerator / denominator
        }
    }
}
fn process(numerator: i32, denominator: i32) -> i32 {
    let engine = Engine;
    let handle = thread::spawn(move || {
        engine.operate(numerator, denominator)
    });
    handle.join().unwrap()
}
pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    process(numerator, denominator)
}
fn main() {
    let result = process(10, 0);
    println!("Calculation result: {}", result);
}