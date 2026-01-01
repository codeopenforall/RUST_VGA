struct Handler {
    buf: Vec<u8>,
}

impl Handler {
    fn new(size: usize) -> Self {
        Handler {
            buf: vec![0u8; size],
        }
    }

    fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.buf.len() {
            self.buf[index] = value;
            Ok(())
        } else {
            Err("index out of bounds")
        }
    }
}

fn main() {
    let mut handler = Handler::new(10);
    match handler.update(10, 255) {
        Ok(_) => println!("Main (fixed): update succeeded unexpectedly."),
        Err(e) => println!("Main (fixed): error occurred: {}", e),
    }
}