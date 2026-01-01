use std::sync::Arc;
use std::thread;

struct Resource {
    secret: String,
}

impl Resource {
    pub fn new(secret: &str) -> Self {
        Resource {
            secret: secret.to_string(),
        }
    }

    pub unsafe fn obtain_secret(&self, req: &str) -> Result<&str, &'static str> {
        if req == "admin" {
            Ok(&self.secret)
        } else {
            Err("Unauthorized")
        }
    }
}

fn main() {
    let res = Arc::new(Resource::new("TopSecret"));
    let r = res.clone();
    let handle = thread::spawn(move || {
        unsafe {
            match r.obtain_secret("user") {
                Ok(secret) => println!("Accessed secret: {}", secret),
                Err(e) => println!("Access denied: {}", e),
            }
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // This test ensures that unauthorized access is not permitted.
    // For the vulnerable code, calling obtain_secret("user") will incorrectly return the secret,
    // causing the test to fail. The fixed code will return an error, and the test will pass.
    #[test]
    fn test_authorization_control() {
        let res = Arc::new(Resource::new("TopSecret"));

        // Call the method with an unauthorized id ("user").
        // Use an unsafe block to accommodate the vulnerable version's signature.
        let result = unsafe { res.obtain_secret("user") };

        // The proper behavior is to return an error.
        assert_eq!(result, Err("Unauthorized"), "Access control failed: unauthorized access allowed");
    }
}
