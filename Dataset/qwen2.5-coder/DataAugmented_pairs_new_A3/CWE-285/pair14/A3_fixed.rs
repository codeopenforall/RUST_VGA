use std::sync::{Arc, Mutex};

struct Data {
    owner: String,
    secret: i32,
}

pub struct Controller {
    inner: Arc<Mutex<Data>>,
}

impl Controller {
    pub fn new(owner: &str, secret: i32) -> Self {
        Controller {
            inner: Arc::new(Mutex::new(Data {
                owner: owner.to_string(),
                secret,
            })),
        }
    }

    pub fn modify(&self, caller: &str, new_val: i32) -> Result<(), &'static str> {
        let mut guard = self.inner.lock().unwrap();
        if caller != guard.owner {
            return Err("Unauthorized access");
        }
        guard.secret = new_val;
        Ok(())
    }

    pub fn read(&self) -> i32 {
        let guard = self.inner.lock().unwrap();
        guard.secret
    }
}