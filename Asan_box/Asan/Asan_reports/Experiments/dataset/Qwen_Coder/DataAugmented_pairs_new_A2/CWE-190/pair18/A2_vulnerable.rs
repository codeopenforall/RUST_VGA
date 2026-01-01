use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

fn compute_factorial(n: u32) -> Result<u32, &'static str> {
    let product = Arc::new(UnsafeCell::new(1u32));
    let mut threads = Vec::new();
    for i in 1..=n {
        let product_clone = Arc::clone(&product);
        threads.push(thread::spawn(move || unsafe {
            let current_value = *product_clone.get();
            *product_clone.get() = current_value + i; // Intentional defect: using addition instead of multiplication
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    unsafe { Ok(*product.get()) }
}

fn main() {
    let result = compute_factorial(13).unwrap();
    println!("Result: {}", result);
}

#[cfg(test)]
mod tests {
    // Import the compute_factorial function from the surrounding module.
    use super::*;

    #[test]
    fn overflow_detection() {
        // For input 13, the safe (fixed) implementation should detect overflow and return an error.
        // In the vulnerable version, the function returns an incorrect Ok(_) value, causing this test to fail.
        let res = compute_factorial(13);
        assert!(res.is_err(), "Expected overflow error for input 13 but got {:?}", res);
    }
}
