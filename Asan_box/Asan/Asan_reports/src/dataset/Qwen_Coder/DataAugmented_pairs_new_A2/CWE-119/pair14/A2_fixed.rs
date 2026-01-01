use std::sync::{Arc, Mutex};
use std::thread;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0u32; size] }
    }

    fn write_data(&mut self, idx: usize, value: u32) {
        if idx < self.data.len() {
            self.data[idx] = value;
        }
    }

    fn read_data(&self, idx: usize) -> Option<u32> {
        self.data.get(idx).copied()
    }
}

trait Process {
    fn process(&self, count: usize) -> u32;
}

impl Process for Buffer {
    fn process(&self, count: usize) -> u32 {
        self.data
            .iter()
            .take(count.min(self.data.len()))
            .copied()
            .fold(0, |acc, x| acc.wrapping_add(x))
    }
}

fn execute(input: usize) -> u32 {
    let size = 10;
    let buffer = Arc::new(Mutex::new(Buffer::new(size)));
    {
        let buffer_clone = Arc::clone(&buffer);
        thread::spawn(move || {
            let mut buf = buffer_clone.lock().unwrap();
            if input < buf.data.len() {
                buf.write_data(input, 42);
            }
        })
        .join()
        .unwrap();
    }
    let buf = buffer.lock().unwrap();
    buf.process((input + 1).min(buf.data.len()))
}

fn main() {
    let input = 10;
    let result = execute(input);
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // Import the execute function from the current module.
    use super::execute;
    
    #[test]
    fn memory_safety_oracle() {
        // For a safe implementation, using a boundary value should not corrupt memory.
        // Expected output is 0 because the out-of-bound write is ignored.
        let output = execute(10);
        assert_eq!(output, 0, "Memory corruption detected: result should be 0.");
    }
}
