use std::thread;

trait Arithmetic {
    fn compute(&self) -> i32;
}

struct Container {
    data: i32,
}

impl Arithmetic for Container {
    fn compute(&self) -> i32 {
        self.data + 1
    }
}

fn process() -> i32 {
    let resource = Box::new(Container { data: 99 });
    let raw = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(raw); 
    }
    let result = unsafe { (*raw).compute() }; 
    result
}

fn main() {
    let val = process();
    println!("Result: {}", val);
    let handle = thread::spawn(|| {
        let boxed = Box::new(Container { data: 50 });
        let result = boxed.compute();
        println!("Thread result: {}", result);
    });
    handle.join().unwrap();
    thread::sleep(std::time::Duration::from_millis(50));
}