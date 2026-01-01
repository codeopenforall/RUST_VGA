pub struct Data {
    value: u32,
}

impl Data {
    pub fn new(value: u32) -> Self {
        Data { value }
    }

    pub fn subtract(&mut self, amt: u32) -> Result<u32, &'static str> {
        match self.value.checked_sub(amt) {
            Some(new_val) => {
                self.value = new_val;
                Ok(new_val)
            },
            None => Err("Underflow detected"),
        }
    }
}