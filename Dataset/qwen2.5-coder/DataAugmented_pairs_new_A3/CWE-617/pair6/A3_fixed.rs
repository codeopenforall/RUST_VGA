pub struct ProcessData {
    pub value: i32,
}

impl ProcessData {
    pub fn new(initial_value: i32) -> Self {
        ProcessData {
            value: initial_value,
        }
    }

    pub fn alter(&mut self, new_val: i32) {
        if new_val < 100 {
            self.value = new_val;
        }
    }
}