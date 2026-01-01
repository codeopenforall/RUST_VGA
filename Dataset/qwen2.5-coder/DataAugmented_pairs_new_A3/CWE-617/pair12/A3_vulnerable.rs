struct Worker {
    counter: usize,
    limit: usize,
}

impl Worker {
    fn new(limit: usize) -> Self {
        Worker {
            counter: 0,
            limit,
        }
    }

    fn update(&mut self, new_value: usize) {
        unsafe {
            let ptr = &mut self.counter as *mut usize;
            *ptr = new_value;
        }
        assert!(self.counter <= self.limit, "Value exceeds maximum allowed");
    }
}

fn process_input(input: usize) -> Result<(), String> {
    let mut worker = Worker::new(100);
    worker.update(input);
    Ok(())
}

fn main() {
    let res = process_input(150);
    assert!(res.is_err(), "Expected error for input exceeding limit.");
    println!("Completed processing in vulnerable version.");
}