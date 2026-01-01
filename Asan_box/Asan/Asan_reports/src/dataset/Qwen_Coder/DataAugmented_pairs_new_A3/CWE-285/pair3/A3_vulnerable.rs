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

//////////////////////////
// Test Oracle Start
//////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authorization() {
        // Create the resource with an initial value of 10.
        let resource = Resource::new(10);
        
        // Attempt to update with an invalid token.
        let result = resource.attempt_update(42, "bad_token");
        
        // In the secure version, the update should be rejected.
        // In the vulnerable version, the update would succeed silently.
        // Therefore, we assert that the update attempt returns an error
        // and that the resource's value remains unchanged.
        match result {
            Ok(_) => {
                // For the vulnerable version, the resource value would have been changed.
                assert_eq!(resource.read(), 10, "Unauthorized update succeeded incorrectly");
            },
            Err(e) => {
                assert_eq!(e, "Unauthorized", "Expected an unauthorized error");
                assert_eq!(resource.read(), 10, "Resource value changed on failed update");
            }
        }
    }
}
//////////////////////////
// Test Oracle End
//////////////////////////
