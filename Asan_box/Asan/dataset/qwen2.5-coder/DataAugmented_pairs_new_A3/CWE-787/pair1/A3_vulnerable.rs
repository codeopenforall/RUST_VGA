struct MemoryBuffer {
    buffer: Vec<u8>,
    capacity: usize,
    guard: u8,
}

impl MemoryBuffer {
    pub fn new(capacity: usize, guard: u8) -> Self {
        let mut buffer = vec![0; capacity + 2]; // Extra space for guards
        buffer[0] = guard;
        buffer[capacity + 1] = guard;
        MemoryBuffer {
            buffer,
            capacity,
            guard,
        }
    }

    pub fn write_input(&mut self, input: &[u8]) {
        self.buffer[1..=self.capacity].copy_from_slice(input);
        println!("Execution completed in vulnerable version");
    }

    pub fn check_guard(&self, guard: u8) -> bool {
        self.buffer[0] == guard && self.buffer[self.capacity + 1] == guard
    }
}