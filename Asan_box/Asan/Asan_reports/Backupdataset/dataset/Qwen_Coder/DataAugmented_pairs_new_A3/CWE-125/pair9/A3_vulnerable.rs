struct Data {
    data: Vec<i32>,
}

impl Data {
    pub fn get(&self, index: usize) -> Result<i32, &'static str> {
        unsafe {
            Ok(*self.data.as_ptr().add(index))
        }
    }
}

pub fn execute(index: usize) -> Result<i32, &'static str> {
    let data = Data { data: vec![1, 2, 3] };
    data.get(index)
}