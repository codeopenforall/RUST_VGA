#![allow(dead_code)]
use std::thread;
trait Arithmetic {
    fn compute(&self, numerator: i32, denominator: i32) -> i32;
}
struct Processor;
impl Arithmetic for Processor {
    fn compute(&self, numerator: i32, denominator: i32) -> i32 {
        unsafe {
            let ptr = &denominator as *const i32;
            let divisor = *ptr;
            numerator / divisor   
        }
    }
}
fn execute(numerator: i32, denominator: i32) -> i32 {
    let processor = Processor;
    let handle = thread::spawn(move || {
        processor.compute(numerator, denominator)
    });
    handle.join().unwrap()
}
pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    execute(numerator, denominator)
}
fn main() {
    let result = execute(10, 0);
    println!("Calculation result: {}", result);
}