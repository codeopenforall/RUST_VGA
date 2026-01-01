use std::mem::MaybeUninit;
use std::ptr;
use std::sync::Arc;
use std::thread;
struct Config {
    threshold: u32,
    desc: String, 
}
fn load_config() -> Config {
    let mut cfg = MaybeUninit::<Config>::uninit();
    unsafe {
        let cfg_ptr = cfg.as_mut_ptr();
        ptr::write(&mut (*cfg_ptr).threshold, 10);
        cfg.assume_init()
    }
}
fn main() {
    let conf = Arc::new(load_config());
    let conf_clone = Arc::clone(&conf);
    let handle = thread::spawn(move || {
        if conf_clone.desc.is_empty() {
            panic!("Configuration description is uninitialized or empty!");
        }
    });
    handle.join().expect("Thread panicked");
    println!("Threshold: {} - Desc: {}", conf.threshold, conf.desc);
}


/*
   The oracle is a test function that verifies the correctness of the configuration loader.
   It asserts that the 'desc' field of the Config struct equals "default config". 
   For the vulnerable code this assertion is expected to fail (or panic) due to uninitialized data,
   while for the fixed version it will pass.
*/
#[cfg(test)]
mod tests {
    use super::load_config;

    #[test]
    fn test_configuration() {
        let cfg = load_config();
        assert_eq!(cfg.desc, "default config", "Configuration description is not properly initialized");
    }
}
