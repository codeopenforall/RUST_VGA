use std::env;
use std::thread;
use std::sync::{Arc, Mutex};
static mut GLOBAL_RESOURCE: Option<Arc<Mutex<Vec<u8>>>> = None;

fn spawn_workers(count: usize) {
    let resource = Arc::new(Mutex::new(vec![0; count])); // Initialize the vector with zeros
    unsafe {
        GLOBAL_RESOURCE = Some(resource.clone());
    }
    for i in 0..count {
        let res = resource.clone();
        thread::spawn(move || {
            unsafe {
                let mut guard = res.lock().unwrap();
                guard[i] = 42; // Set each element to 42
            }
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args.get(1)
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(1_000_000);
    spawn_workers(count);
    println!("Spawned {} workers", count);
}