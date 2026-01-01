use std::process;

const LIMIT: usize = 100;

fn process_input(n: usize) -> Result<u64, &'static str> {
    if n > LIMIT {
        return Err("Input exceeds the allowed limit");
    }
    let mut vec = vec![1; n];
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