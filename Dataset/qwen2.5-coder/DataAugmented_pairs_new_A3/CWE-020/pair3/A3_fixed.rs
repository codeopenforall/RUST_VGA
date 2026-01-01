struct Config {
    data: Vec<i32>,
}

impl Config {
    pub fn new() -> Self {
        Config { data: vec![1, 2, 3, 4, 5] }
    }

    pub unsafe fn process(&self, index: &str) -> Result<i32, String> {
        let idx: usize = match index.parse() {
            Ok(num) => num,
            Err(_) => return Err("Invalid index".to_string()),
        };

        if idx >= self.data.len() {
            return Err("Index out of bounds".to_string());
        }

        Ok(self.data[idx])
    }
}