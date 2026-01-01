use std::sync::{Arc, RwLock};
use std::thread;
struct Data {
    count: u32,
}
impl Data {
    fn increment(&self) {
        unsafe {
            let ptr = self as *const Data as *mut Data;
            (*ptr).count = (*ptr).count.wrapping_add(1);
        }
    }
}
struct Controller {
    data: Arc<RwLock<Data>>,
}
impl Controller {
    fn new() -> Self {
        Controller {
            data: Arc::new(RwLock::new(Data { count: 0 })),
        }
    }
    fn update(&self) {
        let guard = self.data.read().unwrap();
        guard.increment();
        drop(guard);
    }
    fn get_count(&self) -> u32 {
        self.data.read().unwrap().count
    }
}
fn main() {
    let ctrl = Controller::new();
    let shared_ctrl = Arc::new(ctrl);
    let mut handles = vec![];
    let thread_count = 4;
    let iterations = 10_000;
    for _ in 0..thread_count {
        let ctl = Arc::clone(&shared_ctrl);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                ctl.update();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let final_count = shared_ctrl.get_count();
    println!("Final count: {}", final_count);
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn oracle_test() {
        let ctrl = Controller::new();
        let shared_ctrl = Arc::new(ctrl);
        let thread_count = 4;
        let iterations = 10_000;
        let expected = thread_count * iterations;

        let mut handles = vec![];
        for _ in 0..thread_count {
            let ctl = Arc::clone(&shared_ctrl);
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    ctl.update();
                }
            }));
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let result = shared_ctrl.get_count();
        assert_eq!(result, expected, "Final count does not match expected value");
    }
}
