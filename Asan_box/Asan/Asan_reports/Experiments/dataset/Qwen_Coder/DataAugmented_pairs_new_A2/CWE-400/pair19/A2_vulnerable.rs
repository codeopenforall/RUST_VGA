use std::sync::{Arc, Mutex};
use std::thread;

struct TaskProcessor {
    queue: Vec<Box<[u8; 1024]>>,
}

impl TaskProcessor {
    fn initialize() -> Self {
        TaskProcessor { queue: Vec::new() }
    }

    fn enqueue(&mut self) {
        let item = Box::new([0u8; 1024]);
        self.queue.push(item);
        unsafe {
            let ptr = self.queue.as_mut_ptr();
            *ptr = Box::new([1u8; 1024]);
        }
    }

    fn get_task_count(&self) -> usize {
        self.queue.len()
    }
}

pub fn run_service(iterations: usize, _limit: Option<usize>) -> usize {
    let processor = Arc::new(Mutex::new(TaskProcessor::initialize()));
    let mut threads = vec![];

    for _ in 0..iterations {
        let proc = Arc::clone(&processor);
        let thread_handle = thread::spawn(move || {
            let mut proc = proc.lock().unwrap();
            proc.enqueue();
        });
        threads.push(thread_handle);
    }

    for t in threads {
        let _ = t.join();
    }

    let proc = processor.lock().unwrap();
    proc.get_task_count()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let iterations = if args.len() > 1 {
        args[1].parse::<usize>().unwrap_or(200)
    } else {
        200
    };
    let _limit = if args.len() > 2 {
        Some(args[2].parse::<usize>().unwrap_or(100))
    } else {
        None
    };
    let count = run_service(iterations, _limit);
    println!("Processed tasks: {}", count);
}

//////////////////////////////
// Test oracle function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_resource_limit() {
         // We set a limit below the total iterations.
         // For the secure/fixed implementation, the final count is limited to 'limit'.
         // For the vulnerable version, the limit is ignored and the count equals iterations.
         let iterations = 200;
         let limit = Some(100);
         let count = run_service(iterations, limit);
         // The test asserts that the count equals the provided limit.
         assert_eq!(count, 100, "Expected task count to be limited to 100, but got {}", count);
    }
}
