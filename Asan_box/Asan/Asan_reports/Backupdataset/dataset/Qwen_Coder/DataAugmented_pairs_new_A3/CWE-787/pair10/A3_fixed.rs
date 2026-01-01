struct DataPool {
    buffer: Vec<u8>,
}

impl DataPool {
    fn new(size: usize) -> Self {
        let buffer = vec![0u8; size];
        DataPool { buffer }
    }

    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        if index < self.buffer.len() {
            self.buffer[index] = value;
            Ok(())
        } else {
            Err("Index out-of-bounds")
        }
    }
}

trait Modifier {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str>;
}

impl Modifier for DataPool {
    fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        self.modify(index, value)
    }
}