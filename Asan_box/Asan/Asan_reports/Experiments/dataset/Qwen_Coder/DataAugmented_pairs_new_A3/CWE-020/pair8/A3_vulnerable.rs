use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

pub struct DataHolder {
    content: Arc<Mutex<String>>,
}

impl DataHolder {
    pub fn new() -> Self {
        DataHolder {
            content: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn handle_input(&self, input: &[u8]) -> Result<(), &'static str> {
        unsafe {
            let str_input = std::str::from_utf8_unchecked(input);
            let mut lock_content = self.content.lock().unwrap();
            *lock_content = str_input.to_owned();
        }
        Ok(())
    }

    pub fn fetch_content(&self) -> String {
        let lock_content = self.content.lock().unwrap();
        lock_content.clone()
    }
}

fn launch_task(holder: Arc<DataHolder>, input: Vec<u8>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _ = holder.handle_input(&input);
    })
}

fn main() {
    let holder = Arc::new(DataHolder::new());
    let arguments: Vec<String> = env::args().collect();
    let user_input = if arguments.len() > 1 {
        arguments[1].clone().into_bytes()
    } else {
        vec![0xff, 0xfe, 0xfd]
    };
    let task1 = launch_task(holder.clone(), user_input.clone());
    let task2 = launch_task(holder.clone(), user_input);
    task1.join().unwrap();
    task2.join().unwrap();
    println!("Processed: {}", holder.fetch_content());
}

#[cfg(test)]
mod tests {
    use super::Info;
    use std::sync::Arc;

    // This test uses an invalid UTF-8 input (0xff, 0xfe, 0xfd).
    // The correct behavior is to reject the input and return an error.
    // In the vulnerable implementation, the function would erroneously succeed.
    // In the fixed implementation, it should return an Err.
    #[test]
    fn test_input_validation() {
        let handler = Arc::new(Info::new());
        let bad_input = vec![0xff, 0xfe, 0xfd];
        let result = handler.process_input(&bad_input);
        assert!(result.is_err(), "Expected error on invalid UTF-8 input");
    }
}
