fn process_data(input: &str) -> Result<String, &'static str> {
    let user_index: usize = input.parse().map_err(|_| "Invalid input")?;
    let mut data_guard = vec![0; 10];
    let result = vec![0; 10];

    if user_index >= data_guard.len() {
        return Err("Index out-of-bounds");
    }

    data_guard[user_index] = 65;
    let s = std::str::from_utf8(&result).map_err(|_| "UTF-8 conversion error")?;
    Ok(s.to_string())
}

///////////////////////////////////////////////
// Test Oracle for Input Validation Vulnerability
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::panic;
    
    // Import the function under test.
    // For the vulnerable variant, the function is assumed to be in the same module.
    // For fixed variant testing, rename or separate the function import accordingly.
    use super::{process_data};

    #[test]
    fn oracle() {
        // Use an input that exceeds the bounds ("15" for a vector of length 10).
        // For the vulnerable version, this should trigger undefined behavior (panic) or other error.
        let vulnerable_result = panic::catch_unwind(|| {
            // In the vulnerable version, this is expected to panic or invoke UB.
            process_data("15").unwrap()
        });
        assert!(vulnerable_result.is_err(), "The vulnerable variant did not panic on out-of-bound input.");

        // For the fixed variant, the function should gracefully return an error.
        let fixed_result = process_data("15");
        assert!(fixed_result.is_err(), "The fixed variant unexpectedly accepted an out-of-bound input.");
    }
}
