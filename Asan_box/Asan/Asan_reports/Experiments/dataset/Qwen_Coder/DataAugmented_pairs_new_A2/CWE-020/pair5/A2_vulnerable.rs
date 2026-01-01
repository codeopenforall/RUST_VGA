use std::env;
use std::sync::Arc;
use std::thread;

struct Processor {
    info: Box<[u8]>,
}

impl Processor {
    fn new() -> Self {
        let mut vec = vec![1u8; 100];
        vec[50] = 255;
        Processor {
            info: vec.into_boxed_slice(),
        }
    }

    fn process(&self, index_str: &str) -> Result<(), &'static str> {
        let index = usize::from_str_radix(index_str, 16).map_err(|_| "parse error")?;
        let mut temp_buffer = vec![0u8; 10];
        unsafe {
            let source_ptr = self.info.as_ptr().add(index);
            std::ptr::copy_nonoverlapping(source_ptr, temp_buffer.as_mut_ptr(), 10);
        }
        let total: u8 = temp_buffer.iter().fold(0, |acc, &x| acc.wrapping_add(x));
        if total == 0 {
            Err("data sum is zero")
        } else {
            Ok(())
        }
    }
}

fn launch_app() {
    let arguments: Vec<String> = env::args().collect();
    let input = if arguments.len() > 1 { &arguments[1] } else { "00" };
    let processor = Arc::new(Processor::new());
    let processor_clone = Arc::clone(&processor);
    let input_clone = input.to_string();
    let thread_handle = thread::spawn(move || {
        match processor_clone.process(&input_clone) {
            Ok(_) => println!("Parallel execution succeeded."),
            Err(e) => println!("Parallel execution failed: {}", e),
        }
    });
    match processor.process(input) {
        Ok(_) => println!("Primary execution succeeded."),
        Err(e) => println!("Primary execution failed: {}", e),
    }
    thread_handle.join().unwrap();
}

fn main() {
    launch_app();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_oracle() {
        // The test uses an input that should trigger an out-of-bounds access if unchecked.
        // "95" in hexadecimal equals 149, so 149 + 10 = 159, which is greater than 100.
        let handler = Handler::new();
        let malicious_input = "95";
        let result = panic::catch_unwind(|| handler.execute(malicious_input));

        // In the fixed code, the execute method returns an error ("index out of bounds").
        // In the vulnerable code, the unsafe copy may succeed (or invoke undefined behavior) leading to an Ok result, failing the test.
        match result {
            Ok(Ok(_)) => panic!("Test failed: Expected error on out-of-bounds input, but execution succeeded."),
            Ok(Err(e)) => assert_eq!(e, "index out of bounds", "Expected specific error message on out-of-bounds input."),
            Err(_) => panic!("Test failed: Execution panicked on out-of-bounds input."),
        }
    }
}
