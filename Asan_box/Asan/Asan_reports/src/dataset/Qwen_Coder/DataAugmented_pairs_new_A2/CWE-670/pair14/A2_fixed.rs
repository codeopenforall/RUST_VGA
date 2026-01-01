use std::sync::{Arc, Mutex};
use std::thread;

struct Processor;

impl Processor {
    fn verify(&self, token: &str) -> bool {
        match token {
            "admin" => true,
            _ => false,
        }
    }

    unsafe fn get_value(&self) -> i32 {
        1337
    }

    fn process(&self, token: &str) -> i32 {
        if self.verify(token) {
            unsafe { self.get_value() }
        } else {
            0
        }
    }
}

fn main() {
    let proc = Arc::new(Mutex::new(Processor));
    let tokens = vec!["admin", "user"];
    let mut handles = vec![];

    for tok in tokens {
        let proc_clone = Arc::clone(&proc);
        let token = tok.to_owned();
        handles.push(thread::spawn(move || {
            let processor = proc_clone.lock().unwrap();
            processor.process(&token)
        }));
    }

    for handle in handles {
        let result = handle.join().unwrap();
        println!("Result: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // This test oracle is intended to evaluate the processing of the "admin" token.
    // For the correct implementation, processing "admin" must yield 1337.
    // In the flawed version, it would incorrectly return 0.
    #[test]
    fn test_authorization() {
        let processor = Processor;
        let result = processor.process("admin");
        assert_eq!(result, 1337, "Processing with an 'admin' token must yield 1337");
    }
}
