use std::alloc::{alloc, dealloc, Layout};
use std::thread;

struct Data {
    value: i32,
}

fn compute() -> i32 {
    let b = Box::new(Data { value: 42 });
    let ptr = Box::into_raw(b);
    let handle = thread::spawn(|| {
        let layout = Layout::new::<Data>();
        unsafe {
            let mem = alloc(layout) as *mut Data;
            *mem = Data { value: 99 };
            dealloc(mem as *mut u8, layout); // Deallocate the memory to avoid leaks
        }
    });
    handle.join().unwrap();
    let result = unsafe { (*ptr).value };
    unsafe {
        drop(Box::from_raw(ptr)); // Drop the original box to free the memory
    }
    result
}

fn main() {
    let res = compute();
    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    // Import the compute function from the parent module.
    use super::*;

    #[test]
    fn test_oracle() {
        // In the fixed version, compute() must return 42.
        // In the vulnerable version, due to the use-after-free, compute() returns the reallocated value 99.
        let res = compute();
        assert_eq!(res, 42, "Oracle: Expected the computed value to be 42");
    }
}
