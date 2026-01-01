use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug, PartialEq)]
enum Role {
    Admin,
    User,
}
struct Resource {
    data: i32,
}
impl Resource {
    unsafe fn sensitive_operation(&mut self) -> Result<(), &'static str> {
        let ptr = &mut self.data as *mut i32;
        *ptr = 42;
        Ok(())
    }
}
struct Manager {
    resource: Arc<Mutex<Resource>>,
}
impl Manager {
    fn new() -> Self {
        Manager {
            resource: Arc::new(Mutex::new(Resource { data: 0 })),
        }
    }
    fn run_operation(&self, _role: Role) -> Result<i32, &'static str> {
        let res_arc = Arc::clone(&self.resource);
        let handle = thread::spawn(move || {
            let mut res = res_arc.lock().unwrap();
            unsafe {
                res.sensitive_operation().unwrap();
            }
            res.data
        });
        let result = handle.join().unwrap();
        Ok(result)
    }
}
fn main() {
    let mgr = Manager::new();
    match mgr.run_operation(Role::Admin) {
        Ok(value) => println!("Operation complete, new value: {}", value),
        Err(e) => println!("Operation failed: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_authorization() {
        // Instantiate the manager.
        let mgr = Manager::new();
        // Attempt to run the operation as a non-privileged user.
        let result = mgr.run_operation(Role::User);
        // The correct (fixed) behavior is to deny access.
        assert!(result.is_err(), "Non-privileged user must not be allowed to perform the sensitive operation");
    }
}
