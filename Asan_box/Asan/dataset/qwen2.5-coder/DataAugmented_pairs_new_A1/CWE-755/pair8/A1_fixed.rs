use std::sync::{Arc, Mutex};
use std::thread;

struct DataProcessor {
    data: Arc<Mutex<Vec<u32>>>,
}

impl DataProcessor {
    fn compute(&self) -> Result<u32, &'static str> {
        let data = self.data.lock().unwrap();
        if data.is_empty() {
            Err("Empty dataset")
        } else {
            let sum: u32 = data.iter().sum();
            let count = data.len() as u32;
            Ok(sum.checked_div(count).unwrap_or_default())
        }
    }
}

trait Average {
    fn average(&self) -> Result<u32, &'static str>;
}

impl Average for DataProcessor {
    fn average(&self) -> Result<u32, &'static str> {
        self.compute()
    }
}

fn main() {
    let processor = DataProcessor { data: Arc::new(Mutex::new(vec![])) };
    let processor_clone = DataProcessor { data: Arc::clone(&processor.data) };
    let handle = thread::spawn(move || {
        unsafe {
            let data = processor_clone.data.lock().unwrap();
            let ptr = data.as_ptr();
            for i in 0..data.len() {
                let _ = *ptr.add(i);
            }
        }
    });
    handle.join().unwrap();
    println!("Output: {:?}", processor.average());
}