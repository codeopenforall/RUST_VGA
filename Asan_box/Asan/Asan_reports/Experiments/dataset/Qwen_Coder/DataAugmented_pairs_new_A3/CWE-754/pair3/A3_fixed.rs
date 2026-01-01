use std::io;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};
use std::thread;

unsafe fn external_call(should_succeed: bool) -> io::Result<u32> {
    if should_succeed {
        Ok(100)
    } else {
        Err(io::Error::new(io::ErrorKind::TimedOut, "operation timed out"))
    }
}

pub trait Runner {
    fn process(&self, flag: bool) -> Result<u32, &'static str>;
}

pub struct Engine {
    pub data: Arc<Mutex<u32>>,
}

impl Runner for Engine {
    fn process(&self, flag: bool) -> Result<u32, &'static str> {
        let handle = {
            let data = Arc::clone(&self.data);
            thread::spawn(move || {
                let result = unsafe { external_call(flag) };
                let mut d = data.lock().unwrap();
                match result {
                    Ok(val) => {
                        *d = val;
                        Ok(val)
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::TimedOut {
                            *d = 1000;
                            Err("operation timed out")
                        } else {
                            *d = 1000;
                            Err("operation timed out")
                        }
                    }
                }
            })
        };
        handle.join().unwrap()
    }
}

pub fn process_task(flag: bool) -> Result<u32, &'static str> {
    let engine = Engine {
        data: Arc::new(Mutex::new(0)),
    };
    engine.process(flag)
}

fn main() {
    let flag = true;
    match process_task(flag) {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}

/*
The test oracle below verifies the correct behavior for a timeout scenario.
When process_task(false) is called, the proper handling code should detect the timeout and return an Err with the message
"operation timed out". In the vulnerable version, the function incorrectly returns Ok(1000), causing the test to fail.
*/
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn oracle_test() {
         // Trigger the timeout scenario by passing 'false'
         let res = process_task(false);
         // Expect an error for the fixed code when a timeout occurs.
         assert!(res.is_err(), "Expected an error on timeout condition.");
         assert_eq!(res.unwrap_err(), "operation timed out", "Error message mismatch.");
    }
}
