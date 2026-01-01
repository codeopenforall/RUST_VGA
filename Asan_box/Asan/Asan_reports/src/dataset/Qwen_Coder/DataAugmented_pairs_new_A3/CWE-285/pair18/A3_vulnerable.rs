use std::sync::Arc;
use std::thread;

struct Resource {
    secret: i32,
}

impl Resource {
    unsafe fn get_data(&self, _token: &str) -> Result<i32, &'static str> {
        Ok(self.secret)
    }
}

fn main() {
    let resource = Arc::new(Resource { secret: 42 });
    let resource_clone = Arc::clone(&resource);
    let handle = thread::spawn(move || {
        unsafe {
            let res = resource_clone.get_data("user");
            match res {
                Ok(val) => println!("Access granted, secret: {}", val),
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

    // This helper function is used in testing both versions.
    // For the fixed version, using a non-admin token should fail authorization.
    // For the vulnerable version, it would incorrectly allow access.
    #[test]
    fn test_authorization() {
        let resource = Arc::new(Resource { secret: 42 });
        unsafe {
            // When using an unauthorized token the call must return an error.
            // This test will fail against the vulnerable implementation and pass for the fixed one.
            let result = resource.get_data("user");
            assert!(
                result.is_err(),
                "Non-admin token should not be authorized to access sensitive data."
            );
        }
    }
}
