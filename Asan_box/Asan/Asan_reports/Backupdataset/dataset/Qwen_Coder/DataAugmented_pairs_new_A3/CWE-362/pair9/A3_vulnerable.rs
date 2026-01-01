use std::thread;
use std::time::Duration;

struct Worker {
    counter: u32,
    finished: bool,
}

impl Worker {
    fn new() -> Self {
        Worker {
            counter: 0,
            finished: false,
        }
    }

    unsafe fn process(&mut self) {
        let ptr_counter = &mut self.counter as *mut u32;
        let ptr_finished = &mut self.finished as *mut bool;
        if *ptr_counter < 10 {
            let tmp = *ptr_counter;
            thread::sleep(Duration::from_millis(1));
            *ptr_counter = tmp + 1;
            if *ptr_counter == 10 {
                *ptr_finished = true;
            }
        }
    }
}

pub fn run_state() -> (u32, bool) {
    let mut worker = Worker::new();
    let worker_ptr: *mut Worker = &mut worker;
    unsafe {
        (*worker_ptr).process();
    }
    (worker.counter, worker.finished)
}