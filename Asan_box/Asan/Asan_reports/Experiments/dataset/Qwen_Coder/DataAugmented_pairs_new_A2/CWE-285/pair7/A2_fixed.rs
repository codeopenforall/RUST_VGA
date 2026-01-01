use std::sync::Arc;
use std::thread;

struct SecretResource {
    data: i32,
    owner: String,
}

impl SecretResource {
    pub fn get_data(&self, user: &str) -> Result<i32, &'static str> {
        if user == self.owner {
            Ok(self.data)
        } else {
            Err("Unauthorized access")
        }
    }
}

fn main() {
    let resource = Arc::new(SecretResource { data: 42, owner: "admin".to_string() });
    let resource_clone = Arc::clone(&resource);
    let handle = thread::spawn(move || {
        resource_clone.get_data("admin")
    });
    match handle.join().unwrap() {
        Ok(data) => println!("Resource Data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    // Assuming the tested code re-exports the SecretResource definition in the current module.
    use super::SecretResource;
    
    #[test]
    fn test_access_control() {
        // Create the protected resource owned by "admin".
        let resource = Arc::new(SecretResource { data: 42, owner: "admin".to_string() });
        let resource_clone = Arc::clone(&resource);
        
        // Attempt to access the resource using an unauthorized user identifier.
        let handle = thread::spawn(move || {
            // Using an unsafe block to accommodate both versions.
            // In the improved version, this call will return an error.
            unsafe { resource_clone.get_data("attacker") }
        });
        
        let result = handle.join().unwrap();
        
        // The test expects an error message "Unauthorized access".
        // In the vulnerable version, the call erroneously returns Ok(42),
        // which will trigger a test failure.
        match result {
            Ok(_) => panic!("Test failed: unauthorized access granted"),
            Err(msg) => assert_eq!(msg, "Unauthorized access"),
        }
    }
}
