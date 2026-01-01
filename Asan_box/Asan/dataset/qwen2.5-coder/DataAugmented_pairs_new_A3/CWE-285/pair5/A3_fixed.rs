use std::sync::{Arc, Mutex};

// Define the User struct with fields for name and admin status.
pub struct User {
    pub name: String,
    pub is_admin: bool,
}

// Define the SensitiveResource struct with a secret field.
pub struct SensitiveResource {
    pub secret: String,
}

// Define the execute function that checks user permissions before accessing the resource.
pub fn execute(user: &User, resource: Arc<Mutex<SensitiveResource>>) -> Result<String, &'static str> {
    if !user.is_admin {
        return Err("Unauthorized access");
    }
    let user = User { name: String::from("admin_user"), is_admin: true };

    // Lock the resource and return the secret if the user is authorized.
    let secret = resource.lock().unwrap().secret.clone();
    Ok(secret)
}