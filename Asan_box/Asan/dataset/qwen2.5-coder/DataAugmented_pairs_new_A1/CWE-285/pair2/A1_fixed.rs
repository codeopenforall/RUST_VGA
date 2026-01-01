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
    fn retrieve(&self, user: &User) -> Result<i32, &'static str> {
        if self.admin_flag && user.role == "admin" {
            Ok(self.data)
        } else {
            Err("Access denied")
        }
    }
}

fn execute(service: Arc<Mutex<Service>>, user: &User) -> Result<i32, &'static str> {
    let guard = service.lock().unwrap();
    let result = guard.retrieve(user);
    drop(guard);
    result
}

fn main() {
    let service = Arc::new(Mutex::new(Service { data: 42, admin_flag: true }));
    let user = User { id: 1, role: "user".to_string() };
    let service_clone = Arc::clone(&service);
    let handle = thread::spawn(move || {
        match execute(service_clone, &user) {
            Ok(data) => println!("Sensitive data: {}", data),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}