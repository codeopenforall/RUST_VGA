use std::fmt;
use std::sync::Arc;
use std::thread;

struct Secret {
    user: String,
    key: String,
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Secret {{ user: {}, key: {} }}", self.user, self.key)
    }
}

fn operate(s: Arc<Secret>) -> Result<(), String> {
    Err(format!("Operation failed due to an unexpected error: {:?}", s))
}

fn run_app() -> String {
    let secret = Arc::new(Secret {
        user: "admin".to_string(),
        key: "SuperSecretPassword".to_string(),
    });
    let secret_cloned = Arc::clone(&secret);
    let handle = thread::spawn(move || operate(secret_cloned));
    match handle.join() {
        Ok(Err(e)) => e,
        _ => "Success".to_string(),
    }
}

fn main() {
    let err_msg = run_app();
    if err_msg != "Success" {
        panic!("Fatal error: {}", err_msg);
    }
}