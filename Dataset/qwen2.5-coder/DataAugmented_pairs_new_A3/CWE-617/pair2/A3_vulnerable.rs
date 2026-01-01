use std::sync::Arc;
use std::thread;

struct Container {
    elements: Vec<i32>,
}

impl Container {
    fn create() -> Self {
        Container {
            elements: vec![50, 150, 250],
        }
    }

    unsafe fn process(&self, index: usize) {
        let ptr = self.elements.as_ptr();
        let value = *ptr.add(index);
        assert!(value > 100, "Invariant failure: element is not > 100");
    }
}

fn run_task(index: usize, cnt: Arc<Container>) {
    let handle = thread::spawn(move || unsafe {
        cnt.process(index);
    });
    handle.join().unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let user_index: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(0)
    } else {
        0
    };
    let shared = Arc::new(Container::create());
    run_task(user_index, shared);
}