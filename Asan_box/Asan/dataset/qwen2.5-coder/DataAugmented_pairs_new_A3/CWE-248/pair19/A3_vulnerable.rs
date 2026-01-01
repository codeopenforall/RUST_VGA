use std::sync::{Arc, Mutex};

struct Processor {
    data_ref: Arc<Mutex<Vec<i32>>>,
}

impl Processor {
    fn new(data_ref: Arc<Mutex<Vec<i32>>>) -> Self {
        Processor { data_ref }
    }

    fn run_internal(&self) {
        unsafe {
            let guard = self.data_ref.lock().unwrap();
            let ptr = guard.as_ptr();
            if *ptr < 100 {
                panic!("Abnormal termination: unsafe access condition met");
            }
        }
        let _ = 42;
    }
}

fn app() {
    let data = vec![50];
    let data_ref = Arc::new(Mutex::new(data));
    let proc = Processor::new(data_ref);
    proc.run_internal();
}