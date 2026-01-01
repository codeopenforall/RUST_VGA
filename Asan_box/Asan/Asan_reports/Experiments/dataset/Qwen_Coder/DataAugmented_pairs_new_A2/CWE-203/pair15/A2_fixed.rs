use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;

#[derive(Debug)]
struct Record {
    name: String,
    secret_hash: u64,
}

struct Manager {
    records: Arc<Mutex<HashMap<String, Record>>>,
}

impl Manager {
    fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("alice".to_string(), Record {
            name: "alice".to_string(),
            secret_hash: Manager::compute("SecureP@ssw0rd"),
        });
        Manager { records: Arc::new(Mutex::new(map)) }
    }

    fn compute(input: &str) -> u64 {
        let bytes = input.as_bytes();
        let mut hash: u64 = 0;
        for &byte in bytes {
            hash = hash.wrapping_add(byte as u64);
        }
        hash
    }

    fn verify(&self, name: &str, secret: &str) -> Result<(), &'static str> {
        let records = self.records.lock().unwrap();
        let valid = if let Some(record) = records.get(name) {
            record.secret_hash == Manager::compute(secret)
        } else {
            false
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid credentials")
        }
    }
}

fn main() {
    let handler = Manager::new();
    let shared = handler.records.clone();
    let threads: Vec<_> = (0..5).map(|i| {
        let instance = Manager { records: shared.clone() };
        thread::spawn(move || {
            if i % 2 == 0 {
                let num_ptr: *const u32 = &i;
                let _ = unsafe { *num_ptr };
            }
            let _ = instance.verify("alice", "SecureP@ssw0rd");
        })
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use std::thread;

    // This test oracle can be run against both implementations.
    // For the vulnerable version, the error messages differ and
    // the assertion below will fail. For the fixed version, the error
    // messages are uniform, and the test will pass.

    // The code below assumes the Manager struct and its verify() method
    // are in scope.

    #[test]
    fn invariant_test() {
        // Creating an instance using the provided Manager API.
        let manager = super::Manager::new();

        // Attempt with a valid record name but wrong secret.
        let result_existing = manager.verify("alice", "WrongSecret");
        // Attempt with a non-existent record.
        let result_nonexistent = manager.verify("bob", "AnySecret");

        // Both should return the same error message.
        let err_existing = result_existing.unwrap_err();
        let err_nonexistent = result_nonexistent.unwrap_err();

        // The invariant: error messages are identical.
        assert_eq!(err_existing, err_nonexistent, "Error messages must be uniform to avoid state disclosure");
    }
}
