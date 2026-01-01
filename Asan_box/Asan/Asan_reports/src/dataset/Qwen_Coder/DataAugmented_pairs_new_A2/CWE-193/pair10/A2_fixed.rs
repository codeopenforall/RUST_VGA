use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor;

impl DataProcessor {
    pub fn process(&self, input: &[u8]) -> Vec<u8> {
        input.to_vec()
    }
}

fn run(input: &[u8]) -> Vec<u8> {
    let processor = Arc::new(DataProcessor);
    let data = Arc::new(input.to_vec());
    let output = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for _ in 0..4 {
        let proc_clone = Arc::clone(&processor);
        let data_clone = Arc::clone(&data);
        let out_clone = Arc::clone(&output);
        let handle = thread::spawn(move || {
            let res = proc_clone.process(&data_clone);
            let mut guard = out_clone.lock().unwrap();
            *guard = res;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Arc::try_unwrap(output).unwrap().into_inner().unwrap()
}

fn main() {
    let input = b"abcdef";
    let output = run(input);
    println!("Output length: {}", output.len());
}

//////////////////////////////////////////////////////////////
// Test Oracle Function
// This test function invokes the 'run' function with a known input and asserts
// that the output's length and content match the expected values. The test is designed
// to fail against the vulnerable implementation (which returns a vector missing the last byte)
// and pass with the corrected version.
//////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_processing() {
        let input = b"abcdef"; // Expected 6 bytes.
        let output = run(input);
        // Assert the output length equals the input length.
        assert_eq!(output.len(), input.len(), "Output length mismatch");
        // Assert that the content exactly matches the input.
        assert_eq!(output, input.to_vec(), "Output content mismatch");
    }
}
