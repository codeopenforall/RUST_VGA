pub struct DataContainer {
    data: Vec<i32>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        unsafe { v.set_len(size); }
        for i in 0..size {
            v[i] = i as i32;
        }
        DataContainer { data: v }
    }

    pub fn sum(&self) -> i32 {
        self.data.iter().sum()
    }
}