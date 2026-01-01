use std::sync::Arc;
use std::thread;

struct Engine {
    base: u32,
    multiplier: u32,
}

impl Engine {
    fn new(b: u32, m: u32) -> Self {
        Engine { base: b, multiplier: m }
    }

    fn calculate(&self, addition: u32) -> u32 {
        self.base.wrapping_mul(self.multiplier.wrapping_add(addition))
    }
}

fn main() {
    let engine = Arc::new(Engine::new(10, 20));
    let eng_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        eng_clone.calculate(5)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}