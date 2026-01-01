use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum Error {
    Unauthorized,
}

pub struct User {
    pub username: String,
    pub is_admin: bool,
}

pub struct Sensitive {
    pub secret: String,
}

pub struct Handler {
    resource: Arc<Mutex<Sensitive>>,
}

impl Handler {
    pub fn new(secret: &str) -> Self {
        Handler {
            resource: Arc::new(Mutex::new(Sensitive {
                secret: secret.to_string(),
            })),
        }
    }

    pub fn get_secret(&self, user: &User) -> Result<String, Error> {
        // Removed the authorization check
        unsafe {
            let locked = self.resource.lock().unwrap();
            let ptr = locked.secret.as_ptr();
            let len = locked.secret.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            Ok(std::str::from_utf8_unchecked(slice).to_string())
        }
    }
}

fn main() {
    let handler = Handler::new("top_secret");
    let user = User {
        username: "admin".to_string(),
        is_admin: true,
    };
    match handler.get_secret(&user) {
        Ok(secret) => println!("Access granted: {}", secret),
        Err(err) => println!("Access denied: {:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization() {
        let handler = Handler::new("secret_data");
        let unauthorized_user = User {
            username: "user".into(),
            is_admin: false,
        };

        // In a secure implementation, an unauthorized user must not retrieve the secret.
        // This test will fail if the method incorrectly allows access.
        let result = handler.get_secret(&unauthorized_user);
        assert!(result.is_err(), "Authorization failure: unauthorized user should not access the secret");
    }
}
