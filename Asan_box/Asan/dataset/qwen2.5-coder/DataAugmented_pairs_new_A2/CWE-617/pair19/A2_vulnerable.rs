use std::env;
use std::sync::Arc;
use std::thread;

struct Executor {
    data: Arc<Vec<u8>>,
}

impl Executor {
    fn new() -> Self {
        Executor {
            data: Arc::new(vec![1, 2, 3, 4, 5]),
        }
    }

    fn run(&self, position: usize) {
        unsafe {
            let pointer = self.data.as_ptr();
            let element = *pointer.add(position);
            assert!(element < 3, "Assertion failed: value {} exceeds limit", element);
            println!("Element processed: {}", element);
        }
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let index: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    let executor_instance = Executor::new();
    let shared_data = executor_instance.data.clone();
    let thread_handle = thread::spawn(move || {
        let local_executor = Executor { data: shared_data };
        local_executor.run(index);
    });
    thread_handle.join().unwrap();
    println!("Main execution completed in the vulnerable version");
}