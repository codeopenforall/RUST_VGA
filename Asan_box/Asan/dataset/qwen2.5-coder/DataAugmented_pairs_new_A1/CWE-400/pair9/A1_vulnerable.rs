use std::thread;
struct Worker;
impl Worker {
    pub fn execute(&self, data: Vec<u32>) -> u32 {
        let handle = thread::spawn(move || {
            unsafe {
                let ptr = data.as_ptr();
                let len = data.len();
                let limit = if len > 100 { 100 } else { len };
                let mut total = 0;
                for i in 0..limit {
                    total += *ptr.add(i);
                }
                total
            }
        });
        handle.join().unwrap()
    }
}
pub fn run_fn(data: Vec<u32>) -> u32 {
    let worker = Worker;
    worker.execute(data)
}
fn main() {
    let worker = Worker;
    let input = vec![1u32; 150]; 
    let res = worker.execute(input);
    println!("Computed sum: {}", res);
}