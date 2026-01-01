struct BufferManager {
    data: Vec<u8>,
}

impl BufferManager {
    fn get_segment_safe(&self, start: usize, length: usize) -> Result<&str, &'static str> {
        let end = start.checked_add(length).ok_or("overflow in parameters")?;
        if end > self.data.len() {
            return Err("out of bounds");
        }
        let slice = &self.data[start..end];
        std::str::from_utf8(slice).map_err(|_| "invalid utf8")
    }
}

struct Processor {
    manager: std::sync::Arc<std::sync::Mutex<BufferManager>>,
}

impl Processor {
    fn run(&self, start: usize, length: usize) -> Result<&str, &'static str> {
        let manager = self.manager.lock().unwrap();
        manager.get_segment_safe(start, length)
    }
}