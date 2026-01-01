struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn inject_safe(&mut self, count: usize, value: u32) {
        if count > self.data.capacity() {
            panic!("Count {} exceeds capacity {}", count, self.data.capacity());
        }
        self.data.clear();
        for _ in 0..count {
            self.data.push(value);
        }
    }
}

pub fn run() -> u32 {
    let mut holder = Buffer::new(10);
    holder.inject_safe(10, 42);
    holder.data.iter().sum()
}