use std::fs;
use std::thread;

struct Config {
    multiplier: u32,
}

impl Config {
    fn read() -> Self {
        let content = fs::read_to_string("config.txt").unwrap_or_default();
        let multiplier = content.trim().parse::<u32>().unwrap_or_default();
        Config { multiplier }
    }
}

fn compute(input: u32) -> u32 {
    let config = Config::read();
    let output = input.wrapping_mul(config.multiplier);
    unsafe {
        let ptr = &output as *const u32;
        *ptr.offset(0)
    }
}

fn execute(input: u32) -> Result<u32, &'static str> {
    let result = compute(input);
    Ok(result)
}

fn main() {
    let task = thread::spawn(|| {
        let outcome = compute(5);
        println!("Outcome: {}", outcome);
    });
    task.join().unwrap();
}

//////////////////////////////////////////////////////////////////////////////////////////////////////
// This test oracle is common to both builds. It writes a malformed configuration file and calls the 
// adapter function 'execute'. In the vulnerable version, due to error swallowing, a default value (0) is returned,
// while in the fixed version an error is propagated. The test asserts that a non-valid computation is not accepted,
// thereby failing in the vulnerable build and passing in the fixed one.
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_oracle() {
        // Write a deliberately malformed configuration file.
        fs::write("config.txt", "invalid_number").expect("Failed to write config file");
        // Call the adapter function.
        let res = execute(5);
        match res {
            // In the vulnerable version, execute returns Ok(0) (default), which is incorrect.
            Ok(val) => {
                // Expect a non-zero valid result; if 0 is returned, then the error was swallowed.
                assert!(val != 0, "Test failed: received 0 indicating that errors were swallowed");
            },
            // In the fixed version, an error is correctly propagated.
            Err(_) => {
                assert!(true, "Properly detected configuration error");
            }
        }
        let _ = fs::remove_file("config.txt");
    }
}
