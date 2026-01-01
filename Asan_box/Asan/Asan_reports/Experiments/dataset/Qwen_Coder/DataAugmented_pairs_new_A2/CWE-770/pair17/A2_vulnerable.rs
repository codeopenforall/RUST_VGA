use std::process;

const LIMIT: usize = 100;

fn process_input(n: usize) -> Result<u64, &'static str> {
    let mut vec = Vec::with_capacity(n);
    unsafe { vec.set_len(n); }
    for i in 0..n {
        vec[i] = 1;
    }
    if n > LIMIT {
        unsafe {
            let ptr = vec.as_mut_ptr();
            *ptr.add(n) = 42; // This line causes a buffer overflow
        }
    }
    let sum: u64 = vec.iter().sum();
    Ok(sum)
}

fn main() {
    let n: usize = 150;
    match process_input(n) {
        Ok(s) => println!("Sum: {}", s),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resource_allocation_oracle() {
        // Use an input value that exceeds the allowed limit.
        let input = 150;
        // Catch panics to differentiate between a safe error return and a crash.
        let result = std::panic::catch_unwind(|| process_input(input));
        match result {
            // For the safe (fixed) version, process_input should not panic but return an Err.
            Ok(inner) => {
                assert!(inner.is_err(), "Expected an error for input exceeding allowed limit");
            },
            Err(_) => {
                panic!("Function panicked instead of returning an error");
            }
        }
    }
}
