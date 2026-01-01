use std::{
    sync::{Arc, Mutex},
    thread,
};

fn compute_product(x: u32, y: u32) -> u64 {
    let temp = x.wrapping_mul(y);
    let mut res = temp;
    unsafe {
        let ptr: *mut u32 = &mut res as *mut u32;
        *ptr = temp;
    }
    res as u64
}

fn main() {
    let factor1: u32 = 100_000;
    let factor2: u32 = 50_000;
    let shared_output = Arc::new(Mutex::new(0u64));
    let shared_output_clone = Arc::clone(&shared_output);
    let thread_handle = thread::spawn(move || {
        let computed = compute_product(factor1, factor2);
        unsafe {
            let mut guard = shared_output_clone.lock().unwrap();
            *guard = computed;
        }
    });
    thread_handle.join().unwrap();
    let final_result = *shared_output.lock().unwrap();
    println!("Result: {}", final_result);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    // This test function builds and runs both binaries.
    // It verifies that the vulnerable version produces an incorrect (wrapped) result,
    // while the corrected version produces the expected 5,000,000,000.
    #[test]
    fn test_multiplication() {
        let expected: u64 = 5_000_000_000;

        // Execute the vulnerable binary.
        let output_vulnerable = Command::new("cargo")
            .args(&["run", "--bin", "vulnerable"])
            .output()
            .expect("Failed to execute vulnerable binary");
        let stdout_vulnerable = String::from_utf8_lossy(&output_vulnerable.stdout);
        // Extract the last token on the printed line assuming the format "Result: <value>"
        let parsed_vulnerable = stdout_vulnerable
            .trim()
            .split_whitespace()
            .last()
            .expect("Failed to parse vulnerable output")
            .parse::<u64>()
            .expect("Failed to convert vulnerable result to u64");
        // The vulnerable version overflows and thus must NOT equal the expected correct value.
        assert_ne!(parsed_vulnerable, expected, "Vulnerable implementation produced the correct result unexpectedly");

        // Execute the fixed binary.
        let output_fixed = Command::new("cargo")
            .args(&["run", "--bin", "fixed"])
            .output()
            .expect("Failed to execute fixed binary");
        let stdout_fixed = String::from_utf8_lossy(&output_fixed.stdout);
        let parsed_fixed = stdout_fixed
            .trim()
            .split_whitespace()
            .last()
            .expect("Failed to parse fixed output")
            .parse::<u64>()
            .expect("Failed to convert fixed result to u64");
        // The fixed version must exactly match the expected correct value.
        assert_eq!(parsed_fixed, expected, "Fixed implementation did not produce the correct result");
    }
}
