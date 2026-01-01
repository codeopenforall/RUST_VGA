use std::sync::Arc;
use std::thread;
struct Processor;
impl Processor {
    fn run(&self, input: u32) -> u32 {
        let data = Arc::new(input);
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            unsafe {
                let raw = Arc::into_raw(data_clone);
                let value = *raw;
                let result = Self::process(value);
                match result {
                    Ok(v) => v,
                    Err(_) => {
                        0 // Vulnerable: Returning 0 instead of panicking
                    }
                }
            }
        });
        let res = handle.join().unwrap();
        res
    }
    fn process(value: u32) -> Result<u32, u32> {
        if value < 100 {
            Err(value)
        } else {
            Ok(value)
        }
    }
}
fn main() {
    let proc = Processor;
    let res = proc.run(150);
    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_boundary() {
        let processor = Processor;

        // This call should panic when processing an input below the threshold.
        // The fixed version panics, whereas the vulnerable version wrongly returns a value.
        let result = panic::catch_unwind(|| {
            processor.run(50)
        });
        assert!(result.is_err(), "Expected the operation to panic for input below threshold");
    }
}
