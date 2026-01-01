use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};
use std::os::raw::{c_int};
extern "C" {
    fn sprintf(buffer: *mut i8, format: *const i8, ...) -> c_int;
}
struct Processor {
    buffer: Arc<Mutex<Vec<i8>>>,
}
impl Processor {
    fn new() -> Self {
        let buf = vec![0_i8; 256];
        Self {
            buffer: Arc::new(Mutex::new(buf)),
        }
    }
    fn process(&self, _fmt: &str) {
        let safe_fmt = CString::new("%s").expect("CString::new failed");
        let data_c = CString::new("data").expect("CString::new failed");
        let mut guard = self.buffer.lock().unwrap();
        unsafe {
            sprintf(guard.as_mut_ptr(), safe_fmt.as_ptr(), data_c.as_ptr());
        }
    }
    fn output(&self) -> String {
        let guard = self.buffer.lock().unwrap();
        unsafe { CStr::from_ptr(guard.as_ptr()).to_string_lossy().into_owned() }
    }
}
fn main() {
    let proc_inst = Processor::new();
    let shared_inst = Arc::new(proc_inst);
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let local = Arc::clone(&shared_inst);
            thread::spawn(move || {
                local.process("%s%s");
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
    println!("Output Buffer: {:?}", shared_inst.output());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_integrity() {
        // Create an instance of the Processor.
        let processor = Processor::new();
        
        // Pass a malicious format string with extra specifiers.
        // In the vulnerable version, this leads to unpredictable output.
        // In the corrected version, the external input is ignored and "data" is output.
        processor.process("%s%s");
        let result = processor.output();
        
        // The expected result is exactly "data", as the safe (fixed) implementation always
        // uses the constant "%s" format string, and thus extra specifiers have no effect.
        assert_eq!(result, "data", "Output buffer must exactly equal 'data'");
    }
}
