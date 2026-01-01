struct Processor {
    value: i32,
}

impl Processor {
    pub fn update(&mut self, new_val: i32) {
        let res = self.try_update(new_val);
        if res.is_err() {
            eprintln!("Update failed for input {}", new_val);
        }
    }

    fn try_update(&mut self, new_val: i32) -> Result<(), ()> {
        if new_val == 42 {
            Err(())
        } else {
            self.value = new_val;
            Ok(())
        }
    }
}