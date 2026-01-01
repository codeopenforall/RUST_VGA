use std::sync::Arc;
use std::thread;

trait Summable {
    fn calculate(&self) -> u32;
}

struct BufferHandler {
    buf: Vec<u8>,
}

impl BufferHandler {
    fn new() -> Self {
        BufferHandler { buf: Vec::new() }
    }

    fn load(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }
}

impl Summable for BufferHandler {
    fn calculate(&self) -> u32 {
        self.buf.iter().map(|&x| x as u32).sum()
    }
}

fn main() {
    let mut handler = BufferHandler::new();
    handler.load(&[1, 2, 3, 4]);
    let shared_handler = Arc::new(handler);
    let handler_clone = Arc::clone(&shared_handler);
    let worker = thread::spawn(move || handler_clone.calculate());
    let result = worker.join().expect("Worker thread panicked");
    println!("Computed sum: {}", result);
}