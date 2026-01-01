#![allow(dead_code)]
use std::env;
use std::sync::Arc;
use std::thread;
struct Holder {
    elems: Vec<u32>,
}
impl Holder {
    fn new() -> Self {
        Holder {
            elems: vec![10, 20, 30, 40, 50],
        }
    }
    fn fetch(&self, pos: usize) -> u32 {
        if pos < self.elems.len() {
            self.elems[pos]
        } else {
            panic!("Provided index {} is out-of-bound (max index {}).", pos, self.elems.len() - 1);
        }
    }
}
fn execute(pos: usize) -> u32 {
    let data = Arc::new(Holder::new());
    let handle = thread::spawn({
        let data = Arc::clone(&data);
        move || {
            data.fetch(pos)
        }
    });
    handle.join().unwrap()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let idx: usize = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let out = execute(idx);
    println!("Output: {}", out);
}


#[cfg(test)]
mod tests {
    use std::panic;
    // Import the appropriate run function based on whether we're testing the vulnerable or fixed variant.
    // Assume that in the test build, the function 'execute' is available in the parent module.
    // The test triggers the out-of-bound scenario with a predetermined index that should not be valid.
    #[test]
    fn boundary_check_test() {
        // In both variants, an out-of-bound index (e.g., 10) is used.
        // For the fixed variant, the bounds check causes a panic,
        // which is the intended and correct behavior.
        // In the vulnerable variant, undefined behavior might lead to a regular return value
        // or even a silent memory corruption, so the test will fail if no panic occurs.
        let result = panic::catch_unwind(|| {
            super::execute(10);
        });
        assert!(result.is_err(), "Expected a panic due to out-of-bound access, but no panic occurred.");
    }
}
