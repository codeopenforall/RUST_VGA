struct Handler {
    buf: Vec<u8>,
}

impl Handler {
    fn new(size: usize) -> Self {
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
        }
        Handler { buf }
    }

    pub fn update(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            ptr.add(index).write(value);
        }
        Ok(())
    }
}

fn main() {
    let mut handler = Handler::new(10);
    let _ = handler.update(10, 255);
    unsafe {
        let val = *handler.buf.as_ptr().add(10);
        println!("Main (vulnerable): value at index 10 is {}", val);
    }
}