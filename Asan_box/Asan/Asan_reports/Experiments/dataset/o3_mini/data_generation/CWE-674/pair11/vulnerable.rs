use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
unsafe fn rec_parse(s: &str, depth: usize) -> Result<usize, &'static str> {
    if s.is_empty() {
        return Ok(depth);
    }
    let bytes = s.as_bytes();
    if bytes[0] == b'(' {
        rec_parse(&s[1..], depth + 1)
    } else if bytes[0] == b')' {
        if depth == 0 {
            return Err("Unbalanced parentheses");
        }
        rec_parse(&s[1..], depth - 1)
    } else {
        Err("Invalid character")
    }
}
fn parse_nested(s: &str) -> Result<usize, &'static str> {
    unsafe { rec_parse(s, 0) }
}
fn worker(input: String) {
    thread::spawn(move || {
        let counter = Arc::new(Mutex::new(0));
        let raw = Arc::into_raw(counter.clone());
        let result = parse_nested(&input);
        let count = unsafe { (*raw).lock().unwrap() };
        println!("Result: {:?}, counter: {}", result, *count);
    });
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: program <nested parentheses string>");
        return;
    }
    let input = args[1].clone();
    worker(input);
    thread::sleep(std::time::Duration::from_secs(1));
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
