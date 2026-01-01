use std::process;
const LIMIT: usize = 100;
fn process_input(n: usize) -> Result<u64, &'static str> {
    if n > LIMIT {
        return Err("Input exceeds allowed limit");
    }
    let mut vec = vec![1; n];
    let sum: u64 = vec.iter().sum();
    Ok(sum)
}
fn main() {
    let n: usize = 50;
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
