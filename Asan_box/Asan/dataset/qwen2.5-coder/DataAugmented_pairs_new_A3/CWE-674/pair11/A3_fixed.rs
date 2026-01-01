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