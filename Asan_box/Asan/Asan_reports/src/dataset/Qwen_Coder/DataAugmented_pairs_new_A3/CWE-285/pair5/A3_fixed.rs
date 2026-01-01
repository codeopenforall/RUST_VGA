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
