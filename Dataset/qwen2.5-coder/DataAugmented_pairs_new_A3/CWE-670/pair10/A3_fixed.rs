struct App {
    data: Vec<i32>,
}

impl App {
    pub fn new(flag: bool) -> Self {
        if flag {
            App { data: vec![10, 20] }
        } else {
            App { data: vec![30, 40] }
        }
    }

    pub fn process(&self, idx: usize) -> Result<i32, &'static str> {
        self.data.get(idx).copied().ok_or("Index out of bounds")
    }
}