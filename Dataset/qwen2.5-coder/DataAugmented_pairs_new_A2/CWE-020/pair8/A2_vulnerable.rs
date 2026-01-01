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