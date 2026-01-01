use std::env;
use std::sync::Arc;
use std::thread;
use std::fmt;

struct DataProcessor;

#[derive(Debug)]
struct LengthExceedsInputSize;

impl fmt::Display for LengthExceedsInputSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Declared length exceeds input size")
    }
}

impl std::error::Error for LengthExceedsInputSize {}

impl DataProcessor {
    fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
        if input.len() < 4 {
            return Err(Box::new(LengthExceedsInputSize));
        }
        let len_str = &input[..4];
        let len: usize = len_str.parse().map_err(|_| LengthExceedsInputSize)?;
        if input.len() < 4 + len {
            return Err(Box::new(LengthExceedsInputSize));
        }
        let data_slice = &input[4..4 + len];
        let result = unsafe { std::str::from_utf8_unchecked(data_slice.as_bytes()) };
        Ok(result.to_string())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <input>", args[0]);
        return;
    }
    let input = args[1].as_str();
    let shared = Arc::new(DataProcessor);
    let mut threads = vec![];
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let input_clone = input.to_string();
        let handle = thread::spawn(move || {
            let result = shared_clone.process(&input_clone);
            match result {
                Ok(output) => println!("Output: {}", output),
                Err(e) => eprintln!("Error: {}", e),
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}