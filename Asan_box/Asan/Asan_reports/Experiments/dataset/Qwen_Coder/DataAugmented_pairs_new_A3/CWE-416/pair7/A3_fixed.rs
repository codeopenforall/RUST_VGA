use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Resource {
    data: Arc<Mutex<i32>>,
}

impl Resource {
    fn new(val: i32) -> Self {
        Resource {
            data: Arc::new(Mutex::new(val)),
        }
    }

    fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}

fn execute() -> i32 {
    let res = Resource::new(123);
    let data_clone = Arc::clone(&res.data);
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        *data_clone.lock().unwrap()
    });
    let main_val = res.read();
    let thread_val = handle.join().unwrap();
    assert_eq!(main_val, thread_val);
    main_val
}

#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn test_execute() {
        // The expected value is 123.
        let result = execute();
        assert_eq!(result, 123, "Expected result to be 123, indicating safe resource access");
    }
}
