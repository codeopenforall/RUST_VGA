use std::env;
use std::sync::Arc;
use std::thread;
use std::str;
struct Processor {
    buffer: Vec<u8>,
}
impl Processor {
    pub fn new() -> Self {
        Processor { buffer: vec![b'A'; 10] }
    }
    pub fn process(&self, input: &str) -> String {
        // Parse the user-controlled input as a length value.
        let len: usize = match input.parse() {
            Ok(len) => len,
            Err(_) => return "Invalid input size".to_string(),
        };
        // Check if len is within the bounds of the buffer.
        if len > self.buffer.len() {
            return "Invalid input size".to_string();
        }
        // SAFE USAGE: Ensure len is within the bounds of the buffer.
        let slice = &self.buffer[..len];
        match str::from_utf8(slice) {
            Ok(result) => result.to_string(),
            Err(_) => "Invalid UTF-8 sequence".to_string(),
        }
    }
}
fn main() {
    // Read command line argument; default will be "15" to trigger the vulnerability.
    let arg = env::args().nth(1).unwrap_or_else(|| "15".to_string());
    let proc_inst = Arc::new(Processor::new());
    // Spawn a thread to simulate concurrent processing.
    let proc_clone = Arc::clone(&proc_inst);
    let handle = thread::spawn(move || {
        // This call can invoke undefined behavior if input length exceeds the allocated buffer.
        let output = proc_clone.process(&arg);
        println!("Output: {}", output);
    });
    handle.join().expect("Thread panicked");
}