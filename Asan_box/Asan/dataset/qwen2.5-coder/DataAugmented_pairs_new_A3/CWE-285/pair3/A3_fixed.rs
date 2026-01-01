use std::sync::{Arc, Mutex};

struct Resource {
    data: Mutex<i32>,
}

impl Resource {
    const AUTH_TOKEN: &str = "secret123";

    pub fn new(initial_value: i32) -> Arc<Self> {
        Arc::new(Resource {
            data: Mutex::new(initial_value),
        })
    }

    pub fn update(&self, new_val: i32, token: &str) -> Result<(), &'static str> {
        if token != Self::AUTH_TOKEN {
            return Err("Unauthorized");
        }
        let mut guard = self.data.lock().unwrap();
        *guard = new_val;
        Ok(())
    }

    pub fn attempt_update(&self, new_val: i32, token: &str) -> Result<(), &'static str> {
        self.update(new_val, token)
    }

    pub fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}