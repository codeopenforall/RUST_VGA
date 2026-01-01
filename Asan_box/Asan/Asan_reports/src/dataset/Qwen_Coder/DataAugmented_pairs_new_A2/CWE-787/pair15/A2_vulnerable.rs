struct DataBuffer {
    data: Vec<u32>,
}

impl DataBuffer {
    pub fn new(size: usize) -> DataBuffer {
        let mut data = Vec::with_capacity(size);
        unsafe {
            data.set_len(size);
        }
        DataBuffer { data }
    }

    pub fn update(&mut self, index: usize, value: u32) {
        if index < self.data.len() {
            unsafe {
                *self.data.as_mut_ptr().add(index) = value;
            }
        } else {
            // Introduce a vulnerability by not resizing the vector
            unsafe {
                *self.data.as_mut_ptr().add(index) = value;
            }
        }
    }
}

fn main() {
    let mut buffer = DataBuffer::new(5);
    buffer.update(2, 100);
    buffer.update(7, 200);
    println!(
        "Buffer updated. Value at index 2: {}",
        unsafe { *buffer.data.as_ptr().add(2) }
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Oracle function: In the vulnerable version, an out-of-bound write can corrupt memory 
    // and the following assertion may fail or lead to undefined behavior.
    // In the fixed version, the update() method safely resizes the vector so that index 7 can be updated.
    #[test]
    fn test_buffer_update() {
        // Use the safe (fixed) approach by instantiating DataBuffer as defined in the fixed version.
        let mut buffer = DataBuffer::new(5);
        buffer.update(2, 100);
        buffer.update(7, 200);
        // Assert that the value at index 7 is as expected.
        assert_eq!(buffer.data[7], 200, "The update did not correctly set the value at index 7");
    }
}
