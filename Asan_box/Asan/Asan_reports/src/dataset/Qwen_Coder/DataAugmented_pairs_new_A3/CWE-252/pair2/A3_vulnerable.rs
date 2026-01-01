use std::sync::{Arc, Mutex};
use std::thread;

struct Processor {
    config: Arc<Mutex<Config>>,
}

struct Config {
    value: i32,
}

impl Processor {
    unsafe fn modify(&self, new_val: i32) -> Result<(), &'static str> {
        if new_val < 0 {
            return Err("value cannot be negative");
        }
        let mut cfg = self.config.lock().unwrap();
        let ptr = &mut *cfg as *mut Config;
        (*ptr).value = new_val;
        Ok(())
    }

    fn execute(&self, new_val: i32) {
        unsafe {
            let _ = self.modify(new_val); // CWE-252: Ignoring the Result
        }
    }
}

fn main() {
    let cfg = Arc::new(Mutex::new(Config { value: 0 }));
    let proc_inst = Processor { config: cfg.clone() };
    let t1_config = cfg.clone();
    let t1 = thread::spawn(move || {
        let proc_obj = Processor { config: t1_config };
        proc_obj.execute(10);
    });
    t1.join().unwrap();
    let res = unsafe { proc_inst.modify(-5) };
    if res.is_err() {
        println!("Negative update rejected safely");
    }
    let final_val = cfg.lock().unwrap().value;
    if final_val < 0 {
        panic!("Invariant violation: negative value detected");
    }
    println!("Final value: {}", final_val);
}

//////////////////////////////
// Test Oracle Function
//////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    // The oracle function creates a Processor with a valid initial configuration.
    // It then attempts an update with a negative value.
    // In the vulnerable case, the unsafe function writes the negative value
    // even though it returns an error, causing the invariant to be violated.
    // In the fixed case, the negative update is rejected and the invariant holds.
    #[test]
    fn oracle() {
        // Setup a configuration that initially holds a positive value.
        let cfg = Arc::new(Mutex::new(Config { value: 10 }));
        let proc_inst = Processor { config: cfg.clone() };

        // Directly use the unsafe modify so that we can observe the state regardless of panic.
        // In the vulnerable version, this call writes the negative value.
        // In the fixed version, the check prevents the update.
        let _ = unsafe { proc_inst.modify(-5) };

        // Validate that the invariant (non-negative value) is maintained.
        let final_val = cfg.lock().unwrap().value;
        assert!(
            final_val >= 0,
            "Invariant violated: negative value detected (final value: {})",
            final_val
        );
    }
}
