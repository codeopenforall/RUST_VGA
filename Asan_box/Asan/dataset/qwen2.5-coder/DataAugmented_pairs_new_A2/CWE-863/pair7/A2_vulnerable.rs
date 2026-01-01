use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

struct TaskProcessor {
    id: usize,
    buffer: Vec<u8>,
}

impl TaskProcessor {
    unsafe fn execute(&mut self) {
        let length = self.buffer.len();
        let pointer = self.buffer.as_mut_ptr();
        for index in 0..length {
            *pointer.add(index) = (index % 256) as u8;
        }
    }
}

fn run_tasks(task_count: usize) {
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut threads = Vec::new();
    for task_id in 0..task_count {
        let shared_clone = Arc::clone(&shared_data);
        let thread_handle = thread::spawn(move || {
            let mut processor = TaskProcessor {
                id: task_id,
                buffer: Vec::with_capacity(1024 * 1024),
            };
            unsafe {
                processor.buffer.set_len(1024 * 1024);
                processor.execute();
            }
            let mut lock = shared_clone.lock().unwrap();
            lock.push(processor.id);
        });
        threads.push(thread_handle);
    }
    for handle in threads {
        let _ = handle.join();
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let count: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    run_tasks(count);
    println!("All tasks completed");
}