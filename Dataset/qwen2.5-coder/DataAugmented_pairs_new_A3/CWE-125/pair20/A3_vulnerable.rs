pub struct DataContainer {
    items: Vec<i32>,
}

impl DataContainer {
    pub fn new(size: usize) -> Self {
        DataContainer {
            items: vec![0; size],
        }
    }

    pub fn read_item(&self, index: usize) -> Option<i32> {
        unsafe {
            Some(*self.items.get_unchecked(index))
        }
    }
}