struct Data {
    data: Vec<i32>,
}

impl Data {
    pub fn new(data: Vec<i32>) -> Self {
        Data { data }
    }

    pub unsafe fn get(&self, idx: usize) -> Result<i32, &'static str> {
        let value = *self.data.get_unchecked(idx);
        Ok(value)
    }
}

pub fn run(idx: usize) -> Result<i32, &'static str> {
    let data = Data::new(vec![1, 2, 3]);
    unsafe { data.get(idx) }
}