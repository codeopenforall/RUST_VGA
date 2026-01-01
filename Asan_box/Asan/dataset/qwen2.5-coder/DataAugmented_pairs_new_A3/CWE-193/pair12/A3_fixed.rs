struct DataGuard {
    data: Vec<u32>,
}

impl DataGuard {
    fn new(size: usize) -> Self {
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        DataGuard { data }
    }

    fn populate(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = i as u32;
        }
    }
}

fn compute() -> u32 {
    let mut guard = DataGuard::new(10);
    guard.populate();
    guard.data.iter().sum()
}