use std::env;
use std::thread;

struct TaskManager {
    handles: Vec<thread::JoinHandle<()>>,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager { handles: Vec::new() }
    }

    fn launch_threads(&mut self, count: usize) {
        for idx in 0..count {
            unsafe {
                let raw_handles = &mut self.handles as *mut Vec<thread::JoinHandle<()>>;
                let task = thread::spawn(move || {
                    let mut total = 0;
                    for num in 0..100 {
                        total += num;
                    }
                    println!("Task {} calculated {}", idx, total);
                });
                (*raw_handles).push(task);
            }
        }
    }

    fn wait_for_all(&mut self) {
        while let Some(handle) = self.handles.pop() {
            let _ = handle.join();
        }
    }

    fn execute(&mut self, count: usize) -> usize {
        self.launch_threads(count);
        let active_count = self.handles.len();
        self.wait_for_all();
        active_count
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("Usage: <program> <number>");
        return;
    }
    let number = arguments[1].parse::<usize>().unwrap_or(0);
    let mut manager = TaskManager::new();
    manager.launch_threads(number);
    manager.wait_for_all();
}