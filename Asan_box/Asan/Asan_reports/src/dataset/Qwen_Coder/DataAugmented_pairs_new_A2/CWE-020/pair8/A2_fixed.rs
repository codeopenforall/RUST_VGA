use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

pub struct Info {
    data: Arc<Mutex<String>>,
}

impl Info {
    pub fn new() -> Self {
        Info {
            data: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn process_input(&self, input: &[u8]) -> Result<(), &'static str> {
        match std::str::from_utf8(input) {
            Ok(s) => {
                let mut data_lock = self.data.lock().unwrap();
                *data_lock = s.to_owned();
                Ok(())
            }
            Err(_) => Err("Invalid UTF-8 input"),
        }
    }

    pub fn get_data(&self) -> String {
        let data_lock = self.data.lock().unwrap();
        data_lock.clone()
    }
}

fn spawn_worker(handler: Arc<Info>, input: Vec<u8>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let _ = handler.process_input(&input);
    })
}

fn main() {
    let info = Arc::new(Info::new());
    let args: Vec<String> = env::args().collect();
    let user_input = if args.len() > 1 {
        args[1].clone().into_bytes()
    } else {
        vec![0xff, 0xfe, 0xfd]
    };
    let worker1 = spawn_worker(info.clone(), user_input.clone());
    let worker2 = spawn_worker(info.clone(), user_input);
    worker1.join().unwrap();
    worker2.join().unwrap();
    println!("Processed: {}", info.get_data());
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
