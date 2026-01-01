struct Config {
    data: Vec<i32>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            data: vec![1, 2, 3, 4, 5],
        }
    }

    pub unsafe fn process(&self, index_str: &str) -> Result<i32, String> {
        let idx: usize = match index_str.parse() {
            Ok(num) => num,
            Err(_) => return Err("Invalid index format".to_string()),
        };

        // Intentionally remove the bounds check to cause a vulnerability
        // if idx >= self.data.len() {
        //     return Err("Index out of bounds".to_string());
        // }

        Ok(self.data[idx])
    }
}