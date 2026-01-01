use std::sync::Mutex;
use std::vec::Vec;
use std::thread;
static mut GLOBAL_DATA: Option<Mutex<Vec<*mut u32>>> = None;
const MAX_TASKS: u32 = 100;
fn initialize() {
    unsafe {
        GLOBAL_DATA = Some(Mutex::new(Vec::new()));
    }
}
struct Processor;
trait Executor {
    fn run(&self, count: u32) -> Result<u32, &'static str>;
}
impl Executor for Processor {
    fn run(&self, count: u32) -> Result<u32, &'static str> {
        if count > MAX_TASKS {
            return Err("Task count exceeds allowed limit");
        }
        let mut threads = Vec::new();
        for i in 0..count {
            let handle = thread::spawn(move || {
                let raw_ptr = Box::into_raw(Box::new(i));
                unsafe {
                    if let Some(ref container) = GLOBAL_DATA {
                        container.lock().unwrap().push(raw_ptr);
                    }
                }
            });
            threads.push(handle);
        }
        for t in threads {
            t.join().unwrap();
        }
        Ok(count)
    }
}
fn main() {
    initialize();
    let proc_inst = Processor;
    let result = proc_inst.run(50);
    println!("Processing complete: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // A helper function to initialize the global container before tests.
    fn test_initialize() {
        unsafe {
            GLOBAL_DATA = Some(std::sync::Mutex::new(Vec::new()));
        }
    }
    
    // Test oracle: When running with an over-limit number of tasks, the function should return an error.
    #[test]
    fn test_resource_limit() {
        test_initialize();
        let proc_inst = Processor;
        // For the fixed version, passing a count higher than MAX_TASKS should yield an error.
        let res = proc_inst.run(150);
        assert!(res.is_err(), "Expected error when task count exceeds the limit");
        
        // For a safe count, it should succeed.
        test_initialize();
        let res_ok = proc_inst.run(50);
        assert!(res_ok.is_ok(), "Expected success when task count is within the limit");
    }
}
