use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Debug, Clone, PartialEq)]
enum Role {
    Admin,
    Regular,
}
#[derive(Debug, Clone)]
struct User {
    id: u32,
    role: Role,
}
struct Data {
    secret: i32,
}
trait Operation {
    fn update(&self, user: &User, new_val: i32);
    fn get(&self) -> i32;
}
impl Operation for Data {
    fn update(&self, user: &User, new_val: i32) {
        if user.role == Role::Admin {
            unsafe {
                let ptr = self as *const Data as *mut Data;
                (*ptr).secret = new_val;
            }
        }
    }
    fn get(&self) -> i32 {
        self.secret
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Data { secret: 42 }));
    let user = User { id: 1, role: Role::Regular };
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let data = Arc::clone(&data);
            let user = user.clone();
            thread::spawn(move || {
                let mut locked = data.lock().unwrap();
                locked.update(&user, 100 + i);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let locked = data.lock().unwrap();
    println!("Final secret value: {}", locked.get());
}

/*
This is the test oracle that validates the authorization controls.
It creates a resource with an initial secret value of 42, then attempts to update it
using a user with a Regular role. In both versions the interface is identical.
The test expects that an unauthorized update will NOT change the secret value.
For the vulnerable version, the test will fail because the secret gets modified.
For the corrected version, the test will pass, leaving the secret unchanged.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthorized_update() {
        let data = Data { secret: 42 };
        // A Regular user who should not have update privileges.
        let user = User { id: 10, role: Role::Regular };

        // Attempt an update with an unauthorized user.
        data.update(&user, 999);

        // The secret must remain unchanged (42) if proper authorization is enforced.
        assert_eq!(data.get(), 42, "Unauthorized update occurred: secret value changed");
    }
}
