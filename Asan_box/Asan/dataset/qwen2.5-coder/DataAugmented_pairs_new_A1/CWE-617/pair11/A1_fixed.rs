use std::env;
struct Engine {
    counter: usize,
}
impl Engine {
    fn new() -> Self {
        Engine { counter: 0 }
    }
    fn update(&mut self, inc: usize) -> usize {
        unsafe {
            let ptr: *mut usize = &mut self.counter;
            *ptr = self.counter.wrapping_add(inc);
        }
        // Ensure the counter does not exceed the safe threshold
        if self.counter >= 100 {
            self.counter = 99; // Set counter to 99 if it exceeds 100
        }
        self.counter
    }
}
fn run_app(input: String) -> usize {
    let value: usize = input.parse().unwrap();
    let mut eng = Engine::new();
    eng.update(value)
}
fn main() {
    let arg = env::args().nth(1).unwrap_or("0".to_string());
    let result = run_app(arg);
    println!("Final counter value: {}", result);
}