use std::str;

pub fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    unsafe { parse_helper(bytes, 0) }?;
    Ok(())
}

unsafe fn parse_helper(data: &[u8], pos: usize) -> Result<usize, &'static str> {
    if pos >= data.len() {
        return Err("unexpected end of input");
    }

    if data[pos] == b'(' {
        let mut i = pos + 1;
        i = parse_helper(data, i + 1)?;
        if data[i] != b')' {
            return Err("unmatched closing parenthesis");
        }
        Ok(i + 1)
    } else {
        Err("expected opening parenthesis")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    // This test function verifies the behavior using a deeply nested input.
    // For the fixed code, process_input should return an Err with "maximum recursion depth exceeded".
    // For the vulnerable code, process_input will eventually cause a stack overflow (panic),
    // and catch_unwind will capture that panic, causing the test to fail.
    #[test]
    fn recursion_depth_oracle() {
        // Create an input with 1500 nested pairs.
        let deep_input = "(".repeat(1500) + &")".repeat(1500);
        let result = panic::catch_unwind(|| process_input(&deep_input));

        match result {
            // If process_input returned without panicking, check the result.
            Ok(inner) => {
                match inner {
                    Ok(_) => panic!("Unexpected success on deeply nested input."),
                    Err(e) => assert_eq!(e, "maximum recursion depth exceeded", "Expected recursion limit error."),
                }
            }
            // If a panic occurred (as expected in the vulnerable version), the test fails.
            Err(_) => panic!("Function panicked due to uncontrolled recursion."),
        }
    }
}
