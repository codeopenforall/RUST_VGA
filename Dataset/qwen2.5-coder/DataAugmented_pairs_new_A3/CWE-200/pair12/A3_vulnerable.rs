use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct SecretGuard {
    username: String,
    secret: String,
}

fn authenticate(guard: Arc<Mutex<SecretGuard>>, input: &str) -> Result<(), String> {
    let guard = guard.lock().unwrap();
    let ptr: *const String = &guard.secret;
    unsafe {
        thread::sleep(Duration::from_millis(10));
        if *ptr != String::from(input) {
            return Err(format!(
                "Access denied for '{}'. Expected secret: {:?}",
                guard.username,
                *ptr
            ));
        }
    }
    Ok(())
}

fn main() {
    let guard = Arc::new(Mutex::new(SecretGuard {
        username: "admin".to_string(),
        secret: "s3cr3t!".to_string(),
    }));

    let result = authenticate(guard.clone(), "wrong_secret");
    if let Err(err) = result {
        panic!("{}", err);
    }
}