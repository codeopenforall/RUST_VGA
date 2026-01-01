extern crate libc;
use std::env;
use std::ffi::CString;
use std::thread;
struct Processor;
impl Processor {
    fn run(&self, input: &str) {
        let safe_input = CString::new(input).unwrap();
        let safe_fmt = CString::new("User input: %s, fixed number: %d\n").unwrap();
        let handle = thread::spawn(move || {
            unsafe {
                libc::printf(safe_fmt.as_ptr(), safe_input.as_ptr(), 42);
            }
        });
        let _ = handle.join();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <user_input>", args[0]);
        return;
    }
    let input = &args[1];
    let proc_inst = Processor;
    proc_inst.run(input);
}


/*
Test Oracle Function:
This test oracle executes both binaries (assumed to be compiled as "vulnerable_bin" and "fixed_bin")
with a malicious input containing format specifiers. It asserts that the output from the safe version
matches the expected output, while the vulnerable version's output deviates, indicating the vulnerability.
*/

#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn test_oracle() {
        // Malicious input with format specifiers.
        let malicious_input = "%x %x %x";
        
        // Run the vulnerable binary.
        let vulnerable_output = Command::new("./vulnerable_bin")
            .arg(malicious_input)
            .output()
            .expect("Failed to run vulnerable binary");
        let vulnerable_stdout = String::from_utf8_lossy(&vulnerable_output.stdout);
        
        // Run the fixed binary.
        let fixed_output = Command::new("./fixed_bin")
            .arg(malicious_input)
            .output()
            .expect("Failed to run fixed binary");
        let fixed_stdout = String::from_utf8_lossy(&fixed_output.stdout);
        
        // Expected output from the fixed binary.
        let expected_fixed = format!("User input: {}, fixed number: 42\n", malicious_input);
        
        // Assert that the fixed binary produces exactly the expected output.
        assert_eq!(fixed_stdout, expected_fixed, "Safe binary output mismatch");
        
        // For the vulnerable binary, the output should not match the safe expected output.
        // If it did match, it would imply that the unsafe formatting did not occur.
        assert_ne!(vulnerable_stdout, expected_fixed, "Vulnerable binary output unexpectedly matches safe output");
    }
}
