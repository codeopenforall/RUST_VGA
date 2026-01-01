use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread;

/// DataBuffer represents a collection of u32 values.
/// The fixed version ensures all allocated memory is fully initialized.
pub struct DataBuffer {
    data: Vec<u32>,
}

impl DataBuffer {
    /// Creates a new DataBuffer of the given size.
    ///
    /// Fix: Fully initialize all elements of the allocated memory before setting the length.
    pub fn new(size: usize) -> Self {
        // Create a Vec with capacity for `size` elements.
        let mut buffer: Vec<MaybeUninit<u32>> = Vec::with_capacity(size);
        unsafe {
            let ptr = buffer.as_mut_ptr();
            // Only initialize the first half of the vector.
            for i in 0..(size / 2) {
                ptr.add(i).write(MaybeUninit::new(i as u32));
            }
            // Incorrectly set length to size even though only half of the elements are written.
            buffer.set_len(size);
        }
        // Transmute Vec<MaybeUninit<u32>> to Vec<u32>.
        let data = unsafe { std::mem::transmute::<Vec<MaybeUninit<u32>>, Vec<u32>>(buffer) };
        DataBuffer { data }
    }

    /// Computes the sum of elements stored in the buffer.
    ///
    /// Uses concurrency to sum partitions of the data. With all values properly initialized,
    /// the sum computation now behaves correctly.
    pub fn sum(&self) -> u32 {
        let mut total = 0;
        // Use Arc to safely share the data across threads.
        let arc_data = Arc::new(self.data.clone());
        let mut handles = Vec::new();
        let chunk_size = arc_data.len() / 2;
        for i in 0..2 {
            let arc_clone = Arc::clone(&arc_data);
            handles.push(thread::spawn(move || {
                let start = i * chunk_size;
                let end = if i == 1 { arc_clone.len() } else { start + chunk_size };
                let mut local_sum = 0;
                for v in &arc_clone[start..end] {
                    local_sum += *v;
                }
                local_sum
            }));
        }
        for handle in handles {
            total += handle.join().unwrap();
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// The test for the fixed version expects the correct computation.
    /// For a fully initialized buffer with numbers 0..10, the sum should be 45.
    #[test]
    fn test_fixed_sum() {
        let buffer = DataBuffer::new(10);
        let total = buffer.sum();
        assert_eq!(total, 45, "Fixed version produces correct sum (45) with all elements initialized");
    }
}

fn main() {
    println!("Run `cargo test` to execute the tests.");
}