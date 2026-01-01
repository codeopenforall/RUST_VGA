struct Buffer {
    data: Vec<u32>,
}

impl Buffer {
    fn new(size: usize) -> Self {
        Buffer { data: vec![0; size] }
    }

    fn store_safe(&mut self, index: usize, value: u32) -> Result<(), &'static str> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(())
        } else {
            Err("Index out of bounds")
        }
    }
}

trait Summable {
    fn total(&self) -> u32;
}

struct Worker {
    buffer: Buffer,
    total: u32,
}

impl Worker {
    fn new(size: usize) -> Self {
        Worker {
            buffer: Buffer::new(size),
            total: 0,
        }
    }

    fn run(&mut self) {
        if let Err(e) = self.buffer.store_safe(0, 100) {
            eprintln!("Error storing value: {}", e);
        } else {
            self.total = self.buffer.data.iter().sum();
        }
    }
}

impl Summable for Worker {
    fn total(&self) -> u32 {
        self.total
    }
}