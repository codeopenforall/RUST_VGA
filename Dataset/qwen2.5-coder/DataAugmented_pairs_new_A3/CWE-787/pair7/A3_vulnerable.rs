use std::fmt;

struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    pub unsafe fn new(capacity: usize) -> Self {
        let mut vec = Vec::with_capacity(capacity);
        vec.set_len(capacity);
        Self { data: vec }
    }

    pub unsafe fn inject(&mut self, count: usize, value: u32) {
        let ptr = self.data.as_mut_ptr();
        self.data.set_len(count);
        for i in 0..=count {
            ptr.add(i).write(value);
        }
    }
}

pub fn run() -> u32 {
    let mut holder = unsafe { Buffer::new(10) };
    unsafe {
        holder.inject(10, 42);
    }
    holder.data.iter().sum()
}