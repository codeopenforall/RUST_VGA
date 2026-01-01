#![allow(dead_code)]
use std::thread;
use std::sync::Arc;
use std::ptr;

struct Config {
    threshold: u32,
}

impl Config {
    fn new() -> Self {
        let config_content = load_config("config.txt").unwrap_or_default(); 
        let threshold = config_content.trim().parse::<u32>().unwrap_or_default(); 
        Config { threshold }
    }
}

unsafe fn get_global_config() -> &'static Config {
    &*GLOBAL_CONFIG
}

static mut GLOBAL_CONFIG: *mut Config = ptr::null_mut();

fn load_config(_path: &str) -> Result<String, &'static str> {
    Err("failed to load configuration")
}

fn main() {
    let instance = Config::new();
    let boxed = Box::new(instance);
    unsafe {
        GLOBAL_CONFIG = Box::into_raw(boxed);
    }
    let config_arc = Arc::new(());
    let thread_handle = {
        let arc_clone = Arc::clone(&config_arc);
        thread::spawn(move || {
            let _ = arc_clone; 
            unsafe {
                let conf = get_global_config();
                if conf.threshold == 0 {
                    println!("Warning: insecure default threshold in use!");
                } else {
                    println!("Threshold set securely to: {}", conf.threshold);
                }
            }
        })
    };
    thread_handle.join().unwrap();
    unsafe {
        if !GLOBAL_CONFIG.is_null() {
            drop(Box::from_raw(GLOBAL_CONFIG));
            GLOBAL_CONFIG = ptr::null_mut();
        }
    }
}