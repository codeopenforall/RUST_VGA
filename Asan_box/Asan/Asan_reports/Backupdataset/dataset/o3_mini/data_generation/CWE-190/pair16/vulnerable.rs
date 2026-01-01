#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Operator {
    multiplier: u32,
}
impl Operator {
    unsafe fn compute(&self, input: u32) -> u64 {
        let product = input.wrapping_mul(self.multiplier);
        product as u64
    }
}
fn main() {
    let op = Arc::new(Operator { multiplier: 100_000 });
    let test_input: u32 = 50_000; 
    let op_clone = Arc::clone(&op);
    let handle = thread::spawn(move || {
        unsafe { op_clone.compute(test_input) }
    });
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
    if result != 5_000_000_000 {
        std::process::exit(1);
    }
    std::process::exit(0);
}


#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::env;

    #[test]
    fn test_oracle() {
        // This test oracle expects the binary to exit with code 0 if the computation is correct.
        // In the vulnerable version the overflow causes an incorrect result and a non-zero exit code.
        // In the fixed version the correct product is computed and the process exits with 0.
        let current_exe = env::args().next().expect("Failed to get current executable");
        let output = Command::new(current_exe)
            .output()
            .expect("Failed to execute process");

        assert_eq!(
            output.status.code(),
            Some(0),
            "Test oracle failed: Unexpected exit code"
        );
    }
}
