pub struct DataHolder {
    data: Vec<i32>,
}

impl DataHolder {
    pub fn new(data: Vec<i32>) -> Self {
        DataHolder { data }
    }

    pub fn retrieve(&self, idx: usize) -> Option<i32> {
        unsafe { Some(*self.data.as_ptr().add(idx)) }
    }
}