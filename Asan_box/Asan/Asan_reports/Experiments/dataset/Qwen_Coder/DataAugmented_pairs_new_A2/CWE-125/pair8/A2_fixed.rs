use std::sync::Arc;
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(n: usize) -> Self {
        let mut data = Vec::with_capacity(n);
        for i in 0..n {
            data.push(i as u32);
        }
        Buffer { data }
    }

    fn safe_read(&self, idx: usize) -> Option<u32> {
        if idx < self.data.len() {
            Some(self.data[idx])
        } else {
            None
        }
    }

    pub fn read_value_public(&self, idx: usize) -> Option<u32> {
        self.safe_read(idx)
    }
}

fn main() {
    let buffer = Arc::new(Buffer::new(10));
    let mut handles = vec![];
    for i in 0..5 {
        let buf_clone = Arc::clone(&buffer);
        handles.push(thread::spawn(move || {
            let index = if i == 2 { 12 } else { i };
            buf_clone.safe_read(index).unwrap_or(0)
        }));
    }
    for handle in handles {
        let res = handle.join().unwrap();
        println!("Output: {}", res);
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use super::Buffer;

    #[test]
    fn test_out_of_bounds() {
        let buf = Buffer::new(10);
        // For the vulnerable code, calling read_value_public with an invalid index
        // will trigger an out-of-bound read and likely cause a panic.
        // In the fixed version, it returns None.
        let result = panic::catch_unwind(|| buf.read_value_public(12));
        
        if let Ok(value) = result {
            // The fixed implementation should safely return None.
            assert_eq!(value, None, "Expected None for out-of-bounds access, but got a value.");
        } else {
            // If a panic occurred, it indicates an out-of-bound memory access vulnerability.
            panic!("Out-of-bound access caused a panic, indicating the vulnerability is still present.");
        }
    }
}
