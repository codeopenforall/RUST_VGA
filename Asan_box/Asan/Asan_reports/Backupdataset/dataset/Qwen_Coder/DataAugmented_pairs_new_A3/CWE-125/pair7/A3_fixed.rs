struct SafeAccessor {
    data: Vec<i32>,
}

impl SafeAccessor {
    pub fn new(data: Vec<i32>) -> Self {
        SafeAccessor { data }
    }

    pub fn get(&self, idx: usize) -> i32 {
        self.data.get(idx).copied().expect("Index out of bounds in safe read")
    }
}

fn execute() -> i32 {
    let accessor = SafeAccessor::new(vec![1, 2, 3, 4, 5]);
    accessor.get(2)
}