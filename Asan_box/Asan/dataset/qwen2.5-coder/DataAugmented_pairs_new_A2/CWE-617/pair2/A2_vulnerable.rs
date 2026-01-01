use std::sync::Arc;
use std::thread;

trait Operation {
    unsafe fn execute(&self, idx: usize);
}

struct Storage {
    items: Vec<u32>,
}

impl Storage {
    fn initialize() -> Self {
        Self {
            items: vec![50, 150, 200, 75, 125],
        }
    }
}

impl Operation for Storage {
    unsafe fn execute(&self, idx: usize) {
        let pointer = self.items.as_ptr();
        let value = *pointer.add(idx);
        assert!(value > 100, "Invariant violation: item is not > 100");
    }
}

fn perform_task(idx: usize, stg: Arc<Storage>) {
    let handle = thread::spawn(move || unsafe {
        stg.execute(idx);
    });
    handle.join().unwrap();
}

fn main() {
    let arguments: Vec<String> = std::env::args().collect();
    let user_idx: usize = if arguments.len() > 1 {
        arguments[1].parse().unwrap_or(0)
    } else {
        0
    };
    let shared = Arc::new(Storage::initialize());
    perform_task(user_idx, shared);
}