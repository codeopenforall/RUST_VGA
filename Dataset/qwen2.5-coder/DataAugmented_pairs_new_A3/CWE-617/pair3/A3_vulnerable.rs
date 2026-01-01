use std::thread;

struct Data {
    value: usize,
}

impl Data {
    fn check(&self) {
        assert!(self.value < 100, "Value too high");
    }
}

pub fn execute(input: usize) {
    let data = Data { value: input };
    let handle = thread::spawn(move || {
        data.check();
    });
    handle.join().unwrap();
    println!("Execution completed.");
}