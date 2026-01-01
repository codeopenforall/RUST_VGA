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
