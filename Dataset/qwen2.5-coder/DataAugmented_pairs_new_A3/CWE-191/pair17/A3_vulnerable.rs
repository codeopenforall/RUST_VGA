pub struct Data {
    value: u32,
}

impl Data {
    pub fn new(value: u32) -> Self {
        Data { value }
    }

    pub fn subtract(&mut self, amt: u32) -> Result<u32, &'static str> {
        unsafe {
            let ptr: *mut u32 = &mut self.value as *mut u32;
            *ptr = *ptr - amt;
        }
        Ok(self.value)
    }
}