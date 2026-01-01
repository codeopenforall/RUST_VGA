struct Container {
    data: Vec<i32>,
}

impl Container {
    pub fn new() -> Self {
        Container {
            data: (1..=10).collect(),
        }
    }

    pub fn read(&self, idx: usize) -> i32 {
        if idx >= self.data.len() {
            // Intentionally return a wrong value instead of panicking
            return -1;
        }
        self.data[idx]
    }
}

trait DataAccess {
    fn read(&self, idx: usize) -> i32;
}

impl DataAccess for Container {
    fn read(&self, idx: usize) -> i32 {
        self.read(idx)
    }
}