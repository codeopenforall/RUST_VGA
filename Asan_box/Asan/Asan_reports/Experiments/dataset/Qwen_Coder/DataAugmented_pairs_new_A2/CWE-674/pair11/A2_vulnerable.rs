use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

unsafe fn recursive_parser(s: &str, level: usize) -> Result<usize, &'static str> {
    if s.is_empty() {
        return Ok(level);
    }
    let chars = s.as_bytes();
    if chars[0] == b'(' {
        recursive_parser(&s[1..], level + 1)
    } else if chars[0] == b')' {
        if level == 0 {
            return Err("Unbalanced parentheses");
        }
        recursive_parser(&s[1..], level - 1)
    } else {
        Err("Invalid character")
    }
}

fn parse_nested(s: &str) -> Result<usize, &'static str> {
    unsafe { recursive_parser(s, 0) }
}

fn worker_task(input: String) {
    thread::spawn(move || {
        let counter = Arc::new(Mutex::new(0));
        let raw_counter = Arc::into_raw(counter.clone());
        let parsing_result = parse_nested(&input);
        let lock_result = unsafe { (*raw_counter).lock().unwrap() };
        println!("Parsing result: {:?}, counter value: {}", parsing_result, *lock_result);
    });
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("Usage: executable <nested parentheses string>");
        return;
    }
    let input_string = arguments[1].clone();
    worker_task(input_string);
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
