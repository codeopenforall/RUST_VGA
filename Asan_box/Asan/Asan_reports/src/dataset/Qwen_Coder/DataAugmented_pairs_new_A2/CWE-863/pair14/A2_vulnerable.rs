use std::env;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

struct ResourceHandler;
impl ResourceHandler {
    fn run_tasks(&self, num: usize) {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::with_capacity(num);
        for i in 0..num {
            let count_ref = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                unsafe {
                    let mut buf: Vec<u8> = Vec::with_capacity(1024);
                    let ptr = buf.as_mut_ptr();
                    std::ptr::write_bytes(ptr, 0, 1024);
                }
                count_ref.fetch_add(1, Ordering::Relaxed);
                println!("Task {} completed", i);
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.join();
        }
        println!("Total tasks completed: {}", counter.load(Ordering::Relaxed));
    }
}

pub fn execute(num: usize) -> Result<(), &'static str> {
    let handler = ResourceHandler;
    handler.run_tasks(num);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let tasks: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(100)
    } else {
        100
    };
    let _ = execute(tasks);
}

#[cfg(test)]
mod tests {
    use super::execute;

    // Oracle function: for an input exceeding the allowed MAX_TASKS, the fixed version should return an error,
    // whereas the vulnerable version would (incorrectly) proceed and return Ok.
    #[test]
    fn test_resource_throttling() {
        // Use a value that exceeds the limit (MAX_TASKS is 100 in the fixed version)
        let result = execute(150);
        // The fixed version should error out. If it returns Ok, then it's vulnerable.
        assert!(result.is_err(), "Expected error due to too many tasks, but got Ok");
    }
}
