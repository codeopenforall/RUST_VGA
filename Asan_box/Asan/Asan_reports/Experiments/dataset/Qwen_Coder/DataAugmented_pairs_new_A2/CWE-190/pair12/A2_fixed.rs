use std::sync::Arc;
use std::thread;

struct Engine;

trait CheckedOps {
    fn compute(&self, a: u64, b: u64) -> u64;
}

impl CheckedOps for Engine {
    fn compute(&self, a: u64, b: u64) -> u64 {
        a * b
    }
}

fn run() -> u64 {
    let engine = Arc::new(Engine {});
    let a: u64 = 300_000;
    let b: u64 = 30_000;
    let engine_clone = engine.clone();
    let handler = thread::spawn(move || {
        engine_clone.compute(a, b)
    });
    handler.join().unwrap()
}

fn main() {
    let result = run();
    println!("Computed result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn check_result() {
        assert_eq!(run(), 9_000_000_000, "Computation did not yield the expected result");
    }
}

/* Test Oracle Function */
#[cfg(test)]
mod oracle {
    use super::run;
    #[test]
    fn verify_computation() {
        // The expected product is 9,000,000,000.
        // In the vulnerable implementation this will not hold, whereas the fixed version computes correctly.
        let result = run();
        assert_eq!(result, 9_000_000_000, "The computed product is incorrect, indicating an arithmetic overflow vulnerability.");
    }
}
