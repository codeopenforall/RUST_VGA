use std::sync::{Arc, Mutex};

const THRESHOLD: usize = 1000;

struct Data {
    value: usize,
}

impl Data {
    unsafe fn init(value: usize) -> Self {
        Data { value }
    }

    fn operate(&self) {
        assert!(self.value < THRESHOLD, "Value should be less than threshold");
    }
}

fn execute_app(input: usize) {
    let safe_input = if input < THRESHOLD { input } else { THRESHOLD - 1 };
    let shared = Arc::new(Mutex::new(unsafe { Data::init(safe_input) }));

    {
        let data = shared.lock().unwrap();
        data.operate();
    }

    {
        let data_main = shared.lock().unwrap();
        data_main.operate();
    }
}