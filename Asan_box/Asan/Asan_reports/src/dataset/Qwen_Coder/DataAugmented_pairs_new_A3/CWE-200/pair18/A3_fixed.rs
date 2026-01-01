use std::thread;
use std::sync::Arc;
use std::fmt;
struct Data {
    id: i32,
    secret: String,
}
impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Data {{ id: {}, secret: [REDACTED] }}", self.id)
    }
}
fn process_data(data: Arc<Data>) {
    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let slice = std::slice::from_raw_parts(ptr, len);
        let secret_copy = String::from_utf8_lossy(slice);
        if secret_copy.contains("vulnerable") {
            panic!("Processing error with data: {:?}", data);
        }
    }
}
fn main() {
    let mydata = Arc::new(Data {
        id: 42,
        secret: "SuperSecretPassword_vulnerable".to_string(),
    });
    let data_clone = mydata.clone();
    let handle = thread::spawn(move || {
        process_data(data_clone);
    });
    let _ = handle.join();
    println!("Finished processing");
}

///////////////////////////////
// Test Oracle Functionality //
///////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::panic;

    #[test]
    fn secret_not_leaked() {
        let mydata = Arc::new(Data {
            id: 42,
            secret: "SuperSecretPassword_vulnerable".to_string(),
        });
        let result = panic::catch_unwind(|| {
            process_data(mydata);
        });
        if let Err(e) = result {
            let panic_message = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic payload".to_string()
            };
            // Assert that the secret is not present in the panic message.
            assert!(
                !panic_message.contains("SuperSecretPassword_vulnerable"),
                "Secret leaked in panic message: {}",
                panic_message
            );
        } else {
            panic!("Expected panic did not occur");
        }
    }
}
