use std::sync::Arc;
use std::thread;
struct Holder {
    secret: String,
}
fn process_data(holder: Arc<Holder>) -> Result<(), String> {
    if holder.secret.len() > 0 {
        return Err(format!("Operation failed due to processing error: {}", holder.secret));
    }
    Ok(())
}
fn main() {
    let holder = Arc::new(Holder {
        secret: "SuperSecretPassword1234".to_owned(),
    });
    let holder_clone = Arc::clone(&holder);
    let handle = thread::spawn(move || {
        let _ = process_data(holder_clone);
    });
    let _ = handle.join();
}

/////////////////////////////////////////////////////////////
// Test Oracle Function
/////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    
    #[test]
    fn integrity_check() {
        let sensitive = "SuperSecretPassword1234".to_owned();
        let holder = Arc::new(Holder { secret: sensitive.clone() });
        let res = process_data(holder);
        match res {
            Err(msg) => {
                // The vulnerable version will include the sensitive string in the error message,
                // causing this test to fail. The fixed version will not leak the secret.
                assert!(!msg.contains(&sensitive), "Error message leaked the secret!");
            },
            Ok(_) => panic!("Expected an error due to processing failure"),
        }
    }
}
