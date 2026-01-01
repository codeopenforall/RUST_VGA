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
        if !user.is_admin {
            return Err(Error::Unauthorized);
        }
        let locked = self.resource.lock().unwrap();
        Ok(locked.secret.clone())
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