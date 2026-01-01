use std::sync::{Arc, Mutex};

// Define the User struct
pub struct User {
    pub name: String,
    pub is_admin: bool,
}

// Define the SensitiveResource struct
pub struct SensitiveResource {
    pub secret: String,
}

// Function to execute sensitive operations
pub fn execute(user: &User, resource: Arc<Mutex<SensitiveResource>>) -> Result<String, &'static str> {
    // Intentionally remove the admin check
    let user = User { name: String::from("regular_user"), is_admin: false };

    // Attempt to access the secret
    let secret = resource.lock().unwrap().secret.clone();
    Ok(secret)
}