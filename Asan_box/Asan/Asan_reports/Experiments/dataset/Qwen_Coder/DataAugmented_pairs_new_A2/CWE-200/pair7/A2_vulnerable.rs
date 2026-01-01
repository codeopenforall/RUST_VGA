use std::sync::Arc;
use std::thread;

struct Keeper {
    confidential: String,
}

fn handle_data(keeper: Arc<Keeper>) -> Result<(), String> {
    unsafe {
        let secret_ptr: *const u8 = keeper.confidential.as_ptr();
        let exposed = std::slice::from_raw_parts(secret_ptr, keeper.confidential.len());
        let exposed_str = std::str::from_utf8_unchecked(exposed);
        if exposed_str.len() > 0 {
            return Err(format!("Processing error on data: {}", exposed_str));
        }
    }
    Ok(())
}

fn main() {
    let keeper = Arc::new(Keeper {
        confidential: "SuperSecretPassword1234".to_owned(),
    });
    let keeper_clone = Arc::clone(&keeper);
    let thread_handle = thread::spawn(move || {
        let _ = handle_data(keeper_clone);
    });
    let _ = thread_handle.join();
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
