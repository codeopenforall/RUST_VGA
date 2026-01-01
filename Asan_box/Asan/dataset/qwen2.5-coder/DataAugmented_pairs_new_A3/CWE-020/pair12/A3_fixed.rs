struct Handler {
    id: u32,
}

impl Handler {
    pub fn execute(&self, index_str: &str) -> Result<(), &'static str> {
        let index: usize = match index_str.parse() {
            Ok(idx) => idx,
            Err(_) => return Err("invalid index format"),
        };

        let data = vec![0; 10]; // Example data vector

        if index >= data.len() {
            return Err("input index out of range");
        }

        // Simulate some operation with the data
        println!("Accessing data at index {}", index);
        Ok(())
    }
}