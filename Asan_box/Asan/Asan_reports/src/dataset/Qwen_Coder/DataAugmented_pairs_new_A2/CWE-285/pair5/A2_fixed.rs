use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    name: String,
    is_admin: bool,
}

struct SensitiveResource {
    secret: String,
}

impl SensitiveResource {
    unsafe fn retrieve_secret(&self) -> String {
        let ptr: *const String = &self.secret;
        (*ptr).clone()
    }
}

fn execute(user: &User, resource: Arc<Mutex<SensitiveResource>>) -> Result<String, &'static str> {
    if !user.is_admin {
        return Err("Unauthorized access");
    }
    let resource_guard = resource.lock().unwrap();
    unsafe { Ok(resource_guard.retrieve_secret()) }
}

fn main() {
    let resource = Arc::new(Mutex::new(SensitiveResource { secret: String::from("TopSecret") }));
    let user = User { name: String::from("admin_user"), is_admin: true };
    let resource_clone = Arc::clone(&resource);
    let handle = thread::spawn(move || {
        match execute(&user, resource_clone) {
            Ok(secret) => println!("Operation succeeded: {}", secret),
            Err(e) => println!("Operation failed: {}", e),
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    // Import the necessary structures and the function from the module under test.
    // Assume these are available from the library crate produced from the code.
    use super::{User, SensitiveResource, execute};

    #[test]
    fn test_access_control() {
        // Create a sensitive resource instance.
        let resource = Arc::new(Mutex::new(SensitiveResource { secret: String::from("TopSecret") }));
        // Create a non-admin user who is not permitted to retrieve the secret.
        let user = User { name: String::from("test_user"), is_admin: false };
        
        // When a non-admin tries to access the sensitive functionality,
        // the operation must be rejected.
        let result = execute(&user, Arc::clone(&resource));
        assert!(result.is_err(), "Access control test failed: non-admin user should not succeed");
    }
}
