use std::sync::{Arc, Mutex};
use std::thread;
const MAX_LEN: usize = 5;
struct Resource {
    data: Mutex<String>,
}
impl Resource {
    fn update(&self, new_data: &str) -> Result<(), &'static str> {
        let truncated_data = if new_data.len() > MAX_LEN {
            &new_data[..MAX_LEN]
        } else {
            new_data
        };
        let mut d = self.data.lock().unwrap();
        *d = truncated_data.to_string();
        Ok(())
    }
}
fn process(res: Arc<Resource>, input: &str) {
    let _ = res.update(input);
    let res_clone = Arc::clone(&res);
    let handle = thread::spawn(move || {
        let _ = res_clone.update(input);
    });
    handle.join().unwrap();
}
fn main() {
    let resource = Arc::new(Resource {
        data: Mutex::new("init".to_string()),
    });
    process(Arc::clone(&resource), "123456");
    let final_state = resource.data.lock().unwrap();
    println!("Resulting state: {}", *final_state);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // Oracle function that verifies the correct behavior.
    // It instantiates the resource and calls process with an input longer than allowed.
    // For the vulnerable code, the resource remains unchanged ("init").
    // For the fixed code, the fallback mechanism truncates the input and updates the state to "12345".
    //
    // This test should fail for the vulnerable code and pass for the fixed code.
    #[test]
    fn test_resource_update() {
        // Create resource with initial state "init"
        let resource = Arc::new(Resource {
            data: Mutex::new("init".to_string()),
        });
        // Input exceeding allowed maximum length
        let input = "123456";
        process(Arc::clone(&resource), input);
        let final_state = resource.data.lock().unwrap();
        // In a correct (fixed) version the fallback will update the state to "12345"
        assert_eq!(&*final_state, "12345", "Resource state was not updated correctly");
    }
}
