pub struct Service {
    data: Vec<i32>,
}

impl Service {
    pub fn new() -> Self {
        Service { data: Vec::new() }
    }

    pub fn compute(&mut self, value: i32) {
        unsafe {
            let ptr = self.data.as_mut_ptr().add(self.data.len());
            *ptr = value;
        }
        self.data.push(value); // This line is redundant and causes issues
    }
}