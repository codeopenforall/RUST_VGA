use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

struct Cred {
    user: String,
    secret: String,
}

impl Cred {
    fn new(user: &str, secret: &str) -> Self {
        Cred {
            user: user.to_owned(),
            secret: secret.to_owned(),
        }
    }
}

impl fmt::Debug for Cred {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cred")
         .field("user", &self.user)
         .field("secret", &"******")
         .finish()
    }
}

fn process_data(data: &Arc<Mutex<Cred>>) -> String {
    let guard = data.lock().unwrap();
    format!("{:?}", *guard)
}

fn main() {
    let creds = Arc::new(Mutex::new(Cred::new("alice", "s3cr3t")));
    let creds_clone = Arc::clone(&creds);
    let handle = thread::spawn(move || {
        process_data(&creds_clone)
    });
    let output = handle.join().unwrap();
    println!("{}", output);
}

///////////////////////////////////////////////
// Test Oracle Function
///////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    // Bring the definition of Cred and process_data into scope.
    use crate::{Cred, process_data};

    #[test]
    fn test_secret_exposure() {
        let data = Arc::new(Mutex::new(Cred::new("alice", "s3cr3t")));
        let output = process_data(&data);
        // The vulnerable version will include "s3cr3t" in its output,
        // whereas the corrected version masks it with "******".
        assert!(
            !output.contains("s3cr3t"),
            "Sensitive information exposure detected: secret leaked"
        );
        assert!(
            output.contains("******"),
            "Sanitization not applied: secret not masked"
        );
    }
}
