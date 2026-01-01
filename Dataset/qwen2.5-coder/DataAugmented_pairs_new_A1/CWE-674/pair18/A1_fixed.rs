#![allow(unused_unsafe)]
use std::env;
fn process_input(input: &str) -> Result<(), &'static str> {
    let bytes = input.as_bytes();
    unsafe { parse_helper(bytes, 0, 0) }?;
    Ok(())
}
unsafe fn parse_helper(data: &[u8], pos: usize, depth: usize) -> Result<usize, &'static str> {
    const MAX_DEPTH: usize = 1000;
    if depth > MAX_DEPTH {
        return Err("maximum recursion depth exceeded");
    }
    let mut i = pos;
    while i < data.len() {
        let ch = *data.get_unchecked(i);
        if ch == b'(' {
            i = parse_helper(data, i + 1, depth + 1)?;
        } else if ch == b')' {
            return Ok(i + 1);
        }
        i += 1;
    }
    Ok(i)
}
fn main() {
    let deep_input = if let Some(arg) = env::args().nth(1) {
        arg
    } else {
        "(".repeat(3500) + &")".repeat(3500)
    };
    match process_input(&deep_input) {
        Ok(_) => println!("Parsing completed successfully."),
        Err(e) => println!("Error encountered: {}", e),
    }
}