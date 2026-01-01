use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::io;
use std::ptr;
use std::str::FromStr;

struct Handler {
    data: Arc<Mutex<Vec<u8>>>,
}

impl Handler {
    fn new() -> Self {
        Handler {
            data: Arc::new(Mutex::new(vec![0; 10])),
        }
    }

    fn read_input(&self, filename: &str) -> io::Result<u8> {
        let content = fs::read_to_string(filename).unwrap_or_default();
        let val = u8::from_str(content.trim()).unwrap_or_default();
        Ok(val)
    }

    unsafe fn unsafe_update(&self, index: usize, val: u8) {
        let mut vec = self.data.lock().unwrap();
        let ptr = vec.as_mut_ptr();
        if index < vec.len() {
            *ptr.add(index) = val;
        }
    }

    fn process(&self, filename: &str, index: usize) -> io::Result<u8> {
        let value = self.read_input(filename)?;
        {
            let mut data = self.data.lock().unwrap();
            data[index] = value;
        }
        unsafe {
            self.unsafe_update(index, value);
        }
        Ok(value)
    }
}

fn main() {
    let handler = Handler::new();
    let handler_clone = Handler { data: handler.data.clone() };
    let t = thread::spawn(move || {
        let res = handler_clone.process("nonexistent.txt", 2);
        match res {
            Ok(val) => println!("Thread read value: {}", val),
            Err(e) => println!("Thread encountered error: {}", e),
        }
    });
    t.join().unwrap();
    let res_main = handler.process("nonexistent.txt", 4);
    match res_main {
        Ok(val) => println!("Main read value: {}", val),
        Err(e) => println!("Main encountered error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exception_propagation() {
         let handler = Handler::new();
         // For a missing file, the correct behavior is to return an error rather than a default value.
         let result = handler.process("nonexistent.txt", 0);
         match result {
             Ok(val) => {
                 // Vulnerable version will return Ok(0) leading to a silent failure.
                 assert!(false, "Expected an error due to missing file, but got Ok({})", val);
             },
             Err(e) => {
                 // Fixed version correctly propagates the error.
                 assert!(true, "Properly propagated error: {}", e);
             }
         }
    }
}
