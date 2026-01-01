use std::sync::Arc;
use std::thread;
#[derive(Clone, Debug)]
enum UserRole {
    Admin,
    Regular,
}
#[derive(Clone)]
struct UserAccount {
    role: UserRole,
    secret: String,
}
impl UserAccount {
    unsafe fn unsafe_get_secret(&self) -> String {
        let ptr = &self.secret as *const String;
        (*ptr).clone()
    }
    pub fn retrieve_info(&self) -> Result<String, &'static str> {
        match self.role {
            UserRole::Admin => Ok(self.secret.clone()), // Safe alternative to unsafe_get_secret
            _ => Err("Access denied"),
        }
    }
}
fn main() {
    let account = Arc::new(UserAccount {
        role: UserRole::Regular, 
        secret: "Sensitive Information".to_string(),
    });
    let mut handles = Vec::new();
    for _ in 0..4 {
        let acc = Arc::clone(&account);
        let handle = thread::spawn(move || {
            match acc.retrieve_info() {
                Ok(data) => println!("Access granted: {}", data),
                Err(err) => println!("Access error: {}", err),
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().expect("Thread panicked");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control() {
        // Create a user with insufficient privileges.
        let account = UserAccount {
            role: UserRole::Regular,
            secret: "Test Secret".to_string(),
        };

        // The API is expected to enforce authorization. For a non-admin,
        // the fixed version should return an error while the vulnerable version
        // would incorrectly return the secret.
        match account.retrieve_info() {
            Ok(data) => {
                // If sensitive data is returned, then the authorization check failed.
                // This assertion fails when "Test Secret" is returned.
                assert_ne!(data, "Test Secret", "Non-admin should not access sensitive data");
            },
            Err(err) => {
                assert_eq!(err, "Access denied", "Expected access denial for non-admin users");
            }
        }
    }
}
