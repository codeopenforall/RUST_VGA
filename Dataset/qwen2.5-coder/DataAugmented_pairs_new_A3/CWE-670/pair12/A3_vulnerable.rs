use std::sync::{Arc, Mutex};

pub struct Engine {
    flag: Arc<Mutex<u32>>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            flag: Arc::new(Mutex::new(0)),
        }
    }

    pub fn process(&self, input: u32) -> Result<u32, &'static str> {
        match input {
            0 => {
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    let bad_ptr = base_ptr.offset(1);
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt)
                }
            },
            1 | _ => {
                let base_ptr = &self.flag as *const Arc<Mutex<u32>>;
                unsafe {
                    let bad_ptr = base_ptr.offset(1);
                    if bad_ptr.is_null() {
                        return Err("null pointer encountered");
                    }
                    let misplaced_ref: &Arc<Mutex<u32>> = &*bad_ptr;
                    let cnt = misplaced_ref.lock().map_err(|_| "mutex poisoned")?;
                    Ok(*cnt)
                }
            },
        }
    }
}