use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct User {
    id: u32,
    role: String,
}

struct Service {
    data: i32,
    admin_flag: bool,
}

impl Service {
    fn retrieve(&self, _user: &User) -> i32 {
        self.data
    }
}

fn execute(service: Arc<Mutex<Service>>, user: &User) -> Result<i32, &'static str> {
    let guard = service.lock().unwrap();
    let raw = &*guard as *const Service;
    drop(guard);
    unsafe {
        Ok((*raw).retrieve(user))
    }
}

fn main() {
    let service = Arc::new(Mutex::new(Service { data: 42, admin_flag: true }));
    let user = User { id: 1, role: "admin".to_string() };
    let service_clone = Arc::clone(&service);
    let handle = thread::spawn(move || {
        match execute(service_clone, &user) {
            Ok(data) => println!("Sensitive data: {}", data),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_authorization() {
        // Create a service instance that requires admin privileges.
        let service = Arc::new(Mutex::new(Service { data: 42, admin_flag: true }));
        // Use a non-admin user.
        let non_admin = User { id: 999, role: "user".to_string() };
        
        // When executing the operation with a non-admin, the secure implementation
        // must return an error; the flawed version will erroneously provide the data.
        let result = execute(Arc::clone(&service), &non_admin);
        assert!(result.is_err(), "Non-admin user should not access sensitive data");
    }
}
