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