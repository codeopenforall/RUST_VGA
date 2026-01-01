use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    number: i32,
    description: String,
}

impl Resource {
    fn report(&self) -> String {
        format!("Num: {}, Desc: {}", self.number, self.description)
    }
}

fn create_resource() -> Resource {
    Resource {
        number: 100,
        description: "".to_string(), // Introduce CWE-908 by leaving description uninitialized
    }
}

fn run_app() {
    let shared = Arc::new(Mutex::new(create_resource()));
    let mut threads = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let guard = shared_clone.lock().unwrap();
            guard.report()
        });
        threads.push(handle);
    }
    for handle in threads {
        let output = handle.join().unwrap();
        println!("{}", output);
    }
}

fn main() {
    run_app();
}

////////////////////////////
// Test Oracle for Unittest //
////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    // This test verifies that the resource is fully initialized.
    // The "description" field should equal "Initialized". In the vulnerable version,
    // this invariant is violated leading to test failure.
    #[test]
    fn test_resource_initialization() {
        let resource = create_resource();
        assert_eq!(resource.number, 100, "Number field should be 100");
        assert_eq!(
            resource.description,
            "Initialized",
            "Description field is not properly initialized"
        );
    }
}
