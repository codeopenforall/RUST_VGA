struct DataPool {
    buffer: Vec<u8>,
}

impl DataPool {
    pub fn new(size: usize) -> Self {
        let mut buffer = Vec::with_capacity(size);
        unsafe {
            buffer.set_len(size);
        }
        DataPool { buffer }
    }

    pub fn modify(&mut self, index: usize, value: u8) -> Result<(), &'static str> {
        unsafe {
            let ptr = self.buffer.as_mut_ptr();
            *ptr.add(index) = value;
        }
        Ok(())
    }
}

pub struct Modifier;

impl Modifier {
    pub fn modify(&mut self, _index: usize, _value: u8) -> Result<(), &'static str> {
        Ok(())
    }
}