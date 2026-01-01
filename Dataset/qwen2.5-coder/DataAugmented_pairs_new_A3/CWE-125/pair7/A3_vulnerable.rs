struct SafeAccess {
    data: Vec<i32>,
}

impl SafeAccess {
    pub fn new(data: Vec<i32>) -> Self {
        SafeAccess { data }
    }

    pub fn execute(&self) -> i32 {
        let idx = 10; // Intentionally out-of-bounds index
        unsafe { *self.data.get_unchecked(idx) }
    }
}

pub fn execute() -> i32 {
    let safe_access = SafeAccess::new(vec![1, 2, 3, 4, 5]);
    safe_access.execute()
}