use std::sync::{Arc, Mutex};

const MAX_DEPTH: usize = 1000;

fn safe_rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    if depth > MAX_DEPTH {
        return Err("Maximum recursion depth exceeded");
    }
    if s.is_empty() {
        return Ok(0);
    }
    let first = s.as_bytes()[0];
    if first == b'(' {
        safe_rec_parse(&s[1..], depth + 1)
    } else if first == b')' {
        safe_rec_parse(&s[1..], depth - 1)
    } else {
        safe_rec_parse(&s[1..], depth)
    }
}

pub fn parse_nested(s: &str) -> Result<usize, &'static str> {
    let counter = Arc::new(Mutex::new(0));
    let result = safe_rec_parse(s, 0);
    let count = counter.lock().unwrap();
    println!("Counter value: {}", *count);
    result
}

#[cfg(test)]
mod tests {
    use std::panic;
    // Import the parse_nested function from the module containing the production code.
    use super::parse_nested;

    #[test]
    fn recursion_limit_test() {
        // Create deeply nested input: 1500 opening and closing parentheses.
        let depth = 1500;
        let input = "(".repeat(depth) + &")".repeat(depth);
        // Use catch_unwind to intercept a potential stack overflow panic.
        let result = panic::catch_unwind(|| {
            parse_nested(&input)
        });

        match result {
            // For the fixed code, the recursion limit is enforced so the function returns an error.
            Ok(inner_result) => {
                assert_eq!(inner_result, Err("Maximum recursion depth exceeded"),
                           "Expected error due to maximum recursion depth being exceeded");
            },
            // For the vulnerable code, the deep recursion triggers a panic (stack overflow).
            Err(_) => {
                panic!("Test failed: stack overflow occurred when processing deep input");
            }
        }
    }
}
