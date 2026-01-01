struct MemoryHandler {
    data: Vec<i32>,
}

impl MemoryHandler {
    pub fn new() -> Self {
        MemoryHandler {
            data: vec![0; 100], // Example size
        }
    }

    pub fn update(&mut self, idx: usize, value: i32) -> bool {
        if idx < self.data.len() {
            unsafe {
                *self.data.get_unchecked_mut(idx) = value;
            }
            true
        } else {
            false
        }
    }

    pub fn get(&self, idx: usize) -> i32 {
        if idx < self.data.len() {
            unsafe { *self.data.get_unchecked(idx) }
        } else {
            0
        }
    }
}