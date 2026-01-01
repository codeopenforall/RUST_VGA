use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct BufferManager {
    data: Vec<u8>,
}

impl BufferManager {
    unsafe fn get_segment(&self, start: usize, length: usize) -> &str {
        let ptr = self.data.as_ptr().add(start);
        let slice = std::slice::from_raw_parts(ptr, length);
        std::str::from_utf8_unchecked(slice)
    }
}

struct Processor {
    manager: Arc<Mutex<BufferManager>>,
}

impl Processor {
    fn run(&self, start: usize, length: usize) -> Result<u32, &'static str> {
        let guard = self.manager.lock().unwrap();
        if start + length > guard.data.len() {
            return Err("out of bounds");
        }
        let segment = unsafe { guard.get_segment(start, length) };
        segment.trim().parse::<u32>().map_err(|_| "parse error")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: <program> <input_string> <start> <length>");
        return;
    }
    let input = args[1].clone();
    let start: usize = args[2].parse().unwrap_or(0);
    let length: usize = args[3].parse().unwrap_or(0);
    let manager = BufferManager { data: input.into_bytes() };
    let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };
    let handle = thread::spawn(move || {
        match proc_inst.run(start, length) {
            Ok(value) => println!("Extracted number: {}", value),
            Err(err) => println!("Operation failed: {}", err),
        }
    });
    handle.join().unwrap();
}

//////////////////////
// Test Oracle Code
//////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This test verifies that when provided with out-of-bound parameters for the input string,
    // the safe implementation correctly returns an error.
    // For the vulnerable version, this test is expected to fail (panic or undefined behavior).
    #[test]
    fn test_oracle() {
        // "12345" has length 5; parameters (start=3, length=5) result in end=8, which is out-of-bound.
        let input_data = "12345".to_string();
        let manager = BufferManager { data: input_data.into_bytes() };
        let proc_inst = Processor { manager: Arc::new(Mutex::new(manager)) };
        let result = proc_inst.run(3, 5);
        assert!(result.is_err(), "Expected an error due to out-of-bound extraction");
        assert_eq!(result.err().unwrap(), "out of bounds");
    }
}
