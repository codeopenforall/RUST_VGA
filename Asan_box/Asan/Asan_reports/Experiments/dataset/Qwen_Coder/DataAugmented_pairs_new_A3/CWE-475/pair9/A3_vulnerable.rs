use std::sync::Arc;
use std::thread;
struct Data {
    ptr: *const u8,
    length: usize,
}
impl Data {
    unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.length)
    }
}
fn compute_sum(shared: Arc<Data>) -> u32 {
    let mut handles = Vec::new();
    for _ in 0..4 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                let slice = clone.as_slice();
                slice.iter().map(|&b| b as u32).sum::<u32>()
            }
        });
        handles.push(handle);
    }
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    total
}
fn main() {
    let mut valid_data = vec![1u8, 2, 3, 4];
    let instance = Data {
        ptr: valid_data.as_ptr(),
        length: valid_data.len() + 10, 
    };
    let shared_instance = Arc::new(instance);
    let result = compute_sum(shared_instance);
    println!("Result: {}", result);
}


#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::str;

    // This test uses a predetermined expected sum of 10 (1+2+3+4).
    #[test]
    fn integration_test() {
        // Run the vulnerable binary.
        let vulnerable_output = Command::new("cargo")
            .args(&["run", "--quiet", "--bin", "vulnerable_bin"])
            .output()
            .expect("Failed to run vulnerable binary");
        let vulnerable_stdout = str::from_utf8(&vulnerable_output.stdout).unwrap();
        // The vulnerable binary should not output the correct result due to undefined behavior.
        assert!(
            !vulnerable_stdout.contains("Result: 10"),
            "Vulnerable implementation produced expected result, test should fail."
        );

        // Run the fixed binary.
        let fixed_output = Command::new("cargo")
            .args(&["run", "--quiet", "--bin", "fixed_bin"])
            .output()
            .expect("Failed to run fixed binary");
        let fixed_stdout = str::from_utf8(&fixed_output.stdout).unwrap();
        // The fixed binary is expected to output "Result: 10".
        assert!(
            fixed_stdout.contains("Result: 10"),
            "Fixed implementation did not produce expected result, test should fail."
        );
    }
}
