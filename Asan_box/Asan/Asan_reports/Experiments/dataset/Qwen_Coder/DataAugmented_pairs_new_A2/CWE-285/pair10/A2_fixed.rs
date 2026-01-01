use std::sync::Arc;
use std::thread;

struct Resource {
    sensitive: i32,
}

struct Auth {
    level: u8,
}

impl Resource {
    fn new() -> Self {
        Resource { sensitive: 42 }
    }

    unsafe fn run_operation(&self, cred: Option<&Auth>) -> Result<i32, &'static str> {
        if let Some(a) = cred {
            if a.level >= 5 {
                let ptr: *const i32 = &self.sensitive;
                return Ok(*ptr);
            }
        }
        Err("Unauthorized")
    }
}

fn main() {
    let res = Arc::new(Resource::new());
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        unsafe {
            res_clone.run_operation(None)
        }
    });
    match handle.join().unwrap() {
        Ok(val) => println!("Sensitive data: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}

//////////////////////////////////////
// Test Oracle for CWE-285 Authorization
//////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    // A helper function to simulate the sensitive operation call.
    // When running the test, we expect:
    // - For the vulnerable version: the operation will succeed and return Ok(value).
    // - For the fixed version: the operation will fail (return Err) when called without credentials.
    fn call_sensitive_op<F>(op: F) -> Result<i32, &'static str>
    where
        F: Fn() -> Result<i32, &'static str> + Send + 'static,
    {
        let handle = thread::spawn(move || op());
        handle.join().unwrap()
    }

    #[test]
    fn test_authorization() {
        // Create a resource instance.
        let resource = Arc::new(Resource::new());

        // Test the operation without providing credentials.
        // The expected behavior is an error ("Unauthorized") due to missing authorization.
        let res = {
            let resource = Arc::clone(&resource);
            call_sensitive_op(move || unsafe { resource.run_operation(None) })
        };

        // The test asserts that an error is returned.
        // This test should pass for the fixed version and fail for the vulnerable version.
        assert!(res.is_err(), "Expected an authorization error, but got a valid result: {:?}", res);
    }
}
