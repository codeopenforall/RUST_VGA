use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;
use std::thread;
#[no_mangle]
pub extern "C" fn service(input: *const c_char) -> i32 {
    let result = panic::catch_unwind(|| {
        unsafe {
            let c_str = CStr::from_ptr(input);
            let mut s = c_str.to_string_lossy().into_owned();
            s.push_str("!");
            let _ = *s.as_ptr();
        }
        // Remove the internal panic to simulate unhandled panic
        // panic!("handled panic")
    });
    match result {
        Ok(_val) => 0,  
        Err(_)    => -1, 
    }
}
fn main() {
    let input = CString::new("test").unwrap();
    let handle = thread::spawn(move || unsafe { service(input.as_ptr()) });
    let ret = handle.join().unwrap();
    println!("Completed with return code: {}", ret);
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::panic;

    // Declaration for the external function.
    extern "C" {
        fn service(input: *const c_char) -> i32;
    }

    #[test]
    fn validate_service_error_handling() {
        let test_input = CString::new("oracle_test").unwrap();
        // Wrap the call in catch_unwind to detect if a panic occurs.
        let result = panic::catch_unwind(|| unsafe { service(test_input.as_ptr()) });
        // For the fixed implementation, the panic is caught internally and an error code is returned.
        // Thus, catch_unwind should see Ok(...) with the expected error code.
        assert!(result.is_ok(), "Service unexpectedly panicked without proper handling.");
        let ret_code = result.unwrap();
        assert_eq!(ret_code, -1, "Service did not return the expected error code.");
    }
}
