pub struct Data {
    value: i32,
}

impl Data {
    pub fn new(value: i32) -> Self {
        Data { value }
    }

    pub fn update(&mut self, new_value: i32) -> Result<(), String> {
        if new_value < 0 {
            Err("Negative value not allowed".to_string())
        } else {
            self.value = new_value;
            Ok(())
        }
    }
}

pub fn run() {
    let mut data = Data::new(100);
    let data_clone = &mut data;

    // Intentionally ignore the Result returned by update
    let _ = data_clone.update(150);
}