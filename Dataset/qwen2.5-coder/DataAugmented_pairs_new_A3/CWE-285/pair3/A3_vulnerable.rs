use std::sync::Mutex;

pub struct Resource {
    data: Mutex<i32>,
}

impl Resource {
    pub fn new(initial_value: i32) -> Self {
        Resource {
            data: Mutex::new(initial_value),
        }
    }

    pub fn update(&self, new_val: i32) {
        unsafe {
            let ptr: *const Mutex<i32> = &self.data;
            let mut_ptr = ptr as *mut Mutex<i32>;
            if let Ok(mut guard) = (*mut_ptr).lock() {
                *guard = new_val;
            }
        }
    }

    pub fn attempt_update(&self, new_val: i32, _token: &str) -> Result<(), &'static str> {
        self.update(new_val);
        Ok(())
    }

    pub fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}