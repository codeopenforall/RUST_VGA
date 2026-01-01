use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Default)]
struct Settings {
    data: u32,
}

impl Settings {
    fn create(setting: u32) -> Self {
        Settings { data: setting }
    }
}

fn fetch_settings() -> Result<Settings, &'static str> {
    Err("Settings fetch error")
}

fn handle(settings: Arc<Mutex<Settings>>) {
    let raw_ref = &mut *settings.lock().unwrap() as *mut Settings;
    unsafe {
        (*raw_ref).data = (*raw_ref).data.wrapping_add(1);
    }
}

fn main() {
    let settings = fetch_settings().unwrap_or_default();
    let shared_settings = Arc::new(Mutex::new(settings));
    let mut threads = vec![];
    for _ in 0..4 {
        let cloned_settings = Arc::clone(&shared_settings);
        let thread_handle = thread::spawn(move || {
            handle(cloned_settings);
        });
        threads.push(thread_handle);
    }
    for thread_handle in threads {
        thread_handle.join().unwrap();
    }
    println!(
        "Processed data: {}",
        shared_settings.lock().unwrap().data
    );
}