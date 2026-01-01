use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    pub fn compute_sum(&self, start: usize, count: usize) -> Result<u32, &'static str> {
        let mut sum = 0;
        unsafe {
            for i in 0..count {
                if start + i >= self.data.len() {
                    return Err("Out-of-bound access");
                }
                sum += *self.data.get_unchecked(start + i);
            }
        }
        Ok(sum)
    }
}

fn main() {
    let buffer = Arc::new(Buffer { data: vec![1, 2, 3, 4] });
    let buf_clone = Arc::clone(&buffer);
    let handle = thread::spawn(move || {
        let result = buf_clone.compute_sum(0, 5);
        match result {
            Ok(sum) => sum,
            Err(e) => {
                eprintln!("Error: {}", e);
                0
            }
        }
    });
    let safe_sum: u32 = buffer.data.iter().sum();
    let unsafe_sum = handle.join().unwrap();
    println!("Safe sum: {}, Computed sum: {}", safe_sum, unsafe_sum);
}

fn test_oracle() {
    // This oracle verifies that an out-of-bound access is properly detected.
    // For the correct implementation, calling compute_sum with a range extending
    // beyond the available data should return an error.
    let buffer = Buffer { data: vec![1, 2, 3, 4] };
    let result = buffer.compute_sum(0, 5);
    match result {
        Err(msg) => {
            assert_eq!(msg, "Out-of-bound access", "Expected out-of-bound error");
        }
        Ok(sum) => {
            panic!("Test oracle failed: expected error for out-of-bound access, but got sum {}.", sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oracle() {
        test_oracle();
    }
}
