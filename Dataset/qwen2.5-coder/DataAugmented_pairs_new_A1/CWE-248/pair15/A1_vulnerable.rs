use std::{thread, panic};
struct Engine;
impl Engine {
    fn run(&self, trigger: bool) -> i32 {
        let handle = thread::spawn(move || {
            unsafe {
                if trigger {
                    panic!("panic in unsafe block caught later");
                }
                10
            }
        });
        handle.join().unwrap() // Removed the match statement to propagate the panic
    }
}
fn main() {
    let engine = Engine;
    let result = engine.run(true);
    println!("Result: {}", result);
}