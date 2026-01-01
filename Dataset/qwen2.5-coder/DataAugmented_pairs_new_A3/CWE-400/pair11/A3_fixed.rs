struct Service {
    data: Vec<i32>,
}

impl Service {
    pub fn new() -> Self {
        Service { data: Vec::new() }
    }

    pub fn compute(&mut self, value: i32) {
        const MAX_ITEMS: usize = 10;
        if self.data.len() >= MAX_ITEMS {
            return;
        }
        self.data.push(value);
    }
}