use std::env;
use std::sync::Arc;
use std::thread;
struct Handler {
    data: Box<[u8]>,
}
impl Handler {
    fn new() -> Self {
        let mut vec = vec![1u8; 100];
        vec[50] = 255; 
        Handler {
            data: vec.into_boxed_slice(),
        }
    }
    fn execute(&self, input: &str) -> Result<(), &'static str> {
        let idx = usize::from_str_radix(input, 16).map_err(|_| "parse error")?;
        if idx + 10 > self.data.len() {
            return Err("index out of bounds");
        }
        let mut buffer = vec![0u8; 10];
        unsafe {
            let src = self.data.as_ptr().add(idx);
            std::ptr::copy_nonoverlapping(src, buffer.as_mut_ptr(), 10);
        }
        let sum: u8 = buffer.iter().fold(0, |acc, &x| acc.wrapping_add(x));
        if sum == 0 {
            Err("data sum is zero")
        } else {
            Ok(())
        }
    }
}
fn run_app() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "00" };
    let handler = Arc::new(Handler::new());
    let handler_clone = Arc::clone(&handler);
    let input_clone = input.to_string();
    let handle = thread::spawn(move || {
        match handler_clone.execute(&input_clone) {
            Ok(_)  => println!("Concurrent execution succeeded."),
            Err(e) => println!("Concurrent execution failed: {}", e),
        }
    });
    match handler.execute(input) {
        Ok(_)  => println!("Main execution succeeded."),
        Err(e) => println!("Main execution failed: {}", e),
    }
    handle.join().unwrap();
}
fn main() {
    run_app();
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
