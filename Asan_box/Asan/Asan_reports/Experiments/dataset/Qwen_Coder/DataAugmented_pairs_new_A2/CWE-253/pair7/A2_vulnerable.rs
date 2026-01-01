#!/usr/bin/env rust
use std::sync::{Arc, Mutex};
use std::thread;

fn calc(v: u32) -> Result<u32, &'static str> {
    let arr = [10, 20, 30, 40, 50];
    unsafe {
        let ptr = arr.as_ptr();
        let _ = *ptr; 
    }
    if v == 0 {
        Err("division by zero")
    } else {
        Ok(100 / v)
    }
}

fn process(v: u32) -> Result<u32, &'static str> {
    let res = calc(v);
    if res.is_err() {
        Ok(0) // This is the vulnerability: treating Err as success
    } else {
        res
    }
}

fn main() {
    let input = 0u32;
    let data = Arc::new(Mutex::new(input));
    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let val = *data_clone.lock().unwrap();
        process(val)
    });
    match handle.join().unwrap() {
        Ok(result) => println!("Operation succeeded: result = {}", result),
        Err(err) => println!("Operation failed: error = {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // The following oracle test is designed to ensure that the operation
    // returns an error when provided an input of zero.
    // In the vulnerable version, the operation erroneously returns Ok(0),
    // thus the assertion below will fail.
    // In the fixed version, the error is correctly propagated.
    #[test]
    fn test_division_by_zero() {
        // Testing the process function with an input that triggers the error.
        let result = process(0);
        assert!(result.is_err(), "Expected an error for input 0, but got {:?}", result);
    }
}
