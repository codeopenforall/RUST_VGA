struct Container {
    data: Vec<i32>,
}

impl Container {
    fn new() -> Self {
        Container {
            data: (1..=10).collect(),
        }
    }
}

trait DataAccess {
    fn read(&self, idx: usize) -> i32;
}

impl DataAccess for Container {
    fn read(&self, idx: usize) -> i32 {
        if idx >= self.data.len() {
            panic!("Index {} is out of bounds (length {})", idx, self.data.len());
        }
        self.data[idx]
    }
}